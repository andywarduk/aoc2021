use std::ops::{ShlAssign, BitOrAssign};

pub struct Parser<'a> {
    data: &'a [u8],
    cur_pos: usize
}

impl<'a> Parser<'a> {

    pub fn new(data: &'a [u8]) -> Self {
        Parser { data, cur_pos: 0 }
    }

    pub fn get_pos(&self) -> usize {
        self.cur_pos
    }

    pub fn get_bits<B>(&mut self, count: u8) -> B
        where B: BitOrAssign + ShlAssign + From<u8>
    {
        let mut cur_byte = self.cur_pos >> 3;
        let mut cur_bits: u8 = 8 - (self.cur_pos & 0x07) as u8;

        let mut result: B = 0.into();

        let mut left = count;

        while left > 0 {
            let take_bits = std::cmp::min(left, cur_bits);
            let take_mask = ((1u16 << take_bits as u16) - 1) as u8;
            let shift = cur_bits - take_bits;

            let add = (self.data[cur_byte] >> shift) & take_mask;

            result <<= take_bits.into();
            result |= add.into();

            left -= take_bits;
            cur_bits = 8;
            cur_byte += 1;
        }

        self.cur_pos += count as usize;

        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let data = [
            0b10101010, 0b10101010, 0b10101010, 0b10101010,
            0b10101010, 0b10101010, 0b10101010, 0b10101010,
            0b10101010
        ];

        let mut parser = Parser::new(&data);

        assert_eq!(parser.get_bits::<u8>(1),   0b1);
        assert_eq!(parser.get_bits::<u8>(2),   0b01);
        assert_eq!(parser.get_bits::<u8>(3),   0b010);
        assert_eq!(parser.get_bits::<u8>(4),   0b1010);
        assert_eq!(parser.get_bits::<u8>(5),   0b10101);
        assert_eq!(parser.get_bits::<u8>(6),   0b010101);
        assert_eq!(parser.get_bits::<u8>(7),   0b0101010);
        assert_eq!(parser.get_bits::<u8>(8),   0b10101010);
        assert_eq!(parser.get_bits::<u16>(9),  0b101010101);
        assert_eq!(parser.get_bits::<u16>(10), 0b0101010101);
        assert_eq!(parser.get_bits::<u16>(11), 0b01010101010);
    }
    
}
