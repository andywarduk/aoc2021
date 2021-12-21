mod load;
mod image;

use std::error::Error;

use image::Image;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let (algo, raw_image) = load::load_input("input20.txt")?;

    // Run parts
    part1(&algo, &raw_image);
    part2(&algo, &raw_image);

    Ok(())
}

fn part1(algo: &[bool], raw_image: &[Vec<bool>]) {
    let mut image = Image::new(raw_image);

    for _ in 0..2 {
        image = image.enhance(algo);
    }

    println!("Part 1: Pixel count: {}", image.count());
}

fn part2(algo: &[bool], raw_image: &[Vec<bool>]) {
    let mut image = Image::new(raw_image);

    for _ in 0..50 {
        image = image.enhance(algo);
    }

    println!("Part 2: Pixel count: {}", image.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (algo, raw_image) = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        let image = Image::new(&raw_image);

        let image = image.enhance(&algo);
        assert_eq!(format!("{}", image), TEST_IMG1);

        let image = image.enhance(&algo);
        assert_eq!(format!("{}", image), TEST_IMG2);

        assert_eq!(image.count(), 35);
    }

    #[test]
    fn test_part2() {
        let (algo, raw_image) = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        let mut image = Image::new(&raw_image);

        for _ in 0..50 {
            image = image.enhance(&algo);
        }

        assert_eq!(image.count(), 3351);
    }

    const TEST_INPUT: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    const TEST_IMG1: &str = "\
.##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.
";

    const TEST_IMG2: &str = "\
.......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..
";

}