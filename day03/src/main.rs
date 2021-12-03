use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fmt;
use std::fs::File;
use memmap2::Mmap;


fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let bits = load_input("input03.txt")?;

    // Run parts
    part1(&bits);
    part2(&bits);

    Ok(())
}

fn part1(bits: &[Vec<bool>]) {
    let (gamma, epsilon)  = calc_epsilon_gamma(bits);

    println!("Part 1: gamma {}, epsilon {} => {}", gamma, epsilon, gamma * epsilon);
}

fn part2(bits: &[Vec<bool>]) {
    let (oxy, co2) = calc_oxy_co2(bits);

    println!("Part 2: oxygen {}, co2 {} => {}", oxy, co2, oxy * co2);
}

fn true_counts(bits: &[Vec<bool>]) -> Vec<usize> {
    let bit_count = bits[0].len();

    let mut true_count = vec![0; bit_count];

    for ent in bits {
        for (elem, val) in ent.iter().enumerate() {
            if *val { true_count[elem] += 1 }
        }
    }

    true_count
}

fn calc_common(true_count: &[usize], threshold: usize, greater: bool) -> usize {
    let count = true_count.len() - 1;

    true_count.iter().enumerate().fold(0, |acc, (elem, &val)| {
        let bit_val = 1 << (count - elem);
        let mut add = 0;

        if val >= threshold {
            if greater {
                add = bit_val;
            }
        } else if !greater {
            add = bit_val;
        }

        acc + add
    })
}

fn calc_epsilon_gamma(bits: &[Vec<bool>]) -> (usize, usize) {
    let val_count = bits.len();
    let threshold = val_count / 2;

    let true_count = true_counts(bits);

    let gamma = calc_common(&true_count, threshold, true);
    let epsilon = calc_common(&true_count, threshold, false);

    (gamma, epsilon)
}

fn get_common_bits(bits: &[Vec<bool>], most: bool) -> usize {
    let ent = reduce_common_bits(bits.to_vec().clone(), 0, most);

    let count = ent.len() - 1;

    ent.iter().enumerate().fold(0, |acc, (elem, &val)| {
        if val {
            acc + (1 << (count - elem))
        } else {
            acc
        }
    })
}

fn true_count(bits: &[Vec<bool>], bit_pos: usize) -> usize {
    bits.iter().filter(|e| e[bit_pos]).count()
}

fn reduce_common_bits(bits: Vec<Vec<bool>>, bit_pos: usize, most: bool) -> Vec<bool> {
    let true_count = true_count(&bits, bit_pos);
    let filter: bool;

    if true_count >= (bits.len() - true_count) {
        // Mostly ones
        filter = most;
    } else {
        // Mostly zeroes
        filter = !most;
    }

    let next: Vec<Vec<bool>> = bits.iter().filter(|e| e[bit_pos] == filter).cloned().collect();

    if next.len() == 1 {
        next[0].clone()
    } else {
        reduce_common_bits(next, bit_pos + 1, most)
    }
}

fn calc_oxy_co2(bits: &[Vec<bool>]) -> (usize, usize) {
    let oxy = get_common_bits(bits, true);
    let co2 = get_common_bits(bits, false);

    (oxy, co2)
}

#[derive(Debug)]
enum ParseError {
    NotZeroOrOne,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ParseError::NotZeroOrOne => write!(f, "'0' or '1' expected'"),
        }
    }
}

impl Error for ParseError {
}

fn load_input(file: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create bits vector
    let mut bits = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let bits_ent: Result<Vec<bool>, ParseError> = line.chars().map(|c| {
                match c {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err(ParseError::NotZeroOrOne)
                }
            }).collect();

            bits.push(bits_ent?);
        }
    }
   
    Ok(bits)
}

#[cfg(test)]
const EXAMPLE_BUF: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

#[test]
fn example_part1() {
    let example = load_buf(EXAMPLE_BUF.as_bytes()).unwrap();

    let (gamma, epsilon)  = calc_epsilon_gamma(&example);

    assert_eq!(22, gamma, "gamma incorrect");
    assert_eq!(9, epsilon, "epsilon incorrect");
}

#[test]
fn example_part2() {
    let example = load_buf(EXAMPLE_BUF.as_bytes()).unwrap();

    let (oxy, co2)  = calc_oxy_co2(&example);

    assert_eq!(23, oxy, "oxy incorrect");
    assert_eq!(10, co2, "co2 incorrect");
}
