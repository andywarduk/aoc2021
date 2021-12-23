mod load;
mod cube;
mod instruction;

use std::collections::HashSet;
use std::error::Error;

use instruction::Instruction;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let instructions = load::load_input("input22.txt")?;

    // Build axis break points
    let (xaxis, yaxis, zaxis) = build_axes(&instructions);

    // Run parts
    part1(&instructions, &xaxis, &yaxis, &zaxis);
    part2(&instructions, &xaxis, &yaxis, &zaxis);

    Ok(())
}

fn part1(instructions: &[Instruction], xaxis: &[i32], yaxis: &[i32], zaxis: &[i32]) {
    // Count cubes
    let count = count_init_cubes(instructions, xaxis, yaxis, zaxis);

    println!("Part 1: Init cube count: {}", count);
}

fn part2(instructions: &[Instruction], xaxis: &[i32], yaxis: &[i32], zaxis: &[i32]) {
    // Count cubes
    let count = count_all_cubes(instructions, xaxis, yaxis, zaxis);

    println!("Part 2: Cube count: {}", count);
}

fn map_coord(coord: i32, axis: &[i32]) -> usize {
    axis.binary_search(&coord).unwrap()
}

fn count_init_cubes(instructions: &[Instruction], xaxis: &[i32], yaxis: &[i32], zaxis: &[i32]) -> usize {
    let mut map = vec![vec![vec![false; xaxis.len() - 1]; yaxis.len() - 1]; zaxis.len() - 1];

    for instruction in instructions.iter().filter(|i| i.cube.within_range(-50..50, -50..50, -50..50)) {
        let x1 = map_coord(instruction.cube.ranges[0].start, xaxis);
        let x2 = map_coord(instruction.cube.ranges[0].end + 1, xaxis);
        let y1 = map_coord(instruction.cube.ranges[1].start, yaxis);
        let y2 = map_coord(instruction.cube.ranges[1].end + 1, yaxis);
        let z1 = map_coord(instruction.cube.ranges[2].start, zaxis);
        let z2 = map_coord(instruction.cube.ranges[2].end + 1, zaxis);

        for z in z1..z2 {
            for y in y1..y2 {
                for x in x1..x2 {
                    map[z][y][x] = instruction.on;
                }
            }
        }    
    }

    let mut set_count: usize = 0;

    for (z, zrow) in map.iter().enumerate() {
        let zsize = (zaxis[z + 1] - zaxis[z]) as usize;

        for (y, yrow) in zrow.iter().enumerate() {
            let ysize = (yaxis[y + 1] - yaxis[y]) as usize;

            for (x, on) in yrow.iter().enumerate() {
                if *on {
                    let xsize = (xaxis[x + 1] - xaxis[x]) as usize;
                    set_count += xsize * ysize * zsize;
                }
            }
        }
    }

    set_count
}

fn count_all_cubes(instructions: &[Instruction], xaxis: &[i32], yaxis: &[i32], zaxis: &[i32]) -> usize {
    let mut map = vec![vec![vec![false; xaxis.len() - 1]; yaxis.len() - 1]; zaxis.len() - 1];

    for instruction in instructions.iter() {
        let x1 = map_coord(instruction.cube.ranges[0].start, xaxis);
        let x2 = map_coord(instruction.cube.ranges[0].end + 1, xaxis);
        let y1 = map_coord(instruction.cube.ranges[1].start, yaxis);
        let y2 = map_coord(instruction.cube.ranges[1].end + 1, yaxis);
        let z1 = map_coord(instruction.cube.ranges[2].start, zaxis);
        let z2 = map_coord(instruction.cube.ranges[2].end + 1, zaxis);

        for z in z1..z2 {
            for y in y1..y2 {
                for x in x1..x2 {
                    map[z][y][x] = instruction.on;
                }
            }
        }    
    }

    let mut set_count: usize = 0;

    for (z, zrow) in map.iter().enumerate() {
        let zsize = (zaxis[z + 1] - zaxis[z]) as usize;

        for (y, yrow) in zrow.iter().enumerate() {
            let ysize = (yaxis[y + 1] - yaxis[y]) as usize;

            for (x, on) in yrow.iter().enumerate() {
                if *on {
                    let xsize = (xaxis[x + 1] - xaxis[x]) as usize;
                    set_count += xsize * ysize * zsize;
                }
            }
        }
    }

    set_count
}

fn build_axes(instructions: &[Instruction]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let mut zs = HashSet::new();

    for i in instructions {
        xs.insert(i.cube.ranges[0].start);
        xs.insert(i.cube.ranges[0].end + 1);
        ys.insert(i.cube.ranges[1].start);
        ys.insert(i.cube.ranges[1].end + 1);
        zs.insert(i.cube.ranges[2].start);
        zs.insert(i.cube.ranges[2].end + 1);
    }

    let mut xs = xs.into_iter().collect::<Vec<i32>>();
    let mut ys = ys.into_iter().collect::<Vec<i32>>();
    let mut zs = zs.into_iter().collect::<Vec<i32>>();

    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();

    (xs, ys, zs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        // Load the input file
        let instructions = load::load_buf(TEST_INPUT1.as_bytes()).unwrap();

        // Build axis break points
        let (xaxis, yaxis, zaxis) = build_axes(&instructions);

        // Count cubes
        let count = count_init_cubes(&instructions, &xaxis, &yaxis, &zaxis);

        assert_eq!(count, 39);
    }

    #[test]
    fn test_part1_2() {
        // Load the input file
        let instructions = load::load_buf(TEST_INPUT2.as_bytes()).unwrap();

        // Build axis break points
        let (xaxis, yaxis, zaxis) = build_axes(&instructions);

        // Count cubes
        let count = count_init_cubes(&instructions, &xaxis, &yaxis, &zaxis);

        assert_eq!(count, 590784);
    }

    #[test]
    fn test_part2() {
        // let (algo, raw_image) = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        // let mut image = Image::new(&raw_image);

        // for _ in 0..50 {
        //     image = image.enhance(&algo);
        // }

        // assert_eq!(image.count(), 3351);
    }

    const TEST_INPUT1: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const TEST_INPUT2: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

}
