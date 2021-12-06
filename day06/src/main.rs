use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::collections::VecDeque;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let timers = load_input("input06.txt")?;

    // Run parts
    part1(&timers);
    part2(&timers);

    Ok(())
}

fn part1(timers: &[u8]) {
    let count = simulate_growth(timers, 80);

    println!("Part 1: Number of lantern fish after 80 days: {}", count);
}

fn part2(timers: &[u8]) {
    let count = simulate_growth(timers, 256);

    println!("Part 2: Number of lantern fish after 256 days: {}", count);
}

fn simulate_growth(timers: &[u8], days: usize) -> u64 {
    let mut counts: VecDeque<u64> = VecDeque::from([0; 9]);

    for t in timers {
        counts[*t as usize] += 1;
    }

    for _ in 0..days {
        let spawners = counts.pop_front().unwrap();
        counts[6] += spawners;
        counts.push_back(spawners);
    }

    counts.iter().sum()
}

fn load_input(file: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create timers vector
    let mut timers = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let mut nums: Vec<u8> = line
                .split(',')
                .map(|ns| ns.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()?;

            timers.append(&mut nums);
        }
    }
   
    Ok(timers)
}

#[test]
fn test_growth() {
    let timers = vec![3,4,3,1,2];

    let count = simulate_growth(&timers, 18);
    assert_eq!(26, count, "Count incorrect");

    let count = simulate_growth(&timers, 80);
    assert_eq!(5934, count, "Count incorrect");

    let count = simulate_growth(&timers, 256);
    assert_eq!(26984457539, count, "Count incorrect");
}
