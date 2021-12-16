use super::parser::Parser;

pub fn parse_data(data: &[u8]) -> Packet {
    let mut parser = Parser::new(data);

    parse_packet(&mut parser)
}

pub fn parse_packet(parser: &mut Parser) -> Packet {
    let version: u8 = parser.get_bits(3);
    let type_id: u8 = parser.get_bits(3);

    match type_id {
        4 => {
            // Literal
            let mut value: i64 = 0;

            loop {
                let next: u8 = parser.get_bits(5);

                value <<= 4;
                value |= (next & 0x0f) as i64;

                if next & 0x10 == 0 {
                    break
                }
            }

            Packet::new(version, PacketType::Literal(value))
        }
        _ => {
            // Operator packet
            let length_type_id: u8 = parser.get_bits(1);

            let values = match length_type_id {
                0 => {
                    let mut values = Vec::new();
                    let tot_len: usize = parser.get_bits(15);
                    let end_pos = parser.get_pos() + tot_len;

                    while parser.get_pos() < end_pos {
                        values.push(parse_packet(parser));
                    }

                    values
                },
                1 => {
                    let sub_packets: u16 = parser.get_bits(11);

                    (0..sub_packets).map(|_| parse_packet(parser)).collect()
                },
                _ => panic!("Error parsing single bit")
            };

            Packet::new(version,
                match type_id {
                    0 => PacketType::Sum(values),
                    1 => PacketType::Product(values),
                    2 => PacketType::Min(values),
                    3 => PacketType::Max(values),
                    5 => PacketType::Gt(values),
                    6 => PacketType::Lt(values),
                    7 => PacketType::Eq(values),
                    _ => panic!("Unrecognised type ID")
                }
            )
        }
    }
}

type EvalNum = i64;

#[derive(Debug, PartialEq)]
pub enum PacketType {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(EvalNum),
    Gt(Vec<Packet>),
    Lt(Vec<Packet>),
    Eq(Vec<Packet>),
}

impl std::fmt::Display for PacketType {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let padding = f.width().unwrap_or(0);
        let alternate = f.alternate() || f.width().is_some();

        let joined = |values: &Vec<Packet>, pre: &str, post: &str, join: &str| {            
            if values.len() > 1 {
                if alternate {
                    let join_str = values.iter().map(|v| format!("{:>pad$}", v, pad = padding + 2)).collect::<Vec<String>>().join(join);
                    let pad1: String = vec![' '; padding].iter().collect();
                    let pad2: String = vec![' '; padding + 2].iter().collect();
                    format!("{}\n{}{}\n{}{}", pre, pad2, join_str, pad1, post)
                } else {
                    let join_str = values.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(join);
                    format!("{}{}{}", pre, join_str, post)
                }
            } else {
                format!("{}", values[0])
            }
        };

        let ternary = |values: &Vec<Packet>, op: &str| {
            let (v1, v2) = if alternate {
                (format!("{:>pad$}", values[0], pad = padding), format!("{:>pad$}", values[1], pad = padding))
            } else {
                (format!("{}", values[0]), format!("{}", values[1]))
            };

            format!("({} {} {} ? 1 : 0)", v1, op, v2)
        };

        let output = match self {
            PacketType::Sum(values) => joined(values, "(", ")", " + "),
            PacketType::Product(values) => joined(values, "(", ")", " * "),
            PacketType::Min(values) => joined(values, "min(", ")", ", "),
            PacketType::Max(values) => joined(values, "max(", ")", ", "),
            PacketType::Literal(num) => format!("{}", num),
            PacketType::Gt(values) => ternary(values, ">"),
            PacketType::Lt(values) => ternary(values, "<"),
            PacketType::Eq(values) => ternary(values, "=="),
        };

        write!(f, "{}", output)
    }

}

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub version: u8,
    pub content: PacketType
}

impl Packet {

    pub fn new(version: u8, content: PacketType) -> Self {
        Packet { version, content }
    }

