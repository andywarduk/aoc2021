mod load;
mod instruction;

use std::error::Error;

use instruction::{Instruction, Reg, RegNum};

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let program = load::load_input("input24.txt")?;

    // Run parts
    part1(&program);
    part2(&program);

    Ok(())
}

fn part1(program: &[Instruction]) {
    let num_vec = solve(program, true);

    match run_program(program, &num_vec) {
        Ok(state) => {
            if state.z == 0 {
                println!("Part 1: Max serial number: {}", 
                    num_vec.iter().map(|d| (b'0' + *d as u8) as char).collect::<String>());
            } else {
                panic!("Z not zero")
            }
        },
        Err(e) => panic!("Program failed: {:?}", e)
    }
}

fn part2(program: &[Instruction]) {
    let num_vec = solve(program, false);

    match run_program(program, &num_vec) {
        Ok(state) => {
            if state.z == 0 {
                println!("Part 2: Min serial number: {}", 
                    num_vec.iter().map(|d| (b'0' + *d as u8) as char).collect::<String>());
            } else {
                panic!("Z not zero")
            }
        },
        Err(e) => panic!("Program failed: {:?}", e)
    }
}

fn solve(program: &[Instruction], max: bool) -> Vec<i64> {
    let mut terms = Vec::new();

    for block in 0..14 {
        let start_ins = 18 * block;

        let t1 = if let Instruction::Div(Reg::Z, RegNum::Num(x)) = program[start_ins + 4] {
            x
        } else {
            panic!("t1 invalid")
        };

        let t2 = if let Instruction::Add(Reg::X, RegNum::Num(x)) = program[start_ins + 5] {
            x
        } else {
            panic!("t2 invalid")
        };

        let t3 = if let Instruction::Add(Reg::Y, RegNum::Num(x)) = program[start_ins + 15] {
            x
        } else {
            panic!("t3 invalid")
        };

        terms.push((t1, t2, t3));
    }

    let mut digits = vec![0; 14];
    let mut prevs = Vec::new();

    for (i, (t1, t2, t3)) in terms.iter().enumerate() {
        if *t1 == 1 {
            prevs.push((i, t3))
        } else {
            let (prev_i, prev_t3) = prevs.pop().unwrap();
            let complement = prev_t3 + t2;
            if max {
                digits[prev_i] = std::cmp::min(9, 9 - complement);
            } else {
                digits[prev_i] = std::cmp::max(1, 1 - complement);
            }
            digits[i] = digits[prev_i] + complement;
        }
    }

    digits
}

#[derive(Debug)]
enum Exception {
    DivByZero,
    InvalidMod,
    NoInput
}

#[derive(Debug, Default)]
struct State {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

fn run_program(program: &[Instruction], input: &[i64]) -> Result<State, Exception> {
    let mut res = Ok(());
    let mut cur_input = 0;

    let mut state = State::default();

    let get_reg = |state: &State, reg: &Reg| -> i64 {
        match reg {
            Reg::W => state.w,
            Reg::X => state.x,
            Reg::Y => state.y,
            Reg::Z => state.z,
        }
    };

    let get_regnum = |state: &State, regnum: &RegNum| -> i64 {
        match regnum {
            RegNum::Reg(reg) => get_reg(state, reg),
            RegNum::Num(num) => *num
        }
    };

    let set_reg = |state: &mut State, reg: &Reg, val: i64| {
        match reg {
            Reg::W => state.w = val,
            Reg::X => state.x = val,
            Reg::Y => state.y = val,
            Reg::Z => state.z = val,
        }
    };

    let mut input = || -> Option<i64> {
        if cur_input < input.len() {
            let res = Some(input[cur_input]);
            cur_input += 1;
            res
        } else {
            None
        }
    };

    for ins in program {
        res = match ins {
            Instruction::Inp(reg) => {
                if let Some(v1) = input() {
                    set_reg(&mut state, reg, v1);
                    Ok(())
                } else {
                    Err(Exception::NoInput)
                }
            }
            Instruction::Add(reg, regnum) => {
                let v1 = get_reg(&state, reg);
                let v2 = get_regnum(&state, regnum);

                set_reg(&mut state, reg, v1 + v2);
                Ok(())
            },
            Instruction::Mul(reg, regnum) => {
                let v1 = get_reg(&state, reg);
                let v2 = get_regnum(&state, regnum);

                set_reg(&mut state, reg, v1 * v2);
                Ok(())
            },
            Instruction::Div(reg, regnum) => {
                let v1 = get_reg(&state, reg);
                let v2 = get_regnum(&state, regnum);

                if v2 == 0 {
                    Err(Exception::DivByZero)
                } else {
                    set_reg(&mut state, reg, v1 / v2);
                    Ok(())
                }
            },
            Instruction::Mod(reg, regnum) => {
                let v1 = get_reg(&state, reg);
                let v2 = get_regnum(&state, regnum);

                if v1 < 0 || v2 <= 0{
                    Err(Exception::InvalidMod)
                } else {
                    set_reg(&mut state, reg, v1 % v2);
                    Ok(())
                }
            },
            Instruction::Eql(reg, regnum) => {
                let v1 = get_reg(&state, reg);
                let v2 = get_regnum(&state, regnum);

                set_reg(&mut state, reg, if v1 == v2 { 1 } else { 0 });
                Ok(())
            }
        };

        if res.is_err() {
            break
        }
    }

    if res.is_err() {
        Err(res.err().unwrap())
    } else {
        Ok(state)
    }
}
