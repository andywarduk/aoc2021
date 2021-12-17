use std::cmp::{min, max, Ordering};

fn main() {
    let target = Square::new((79, -176), (137, -117));

    let xvs = get_xv_list(&target);
    let yvs = get_yv_list(&target);

    // Run parts
    part1(&yvs);
    part2(&xvs, &yvs);
}

fn part1(yvs: &[Velocity]) {
    println!("Part 1: Highest trajectory point: {}", highest_trajectory_point(yvs))
}

fn part2(xvs: &[Velocity], yvs: &[Velocity]) {
    let trajvec = trajectories(xvs, yvs);

    println!("Part 2: Number of trajetories: {}", trajvec.len())
}

type Coord = i16;
type Coords = (Coord, Coord);

#[derive(Debug)]
enum Relative {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Hit
}

struct Square {
    min_x: Coord,
    max_x: Coord,
    min_y: Coord,
    max_y: Coord
}

impl Square {

    fn new(corner1: Coords, corner2: Coords) -> Self {
        Square {
            min_x: min(corner1.0, corner2.0),
            max_x: max(corner1.0, corner2.0),
            min_y: min(corner1.1, corner2.1),
            max_y: max(corner1.1, corner2.1)
        }
    }

    fn pt_relative(&self, coord: Coords) -> Relative {
        if coord.0 < self.min_x {
            // Position is west of target
            if coord.1 > self.max_y {
                Relative::NW
            } else if coord.1 < self.min_y {
                Relative::SW
            } else {
                Relative::W
            }
        } else if coord.0 > self.max_x {
            // Position is east of target
            if coord.1 > self.max_y {
                Relative::NE
            } else if coord.1 < self.min_y {
                Relative::SE
            } else {
                Relative::E
            }
        } else if coord.1 < self.min_y {
            Relative::S
        } else if coord.1 > self.max_y {
            Relative::N
        } else {
            Relative::Hit
        }
    }

}

fn highest_trajectory_point(yvs: &[Velocity]) -> i16 {
    let fastest = yvs.iter().map(|yv| yv.v).max().unwrap();

    (0..=fastest).sum()
}

fn trajectories(xvs: &[Velocity], yvs: &[Velocity]) -> Vec<(i16, i16)> {
    let mut trajvec = Vec::new();

    for yv in yvs {
        for xv in xvs {
            if yv.min_steps >= xv.min_steps && yv.min_steps <= xv.max_steps ||
                yv.max_steps >= xv.min_steps && yv.max_steps <= xv.max_steps ||
                yv.min_steps < xv.min_steps && yv.max_steps > xv.max_steps {
                trajvec.push((xv.v, yv.v))
            }
        }
    }

    trajvec
}

#[derive(Debug)]
struct Velocity {
    v: i16,
    min_steps: u16,
    max_steps: u16,
}

fn get_xv_list(target: &Square) -> Vec<Velocity> {
    let mut xvs = Vec::new();

    for xv in 1..=target.max_x {
        if let Some(hit) = fire_test_x(target, xv) {
            xvs.push(hit)
        }
    }

    xvs
}

fn fire_test_x(target: &Square, xv1: i16) -> Option<Velocity> {
    let mut x = 0;
    let mut xv = xv1;
    let mut hit = false;
    let mut steps = 0;
    let mut min_steps = u16::MAX;
    let mut max_steps = 0;

    loop {
        x += xv;
        steps += 1;
        
        let rel_pos = target.pt_relative((x, target.min_y));

        match rel_pos {
            Relative::W => {
                // Not reached yet
                if xv == 0 {
                    break
                }        
            }
            Relative::E => {
                // Overshot
                break
            }
            Relative::Hit => {
                if !hit {
                    min_steps = steps;
                    hit = true;
                }

                if xv == 0 {
                    max_steps = u16::MAX;
                    break
                }

                max_steps = steps;
            }
            _ => { panic!("Not expected!") }
        }

        xv = match x.cmp(&0) {
            Ordering::Greater => xv - 1,
            Ordering::Less => xv + 1,
            Ordering::Equal => xv
        };
    }

    if hit {
        Some(Velocity {
            v: xv1,
            min_steps,
            max_steps
        })
    } else {
        None
    }
}

fn get_yv_list(target: &Square) -> Vec<Velocity> {
    let mut yvs = Vec::new();

    for yv in -target.min_y.abs()..target.min_y.abs() {
        if let Some(hit) = fire_test_y(target, yv) {
            yvs.push(hit)
        }
    }

    yvs
}

fn fire_test_y(target: &Square, yv1: i16) -> Option<Velocity> {
    let mut y = 0;
    let mut yv = yv1;
    let mut hit = false;
    let mut steps = 0;
    let mut min_steps = u16::MAX;
    let mut max_steps = 0;

    loop {
        y += yv;
        steps += 1;
        
        let rel_pos = target.pt_relative((target.min_x, y));

        match rel_pos {
            Relative::N => {
                // Above target
            }
            Relative::S => {
                // Below target
                break
            }
            Relative::Hit => {
                if !hit {
                    min_steps = steps;
                    hit = true;
                }

                max_steps = steps;
            }
            _ => { panic!("Not expected!") }
        }

        yv -= 1;
    }

    if hit {
        Some(Velocity {
            v: yv1,
            min_steps,
            max_steps
        })
    } else {
        None
    }
}

#[test]
fn test_highest() {
    let target = Square::new((20, -10), (30, -5));

    let yvs = get_yv_list(&target);

    let highest = highest_trajectory_point(&yvs);

    assert_eq!(highest, 45)
}

#[test]
fn test_trajectory_count() {
    let target = Square::new((20, -10), (30, -5));

    let xvs = get_xv_list(&target);
    let yvs = get_yv_list(&target);

    let mut trajvec = trajectories(&xvs, &yvs);

    trajvec.sort_unstable();

    const EXPECTED: [(i16, i16); 112] = [
        (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7), (6, 8), (6, 9),
        (7, -1), (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7), (7, 8), (7, 9),
        (8, -2), (8, -1), (8, 0), (8, 1),
        (9, -2), (9, -1), (9, 0),
        (10, -2), (10, -1),
        (11, -4), (11, -3), (11, -2), (11, -1),
        (12, -4), (12, -3), (12, -2),
        (13, -4), (13, -3), (13, -2),
        (14, -4), (14, -3), (14, -2),
        (15, -4), (15, -3), (15, -2),
        (20, -10), (20, -9), (20, -8), (20, -7), (20, -6), (20, -5),
        (21, -10), (21, -9), (21, -8), (21, -7), (21, -6), (21, -5),
        (22, -10), (22, -9), (22, -8), (22, -7), (22, -6), (22, -5),
        (23, -10), (23, -9), (23, -8), (23, -7), (23, -6), (23, -5),
        (24, -10), (24, -9), (24, -8), (24, -7), (24, -6), (24, -5),
        (25, -10), (25, -9), (25, -8), (25, -7), (25, -6), (25, -5),
        (26, -10), (26, -9), (26, -8), (26, -7), (26, -6), (26, -5),
        (27, -10), (27, -9), (27, -8), (27, -7), (27, -6), (27, -5),
        (28, -10), (28, -9), (28, -8), (28, -7), (28, -6), (28, -5),
        (29, -10), (29, -9), (29, -8), (29, -7), (29, -6), (29, -5),
        (30, -10), (30, -9), (30, -8), (30, -7), (30, -6), (30, -5)
    ];

    assert_eq!(trajvec, EXPECTED);
}
