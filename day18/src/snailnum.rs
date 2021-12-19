use std::error::Error as StdError;
use std::fmt;
use impl_ops::*;
use std::ops;

type SnailNumBase = u8;

#[derive(Debug, PartialEq, Eq, Clone)]
enum SnailNumToken {
    OpenBracket,
    ClosedBracket,
    Comma,
    Number(SnailNumBase)
}

impl From<&SnailNumToken> for String {

    fn from(token: &SnailNumToken) -> Self {
        match token {
            SnailNumToken::OpenBracket => "[".to_string(),
            SnailNumToken::ClosedBracket => "]".to_string(),
            SnailNumToken::Comma => ",".to_string(),
            SnailNumToken::Number(num) => format!("{}", num)
        }
    }

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnailNum {
    tokens: Vec<SnailNumToken>
}

impl SnailNum {

    pub fn reduce(&mut self) {
        self.explode();
        self.split();
    }

    fn explode(&mut self) {
        loop {
            let mut depth = 0;
            let mut item = None;

            for (i, tok) in self.tokens.iter().enumerate() {
                match tok {
                    SnailNumToken::OpenBracket => {
                        depth += 1;

                        if depth > 4 {
                            item = Some(i);
                            break
                        }
                    }
                    SnailNumToken::ClosedBracket => {
                        depth -= 1;
                    }
                    _ => {}
                }
            }

            if let Some(pos) = item {
                // Find pair length and numbers
                let mut pair_len = 0;
                let mut numbers = Vec::new();
                let mut depth = 0;

                for tok in self.tokens.iter().skip(pos) {
                    match tok {
                        SnailNumToken::OpenBracket => {
                            depth += 1;

                            if depth > 1 {
                                panic!("Nested brackets unexpected")
                            }
                        }
                        SnailNumToken::ClosedBracket => {
                            depth -= 1;

                            if depth == 0 {
                                break
                            }
                        }
                        SnailNumToken::Number(num) => numbers.push(num),
                        _ => {}
                    }

                    pair_len += 1;
                }

                // Find last number
                let last_num = self.tokens.iter().take(pos - 1).enumerate()
                    .fold(None, |acc, (i, tok)| {
                        if let SnailNumToken::Number(_) = tok { Some(i) } else { acc }
                    });

                // Build new token stream up to the replacement
                let start_iter = self.tokens.iter().take(pos).enumerate().map(|(i, tok)| {
                    match tok {
                        SnailNumToken::Number(num) => {
                            if let Some(pos) = last_num {
                                if i == pos {
                                    SnailNumToken::Number(num + numbers[0])
                                } else {
                                    tok.clone()
                                }
                            } else {
                                tok.clone()
                            }        
                        }
                        _ => tok.clone()
                    }
                });

                // Build replacement iterator
                let rep_iter = vec![SnailNumToken::Number(0)].into_iter();

                // Build remainder iterator
                let mut end_repl = false;
                let end_iter = self.tokens.iter().skip(pos + pair_len + 1).map(|tok| {
                    match tok {
                        SnailNumToken::Number(num) => {
                            if !end_repl {
                                end_repl = true;
                                SnailNumToken::Number(num + numbers[1])
                            } else {
                                tok.clone()
                            }        
                        }
                        _ => tok.clone()
                    }
                });

                self.tokens = start_iter.chain(rep_iter).chain(end_iter).collect();
            } else {
                break
            }
        }
    }

    fn split(&mut self) {
        loop {
            let mut item = None;

            for (i, tok) in self.tokens.iter().enumerate() {
                if let SnailNumToken::Number(num) = tok {
                    if *num >= 10 {
                        item = Some((i, *num));
                        break
                    }
                }
            }

            if let Some((pos, num)) = item {
                // Build new token stream up to the replacement
                let start_iter = self.tokens.iter().take(pos).cloned();

                // Build replacement iterator
                let left = num / 2;
                let right = num - left;

                let rep_iter = vec![
                    SnailNumToken::OpenBracket,
                    SnailNumToken::Number(left),
                    SnailNumToken::Comma,
                    SnailNumToken::Number(right),
                    SnailNumToken::ClosedBracket,
                ].into_iter();

                // Build remainder iterator
                let end_iter = self.tokens.iter().skip(pos + 1).cloned();

                self.tokens = start_iter.chain(rep_iter).chain(end_iter).collect();
                self.explode();
            } else {
                break
            }
        }
    }

