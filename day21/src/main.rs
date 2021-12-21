use std::collections::VecDeque;
use std::cmp;
use itertools::Itertools;

const P1_START: u8 = 10;
const P2_START: u8 = 2;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (rolls, other_score) = play1(P1_START, P2_START);

    println!("Part 1: {} rolls, other player score {}, total: {}", rolls, other_score, rolls * other_score as usize);
}

fn part2() {
    let (win1, win2) = play2(P1_START, P2_START);

    println!("Part 2: player 1 wins {}, player 2 wins {}, max: {}", win1, win2, cmp::max(win1, win2));
}

fn play1(p1_start: u8, p2_start: u8) -> (usize, u16) {
    let mut p1_pos = p1_start;
    let mut p2_pos = p2_start;

    let mut p1_score: u16 = 0;
    let mut p2_score: u16 = 0;

    let mut final_score = (0, 0);

    for full_rolls in &(1..=100).cycle().enumerate().chunks(6) {
        let rolls: Vec<Vec<(usize, usize)>> = full_rolls.chunks(3).into_iter().map(|iter| {
            iter.collect()
        }).collect();

        let r1 = rolls[0].iter().map(|(_, val)| val).sum::<usize>();

        p1_pos = ((((p1_pos as usize + r1) - 1) % 10) + 1) as u8;

        p1_score += p1_pos as u16;

        if p1_score >= 1000 {
            final_score = (rolls[0][2].0 + 1, p2_score);
            break
        }

        let r2 = rolls[1].iter().map(|(_, val)| val).sum::<usize>();

        p2_pos = ((((p2_pos as usize + r2) - 1) % 10) + 1) as u8;

        p2_score += p2_pos as u16;

        if p2_score >= 1000 {
            final_score = (rolls[1][2].0 + 1, p1_score);
            break
        }
    };

    final_score
}

#[derive(Clone)]
struct Player {
    score: u8,
    pos: u8
}

#[derive(Clone)]
struct State {
    players: [Player; 2],
    turn: u8,
    universes: u64
}

const NEW_UNIVERSES: [u64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn play2(p1_start: u8, p2_start: u8) -> (u64, u64) {
    let mut states: VecDeque<State> = VecDeque::with_capacity(1_750_000);
    let mut winning_universes: [u64; 2] = [0, 0];

    states.push_back(State {
        players: [Player { score: 0, pos: p1_start - 1 }, Player { score: 0, pos: p2_start - 1 }],
        turn: 0,
        universes: 1
    });

    while let Some(cur) = states.pop_front() {
        let cur_turn = cur.turn;
        let cur_player_num = cur_turn as usize % 2;
        let cur_player = &cur.players[cur_player_num];
        let cur_pos = cur_player.pos;
        let cur_score = cur_player.score;
        let cur_universes = cur.universes;

        for roll in 3..=9 {
            let next_pos = (cur_pos + roll) % 10;
            let next_score = cur_score + next_pos + 1;
            let next_universes = cur_universes * NEW_UNIVERSES[roll as usize];

            if next_score >= 21 {
                winning_universes[cur_player_num] += next_universes;
            } else {
                let mut players = cur.players.clone();
                players[cur_player_num].pos = next_pos;
                players[cur_player_num].score = next_score;

                states.push_back(State {
                    players,
                    turn: cur_turn + 1,
                    universes: next_universes
                })
            }
        }
    }

    (winning_universes[0], winning_universes[1])
}

#[test]
fn test_part1() {
    let (rolls, other_score) = play1(4, 8);

    assert_eq!(rolls, 993);
    assert_eq!(other_score, 745);
}

#[test]
fn test_part2() {
    let (win1, win2) = play2(4, 8);

    assert_eq!(win1, 444356092776315);
    assert_eq!(win2, 341960390180808);
}
