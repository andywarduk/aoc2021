use std::collections::HashSet;
use std::cmp::Reverse;

use enum_map::{Enum, EnumMap};
use lazy_static::lazy_static;
use priority_queue::PriorityQueue;

#[derive(Debug, Enum, Hash, PartialEq, Eq, Clone, Copy)]
enum Loc {
    None,
    Hall1,
    Hall2,
    Hall3,
    Hall4,
    Hall5,
    Hall6,
    Hall7,
    RoomA1,
    RoomA2,
    RoomA3,
    RoomA4,
    RoomB1,
    RoomB2,
    RoomB3,
    RoomB4,
    RoomC1,
    RoomC2,
    RoomC3,
    RoomC4,
    RoomD1,
    RoomD2,
    RoomD3,
    RoomD4,
}

#[derive(Debug, Enum, Hash, PartialEq, Eq, Clone, Copy)]
enum Piece {
    None,
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    C1,
    C2,
    C3,
    C4,
    D1,
    D2,
    D3,
    D4,
}

impl std::fmt::Display for Piece {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Piece::None => "??",
            Piece::A1 => "A1",
            Piece::A2 => "A2",
            Piece::A3 => "A3",
            Piece::A4 => "A4",
            Piece::B1 => "B1",
            Piece::B2 => "B2",
            Piece::B3 => "B3",
            Piece::B4 => "B4",
            Piece::C1 => "C1",
            Piece::C2 => "C2",
            Piece::C3 => "C3",
            Piece::C4 => "C4",
            Piece::D1 => "D1",
            Piece::D2 => "D2",
            Piece::D3 => "D3",
            Piece::D4 => "D4",
        })
    }

}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Board {
    loc_map: EnumMap<Loc, Piece>,
    moves: u8
}

impl std::fmt::Display for Board {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {} ** {} ** {} ** {} ** {} {}\n\
      {}    {}    {}    {}\n\
      {}    {}    {}    {}\n\
      {}    {}    {}    {}\n\
      {}    {}    {}    {}",
            self.loc_map[Loc::Hall1],
            self.loc_map[Loc::Hall2],
            self.loc_map[Loc::Hall3],
            self.loc_map[Loc::Hall4],
            self.loc_map[Loc::Hall5],
            self.loc_map[Loc::Hall6],
            self.loc_map[Loc::Hall7],
            self.loc_map[Loc::RoomA1],
            self.loc_map[Loc::RoomB1],
            self.loc_map[Loc::RoomC1],
            self.loc_map[Loc::RoomD1],
            self.loc_map[Loc::RoomA2],
            self.loc_map[Loc::RoomB2],
            self.loc_map[Loc::RoomC2],
            self.loc_map[Loc::RoomD2],
            self.loc_map[Loc::RoomA3],
            self.loc_map[Loc::RoomB3],
            self.loc_map[Loc::RoomC3],
            self.loc_map[Loc::RoomD3],
            self.loc_map[Loc::RoomA4],
            self.loc_map[Loc::RoomB4],
            self.loc_map[Loc::RoomC4],
            self.loc_map[Loc::RoomD4],
        )
    }

}

#[derive(Debug)]
struct Move {
    dest: Loc,
    dist: u8,
    via: Vec<Loc>,
}

// #############
// #...........#
// ###B#B#D#D###
//   #D#C#B#A#
//   #D#B#A#C#
//   #C#A#A#C#
//   #########

pub fn part2() {
    let lowest_energy = find_lowest_energy_solution(Board {
        loc_map: EnumMap::from_array([
            Piece::None, // None
            Piece::None, // Hall1
            Piece::None, // Hall2
            Piece::None, // Hall3
            Piece::None, // Hall4
            Piece::None, // Hall5
            Piece::None, // Hall6
            Piece::None, // Hall7
            Piece::B1,   // RoomA1
            Piece::D1,   // RoomA2
            Piece::D2,   // RoomA3
            Piece::C1,   // RoomA4
            Piece::B2,   // RoomB1
            Piece::C2,   // RoomB2
            Piece::B3,   // RoomB3
            Piece::A1,   // RoomB4
            Piece::D3,   // RoomC1
            Piece::B4,   // RoomC2
            Piece::A2,   // RoomC3
            Piece::A3,   // RoomC4
            Piece::D4,   // RoomD1
            Piece::A4,   // RoomD2
            Piece::C3,   // RoomD3
            Piece::C4,   // RoomD4
        ]),
        moves: 0
    });

    println!("Part 2: Lowest energy solution: {}", lowest_energy);
}

