mod load;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let map = load::load_input("input25.txt")?;

    // Run parts
    part1(&mut map.clone());
    //part2(&program);

    Ok(())
}

fn part1(map: &mut Vec<Vec<char>>) {
    let steps = do_moves(map);

    println!("Part 1: Steps: {}", steps);
}

fn do_moves(map: &mut Vec<Vec<char>>) -> usize {
    let mut steps = 0;

    loop {
        steps += 1;

        if do_move(map) == 0 {
            break steps
        }
    }
}

fn do_move(map: &mut Vec<Vec<char>>) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut moved = 0;

    // East moves
    for y in 0..height {
        let mut x = 0;
        let x0 = map[y][x];

        while x < width {
            if map[y][x] == '>' {
                let new_x = (x + 1) % width;

                if (new_x == 0 && x0 == '.') || (new_x > 0 && map[y][new_x] == '.') {
                    map[y][new_x] = '>';
                    map[y][x] = '.';
                    moved += 1;
                    x += 1;
                }
            }

            x += 1;
        }
    }

    // South moves
    for x in 0..width {
        let mut y = 0;
        let y0 = map[y][x];

        while y < height {
            if map[y][x] == 'v' {
                let new_y = (y + 1) % height;

                if (new_y == 0 && y0 == '.') || (new_y > 0 && map[new_y][x] == '.') {
                    map[new_y][x] = 'v';
                    map[y][x] = '.';
                    moved += 1;
                    y += 1;
                }
            }

            y += 1;
        }
    }

    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    fn maps_equal(m1: &Vec<Vec<char>>, m2: &Vec<Vec<char>>) {
        for i in 0..m1.len() {
            assert_eq!(m1[i], m2[i], "Row {} incorrect", i)
        }
    }

    #[test]
    fn test_move1() {
        let mut map = vec![
            "...>>>>>...".chars().collect::<Vec<char>>()
        ];

        do_move(&mut map);

        assert_eq!(map, vec![
            "...>>>>.>..".chars().collect::<Vec<char>>()
        ]);
    }

    #[test]
    fn test_move2() {
        let mut map = load::load_buf("\
..........
.>v....v..
.......>..
..........".as_bytes()).unwrap();

        do_move(&mut map);

        let expected = load::load_buf("\
..........
.>........
..v....v>.
..........".as_bytes()).unwrap();
        
        assert_eq!(map, expected);
    }

    #[test]
    fn test_move3() {
        let mut map = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        do_move(&mut map);
        let expected = load::load_buf(MOVE_1.as_bytes()).unwrap();
        maps_equal(&map, &expected);

        do_move(&mut map);
        let expected = load::load_buf(MOVE_2.as_bytes()).unwrap();
        maps_equal(&map, &expected);
    }

    #[test]
    fn test_part1() {
        let mut map = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        let moves = do_moves(&mut map);

        assert_eq!(moves, 58);
    }

    const TEST_INPUT: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    const MOVE_1: &str = "\
....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v";

    const MOVE_2: &str = "\
>.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv.";

}