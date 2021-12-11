use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let energy = load_input("input11.txt")?;

    // Run parts
    part1(&energy);
    part2(&energy);

    Ok(())
}

fn part1(energy: &[Vec<u8>]) {
    let flashes = count_flashes(energy, 100);

    println!("Part 1: Total flashes: {}", flashes);
}

fn part2(energy: &[Vec<u8>]) {
    let all_flash_step = find_sync_step(energy);
    
    println!("Part 2: All flash on step: {}", all_flash_step);
}

fn count_flashes(energy: &[Vec<u8>], steps: usize) -> usize {
    let mut total = 0;

    let mut loc_energy = energy.to_vec();
    
    for _ in 0..steps {
        total += step(&mut loc_energy);
    }

    total
}

fn find_sync_step(energy: &[Vec<u8>]) -> usize {
    let width = energy[0].len();
    let height = energy.len();
    let all_flash_cnt = width * height;

    let mut step_cnt = 0;
    let mut loc_energy = energy.to_vec();

    loop {
        step_cnt += 1;

        if step(&mut loc_energy) == all_flash_cnt {
            break
        }
    }

    step_cnt
}

fn step(energy: &mut [Vec<u8>]) -> usize {
    let mut flashers: Vec<(usize, usize)> = Vec::new();

    // Increase energy
    for (y, line) in energy.iter_mut().enumerate() {
        for (x, e) in line.iter_mut().enumerate() {
            *e += 1;

            if *e > 9 {
                flashers.push((x, y));
            }
        }
    }

    // Process flashes
    while let Some((x, y)) = flashers.pop() {
        let (yskip, ytake) = if y == 0 { (0, 2) } else { (y - 1, 3) };
        let (xskip, xtake) = if x == 0 { (0, 2) } else { (x - 1, 3) };

        for (y, line) in energy.iter_mut().enumerate().skip(yskip).take(ytake) {
            for (x, e) in line.iter_mut().enumerate().skip(xskip).take(xtake) {
                if *e < 10 {
                    *e += 1;

                    if *e > 9 {
                        flashers.push((x, y));
                    }
                }
            }
        }
    }

    // Count and reset flashers
    let mut flash_cnt = 0;

    for line in energy.iter_mut() {
        for e in line.iter_mut() {
            if *e > 9 {
                *e = 0;
                flash_cnt += 1;
            }
        }
    }

    flash_cnt
}

type ParseResult = Vec<Vec<u8>>;

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
    let mut energy = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            energy.push(line.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>());
        }
    }
   
    Ok(energy)
}

#[test]
fn test_step() {
    let energy_input = "\
11111
19991
19191
19991
11111";

    let mut energy = load_buf(energy_input.as_bytes()).unwrap();

    let flash_cnt = step(&mut energy);

    assert_eq!(flash_cnt, 9);

    assert_eq!(energy, vec![
        vec![3, 4, 5, 4, 3],
        vec![4, 0, 0, 0, 4],
        vec![5, 0, 0, 0, 5],
        vec![4, 0, 0, 0, 4],
        vec![3, 4, 5, 4, 3],
    ]);

    let flash_cnt = step(&mut energy);

    assert_eq!(flash_cnt, 0);

    assert_eq!(energy, vec![
        vec![4, 5, 6, 5, 4],
        vec![5, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 6],
        vec![5, 1, 1, 1, 5],
        vec![4, 5, 6, 5, 4],
    ]);
}