fn find_lowest_energy_solution(init_board: Board) -> u64 {
    let mut lowest_energy = u64::MAX;

    let mut states = PriorityQueue::new();

    states.push(init_board, Reverse(0));
    
    while let Some((board, Reverse(energy))) = states.pop() {
        // Check energy
        if energy >= lowest_energy {
            break
        }

        // Calculate moves
        for (from_loc, piece) in board.loc_map.iter().skip(1).filter(|(_, &piece)| piece != Piece::None) {
            for mv in &MOVES[from_loc] {
                // Check the space is clear
                if board.loc_map[mv.dest] != Piece::None {
                    continue
                }

                // Check the path is clear
                let mut blocked = false;

                for check_loc in &mv.via {
                    if board.loc_map[*check_loc] != Piece::None {
                        blocked = true;
                        break
                    }
                }

                if blocked {
                    continue
                }

                // Check move is sensible
                if !is_home(&from_loc) && !is_home(&mv.dest) {
                    // Hall to hall
                    continue
                }

                if is_home(&from_loc) && home_type(&from_loc) == piece_type(piece) {
                    // Piece is already in home - is it on top of another correct piece?
                    if home_filled(&from_loc, &board) {
                        continue
                    }
                }

                if is_home(&mv.dest) {
                    if home_type(&mv.dest) != piece_type(piece){
                        continue
                    }

                    if !home_filled(&mv.dest, &board) {
                        continue
                    }
                }

                // Calculate and check energy
                let new_energy = energy + (move_cost(piece) * mv.dist as u64);

                if new_energy >= lowest_energy {
                    continue
                }

                // Build new state
                let mut new_board = board.clone();

                new_board.loc_map[from_loc] = Piece::None;
                new_board.loc_map[mv.dest] = *piece;
                new_board.moves += 1;

                // Finished?
                if a_filled(&new_board) && b_filled(&new_board) && 
                    c_filled(&new_board) && d_filled(&new_board) {
                    println!("Solution found with energy {}, {} moves", new_energy, new_board.moves);
                    lowest_energy = new_energy;
                } else {
                    states.push(new_board, Reverse(new_energy));
                }
            }
        }
    }

    lowest_energy
}

fn move_cost(piece: &Piece) -> u64 {
    match piece {
        Piece::A1 | Piece::A2 | Piece::A3 | Piece::A4 => 1,
        Piece::B1 | Piece::B2 | Piece::B3 | Piece::B4 => 10,
        Piece::C1 | Piece::C2 | Piece::C3 | Piece::C4 => 100,
        Piece::D1 | Piece::D2 | Piece::D3 | Piece::D4 => 1000,
        _ => panic!("Invalid piece")
    }
}

fn is_home(loc: &Loc) -> bool {
    matches!(loc,
        Loc::RoomA1 | Loc::RoomA2 | Loc::RoomA3 | Loc::RoomA4 |
        Loc::RoomB1 | Loc::RoomB2 | Loc::RoomB3 | Loc::RoomB4 |
        Loc::RoomC1 | Loc::RoomC2 | Loc::RoomC3 | Loc::RoomC4 |
        Loc::RoomD1 | Loc::RoomD2 | Loc::RoomD3 | Loc::RoomD4
    )
}

#[derive(PartialEq, Eq)]
enum Type {
    A,
    B,
    C,
    D,
    Unknown
}

fn home_type(loc: &Loc) -> Type {
    match loc {
        Loc::RoomA1 | Loc::RoomA2 | Loc::RoomA3 | Loc::RoomA4 => Type::A,
        Loc::RoomB1 | Loc::RoomB2 | Loc::RoomB3 | Loc::RoomB4 => Type::B,
        Loc::RoomC1 | Loc::RoomC2 | Loc::RoomC3 | Loc::RoomC4 => Type::C,
        Loc::RoomD1 | Loc::RoomD2 | Loc::RoomD3 | Loc::RoomD4 => Type::D,
        _ => Type::Unknown
    }
}

fn piece_type(piece: &Piece) -> Type {
    match piece {
        Piece::A1 | Piece::A2 | Piece::A3 | Piece::A4 => Type::A,
        Piece::B1 | Piece::B2 | Piece::B3 | Piece::B4 => Type::B,
        Piece::C1 | Piece::C2 | Piece::C3 | Piece::C4 => Type::C,
        Piece::D1 | Piece::D2 | Piece::D3 | Piece::D4 => Type::D,
        _ => Type::Unknown
    }
}

