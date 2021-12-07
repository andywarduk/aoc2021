#[macro_use] extern crate cached;

use std::collections::hash_map::DefaultHasher;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use memmap2::Mmap;
use cached::SizedCache;
use cached::proc_macro::cached;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let positions = load_input("input07.txt")?;

    // Run parts
    part1(&positions);
    part2(&positions);

    Ok(())
}

fn part1(positions: &[u16]) {
    let (pos, fuel) = calculate_hpos1(positions);

    println!("Part 1: Optimum position {}, fuel used {}", pos, fuel);
}

fn part2(positions: &[u16]) {
    let (pos, fuel) = calculate_hpos2(positions);

    println!("Part 2: Optimum position {}, fuel used {}", pos, fuel);
}

fn array_hash(positions: &[u16]) -> u64 {
    let mut hasher = DefaultHasher::new();
    positions.hash(&mut hasher);
    hasher.finish()
}

fn calculate_hpos1(positions: &[u16]) -> (u16, u32) {
    // Calculate the integer mean position
    let sum: u32 = positions.iter().map(|p| *p as u32).sum();

    let mean = sum / positions.len() as u32;

    // Calculate the array hash
    let pos_hash = array_hash(positions);

    // Calculate the optimum position
    let mut test_pos = mean as u16;

    let mut test_fuel = calculate_distance1(positions, pos_hash, test_pos);

    loop {
        let test_down = calculate_distance1(positions, pos_hash, test_pos - 1);
        let test_up = calculate_distance1(positions, pos_hash, test_pos + 1);

        if test_down < test_fuel {
            test_pos -= 1;
            test_fuel = test_down;
        } else if test_up < test_fuel {
            test_pos += 1;
            test_fuel = test_up;
        } else {
            break
        }
    }

    (test_pos, test_fuel)
}

cached_key!{
    DIST1: SizedCache<String, u32> = SizedCache::with_size(10);
    Key = { format!("{}{}", pos_hash, from) };

    fn calculate_distance1(positions: &[u16], pos_hash: u64, from: u16) -> u32 = {
        positions.iter().map(|p| (from as i32 - *p as i32).abs() as u32).sum::<u32>()
    }
}

fn calculate_hpos2(positions: &[u16]) -> (u16, u32) {
    // Calculate the integer mean position
    let sum: u32 = positions.iter().map(|p| *p as u32).sum();

    let mean = sum / positions.len() as u32;

    // Calculate the array hash
    let pos_hash = array_hash(positions);

    // Calculate the optimum position
    let mut test_pos = mean as u16;

    let mut test_fuel = calculate_distance2(positions, pos_hash, test_pos);

    loop {
        let test_down = calculate_distance2(positions, pos_hash, test_pos - 1);
        let test_up = calculate_distance2(positions, pos_hash, test_pos + 1);

        if test_down < test_fuel {
            test_pos -= 1;
            test_fuel = test_down;
        } else if test_up < test_fuel {
            test_pos += 1;
            test_fuel = test_up;
        } else {
            break
        }
    }

    (test_pos, test_fuel)
}

#[cached]
fn fuel2(dist: u32) -> u32 {
    match dist {
        0 => 0,
        1 => 1,
        dist => dist + fuel2(dist - 1)
    }
}

cached_key!{
    DIST2: SizedCache<String, u32> = SizedCache::with_size(10);
    Key = { format!("{}{}", pos_hash, from) };

    fn calculate_distance2(positions: &[u16], pos_hash: u64, from: u16) -> u32 = {
        positions.iter().map(|p| {
            let dist = (from as i32 - *p as i32).abs() as u32;
            fuel2(dist)
        }).sum::<u32>()
    }
}

fn load_input(file: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Vec<u16>, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create position vector
    let mut positions = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let mut nums: Vec<u16> = line
                .split(',')
                .map(|ns| ns.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()?;

            positions.append(&mut nums);
        }
    }
   
    Ok(positions)
}

#[test]
fn test_part1() {
    let positions = vec![16,1,2,0,4,2,7,1,2,14];

    let (pos, fuel) = calculate_hpos1(&positions);

    assert_eq!(2, pos, "Optimum position incorrect");
    assert_eq!(37, fuel, "Fuel used incorrect");
}

#[test]
fn test_part2() {
    let positions = vec![16,1,2,0,4,2,7,1,2,14];

    let (pos, fuel) = calculate_hpos2(&positions);

    assert_eq!(5, pos, "Optimum position incorrect");
    assert_eq!(168, fuel, "Fuel used incorrect");
}
