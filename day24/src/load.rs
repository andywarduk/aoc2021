use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use memmap2::Mmap;

use super::instruction::{Instruction, Reg, RegNum};

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

fn parse_reg_iter(iter: &mut dyn Iterator<Item = &str>) -> Reg {
    parse_reg(iter.next().unwrap()).unwrap()
}

fn parse_reg(string: &str) -> Option<Reg> {
    match string {
        "w" => Some(Reg::W),
        "x" => Some(Reg::X),
        "y" => Some(Reg::Y),
        "z" => Some(Reg::Z),
        _ => None
    }
}

fn parse_regnum_iter(iter: &mut dyn Iterator<Item = &str>) -> RegNum {
    let string = iter.next().unwrap();

    if let Some(reg) = parse_reg(string) {
        RegNum::Reg(reg)
    } else {
        RegNum::Num(string.parse::<i64>().unwrap())
    }
}

pub fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vectors
    let mut program = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        let mut split = line.split(' ');

        let ins = match split.next().unwrap() {
            "inp" => Instruction::Inp(parse_reg_iter(&mut split)),
            "add" => Instruction::Add(parse_reg_iter(&mut split), parse_regnum_iter(&mut split)),
            "mul" => Instruction::Mul(parse_reg_iter(&mut split), parse_regnum_iter(&mut split)),
            "div" => Instruction::Div(parse_reg_iter(&mut split), parse_regnum_iter(&mut split)),
            "mod" => Instruction::Mod(parse_reg_iter(&mut split), parse_regnum_iter(&mut split)),
            "eql" => Instruction::Eql(parse_reg_iter(&mut split), parse_regnum_iter(&mut split)),
            ins_str => panic!("Unrecognised instruction {}", ins_str)
        };

        program.push(ins);
    }

    Ok(program)
}
