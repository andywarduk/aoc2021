mod load;
mod parser;
mod packet;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let data = load::load_input("input16.txt")?;

    // Build tree
    let tree = packet::parse_data(&data);

    // Print tree
    println!("Parse tree:\n{:#}\n", tree);
    println!("Compact: {}\n", tree);

    // Run parts
    part1(&tree);
    part2(&tree);

    Ok(())
}

fn part1(tree: &packet::Packet) {
    println!("Part 1: Sum of versions: {}", tree.sum_versions());
}

fn part2(tree: &packet::Packet) {
    println!("Part 2: Calculation result: {}", tree.eval());
}