    pub fn magnitude(&self) -> u32 {
        let mut mag: u32 = 0;
        let mut iter = self.tokens.iter();

        match iter.next() {
            Some(SnailNumToken::OpenBracket) => {
                mag = SnailNum::calc_bracket(&mut iter)
            }
            None => {}
            Some(tok) => panic!("Unexpected token {:?}", tok)
        }

        mag
    }

    fn calc_bracket(iter: &mut std::slice::Iter<SnailNumToken>) -> u32 {
        let num1 = match iter.next() {
            Some(SnailNumToken::OpenBracket) => SnailNum::calc_bracket(iter),
            Some(SnailNumToken::Number(num)) => *num as u32,
            Some(tok) => panic!("Unexpected token {:?}", tok),
            None => panic!("Unexpected end of token list")
        };

        match iter.next() {
            Some(SnailNumToken::Comma) => {}
            Some(tok) => panic!("Unexpected token {:?}", tok),
            None => panic!("Unexpected end of token list")
        }                        

        let num2 = match iter.next() {
            Some(SnailNumToken::OpenBracket) => SnailNum::calc_bracket(iter),
            Some(SnailNumToken::Number(num)) => *num as u32,
            Some(tok) => panic!("Unexpected token {:?}", tok),
            None => panic!("Unexpected end of token list")
        };

        match iter.next() {
            Some(SnailNumToken::ClosedBracket) => {}
            Some(tok) => panic!("Unexpected token {:?}", tok),
            None => panic!("Unexpected end of token list")
        }                        

        (num1 * 3) + (num2 * 2)
    }

    fn add_tokens(a: &SnailNum, b: &SnailNum) -> Vec<SnailNumToken> {
        let mut tokens = Vec::with_capacity(a.tokens.len() + b.tokens.len() + 3);

        tokens.push(SnailNumToken::OpenBracket);

        tokens.extend_from_slice(&a.tokens[..]);
        tokens.push(SnailNumToken::Comma);
        tokens.extend_from_slice(&b.tokens[..]);
        tokens.push(SnailNumToken::ClosedBracket);

        tokens
    }

}

impl std::fmt::Display for SnailNum {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let strings: Vec<String> = self.tokens.iter().map(String::from).collect();
        write!(f, "{}", strings.join(""))
    }

}

impl TryFrom<&String> for SnailNum {
    type Error = Box<dyn StdError>;

    fn try_from(item: &String) -> Result<Self, Self::Error> {
        SnailNum::try_from(&item[..])
    }

}

impl TryFrom<&str> for SnailNum {
    type Error = Box<dyn StdError>;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        let mut tokens = Vec::new();
        let mut chars = item.chars().enumerate();
        let mut num_start: Option<usize> = None;

        let flush_num = |end, tokens: &mut Vec<_>, num_start: &mut Option<usize>| {
            if let Some(start) = *num_start {
                tokens.push(SnailNumToken::Number((&item[start..end]).parse::<SnailNumBase>().unwrap()));
                *num_start = None;
            }
        };

        loop {
            match chars.next() {
                Some((pos, '[')) => {
                    flush_num(pos, &mut tokens, &mut num_start);
                    tokens.push(SnailNumToken::OpenBracket);
                }
                Some((pos, ']')) => {
                    flush_num(pos, &mut tokens, &mut num_start);
                    tokens.push(SnailNumToken::ClosedBracket);
                }
                Some((pos, ',')) => {
                    flush_num(pos, &mut tokens, &mut num_start);
                    tokens.push(SnailNumToken::Comma);
                }
                Some((pos, char)) if char.is_numeric() => {
                    if num_start.is_none() {
                        num_start = Some(pos)
                    }
                }
                None => break,
                _ => return Err(ParseError::InvalidChar.into())
            }
        }

