use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fmt;
use std::fs::File;
use memmap2::Mmap;

enum Action {
    Forward(u16),
    Down(u16),
    Up(u16)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let instructions = load_input("input02.txt")?;

    // Run parts
    part1(&instructions);
    part2(&instructions);

    Ok(())
}

struct State1 {
    distance: u16,
    depth: u16
}

fn part1(instructions: &[Action]) {
    let mut state = State1 {
        distance: 0,
        depth: 0
    };

    for action in instructions {
        match action {
            Action::Forward(amount) => { state.distance += amount }
            Action::Down(amount) => { state.depth += amount}
            Action::Up(amount) => { state.depth -= amount }
        }
    }

    println!("Part 1: distance {}, depth {} => {}", state.distance, state.depth,
        state.distance as u32 * state.depth as u32);
}

struct State2 {
    distance: u16,
    depth: u32,
    aim: u16
}

fn part2(instructions: &[Action]) {
    let mut state = State2 {
        distance: 0,
        depth: 0,
        aim: 0
    };

    for action in instructions {
        match action {
            Action::Forward(amount) => {
                state.distance += amount;
                state.depth += state.aim as u32 * *amount as u32;
            }
            Action::Down(amount) => { state.aim += amount}
            Action::Up(amount) => { state.aim -= amount }
        }
    }

    println!("Part 2: distance {}, depth {} => {}", state.distance, state.depth,
        state.distance as u32 * state.depth);
}

#[derive(Debug)]
enum ParseError {
    Expect2Terms,
    UnknownAction(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Expect2Terms => write!(f, "two terms expected in action"),
            ParseError::UnknownAction(action) => write!(f, "action {} is unrecognised", action)
        }
    }
}

impl Error for ParseError {
}

fn load_input(file: &str) -> Result<Vec<Action>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create actions vector
    let mut actions = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let parts: Vec<&str> = line.split(' ').collect();

            if parts.len() != 2 {
                return Err(ParseError::Expect2Terms.into());
            }

            let amount = parts[1].parse::<u16>()?;

            let action = match parts[0] {
                "forward" => Action::Forward(amount),
                "up" => Action::Up(amount),
                "down" => Action::Down(amount),
                _ => return Err(ParseError::UnknownAction(parts[0].to_string()).into())
            };

            actions.push(action);
        }
    }
   
    Ok(actions)
}
