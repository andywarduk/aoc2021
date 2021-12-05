use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let (numbers, boards) = load_input("input04.txt")?;

    // Run parts
    part1(&numbers, &boards);
    part2(&numbers, &boards);

    Ok(())
}

fn part1(numbers: &[u8], boards: &[Board]) {
    let (completed, last_num, score) = first_rowcol(numbers, boards).unwrap();

    println!("Part 1: winning board is {}, last number {}, score {}", completed, last_num, score);
}

fn part2(numbers: &[u8], boards: &[Board]) {
    let (completed, last_num, score) = last_rowcol(numbers, boards).unwrap();

    println!("Part 2: losing board is {}, last number {}, score {}", completed, last_num, score);
}

struct Board {
    numbers: Vec<Vec<u8>>
}

impl Board {

    fn find_number(&self, n: u8) -> Option<(usize, usize)> {
        for (r, row) in self.numbers.iter().enumerate() {
            for (c, &num) in row.iter().enumerate() {
                if num == n {
                    return Some((r, c));
                }
            }
        }

        None
    }

}

type GameLine = Vec<bool>;
type GameBoard = Vec<GameLine>;

struct Game<'a> {
    boards: &'a [Board],
    marks: Vec<GameBoard>,
    complete: Vec<bool>
}

impl<'a> Game<'a> {

    fn new(boards: &'a [Board]) -> Self {
        let board_height = boards[0].numbers.len();
        let board_width = boards[0].numbers[0].len();
    
        Game {
            boards,
            marks: vec![vec![vec![false; board_width]; board_height]; boards.len()],
            complete: vec![false; boards.len()]
        }
    }

    fn mark_number(&mut self, called: u8) -> Option<Vec<usize>> {
        let mut completed = None;

        for (b, board) in self.boards.iter().enumerate() {
            if let Some((r,c)) = board.find_number(called) {
                self.marks[b][r][c] = true;

                if !self.complete[b] {
                    let mut row_complete = true;

                    for mark in &self.marks[b][r] {
                        if !mark {
                            row_complete = false;
                            break
                        }
                    }

                    let mut col_complete = true;

                    for row in &self.marks[b] {
                        if !row[c] {
                            col_complete = false;
                            break
                        }
                    }

                    if row_complete || col_complete {
                        if completed.is_none() {
                            completed = Some(Vec::new());
                        }
                        completed.as_mut().unwrap().push(b);
                        self.complete[b] = true;
                    }
                }
            }
        }

        completed
    }

}

fn first_rowcol(numbers: &[u8], boards: &[Board]) -> Option<(usize, u8, u32)> {
    let mut game = Game::new(boards);

    for &n in numbers {
        if let Some(completed) = game.mark_number(n) {
            assert_eq!(1, completed.len());
            let score = score_board(&game, completed[0], n);
            return Some((completed[0], n, score));
        }
    }

    None
}

fn last_rowcol(numbers: &[u8], boards: &[Board]) -> Option<(usize, u8, u32)> {
    let mut game = Game::new(boards);
    let mut boards_left = game.boards.len();

    for &n in numbers {
        if let Some(completed) = game.mark_number(n) {
            boards_left -= completed.len();

            if boards_left == 0 {
                assert_eq!(1, completed.len());
                let score = score_board(&game, completed[0], n);
                return Some((completed[0], n, score));
            }
        }
    }

    None
}

fn score_board(game: &Game, board: usize, last_num: u8) -> u32 {
    let mut score = 0;

    for (r, row) in game.marks[board].iter().enumerate() {
        for (c, mark) in row.iter().enumerate() {
            if !mark {
                score += game.boards[board].numbers[r][c] as u32;
            }
        }
    }

    score * last_num as u32
}

fn load_input(file: &str) -> Result<(Vec<u8>, Vec<Board>), Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

enum ParseStage {
    Numbers,
    Board
}

fn load_buf(buf: &[u8]) -> Result<(Vec<u8>, Vec<Board>), Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    let mut numbers = Vec::new();
    let mut boards = Vec::new();

    let mut parse_stage = ParseStage::Numbers;
    let mut cur_board: Option<Board> = None;

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            if let Some(board) = cur_board {
                boards.push(board);
                cur_board = None;
            }
        } else {
            match parse_stage {
                ParseStage::Numbers => {
                    numbers = line.split(',')
                        .map(|s| s.parse::<u8>())
                        .collect::<Result<Vec<u8>, _>>()?;

                    parse_stage = ParseStage::Board;
                }
                ParseStage::Board => {
                    let board = match cur_board.as_mut() {
                        None => {
                            cur_board = Some(Board {
                                numbers: Vec::new()
                            });
                            cur_board.as_mut().unwrap()
                        }
                        Some(board) => board
                    };

                    let str_nums = line
                        .as_bytes()
                        .chunks(3)
                        .map(std::str::from_utf8)
                        .collect::<Result<Vec<&str>, _>>()?;

                    let nums = str_nums
                        .iter()
                        .map(|s| s.trim().parse::<u8>())
                        .collect::<Result<Vec<u8>, _>>()?;
                    
                    board.numbers.push(nums);
                }
            }
        }
    }
   
    if let Some(board) = cur_board {
        boards.push(board);
    }

    Ok((numbers, boards))
}

#[cfg(test)]
const TEST_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[test]
fn test_part1() {
    // Load the input file
    let (numbers, boards) = load_buf(TEST_INPUT.as_bytes()).unwrap();

    let (completed, last_num, score) = first_rowcol(&numbers, &boards).unwrap();

    assert_eq!(2, completed, "3rd board should win");
    assert_eq!(24, last_num, "Last number incorrect");
    assert_eq!(4512, score, "Incorrect score");
}

#[test]
fn test_part2() {
    // Load the input file
    let (numbers, boards) = load_buf(TEST_INPUT.as_bytes()).unwrap();

    let (completed, last_num, score) = last_rowcol(&numbers, &boards).unwrap();

    assert_eq!(1, completed, "2nd board should lose");
    assert_eq!(13, last_num, "Last number incorrect");
    assert_eq!(1924, score, "Incorrect score");
}
