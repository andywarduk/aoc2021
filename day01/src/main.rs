use std::io::{BufRead, BufReader};
use memmap2::Mmap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the input file
    let depths = load_input("input01.txt")?;

    // Run parts
    part1(&depths);
    part2(&depths);

    Ok(())
}

fn part1(depths: &Vec<u16>) {
    let greater = depths.windows(2).fold(0, |acc, slice| {
        if slice[1] > slice[0] {
            acc + 1
        } else {
            acc
        }
    });

    println!("Number of individual depths greater than the last: {}", greater);
}

fn part2(depths: &Vec<u16>) {
    let window_sums: Vec<u16> = depths.windows(3).map(|slice| slice.iter().sum()).collect();

    let greater = window_sums.windows(2).fold(0, |acc, slice| {
        if slice[1] > slice[0] {
            acc + 1
        } else {
            acc
        }
    });

    println!("Number of sliding window depths greater than the last: {}", greater);
}

fn load_input(file: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create depths vector
    let mut depths = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line != "" {
            depths.push(line.parse::<u16>()?);
        }
    }
   
    Ok(depths)
}