        flush_num(item.len(), &mut tokens, &mut num_start);

        Ok(Self { tokens })
    }

}

impl_op_ex!(+= |a: &mut SnailNum, b: &SnailNum| {
    a.tokens = SnailNum::add_tokens(a, b);
});

impl_op_ex!(+ |a: &SnailNum, b: &SnailNum| -> SnailNum {
    SnailNum {
        tokens: SnailNum::add_tokens(a, b)
    }
});

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidChar
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ParseError::InvalidChar => write!(f, "invald character"),
        }
    }
}

impl StdError for ParseError {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_parse() {
        assert_eq!(SnailNum::try_from("[1,2]").unwrap(), SnailNum {
            tokens: vec![
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(1),
                SnailNumToken::Comma,
                SnailNumToken::Number(2),
                SnailNumToken::ClosedBracket
            ]
        });

        assert_eq!(SnailNum::try_from("[[1,2],3]").unwrap(), SnailNum {
            tokens: vec![
                SnailNumToken::OpenBracket,
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(1),
                SnailNumToken::Comma,
                SnailNumToken::Number(2),
                SnailNumToken::ClosedBracket,
                SnailNumToken::Comma,
                SnailNumToken::Number(3),
                SnailNumToken::ClosedBracket
            ]
        });

        assert_eq!(SnailNum::try_from("[1,[2,3]]").unwrap(), SnailNum {
            tokens: vec![
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(1),
                SnailNumToken::Comma,
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(2),
                SnailNumToken::Comma,
                SnailNumToken::Number(3),
                SnailNumToken::ClosedBracket,
                SnailNumToken::ClosedBracket
            ]
        });

        assert_eq!(SnailNum::try_from("[[1,2],[3,4]]").unwrap(), SnailNum {
            tokens: vec![
                SnailNumToken::OpenBracket,
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(1),
                SnailNumToken::Comma,
                SnailNumToken::Number(2),
                SnailNumToken::ClosedBracket,
                SnailNumToken::Comma,
                SnailNumToken::OpenBracket,
                SnailNumToken::Number(3),
                SnailNumToken::Comma,
                SnailNumToken::Number(4),
                SnailNumToken::ClosedBracket,
                SnailNumToken::ClosedBracket
            ]
        });
    }

    #[test]
    fn test_reduce() {
        let test_reduce_inner = |src: &str, expected: &str| {
            let mut num;
            
            num = SnailNum::try_from(src).unwrap();
            num.reduce();
            assert_eq!(format!("{}", num), expected);
        };

        test_reduce_inner("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_reduce_inner("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_reduce_inner("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_reduce_inner("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        test_reduce_inner("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add() {
        let num1 = SnailNum::try_from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let num2 = SnailNum::try_from("[1,1]").unwrap();
        let num3 = num1 + num2;

        assert_eq!(format!("{}", num3), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

        let mut num1 = SnailNum::try_from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let num2 = SnailNum::try_from("[1,1]").unwrap();

        num1 += num2;

        assert_eq!(format!("{}", num1), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    }

    #[test]
    fn test_magnitude() {
        let num = SnailNum::try_from("[9,1]").unwrap();
        assert_eq!(29, num.magnitude());

        let num = SnailNum::try_from("[1,9]").unwrap();
        assert_eq!(21, num.magnitude());

        let num = SnailNum::try_from("[[9,1],[1,9]]").unwrap();
        assert_eq!(129, num.magnitude());

        let num = SnailNum::try_from("[[1,2],[[3,4],5]]").unwrap();
        assert_eq!(143, num.magnitude());

        let num = SnailNum::try_from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(1384, num.magnitude());

        let num = SnailNum::try_from("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(445, num.magnitude());

        let num = SnailNum::try_from("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(791, num.magnitude());

        let num = SnailNum::try_from("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(1137, num.magnitude());

        let num = SnailNum::try_from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(3488, num.magnitude());
    }

}