    pub fn sum_versions(&self) -> EvalNum {
        self.version as EvalNum + match &self.content {
            PacketType::Sum(values) | 
            PacketType::Product(values) |
            PacketType::Min(values) |
            PacketType::Max(values) |
            PacketType::Gt(values) |
            PacketType::Lt(values) |
            PacketType::Eq(values)
                => values.iter().map(Packet::sum_versions).sum(),
            PacketType::Literal(_) => 0
        }
    }

    pub fn eval(&self) -> EvalNum {
        match &self.content {
            PacketType::Sum(values) => values.iter().map(Packet::eval).sum(),
            PacketType::Product(values) => values.iter().map(Packet::eval).product(),
            PacketType::Min(values) => values.iter().map(Packet::eval).min().unwrap(),
            PacketType::Max(values) => values.iter().map(Packet::eval).max().unwrap(),
            PacketType::Literal(num) => *num,
            PacketType::Gt(values) => if values[0].eval() > values[1].eval() { 1 } else { 0 },
            PacketType::Lt(values) => if values[0].eval() < values[1].eval() { 1 } else { 0 },
            PacketType::Eq(values) => if values[0].eval() == values[1].eval() { 1 } else { 0 },
        }
    }

}

impl std::fmt::Display for Packet {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let width = f.width();

