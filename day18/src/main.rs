//#[macro_use] extern crate impl_ops;

mod load;
mod snailnum;

use std::error::Error;
use itertools::Itertools;
use crate::snailnum::SnailNum;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let lines = load::load_input("input18.txt")?;

    // Parse lines
    let numbers = parse_numbers(&lines)?;

    // Run parts
    part1(&numbers);
    part2(&numbers);

    Ok(())
}

fn part1(numbers: &[SnailNum]) {
    let sum = sum_numbers(numbers);

    println!("Part 1: Sum of numbers is: {}, magnitude {}", sum, sum.magnitude());
}

fn part2(numbers: &[SnailNum]) {
    let max_sum = max_sum(numbers);

    println!("Part 2: Maximum sum magnitude: {}", max_sum);
}

fn parse_numbers(lines: &[String]) -> Result<Vec<SnailNum>, Box<dyn Error>> {
    lines.iter().map(SnailNum::try_from).collect::<Result<Vec<SnailNum>, _>>()
}

fn sum_numbers(numbers: &[SnailNum]) -> SnailNum {
    let mut sum = numbers[0].clone();

    for n in numbers.iter().skip(1) {
        sum += n;
        sum.reduce();
    }

    sum
}

fn max_sum(numbers: &[SnailNum]) -> u32 {
    let magnitudes: Vec<u32> = numbers.iter().permutations(2).map(|pair| {
        let mut sum = pair[0] + pair[1];
        sum.reduce();
        sum.magnitude()
    }).collect();

    *magnitudes.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sum(expected_sum: &str, input: &str) {
        let lines = load::load_buf(input.as_bytes()).unwrap();
        let numbers = parse_numbers(&lines).unwrap();
        let sum = sum_numbers(&numbers);
        assert_eq!(format!("{}", sum), expected_sum);
    }

    #[test]
    fn test_sum1() {
        test_sum("[[[[1,1],[2,2]],[3,3]],[4,4]]", "\
[1,1]
[2,2]
[3,3]
[4,4]");
    }

    #[test]
    fn test_sum2() {
        test_sum("[[[[3,0],[5,3]],[4,4]],[5,5]]", "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]");
    }

    #[test]
    fn test_sum3() {
        test_sum("[[[[5,0],[7,4]],[5,5]],[6,6]]", "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]");
    }

    #[test]
    fn test_sum4() {
        test_sum("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]");
    }

    #[test]
    fn test_max_sum() {
        let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let lines = load::load_buf(input.as_bytes()).unwrap();
        let numbers = parse_numbers(&lines).unwrap();
        let max_sum = max_sum(&numbers);
        assert_eq!(3993, max_sum);
    }

}
