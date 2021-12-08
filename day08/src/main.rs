use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let (signals, digits) = load_input("input08.txt")?;

    // Run parts
    part1(&digits);
    part2(&signals, &digits);

    Ok(())
}

fn part1(digits: &[Vec<String>]) {
    let count = count_known_digits(digits);

    println!("Part 1: Known digits in output: {}", count);
}

fn count_known_digits(digits: &[Vec<String>]) -> u16 {
    digits.iter().fold(0, |acc, dc| {
        acc + dc.iter().fold(0, |acc, d| {
            acc + match d.len() {
                2 => 1,
                4 => 1,
                3 => 1,
                7 => 1,
                _ => 0
            }
        })
    })
}

fn part2(signals: &[Vec<String>], digits: &[Vec<String>]) {
    let mut sum: u32 = 0;

    for elem in 0..signals.len() {
        let signal = &signals[elem];
        let digits = &digits[elem];

        let segments = deduce_segments(signal);
        let numbers = convert_digits(digits, &segments);
        let number = digits_to_number(&numbers);
        sum += number;
    }

    println!("Part 2: Sum of numbers: {}", sum);
}

// Segment deduction

//  1111
// 2    3
// 2    3
//  4444
// 5    6
// 5    6
//  7777

// Digits to segments:
// 0 - 1 2 3   5 6 7 (6)
// 1 -     3     6   (2)
// 2 - 1   3 4 5   7 (5)
// 3 - 1   3 4   6 7 (5)
// 4 -   2 3 4   6   (4)
// 5 - 1 2   4   6 7 (5)
// 6 - 1 2   4 5 6 7 (6)
// 7 - 1   3     6   (3)
// 8 - 1 2 3 4 5 6 7 (7)
// 9 - 1 2 3 4   6 7 (6)

// A: find signal with length 2 to get 3 & 6
// B: find signal with length 3. Subtract (A) to find segment 1
// C: find signal with length 4. Subtract (A) to C1
// D: find signal with length 6 containing all of (C), digit 9. Segment 5 can be deduced (not present)
// E: find signal with length 6 containing C1 and not all of A, digit 5. Segments 3 and 6 can be deduced
// F: find signal with length 6 containing A and not all of C1, digit 0. Segments 2 and 4 can be deduced
// G: Only segment 7 left

fn deduce_segments(signals: &[String]) -> [char; 7] {
    let a = signals.iter().find(|s| s.len() == 2).unwrap();

    let seg1 = {
        let b1 = signals.iter().find(|s| s.len() == 3).unwrap();
        char_not_in(b1, a)
    };

    let c = signals.iter().find(|s| s.len() == 4).unwrap();
    let c1 = str_not_in(c, a);

    let seg5 = {
        let d1 = signals.iter()
            .find(|s| s.len() == 6 && str_contains(s, c))
            .unwrap();
        char_not_in("abcdefg", d1)
    };

    let (seg3, seg6) = {
        let e1 = signals.iter().find(|s| s.len() == 6 && str_contains(s, &c1) && !str_contains(s, a)).unwrap();
        let e2 = common_char(a, e1);
        (char_not_in(a, &String::from(e2)), e2)
    };

    let (seg2, seg4) = {
        let f1 = signals.iter().find(|s| s.len() == 6 && str_contains(s, a) && !str_contains(s, &c1)).unwrap();
        let f2 = common_char(&c1, f1);
        (f2, char_not_in(&c1, &String::from(f2)))
    };

    let seg7 = {
        let g1: String = vec![seg1, seg2, seg3, seg4, seg5, seg6].into_iter().collect();
        char_not_in("abcdefg", &g1)
    };

    // println!("{} {} {} {} {} {} {} {} {} {}", a, seg1, c, c1, seg5, seg3, seg6, seg2, seg4, seg7);

    [seg1, seg2, seg3, seg4, seg5, seg6, seg7]
}

fn char_not_in(s1: &str, s2: &str) -> char {
    let c1: HashSet<_> = s1.chars().collect();
    let c2: HashSet<_> = s2.chars().collect();

    let diff: Vec<&char> = c1.difference(&c2).collect();

    assert!(diff.len() == 1);

    *diff[0]
}

fn str_not_in(s1: &str, s2: &str) -> String {
    let c1: HashSet<_> = s1.chars().collect();
    let c2: HashSet<_> = s2.chars().collect();

    let diff: String = c1.difference(&c2).collect();

    diff
}

fn str_contains(s1: &str, s2: &str) -> bool {
    let c1: HashSet<_> = s1.chars().collect();
    let c2: HashSet<_> = s2.chars().collect();

    c1.is_superset(&c2)
}

fn common_char(s1: &str, s2: &str) -> char {
    let c1: HashSet<_> = s1.chars().collect();
    let c2: HashSet<_> = s2.chars().collect();

    let intersection: Vec<&char> = c1.intersection(&c2).collect();

    assert!(intersection.len() == 1);

    *intersection[0]
}

const NUM_SEGMENTS: [u8; 10] = [
    0b1110111, // 0
    0b0100100, // 1
    0b1011101, // 2
    0b1101101, // 3
    0b0101110, // 4
    0b1101011, // 5
    0b1111011, // 6
    0b0100101, // 7
    0b1111111, // 8
    0b1101111, // 9
];

fn convert_digits(digits: &[String], segments: &[char; 7]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(digits.len());

    for digit in digits {
        let find_digit = digit
            .chars()
            .map(|c| segments.iter().position(|&s| s == c).unwrap())
            .fold(0u8, |acc, seg| acc + (1 << seg));

        let digit = NUM_SEGMENTS.iter().position(|&segs| segs == find_digit).unwrap() as u8;

        result.push(digit);
    }

    result
}

fn digits_to_number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (elem, &digit)| {
            acc + (digit as u32 * (10u32.pow(elem as u32)))
        })
}

type ParseResult = (Vec<Vec<String>>, Vec<Vec<String>>);

fn load_input(file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vectors
    let mut signals = Vec::new();
    let mut digits = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let parts: Vec<&str> = line.split(" | ").collect();

            let signal: Vec<String> = parts[0].split_whitespace().map(String::from).collect();
            let digit: Vec<String> = parts[1].split_whitespace().map(String::from).collect();

            signals.push(signal);
            digits.push(digit);
        }
    }
   
    Ok((signals, digits))
}

#[test]
fn test_part2() {
    let (signals, digits) = load_buf("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".as_bytes()).unwrap();

    let segments = deduce_segments(&signals[0]);

    assert_eq!(segments, ['d', 'e', 'a', 'f', 'g', 'b', 'c']);

    let numbers = convert_digits(&digits[0], &segments);

    assert_eq!(numbers, [5, 3, 5, 3]);

    let number = digits_to_number(&numbers);

    assert_eq!(5353, number);
}