        if f.alternate() || width.is_some() {
            write!(f, "{:>pad$}", self.content, pad = width.unwrap_or(0))
        } else {
            write!(f, "{}", self.content)
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    const TEST_INPUT1_1: &str = "D2FE28";
    const TEST_INPUT1_2: &str = "38006F45291200";
    const TEST_INPUT1_3: &str = "EE00D40C823060";
    const TEST_INPUT1_4: &str = "8A004A801A8002F478";
    const TEST_INPUT1_5: &str = "620080001611562C8802118E34";
    const TEST_INPUT1_6: &str = "C0015000016115A2E0802F182340";
    const TEST_INPUT1_7: &str = "A0016C880162017C3686B18A3D4780";

    const TEST_INPUT2_1: &str = "C200B40A82";
    const TEST_INPUT2_2: &str = "04005AC33890";
    const TEST_INPUT2_3: &str = "880086C3E88112";
    const TEST_INPUT2_4: &str = "CE00C43D881120";
    const TEST_INPUT2_5: &str = "D8005AC2A8F0";
    const TEST_INPUT2_6: &str = "F600BC2D8F";
    const TEST_INPUT2_7: &str = "9C005AC2F8F0";
    const TEST_INPUT2_8: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test1_1() {
        let data = load::load_buf(TEST_INPUT1_1.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "2021");

        assert_eq!(tree, Packet {
            version: 6, content: PacketType::Literal(2021)
        });
    }

    #[test]
    fn test1_2() {
        let data = load::load_buf(TEST_INPUT1_2.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(10 < 20 ? 1 : 0)");

        assert_eq!(tree, Packet {
            version: 1, content: PacketType::Lt(vec![
                Packet { version: 6, content: PacketType::Literal(10) },
                Packet { version: 2, content: PacketType::Literal(20) },
            ])
        });
    }

    #[test]
    fn test1_3() {
        let data = load::load_buf(TEST_INPUT1_3.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "max(1, 2, 3)");

        assert_eq!(tree, Packet {
            version: 7, content: PacketType::Max(vec![
                Packet { version: 2, content: PacketType::Literal(1) },
                Packet { version: 4, content: PacketType::Literal(2) },
                Packet { version: 1, content: PacketType::Literal(3) },
            ])
        });
    }

    #[test]
    fn test1_4() {
        let data = load::load_buf(TEST_INPUT1_4.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "15");

        assert_eq!(tree, Packet {
            version: 4, content: PacketType::Min(vec![
                Packet { version: 1, content: PacketType::Min(vec![
                    Packet { version: 5, content: PacketType::Min(vec![
                        Packet { version: 6, content: PacketType::Literal(15) },
                    ])}
                ])}
            ])
        });

        assert_eq!(tree.sum_versions(), 16);
    }

    #[test]
    fn test1_5() {
        let data = load::load_buf(TEST_INPUT1_5.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "((10 + 11) + (12 + 13))");

        assert_eq!(tree, Packet {
            version: 3, content: PacketType::Sum(vec![
                Packet { version: 0, content: PacketType::Sum(vec![
                    Packet { version: 0, content: PacketType::Literal(10) },
                    Packet { version: 5, content: PacketType::Literal(11) }
                ])},
                Packet { version: 1, content: PacketType::Sum(vec![
                    Packet { version: 0, content: PacketType::Literal(12) },
                    Packet { version: 3, content: PacketType::Literal(13)
                }])
            }])
        });

        assert_eq!(tree.sum_versions(), 12);
    }

    #[test]
    fn test1_6() {
        let data = load::load_buf(TEST_INPUT1_6.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "((10 + 11) + (12 + 13))");

        assert_eq!(tree, Packet {
            version: 6, content: PacketType::Sum(vec![
                Packet { version: 0, content: PacketType::Sum(vec![
                    Packet { version: 0, content: PacketType::Literal(10) },
                    Packet { version: 6, content: PacketType::Literal(11) }
                ])},
                Packet { version: 4, content: PacketType::Sum(vec![
                    Packet { version: 7, content: PacketType::Literal(12) },
                    Packet { version: 0, content: PacketType::Literal(13)
                }])
            }])
        });

        assert_eq!(tree.sum_versions(), 23);
    }

    #[test]
    fn test1_7() {
        let data = load::load_buf(TEST_INPUT1_7.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(6 + 6 + 12 + 15 + 15)");

        assert_eq!(tree, Packet {
            version: 5, content: PacketType::Sum(vec![
                Packet { version: 1, content: PacketType::Sum(vec![
                    Packet { version: 3, content: PacketType::Sum(vec![
                        Packet { version: 7, content: PacketType::Literal(6) },
                        Packet { version: 6, content: PacketType::Literal(6) },
                        Packet { version: 5, content: PacketType::Literal(12) },
                        Packet { version: 2, content: PacketType::Literal(15) },
                        Packet { version: 2, content: PacketType::Literal(15)
                    }])
                }])
            }])
        });

        assert_eq!(tree.sum_versions(), 31);
    }

    #[test]
    fn test2_1() {
        let data = load::load_buf(TEST_INPUT2_1.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(1 + 2)");
        assert_eq!(tree.eval(), 3);
    }

    #[test]
    fn test2_2() {
        let data = load::load_buf(TEST_INPUT2_2.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(6 * 9)");
        assert_eq!(tree.eval(), 54);
    }

    #[test]
    fn test2_3() {
        let data = load::load_buf(TEST_INPUT2_3.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "min(7, 8, 9)");
        assert_eq!(tree.eval(), 7);
    }

    #[test]
    fn test2_4() {
        let data = load::load_buf(TEST_INPUT2_4.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "max(7, 8, 9)");
        assert_eq!(tree.eval(), 9);
    }

    #[test]
    fn test2_5() {
        let data = load::load_buf(TEST_INPUT2_5.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(5 < 15 ? 1 : 0)");
        assert_eq!(tree.eval(), 1);
    }

    #[test]
    fn test2_6() {
        let data = load::load_buf(TEST_INPUT2_6.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(5 > 15 ? 1 : 0)");
        assert_eq!(tree.eval(), 0);
    }

    #[test]
    fn test2_7() {
        let data = load::load_buf(TEST_INPUT2_7.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "(5 == 15 ? 1 : 0)");
        assert_eq!(tree.eval(), 0);
    }

    #[test]
    fn test2_8() {
        let data = load::load_buf(TEST_INPUT2_8.as_bytes()).unwrap();
        let tree = parse_data(&data);

        assert_eq!(format!("{}", tree), "((1 + 3) == (2 * 2) ? 1 : 0)");
        assert_eq!(tree.eval(), 1);
    }

}
