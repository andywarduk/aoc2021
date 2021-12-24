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
    piece_map: EnumMap<Piece, Loc>,
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    energy_used: u64,
    board: Board
}

struct Move {
    dest: Loc,
    dist: u8,
    via: Vec<Loc>,
    checkfn: fn(piece: &Piece, board: &Board) -> bool
}

fn main() {
    let init_board = Board {
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
        piece_map: EnumMap::from_array([
            Loc::None,       // None
            Loc::RoomBLower, // A1
            Loc::RoomCLower, // A2
            Loc::RoomAUpper, // B1
            Loc::RoomBUpper, // B2
            Loc::RoomALower, // C1
            Loc::RoomDLower, // C2
            Loc::RoomCUpper, // D1
            Loc::RoomDUpper, // D2
        ])
    };

    let mut lowest_energy = None;

    let mut states = PriorityQueue::new();

    states.push(State {
        energy_used: 0,
        board: init_board.clone()
    }, Reverse(0));
    
    while let Some((state, _)) = states.pop() {
        let board = &state.board;

        //println!("Current board:\n{}", board);

        // Calculate moves
        for (piece, from_loc) in state.board.piece_map.iter().skip(1) {
            for mv in &MOVES[*from_loc] {
                // Check this move is possible
                if !(mv.checkfn)(&piece, board) {
                    continue
                }

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

                // Calculate and check energy
                let energy = state.energy_used + move_cost(&piece) * mv.dist as u64;

                if let Some(lowest) = lowest_energy {
                    if energy > lowest {
                        continue
                    }
                }

                // println!("Move {:?} from {:?} to {:?} possible, dist {}, energy {}",
                // piece, from_loc, mv.dest, mv.dist, energy);

                // Build new state
                let mut new_state = state.clone();

                new_state.energy_used = energy;

                new_state.board.loc_map[*from_loc] = Piece::None;
                new_state.board.loc_map[mv.dest] = piece;

                new_state.board.piece_map[piece] = mv.dest;

                //println!("New board after {:?} from {:?} to {:?}:\n{}", piece, from_loc, mv.dest, new_state.board);

                // Finished?
                if a_filled(&new_state.board) && b_filled(&new_state.board) && 
                    c_filled(&new_state.board) && d_filled(&new_state.board) {
                    println!("Solution found with energy {}", new_state.energy_used);
                    lowest_energy = Some(energy);
                } else {
                    states.push(new_state, Reverse(energy));
                }
            }
        }
    }
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

fn is_a_piece_and_lower_filled_cb(piece: &Piece, board: &Board) -> bool {
    is_a_piece(piece) && is_a_piece(&board.loc_map[Loc::RoomALower])
}

fn is_a_piece_cb(piece: &Piece, _board: &Board) -> bool {
    is_a_piece(piece)
}

fn is_b_piece_and_lower_filled_cb(piece: &Piece, board: &Board) -> bool {
    is_b_piece(piece) && is_b_piece(&board.loc_map[Loc::RoomBLower])
}

fn is_b_piece_cb(piece: &Piece, _board: &Board) -> bool {
    is_b_piece(piece)
}

fn is_c_piece_and_lower_filled_cb(piece: &Piece, board: &Board) -> bool {
    is_c_piece(piece) && is_c_piece(&board.loc_map[Loc::RoomCLower])
}

fn is_c_piece_cb(piece: &Piece, _board: &Board) -> bool {
    is_c_piece(piece)
}

fn is_d_piece_and_lower_filled_cb(piece: &Piece, board: &Board) -> bool {
    is_d_piece(piece) && is_d_piece(&board.loc_map[Loc::RoomDLower])
}

fn is_d_piece_cb(piece: &Piece, _board: &Board) -> bool {
    is_d_piece(piece)
}

fn a_not_filled_cb(_piece: &Piece, board: &Board) -> bool {
    !a_filled(board)
}

fn b_not_filled_cb(_piece: &Piece, board: &Board) -> bool {
    !b_filled(board)
}

fn c_not_filled_cb(_piece: &Piece, board: &Board) -> bool {
    !c_filled(board)
}

fn d_not_filled_cb(_piece: &Piece, board: &Board) -> bool {
    !d_filled(board)
}

fn not_a_cb(piece: &Piece, _board: &Board) -> bool {
    !is_a_piece(piece)
}

fn not_b_cb(piece: &Piece, _board: &Board) -> bool {
    !is_b_piece(piece)
}

fn not_c_cb(piece: &Piece, _board: &Board) -> bool {
    !is_c_piece(piece)
}

fn not_d_cb(piece: &Piece, _board: &Board) -> bool {
    !is_d_piece(piece)
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
    static ref MOVES: EnumMap<Loc, Vec<Move>> = {
        EnumMap::from_array([
            vec![], // None
            vec![   // H1
                Move { dest: Loc::RoomAUpper, dist: 3, via: vec![
                        Loc::Hall2
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 4, via: vec![
                        Loc::Hall2, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 5, via: vec![
                        Loc::Hall2, Loc::Hall3
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 6, via: vec![
                        Loc::Hall2, Loc::Hall3, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 7, via: vec![
                        Loc::Hall2, Loc::Hall3, Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 8, via: vec![
                        Loc::Hall2, Loc::Hall3, Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 9, via: vec![
                        Loc::Hall2, Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 10, via: vec![
                        Loc::Hall2, Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H2
                Move { dest: Loc::RoomAUpper, dist: 2, via: vec![
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 3, via: vec![
                        Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 5, via: vec![
                        Loc::Hall3, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 6, via: vec![
                        Loc::Hall3, Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 7, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 8, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 9, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H3
                Move { dest: Loc::RoomAUpper, dist: 2, via: vec![
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 3, via: vec![
                        Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 2, via: vec![
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 3, via: vec![
                        Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 5, via: vec![
                        Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H4
                Move { dest: Loc::RoomAUpper, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 5, via: vec![
                        Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 2, via: vec![
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 3, via: vec![
                        Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 2, via: vec![
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 3, via: vec![
                        Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 4, via: vec![
                        Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 5, via: vec![
                        Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H5
                Move { dest: Loc::RoomAUpper, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 5, via: vec![
                        Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 2, via: vec![
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 3, via: vec![
                        Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 2, via: vec![
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 3, via: vec![
                        Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H6
                Move { dest: Loc::RoomAUpper, dist: 8, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 9, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 6, via: vec![
                        Loc::Hall5, Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 7, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 4, via: vec![
                        Loc::Hall5
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 5, via: vec![
                        Loc::Hall5, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 2, via: vec![
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 3, via: vec![
                        Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // H7
                Move { dest: Loc::RoomAUpper, dist: 9, via: vec![
                        Loc::Hall6, Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomALower, dist: 10, via: vec![
                        Loc::Hall6, Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb },
                Move { dest: Loc::RoomBUpper, dist: 7, via: vec![
                        Loc::Hall6, Loc::Hall5, Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomBLower, dist: 8, via: vec![
                        Loc::Hall6, Loc::Hall5, Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb },
                Move { dest: Loc::RoomCUpper, dist: 5, via: vec![
                        Loc::Hall6, Loc::Hall5
                    ], checkfn: is_c_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomCLower, dist: 6, via: vec![
                        Loc::Hall6, Loc::Hall5, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb },
                Move { dest: Loc::RoomDUpper, dist: 3, via: vec![
                        Loc::Hall6
                    ], checkfn: is_d_piece_and_lower_filled_cb },
                Move { dest: Loc::RoomDLower, dist: 4, via: vec![
                        Loc::Hall6, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb }
            ],
            vec![   // AU
                Move { dest: Loc::Hall1, dist: 3, via: vec![
                        Loc::Hall2
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall2, dist: 2, via: vec![
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall3, dist: 2, via: vec![
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall4, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall5, dist: 6, via: vec![
                        Loc::Hall3, Loc::Hall4
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall6, dist: 8, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: a_not_filled_cb},
                Move { dest: Loc::Hall7, dist: 9, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::Hall6
                    ], checkfn: a_not_filled_cb},

                Move { dest: Loc::RoomBLower, dist: 5, via: vec![
                        Loc::Hall3, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 7, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 6, via: vec![
                        Loc::Hall3, Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 9, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 8, via: vec![
                        Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // AL
                Move { dest: Loc::Hall1, dist: 4, via: vec![
                        Loc::RoomAUpper, Loc::Hall2
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall2, dist: 3, via: vec![
                        Loc::RoomAUpper
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall3, dist: 3, via: vec![
                        Loc::RoomAUpper
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall4, dist: 5, via: vec![
                        Loc::RoomAUpper, Loc::Hall3
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall5, dist: 7, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall6, dist: 9, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: not_a_cb},
                Move { dest: Loc::Hall7, dist: 10, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::Hall6
                    ], checkfn: not_a_cb},

                Move { dest: Loc::RoomBLower, dist: 6, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 5, via: vec![
                        Loc::RoomAUpper, Loc::Hall3
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 8, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 7, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 10, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 9, via: vec![
                        Loc::RoomAUpper, Loc::Hall3, Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // BU
                Move { dest: Loc::Hall1, dist: 5, via: vec![
                        Loc::Hall3, Loc::Hall2
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall2, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall3, dist: 2, via: vec![
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall4, dist: 2, via: vec![
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall5, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall6, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall5
                    ], checkfn: b_not_filled_cb},
                Move { dest: Loc::Hall7, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall5, Loc::Hall6
                    ], checkfn: b_not_filled_cb},

                Move { dest: Loc::RoomALower, dist: 5, via: vec![
                        Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 4, via: vec![
                        Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 5, via: vec![
                        Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // BL
                Move { dest: Loc::Hall1, dist: 6, via: vec![
                        Loc::RoomBUpper, Loc::Hall3, Loc::Hall2
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall2, dist: 5, via: vec![
                        Loc::RoomBUpper, Loc::Hall3
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall3, dist: 3, via: vec![
                        Loc::RoomBUpper
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall4, dist: 3, via: vec![
                        Loc::RoomBUpper
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall5, dist: 5, via: vec![
                        Loc::RoomBUpper, Loc::Hall4
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall6, dist: 7, via: vec![
                        Loc::RoomBUpper, Loc::Hall4, Loc::Hall5
                    ], checkfn: not_b_cb},
                Move { dest: Loc::Hall7, dist: 8, via: vec![
                        Loc::RoomBUpper, Loc::Hall4, Loc::Hall5, Loc::Hall6
                    ], checkfn: not_b_cb},

                Move { dest: Loc::RoomALower, dist: 6, via: vec![
                        Loc::RoomBUpper, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 5, via: vec![
                        Loc::RoomBUpper, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 6, via: vec![
                        Loc::RoomBUpper, Loc::Hall4, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 5, via: vec![
                        Loc::RoomBUpper, Loc::Hall4
                    ], checkfn: is_c_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 8, via: vec![
                        Loc::RoomBUpper, Loc::Hall4, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 7, via: vec![
                        Loc::RoomBUpper, Loc::Hall4, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // CU
                Move { dest: Loc::Hall1, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall3, Loc::Hall2
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall2, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall3
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall3, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall4, dist: 2, via: vec![
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall5, dist: 2, via: vec![
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall6, dist: 4, via: vec![
                        Loc::Hall5
                    ], checkfn: c_not_filled_cb},
                Move { dest: Loc::Hall7, dist: 5, via: vec![
                        Loc::Hall5, Loc::Hall6
                    ], checkfn: c_not_filled_cb},

                Move { dest: Loc::RoomALower, dist: 7, via: vec![
                        Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 6, via: vec![
                        Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomBLower, dist: 5, via: vec![
                        Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 4, via: vec![
                        Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 5, via: vec![
                        Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 4, via: vec![
                        Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // CL
                Move { dest: Loc::Hall1, dist: 8, via: vec![
                        Loc::RoomCUpper, Loc::Hall4, Loc::Hall3, Loc::Hall2
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall2, dist: 7, via: vec![
                        Loc::RoomCUpper, Loc::Hall4, Loc::Hall3
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall3, dist: 5, via: vec![
                        Loc::RoomCUpper, Loc::Hall4
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall4, dist: 3, via: vec![
                        Loc::RoomCUpper
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall5, dist: 3, via: vec![
                        Loc::RoomCUpper
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall6, dist: 5, via: vec![
                        Loc::RoomCUpper, Loc::Hall5
                    ], checkfn: not_c_cb},
                Move { dest: Loc::Hall7, dist: 6, via: vec![
                        Loc::RoomCUpper, Loc::Hall5, Loc::Hall6
                    ], checkfn: not_c_cb},

                Move { dest: Loc::RoomALower, dist: 8, via: vec![
                        Loc::RoomCUpper, Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 7, via: vec![
                        Loc::RoomCUpper, Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomBLower, dist: 6, via: vec![
                        Loc::RoomCUpper, Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 5, via: vec![
                        Loc::RoomCUpper, Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomDLower, dist: 6, via: vec![
                        Loc::RoomCUpper, Loc::Hall5, Loc::RoomDUpper
                    ], checkfn: is_d_piece_cb},
                Move { dest: Loc::RoomDUpper, dist: 5, via: vec![
                        Loc::RoomCUpper, Loc::Hall5
                    ], checkfn: is_d_piece_and_lower_filled_cb},
            ],
            vec![   // DU
                Move { dest: Loc::Hall1, dist: 9, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::Hall2
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall2, dist: 8, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall3, dist: 6, via: vec![
                        Loc::Hall5, Loc::Hall4
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall4, dist: 4, via: vec![
                        Loc::Hall5 
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall5, dist: 2, via: vec![
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall6, dist: 2, via: vec![
                    ], checkfn: d_not_filled_cb},
                Move { dest: Loc::Hall7, dist: 3, via: vec![
                        Loc::Hall6
                    ], checkfn: d_not_filled_cb},

                Move { dest: Loc::RoomALower, dist: 9, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 8, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomBLower, dist: 7, via: vec![
                        Loc::Hall5, Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 6, via: vec![
                        Loc::Hall5, Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 5, via: vec![
                        Loc::Hall5, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 4, via: vec![
                        Loc::Hall5
                    ], checkfn: is_c_piece_and_lower_filled_cb},
            ],
            vec![   // DL
                Move { dest: Loc::Hall1, dist: 10, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::Hall2
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall2, dist: 9, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall3, dist: 7, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall4, dist: 5, via: vec![
                        Loc::RoomDUpper, Loc::Hall5
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall5, dist: 3, via: vec![
                        Loc::RoomDUpper
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall6, dist: 3, via: vec![
                        Loc::RoomDUpper
                    ], checkfn: not_d_cb},
                Move { dest: Loc::Hall7, dist: 4, via: vec![
                        Loc::RoomDUpper, Loc::Hall6
                    ], checkfn: not_d_cb},

                Move { dest: Loc::RoomALower, dist: 10, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4, Loc::Hall3, Loc::RoomAUpper
                    ], checkfn: is_a_piece_cb},
                Move { dest: Loc::RoomAUpper, dist: 9, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4, Loc::Hall3
                    ], checkfn: is_a_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomBLower, dist: 8, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4, Loc::RoomBUpper
                    ], checkfn: is_b_piece_cb},
                Move { dest: Loc::RoomBUpper, dist: 7, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::Hall4
                    ], checkfn: is_b_piece_and_lower_filled_cb},
                Move { dest: Loc::RoomCLower, dist: 6, via: vec![
                        Loc::RoomDUpper, Loc::Hall5, Loc::RoomCUpper
                    ], checkfn: is_c_piece_cb},
                Move { dest: Loc::RoomCUpper, dist: 5, via: vec![
                        Loc::RoomDUpper, Loc::Hall5
                    ], checkfn: is_c_piece_and_lower_filled_cb},
            ],
        ])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

    }

}
