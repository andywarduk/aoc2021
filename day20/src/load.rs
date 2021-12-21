use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use memmap2::Mmap;

type ParseResult = (Vec<bool>, Vec<Vec<bool>>);

pub fn load_input(file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

pub fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vectors
    let mut algo = Vec::new();
    let mut got_algo = false;

    let mut raw_image = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        let trans_line = line.chars().map(|c| {
            match c {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid character {}", c)
            }
        }).collect();

        if !got_algo {
            algo = trans_line;
            got_algo = true;
        } else {
            raw_image.push(trans_line);
        }
    }

    Ok((algo, raw_image))
}