fn home_filled(loc: &Loc, board: &Board) -> bool {
    match loc {
        Loc::RoomA1 => {
            piece_type(&board.loc_map[Loc::RoomA2]) == Type::A &&
            piece_type(&board.loc_map[Loc::RoomA3]) == Type::A &&
            piece_type(&board.loc_map[Loc::RoomA4]) == Type::A
        },
        Loc::RoomA2 => {
            piece_type(&board.loc_map[Loc::RoomA3]) == Type::A &&
            piece_type(&board.loc_map[Loc::RoomA4]) == Type::A
        },
        Loc::RoomA3 => {
            piece_type(&board.loc_map[Loc::RoomA4]) == Type::A
        },
        Loc::RoomA4 => { true },
        Loc::RoomB1 => {
            piece_type(&board.loc_map[Loc::RoomB2]) == Type::B &&
            piece_type(&board.loc_map[Loc::RoomB3]) == Type::B &&
            piece_type(&board.loc_map[Loc::RoomB4]) == Type::B
        },
        Loc::RoomB2 => {
            piece_type(&board.loc_map[Loc::RoomB3]) == Type::B &&
            piece_type(&board.loc_map[Loc::RoomB4]) == Type::B
        },
        Loc::RoomB3 => {
            piece_type(&board.loc_map[Loc::RoomB4]) == Type::B
        },
        Loc::RoomB4 => { true },
        Loc::RoomC1 => {
            piece_type(&board.loc_map[Loc::RoomC2]) == Type::C &&
            piece_type(&board.loc_map[Loc::RoomC3]) == Type::C &&
            piece_type(&board.loc_map[Loc::RoomC4]) == Type::C
        },
        Loc::RoomC2 => {
            piece_type(&board.loc_map[Loc::RoomC3]) == Type::C &&
            piece_type(&board.loc_map[Loc::RoomC4]) == Type::C
        },
        Loc::RoomC3 => {
            piece_type(&board.loc_map[Loc::RoomC4]) == Type::C
        },
        Loc::RoomC4 => { true },
        Loc::RoomD1 => {
            piece_type(&board.loc_map[Loc::RoomD2]) == Type::D &&
            piece_type(&board.loc_map[Loc::RoomD3]) == Type::D &&
            piece_type(&board.loc_map[Loc::RoomD4]) == Type::D
        },
        Loc::RoomD2 => {
            piece_type(&board.loc_map[Loc::RoomD3]) == Type::D &&
            piece_type(&board.loc_map[Loc::RoomD4]) == Type::D
        },
        Loc::RoomD3 => {
            piece_type(&board.loc_map[Loc::RoomD4]) == Type::D
        },
        Loc::RoomD4 => { true },
        _ => panic!("Not a home")
    }
}

fn a_filled(board: &Board) -> bool {
    piece_type(&board.loc_map[Loc::RoomA1]) == Type::A &&
    piece_type(&board.loc_map[Loc::RoomA2]) == Type::A &&
    piece_type(&board.loc_map[Loc::RoomA3]) == Type::A &&
    piece_type(&board.loc_map[Loc::RoomA4]) == Type::A 
}

fn b_filled(board: &Board) -> bool {
    piece_type(&board.loc_map[Loc::RoomB1]) == Type::B &&
    piece_type(&board.loc_map[Loc::RoomB2]) == Type::B &&
    piece_type(&board.loc_map[Loc::RoomB3]) == Type::B &&
    piece_type(&board.loc_map[Loc::RoomB4]) == Type::B 
}

fn c_filled(board: &Board) -> bool {
    piece_type(&board.loc_map[Loc::RoomC1]) == Type::C &&
    piece_type(&board.loc_map[Loc::RoomC2]) == Type::C &&
    piece_type(&board.loc_map[Loc::RoomC3]) == Type::C &&
    piece_type(&board.loc_map[Loc::RoomC4]) == Type::C 
}

fn d_filled(board: &Board) -> bool {
    piece_type(&board.loc_map[Loc::RoomD1]) == Type::D &&
    piece_type(&board.loc_map[Loc::RoomD2]) == Type::D &&
    piece_type(&board.loc_map[Loc::RoomD3]) == Type::D &&
    piece_type(&board.loc_map[Loc::RoomD4]) == Type::D 
}

// ####################################
// # H1 H2 .. H3 .. H4 .. H5 .. H6 H7 #
// ####### AU ## BU ## CU ## DU #######
//       # AL ## BL ## CL ## DL #
//       ########################

