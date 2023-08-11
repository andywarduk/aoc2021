use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::collections::HashSet;
use std::cmp::max;
use std::fmt;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let (dots, folds) = load_input("input13.txt")?;

    // Run parts
    part1(&dots, &folds);
    part2(&dots, &folds);

    Ok(())
}

fn part1(dots: &[Coord], folds: &[Fold]) {
    let mut work_dots: HashSet<Coord> = HashSet::from_iter(dots.iter().cloned());

    work_dots = fold_page(work_dots, &folds[0]);

    println!("Part 1: Number of dots visible after first fold: {}", work_dots.len());
}

fn part2(dots: &[Coord], folds: &[Fold]) {
    let mut work_dots: HashSet<Coord> = HashSet::from_iter(dots.iter().cloned());

    for fold in folds {
        work_dots = fold_page(work_dots, fold);
    }

    let max = work_dots.iter().fold((0, 0), |(maxx, maxy), dot| {
        (max(maxx, dot.x), max(maxy, dot.y))
    });

    println!("Part 2:");

    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if work_dots.contains(&Coord {x, y}) {
                print!("\u{2588}")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn fold_page(dots: HashSet<Coord>, fold: &Fold) -> HashSet<Coord> {
    let calc_fold = |pos: u16, coord: u16| if coord > pos { pos - (coord - pos) } else { coord };

    let new_dots: HashSet<Coord> = dots.into_iter().map(|coord| {
        match fold {
            Fold::XAxis(pos) => Coord::new(calc_fold(*pos, coord.x), coord.y),
            Fold::YAxis(pos) => Coord::new(coord.x, calc_fold(*pos, coord.y))
        }
    }).collect();

    new_dots
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u16,
    y: u16
}

impl Coord {

    fn new(x: u16, y: u16) -> Self {
        Coord {x, y}
    }
    
}

#[derive(Debug, PartialEq)]
enum Fold {
    XAxis(u16),
    YAxis(u16),
}

impl Fold {

    fn new(axis: char, pos: u16) -> Self {
        match axis {
            'x' => Fold::XAxis(pos),
            'y' => Fold::YAxis(pos),
            _ => panic!("Invalid axis")
        }
    }

}

type ParseResult = (Vec<Coord>, Vec<Fold>);

#[derive(Debug, PartialEq)]
enum ParseError {
    Expect2Coords,
    Expect2FoldTerms,
    ExpectFoldAlong,
    InvalidAxis(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Expect2Coords => write!(f, "two coordinates expected in dot position"),
            ParseError::Expect2FoldTerms => write!(f, "two terms expected in fold position"),
            ParseError::ExpectFoldAlong => write!(f, "expecting line to start 'fold along '"),
            ParseError::InvalidAxis(string) => write!(f, "Axis '{}' is invalid", string),
        }
    }
}

impl Error for ParseError {
}

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
    let mut coords = Vec::new();
    let mut folds = Vec::new();

    let mut in_coords = true;

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            if in_coords { in_coords = false };
            continue
        }

        if in_coords {
            let coord: Vec<u16> = line
                .split(',')
                .map(|n| n.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()?;

            if coord.len() != 2 {
                return Err(ParseError::Expect2Coords.into());
            }

            coords.push(Coord::new(coord[0], coord[1]));

        } else {
            if line.len() < 11 || line[..11] != *"fold along " {
                return Err(ParseError::ExpectFoldAlong.into());
            }

            let split: Vec<&str> = line[11..].split('=').collect();

            if split.len() != 2 {
                return Err(ParseError::Expect2FoldTerms.into());
            }

            let axis = match split[0] {
                "x" => Ok('x'),
                "y" => Ok('y'),
                _ => Err(ParseError::InvalidAxis(split[0].to_string()))
            }?;

            folds.push(Fold::new(axis, split[1].parse::<u16>()?));

        }
    }
   
    Ok((coords, folds))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input(coord_line: &str, fold_line: &str) -> Result<ParseResult, Box<dyn Error>> {
        load_buf(format!("{}\n\n{}", coord_line, fold_line).as_bytes())
    }

    #[test]
    fn test_parser() {
        let result = gen_input("1,2", "fold along x=1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (vec![Coord::new(1, 2)], vec![Fold::XAxis(1)]));

        let result = gen_input("1", "fold along x=1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::Expect2Coords);

        let result = gen_input("1,2,3", "fold along x=1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::Expect2Coords);

        let result = gen_input("1,x", "fold along x=1");
        assert!(result.is_err());
        assert!(result.unwrap_err().downcast_ref::<std::num::ParseIntError>().is_some());

        let result = gen_input("1,2", "fxxx along x=1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::ExpectFoldAlong);

        let result = gen_input("1,2", "x=1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::ExpectFoldAlong);

        let result = gen_input("1,2", "fold along ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::Expect2FoldTerms);

        let result = gen_input("1,2", "fold along x");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::Expect2FoldTerms);

        let result = gen_input("1,2", "fold along x=");
        assert!(result.is_err());
        assert!(result.unwrap_err().downcast_ref::<std::num::ParseIntError>().is_some());

        let result = gen_input("1,2", "fold along x=1=2");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::Expect2FoldTerms);

        let result = gen_input("1,2", "fold along z=1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().downcast_ref::<ParseError>().unwrap(), &ParseError::InvalidAxis("z".to_string()));
    }

}
