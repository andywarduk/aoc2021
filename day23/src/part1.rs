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
    RoomAUpper,
    RoomALower,
    RoomBUpper,
    RoomBLower,
    RoomCUpper,
    RoomCLower,
    RoomDUpper,
    RoomDLower,
}

#[derive(Debug, Enum, Hash, PartialEq, Eq, Clone, Copy)]
enum Piece {
    None,
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    D1,
    D2
}

impl std::fmt::Display for Piece {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Piece::None => "??",
            Piece::A1 => "A1",
            Piece::A2 => "A2",
            Piece::B1 => "B1",
            Piece::B2 => "B2",
            Piece::C1 => "C1",
            Piece::C2 => "C2",
            Piece::D1 => "D1",
            Piece::D2 => "D2"
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
        write!(f, "{} {} ** {} ** {} ** {} ** {} {}\n      {}    {}    {}    {}\n      {}    {}    {}    {}",
            self.loc_map[Loc::Hall1],
            self.loc_map[Loc::Hall2],
            self.loc_map[Loc::Hall3],
            self.loc_map[Loc::Hall4],
            self.loc_map[Loc::Hall5],
            self.loc_map[Loc::Hall6],
            self.loc_map[Loc::Hall7],
            self.loc_map[Loc::RoomAUpper],
            self.loc_map[Loc::RoomBUpper],
            self.loc_map[Loc::RoomCUpper],
            self.loc_map[Loc::RoomDUpper],
            self.loc_map[Loc::RoomALower],
            self.loc_map[Loc::RoomBLower],
            self.loc_map[Loc::RoomCLower],
            self.loc_map[Loc::RoomDLower],
        )
    }

}

#[derive(Debug)]
struct Move {
    dest: Loc,
    dist: u8,
    via: Vec<Loc>,
}

pub fn part1() {
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
            Piece::B1,   // RoomAUpper
            Piece::C1,   // RoomALower
            Piece::B2,   // RoomBUpper
            Piece::A1,   // RoomBLower
            Piece::D1,   // RoomCUpper
            Piece::A2,   // RoomCLower
            Piece::D2,   // RoomDUpper
            Piece::C2,   // RoomDLower
        ]),
        moves: 0
    });

    println!("Part 1: Lowest energy solution: {}", lowest_energy);
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
        Piece::A1 | Piece::A2 => 1,
        Piece::B1 | Piece::B2 => 10,
        Piece::C1 | Piece::C2 => 100,
        Piece::D1 | Piece::D2 => 1000,
        _ => panic!("Invalid piece")
    }
}

fn is_home(loc: &Loc) -> bool {
    matches!(loc,
        Loc::RoomAUpper | Loc::RoomALower |
        Loc::RoomBUpper | Loc::RoomBLower |
        Loc::RoomCUpper | Loc::RoomCLower |
        Loc::RoomDUpper | Loc::RoomDLower
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
        Loc::RoomAUpper | Loc::RoomALower => Type::A,
        Loc::RoomBUpper | Loc::RoomBLower => Type::B,
        Loc::RoomCUpper | Loc::RoomCLower => Type::C,
        Loc::RoomDUpper | Loc::RoomDLower => Type::D,
        _ => Type::Unknown
    }
}

fn piece_type(piece: &Piece) -> Type {
    match piece {
        Piece::A1 | Piece::A2 => Type::A,
        Piece::B1 | Piece::B2 => Type::B,
        Piece::C1 | Piece::C2 => Type::C,
        Piece::D1 | Piece::D2 => Type::D,
        _ => Type::Unknown
    }
}

fn home_filled(loc: &Loc, board: &Board) -> bool {
    match loc {
        Loc::RoomAUpper => { piece_type(&board.loc_map[Loc::RoomALower]) == Type::A }
        Loc::RoomALower => { true }
        Loc::RoomBUpper => { piece_type(&board.loc_map[Loc::RoomBLower]) == Type::B }
        Loc::RoomBLower => { true }
        Loc::RoomCUpper => { piece_type(&board.loc_map[Loc::RoomCLower]) == Type::C }
        Loc::RoomCLower => { true }
        Loc::RoomDUpper => { piece_type(&board.loc_map[Loc::RoomDLower]) == Type::D }
        Loc::RoomDLower => { true }
        _ => panic!("Not a home")
    }
}

fn a_filled(board: &Board) -> bool {
    is_a_piece(&board.loc_map[Loc::RoomALower]) && is_a_piece(&board.loc_map[Loc::RoomAUpper])
}

fn b_filled(board: &Board) -> bool {
    is_b_piece(&board.loc_map[Loc::RoomBLower]) && is_b_piece(&board.loc_map[Loc::RoomBUpper])
}

fn c_filled(board: &Board) -> bool {
    is_c_piece(&board.loc_map[Loc::RoomCLower]) && is_c_piece(&board.loc_map[Loc::RoomCUpper])
}

fn d_filled(board: &Board) -> bool {
    is_d_piece(&board.loc_map[Loc::RoomDLower]) && is_d_piece(&board.loc_map[Loc::RoomDUpper])
}

fn is_a_piece(piece: &Piece) -> bool {
    matches!(piece, Piece::A1 | Piece::A2)
}

fn is_b_piece(piece: &Piece) -> bool {
    matches!(piece, Piece::B1 | Piece::B2)
}

fn is_c_piece(piece: &Piece) -> bool {
    matches!(piece, Piece::C1 | Piece::C2)
}

fn is_d_piece(piece: &Piece) -> bool {
    matches!(piece, Piece::D1 | Piece::D2)
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
            vec![(Loc::Hall1, 1), (Loc::RoomAUpper, 2), (Loc::Hall3, 2)],
            vec![(Loc::Hall2, 2), (Loc::RoomAUpper, 2), (Loc::RoomBUpper, 2), (Loc::Hall4, 2)],
            vec![(Loc::Hall3, 2), (Loc::RoomBUpper, 2), (Loc::RoomCUpper, 2), (Loc::Hall5, 2)],
            vec![(Loc::Hall4, 2), (Loc::RoomCUpper, 2), (Loc::RoomDUpper, 2), (Loc::Hall6, 2)],
            vec![(Loc::Hall5, 2), (Loc::RoomDUpper, 2), (Loc::Hall7, 1)],
            vec![(Loc::Hall6, 1)],
            vec![(Loc::Hall2, 2), (Loc::Hall3, 2), (Loc::RoomALower, 1)],
            vec![(Loc::RoomAUpper, 1)],
            vec![(Loc::Hall3, 2), (Loc::Hall4, 2), (Loc::RoomBLower, 1)],
            vec![(Loc::RoomBUpper, 1)],
            vec![(Loc::Hall4, 2), (Loc::Hall5, 2), (Loc::RoomCLower, 1)],
            vec![(Loc::RoomCUpper, 1)],
            vec![(Loc::Hall5, 2), (Loc::Hall6, 2), (Loc::RoomDLower, 1)],
            vec![(Loc::RoomDUpper, 1)],
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
        let mut array: [Vec<Move>; 16] = Default::default();

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
    fn test_part1() {
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
                Piece::B1,   // RoomAUpper
                Piece::A1,   // RoomALower
                Piece::C1,   // RoomBUpper
                Piece::D1,   // RoomBLower
                Piece::B2,   // RoomCUpper
                Piece::C2,   // RoomCLower
                Piece::D2,   // RoomDUpper
                Piece::A2,   // RoomDLower
            ]),
            moves: 0
        });

        assert_eq!(lowest_energy, 12521)
    }

}