lazy_static! {
    static ref CONNS: EnumMap<Loc, Vec<(Loc, u8)>> = {
        EnumMap::from_array([
            vec![],
            vec![(Loc::Hall2, 1)],
            vec![(Loc::Hall1, 1), (Loc::RoomA1, 2), (Loc::Hall3, 2)],
            vec![(Loc::Hall2, 2), (Loc::RoomA1, 2), (Loc::RoomB1, 2), (Loc::Hall4, 2)],
            vec![(Loc::Hall3, 2), (Loc::RoomB1, 2), (Loc::RoomC1, 2), (Loc::Hall5, 2)],
            vec![(Loc::Hall4, 2), (Loc::RoomC1, 2), (Loc::RoomD1, 2), (Loc::Hall6, 2)],
            vec![(Loc::Hall5, 2), (Loc::RoomD1, 2), (Loc::Hall7, 1)],
            vec![(Loc::Hall6, 1)],
            vec![(Loc::Hall2, 2), (Loc::Hall3, 2), (Loc::RoomA2, 1)], // A1
            vec![(Loc::RoomA1, 1), (Loc::RoomA3, 1)], // A2
            vec![(Loc::RoomA2, 1), (Loc::RoomA4, 1)], // A3
            vec![(Loc::RoomA3, 1)], // A4
            vec![(Loc::Hall3, 2), (Loc::Hall4, 2), (Loc::RoomB2, 1)], // B1
            vec![(Loc::RoomB1, 1), (Loc::RoomB3, 1)], // B2
            vec![(Loc::RoomB2, 1), (Loc::RoomB4, 1)], // B3
            vec![(Loc::RoomB3, 1)], // B4
            vec![(Loc::Hall4, 2), (Loc::Hall5, 2), (Loc::RoomC2, 1)], // C1
            vec![(Loc::RoomC1, 1), (Loc::RoomC3, 1)], // C2
            vec![(Loc::RoomC2, 1), (Loc::RoomC4, 1)], // C3
            vec![(Loc::RoomC3, 1)], // C4
            vec![(Loc::Hall5, 2), (Loc::Hall6, 2), (Loc::RoomD2, 1)], // D1
            vec![(Loc::RoomD1, 1), (Loc::RoomD3, 1)], // D2
            vec![(Loc::RoomD2, 1), (Loc::RoomD4, 1)], // BD3
            vec![(Loc::RoomD3, 1)], // D4
        ])
    };
}

fn walk(loc: &Loc, set: HashSet<Loc>, route: Vec<Loc>, dist: u8, moves: &mut Vec<Move>) {

    let conns = &*CONNS[*loc].iter()
        .filter(|(loc, _)| !set.contains(loc))
        .collect::<Vec<&(Loc, u8)>>();

    if !conns.is_empty() {
        let mut next_set = set.clone();
        for (loc, _) in conns {
            next_set.insert(*loc);
        }

        for (next_loc, next_dist) in conns {
            moves.push(Move {
                dest: *next_loc,
                dist: dist + next_dist,
                via: route.clone()
            });

            let mut next_route = route.clone();
            next_route.push(*next_loc);

            walk(next_loc, next_set.clone(), next_route, dist + next_dist, moves)
        }
    }
}

lazy_static! {
    static ref MOVES: EnumMap<Loc, Vec<Move>> = {
        let mut array: [Vec<Move>; 24] = Default::default();

        for (i, (loc, _)) in CONNS.iter().enumerate() {
            let mut set = HashSet::new();
            set.insert(loc);

            walk(&loc, set, Vec::new(), 0, &mut array[i]);
        }

        EnumMap::from_array(array)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let lowest_energy = find_lowest_energy_solution(Board {
            loc_map: EnumMap::from_array([
                Piece::None, // None
                Piece::None, // Hall1
                Piece::None, // Hall2
                Piece::None, // Hall3
                Piece::None, // Hall4
                Piece::None, // Hall5
                Piece::None, // Hall6
                Piece::None, // Hall7
                Piece::B1,   // RoomA1
                Piece::D1,   // RoomA2
                Piece::D2,   // RoomA3
                Piece::A1,   // RoomA4
                Piece::C1,   // RoomB1
                Piece::C2,   // RoomB2
                Piece::B2,   // RoomB3
                Piece::D3,   // RoomB4
                Piece::B3,   // RoomC1
                Piece::B4,   // RoomC2
                Piece::A2,   // RoomC3
                Piece::C3,   // RoomC4
                Piece::D4,   // RoomD1
                Piece::A3,   // RoomD2
                Piece::C4,   // RoomD3
                Piece::A4,   // RoomD4
            ]),
            moves: 0
        });

        assert_eq!(lowest_energy, 44169)
    }

}
