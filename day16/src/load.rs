use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use memmap2::Mmap;

type ParseResult = Vec<u8>;

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

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        let data = line.as_bytes().chunks(2).map(|pair| {
            if pair.len() == 1 {
                let s1 = [pair[0], b'0'];
                let s = std::str::from_utf8(&s1).unwrap();
                u8::from_str_radix(s, 16)
            } else {
                let s = std::str::from_utf8(pair).unwrap();
                u8::from_str_radix(s, 16)
            }
        }).collect::<Result<Vec<u8>, _>>()?;

        return Ok(data);
    }
   
    Err("No data found".into())
}
