use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let code = load_input("input10.txt")?;

    // Run parts
    let (score1, score2) = score_syntax_errors(&code);

    println!("Part 1: Syntax checker score: {}", score1);
    println!("Part 2: Autcorrect score: {}", score2);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    OpenRound,
    OpenSquare,
    OpenCurly,
    OpenAngle,
    ClosedRound,
    ClosedSquare,
    ClosedCurly,
    ClosedAngle,
}

impl Token {

    fn is_open(&self) -> bool {
        matches!(self, Token::OpenRound | Token::OpenSquare | Token::OpenCurly | Token::OpenAngle)
    }

    fn opposite(&self) -> Token {
        match self {
            Token::OpenRound => Token::ClosedRound,
            Token::OpenSquare => Token::ClosedSquare,
            Token::OpenCurly => Token::ClosedCurly,
            Token::OpenAngle => Token::ClosedAngle,
            Token::ClosedRound => Token::OpenRound,
            Token::ClosedSquare => Token::OpenSquare,
            Token::ClosedCurly => Token::OpenCurly,
            Token::ClosedAngle => Token::OpenAngle,
        }
    }

}

impl From<char> for Token {

    fn from(c: char) -> Self {
        match c {
            '(' => Token::OpenRound,
            '[' => Token::OpenSquare,
            '{' => Token::OpenCurly,
            '<' => Token::OpenAngle,
            ')' => Token::ClosedRound,
            ']' => Token::ClosedSquare,
            '}' => Token::ClosedCurly,
            '>' => Token::ClosedAngle,
            _ => panic!("Invalid bracket")
        }
    }

}

fn score_syntax_errors(code: &[Vec<char>]) -> (usize, usize) {
    let mut score1 = 0;
    let mut part2_scores = Vec::new();

    for line in code {
        let mut stack = Vec::new();

        for &token_char in line {
            let token: Token = token_char.into();

            if token.is_open() {
                stack.push(token);
                continue
            }

            if stack.is_empty() {
                panic!("Empty stack!")
            }

            if token.opposite() != *stack.last().unwrap() {
                score1 += match token {
                    Token::ClosedRound => 3,
                    Token::ClosedSquare => 57,
                    Token::ClosedCurly => 1197,
                    Token::ClosedAngle => 25137,
                    _ => panic!("Unexpected char")
                };
                
                stack.clear();
                break
            }

            stack.pop();
        }

        if !stack.is_empty() {
            let mut score2 = 0;

            while let Some(e) = stack.pop() {
                score2 = (score2 * 5) + match e {
                    Token::OpenRound => 1,
                    Token::OpenSquare => 2,
                    Token::OpenCurly => 3,
                    Token::OpenAngle => 4,
                    _ => panic!("Invalid bracket in stack")
                };
            }

            part2_scores.push(score2);
        }
    }

    part2_scores.sort_unstable();
    let score2 = part2_scores[(part2_scores.len() - 1) / 2];

    (score1, score2)
}

type ParseResult = Vec<Vec<char>>;

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

fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vector
    let mut code = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            code.push(line.chars().collect::<Vec<char>>());
        }
    }
   
    Ok(code)
}

#[test]
fn test_scoring() {
    let test_input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    let code = load_buf(test_input.as_bytes()).unwrap();

    // Run parts
    let (score1, score2) = score_syntax_errors(&code);

    assert_eq!(score1, 26397, "Part 1 score incorrect");
    assert_eq!(score2, 288957, "Part 2 score incorrect");
}
