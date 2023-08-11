use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::cmp::{min, max};
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let coords = load_input("input05.txt")?;

    // Run parts
    part1(&coords);
    part2(&coords);

    Ok(())
}

fn part1(coords: &[Coords]) {
    let sum = calc_straight_overlaps(coords);

    println!("Part 1: Number of straight lines with > 1 overlap: {}", sum);
}

fn part2(coords: &[Coords]) {
    let sum = calc_overlaps(coords);

    println!("Part 2: Number of lines with > 1 overlap: {}", sum);
}

struct Coords {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16
}

impl Coords {

    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

}

fn calc_straight_overlaps(coords: &[Coords]) -> u16 {
    let mut maxx: usize = 0;
    let mut maxy: usize = 0;

    // Determine max x and y
    for coord in coords {
        if coord.is_straight() {
            if coord.x1 as usize > maxx { maxx = coord.x1 as usize };
            if coord.y1 as usize > maxy { maxy = coord.y1 as usize };
            if coord.x2 as usize > maxx { maxx = coord.x2 as usize };
            if coord.y2 as usize > maxy { maxy = coord.y2 as usize };
        }
    }

    // Draw lines
    let mut map: Vec<Vec<u16>> = vec![vec![0; maxx + 1]; maxy + 1];

    for coord in coords {
        if coord.is_straight() {
            if coord.x1 == coord.x2 {
                let miny = min(coord.y1, coord.y2);
                let maxy = max(coord.y1, coord.y2);

                for y in miny..=maxy {
                    map[y as usize][coord.x1 as usize] += 1;
                }
            } else {
                let minx = min(coord.x1, coord.x2);
                let maxx = max(coord.x1, coord.x2);

                for x in minx..=maxx {
                    map[coord.y1 as usize][x as usize] += 1;
                }
            }
        }
    }

    map.iter().map(|line| {
        line.iter().map(|&p| if p > 1 { 1 } else { 0 }).sum::<u16>()
    }).sum()
}

fn calc_overlaps(coords: &[Coords]) -> u16 {
    let mut maxx: usize = 0;
    let mut maxy: usize = 0;

    // Determine max x and y
    for coord in coords {
        if coord.x1 as usize > maxx { maxx = coord.x1 as usize };
        if coord.y1 as usize > maxy { maxy = coord.y1 as usize };
        if coord.x2 as usize > maxx { maxx = coord.x2 as usize };
        if coord.y2 as usize > maxy { maxy = coord.y2 as usize };
    }

    // Draw lines
    let mut map: Vec<Vec<u16>> = vec![vec![0; maxx + 1]; maxy + 1];

    for coord in coords {
        let mut x: isize = coord.x1 as isize;
        let mut y: isize = coord.y1 as isize;
        let mut xadd: isize = 0;
        let mut yadd: isize = 0;
        let mut count: usize = 0;

        if coord.x1 != coord.x2 {
            if coord.x1 < coord.x2 {
                xadd = 1;
                count = (coord.x2 - coord.x1) as usize;
            } else {
                xadd = -1;
                count = (coord.x1 - coord.x2) as usize;
            }
        }

        if coord.y1 != coord.y2 {
            if coord.y1 < coord.y2 {
                yadd = 1;
                count = (coord.y2 - coord.y1) as usize;
            } else {
                yadd = -1;
                count = (coord.y1 - coord.y2) as usize;
            }
        }

        for _ in 0..=count {
            map[y as usize][x as usize] += 1;
            x += xadd;
            y += yadd;
        }
    }

    map.iter().map(|line| {
        line.iter().map(|&p| if p > 1 { 1 } else { 0 }).sum::<u16>()
    }).sum()
}

fn load_input(file: &str) -> Result<Vec<Coords>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Vec<Coords>, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create coords vector
    let mut coords = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let nums: Vec<u16> = line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| s != &"")
                .map(|ns| ns.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()?;

            coords.push(Coords {
                x1: nums[0],
                y1: nums[1],
                x2: nums[2],
                y2: nums[3]
            });
        }
    }
   
    Ok(coords)
}

#[cfg(test)]
const TEST_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn test_part1() {
    let coords = load_buf(TEST_INPUT.as_bytes()).unwrap();

    let sum = calc_straight_overlaps(&coords);

    assert_eq!(5, sum, "Sum incorrect");
}

#[test]
fn test_part2() {
    let coords = load_buf(TEST_INPUT.as_bytes()).unwrap();

    let sum = calc_overlaps(&coords);

    assert_eq!(12, sum, "Sum incorrect");
}
