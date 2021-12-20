use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use memmap2::Mmap;

use super::coord::{Coord, CoordVal};

type ParseResult = Vec<Vec<Coord>>;

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

    // Create report vector
    let mut reports = Vec::new();
    let mut report = Vec::new();

    let flush_scanner = |reports: &mut ParseResult, report: Vec<Coord>| {
        if !report.is_empty() {
            reports.push(report);
        }
    };
    
    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        if line.starts_with("--- scanner ") {
            flush_scanner(&mut reports, report);
            report = Vec::new();
        } else {
            let nums = line.split(',').map(|s| s.parse::<CoordVal>()).collect::<Result<Vec<CoordVal>, _>>()?;
            report.push(Coord::from(&nums));
        }
    }

    flush_scanner(&mut reports, report);

    Ok(reports)
}
