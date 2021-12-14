use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::fmt;
use memmap2::Mmap;
use itertools::Itertools;
use num_format::{SystemLocale, ToFormattedString, Format};

fn main() -> Result<(), Box<dyn Error>> {
    let locale = SystemLocale::default().unwrap();

    // Load the input file
    let (polymer, subs) = load_input("input14.txt")?;

    // Run parts
    part1(&polymer, &subs, &locale);
    part2(&polymer, &subs, &locale);

    Ok(())
}

fn part1<F: Format>(polymer: &str, subs: &Subs, locale: &F) {
    let freq_map = run_sub(polymer, subs, 10);
    let (min_cnt, max_cnt) = freq_min_max(&freq_map);

    print!("Part 1: frequencies:");
    dump_freq_map(&freq_map, locale);
    println!();

    println!("        max {}, min {}, difference: {}",
        max_cnt.to_formatted_string(locale), min_cnt.to_formatted_string(locale),
        max_cnt - min_cnt);
}

fn part2<F: Format>(polymer: &str, subs: &Subs, locale: &F) {
    let freq_map = run_sub(polymer, subs, 40);
    let (min_cnt, max_cnt) = freq_min_max(&freq_map);

    print!("Part 2: frequencies:");
    dump_freq_map(&freq_map, locale);
    println!();

    println!("        max {}, min {}, difference: {}",
        max_cnt.to_formatted_string(locale), min_cnt.to_formatted_string(locale),
        max_cnt - min_cnt);
}

fn run_sub(polymer: &str, subs: &Subs, iters: usize) -> HashMap<char, usize> {
    let mut counts: HashMap<(char, char), usize> = HashMap::new();

    // Build initial counts hash map
    for (c1, c2) in polymer.chars().tuple_windows() {
        *counts.entry((c1, c2)).or_default() += 1;
    }

    for _ in 0..iters {
        let mut new_counts: HashMap<(char, char), usize> = HashMap::with_capacity(subs.len());

        // Build new counts hashmap
        for (pair, &count) in &counts {
            let &sub = subs.get(pair).unwrap();

            *new_counts.entry((pair.0, sub)).or_default() += count;
            *new_counts.entry((sub, pair.1)).or_default() += count;
        }

        counts = new_counts;
    }

    // Build character frequency map
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    
    *freq_map.entry(polymer.chars().next().unwrap()).or_default() += 1;

    for ((_, c2), count) in counts {
        *freq_map.entry(c2).or_default() += count;
    }

    freq_map
}

fn freq_min_max(freq_map: &HashMap<char, usize>) -> (usize, usize) {
    // Get min and max frequency entries
    let max_cnt = freq_map.iter().map(|(_, &cnt)| cnt).max().unwrap();
    let min_cnt = freq_map.iter().map(|(_, &cnt)| cnt).min().unwrap();

    (min_cnt, max_cnt)
}

fn dump_freq_map<F: Format>(freq_map: &HashMap<char, usize>, locale: &F) {
    for (&c, &cnt) in freq_map {
        print!(" {}={}", c, cnt.to_formatted_string(locale));
    }
}

type Subs = HashMap<(char, char), char>;
type ParseResult = (String, Subs);

#[derive(Debug, PartialEq)]
enum ParseError {
    ExpectArrow,
    Expect2SubSrcChars,
    Expect1SubDstChar,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ParseError::ExpectArrow => write!(f, "single arrow operator expected"),
            ParseError::Expect2SubSrcChars => write!(f, "two characters expected in substitution source"),
            ParseError::Expect1SubDstChar => write!(f, "one characters expected in substitution destination"),
        }
    }
}

impl Error for ParseError {
}

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

    // Create returns
    let mut polymer = String::new();
    let mut subs = HashMap::new();
    let mut in_subs = false;

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            if !in_subs { in_subs = true; }
            continue;
        }

        if in_subs {
            let parts: Vec<&str> = line.split(" -> ").collect();

            if parts.len() != 2 {
                return Err(ParseError::ExpectArrow.into());
            }

            if parts[0].len() != 2 {
                return Err(ParseError::Expect2SubSrcChars.into());
            }

            if parts[1].len() != 1 {
                return Err(ParseError::Expect1SubDstChar.into());
            }

            let src = (parts[0].chars().next().unwrap(), parts[0].chars().nth(1).unwrap());
            let dst = parts[1].chars().next().unwrap();

            subs.insert(src, dst);
        } else {
            polymer += &line;
        }
    }
   
    Ok((polymer, subs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_count() {
        let (polymer, subs) = load_buf(TEST_INPUT.as_bytes()).unwrap();

        let freqs = run_sub(&polymer, &subs, 10);
        let (min, max) = freq_min_max(&freqs);

        assert_eq!(min, 161);
        assert_eq!(max, 1749);

        let freqs = run_sub(&polymer, &subs, 40);
        let (min, max) = freq_min_max(&freqs);

        assert_eq!(min, 3849876073);
        assert_eq!(max, 2192039569602);
    }

}
