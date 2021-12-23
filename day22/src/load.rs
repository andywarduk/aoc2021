use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use memmap2::Mmap;

use super::cube::Cube;
use super::instruction::Instruction;

type ParseResult = Vec<Instruction>;

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
    let mut reboot = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        let mut split1 = line.split(' ');

        // TODO remove unwraps

        let mut cube = Cube::default();

        let on = match split1.next().unwrap() {
            "on" => true,
            "off" => false,
            _ => panic!("on or off expected")
        };

        let mut axis = 0;
        split1.next().unwrap().split(',').for_each(|s| {
            let range_str = s.split('=').nth(1).unwrap();

            let range: Vec<i32> = range_str.split("..").map(|e| e.parse()).collect::<Result<_, _>>().unwrap();

            cube.ranges[axis] = range[0]..range[1];
            axis += 1;
        });

        reboot.push(Instruction {
            on,
            cube
        });
    }

    Ok(reboot)
}
