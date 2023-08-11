use std::collections::{VecDeque, HashSet};
use std::io::{BufRead, BufReader, BufWriter};
use std::error::Error;
use std::path::Path;
use std::fs::File;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let heights = load_input("input09.txt")?;

    // Run parts
    part1(&heights);
    part2(&heights);

    // Generate map PNG
    gen_map_png(&heights, "output09-1.png");
    gen_basin_png(&heights, "output09-2.png");

    Ok(())
}

fn part1(heights: &[Vec<u8>]) {
    let low_points = find_low_points(heights);

    let risk: u16 = low_points.iter().map(|&(x, y)| (1 + heights[y][x]) as u16).sum();

    println!("Part 1: Risk level sum: {}", risk);
}

fn part2(heights: &[Vec<u8>]) {
    let low_points = find_low_points(heights);

    let basins = build_basins(heights, low_points);

    let result = basins.iter().take(3).fold(1, |acc, b| acc * b.size);

    println!("Part 2: Product of 3 largest basin sizes: {}", result);
}

fn find_low_points(heights: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let width = heights[0].len();
    let height = heights.len();

    let is_low = |x: usize, y: usize| -> bool {
        let this_val = heights[y][x];

        if y > 0 && heights[y - 1][x] <= this_val { return false };
        if x > 0 && heights[y][x - 1] <= this_val { return false };
        if y < height - 1 && heights[y + 1][x] <= this_val { return false };
        if x < width - 1 && heights[y][x + 1] <= this_val { return false };

        true
    };

    let mut low_points = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if is_low(x, y) {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Basin {
    size: usize,
    x: usize,
    y: usize,
}

impl Basin {

    fn build_from(x: usize, y: usize, heights: &[Vec<u8>]) -> Self {
        let width = heights[0].len();
        let height = heights.len();
    
        let mut size = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut visit_queue: VecDeque<(usize, usize)> = VecDeque::new();
    
        let mut visit = |x: usize, y: usize, visit_queue: &mut VecDeque<(usize, usize)>| {
            if heights[y][x] < 9 {
                let coord = (x, y);
    
                if !visited.contains(&coord) {
                    visit_queue.push_back(coord);
                    visited.insert(coord);
                }
            }
        };
    
        visit(x, y, &mut visit_queue);
    
        while let Some((x, y)) = visit_queue.pop_front() {
            size += 1;
    
            if y > 0 { visit(x, y - 1, &mut visit_queue) };
            if x > 0 { visit(x - 1, y, &mut visit_queue) };
            if y < height - 1 { visit(x, y + 1, &mut visit_queue) };
            if x < width - 1 { visit(x + 1, y, &mut visit_queue) };
        }
    
        Self { x, y, size }    
    }

}

fn build_basins(heights: &[Vec<u8>], low_points: Vec<(usize, usize)>) -> Vec<Basin> {
    let mut basins: Vec<Basin> = low_points.iter().map(|&(x, y)| {
        Basin::build_from(x, y, heights)
    }).collect();

    basins.sort();
    basins.reverse();

    basins
}

fn gen_map_png(heights: &[Vec<u8>], filename: &str) {
    let width = heights[0].len();
    let height = heights.len();

    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let buf_writer = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, width as u32, height as u32);

    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Four);
    encoder.set_palette(vec![
        0x01, 0x2a, 0x4a,
        0x01, 0x3a, 0x63,
        0x01, 0x49, 0x7c,
        0x01, 0x4f, 0x86,
        0x2a, 0x6f, 0x97,
        0x2c, 0x7d, 0xa0,
        0x46, 0x8f, 0xaf,
        0x61, 0xa5, 0xc2,
        0x89, 0xc2, 0xd9,
        0xa9, 0xd6, 0xe5,
    ]);

    let mut writer = encoder.write_header().unwrap();
    let data: Vec<u8> = heights.iter().flatten().cloned().collect();
    let four_bit_data: Vec<u8> = data.chunks(2).map(|a| a[0] << 4 | a[1]).collect();
    writer.write_image_data(&four_bit_data).unwrap();
}

fn gen_basin_png(heights: &[Vec<u8>], filename: &str) {
    let width = heights[0].len();
    let height = heights.len();

    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let buf_writer = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, width as u32, height as u32);

    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::One);
    encoder.set_palette(vec![
        0x00, 0x00, 0x00,
        0xff, 0xff, 0xff,
    ]);

    let mut writer = encoder.write_header().unwrap();

    let one_bit_data: Vec<u8> = heights.iter().flat_map(|row| {
        let rim: Vec<u8> = row.iter().map(|&h| if h == 9 { 1 } else { 0 }).collect();

        rim.chunks(8).map(|a| {
            a.iter().fold(0, |acc, bit| acc << 1 | bit)
        }).collect::<Vec<u8>>()
    }).collect();

    writer.write_image_data(&one_bit_data).unwrap();
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

type ParseResult = Vec<Vec<u8>>;

fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vectors
    let mut heights = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            heights.push(line.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>());
        }
    }
   
    Ok(heights)
}

#[test]
fn test_part2() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    let heights = load_buf(test_input.as_bytes()).unwrap();

    let low_points = find_low_points(&heights);

    let basins = build_basins(&heights, low_points);

    assert_eq!(basins, vec![
        Basin { size: 14, x: 2, y: 2 },
        Basin { size: 9, x: 9, y: 0 },
        Basin { size: 9, x: 6, y: 4 },
        Basin { size: 3, x: 1, y: 0 }
    ]);
}
