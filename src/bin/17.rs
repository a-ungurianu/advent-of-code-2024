use std::{collections::HashSet, fs::read};

advent_of_code::solution!(17);

fn read_reg(line: &str) -> u64 {
    line.split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn op_s(o: &u64) -> &'static str {
    match o {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "A",
        5 => "B",
        6 => "C",
        _ => "!",
    }
}

fn print_program(program: &Vec<u64>) {
    let mut rest = &program[..];

    while !rest.is_empty() {
        match rest {
            [0, op, r @ ..] => {
                println!("adv {}", op_s(op));
                rest = r;
            }
            [1, op, r @ ..] => {
                println!("bxl {}", op_s(op));
                rest = r;
            }
            [2, op, r @ ..] => {
                println!("bst {}", op_s(op));
                rest = r;
            }
            [3, op, r @ ..] => {
                println!("jnz {}", op_s(op));
                rest = r;
            }
            [4, op, r @ ..] => {
                println!("bxc {}", op_s(op));
                rest = r;
            }
            [5, op, r @ ..] => {
                println!("out {}", op_s(op));
                rest = r;
            }
            [6, op, r @ ..] => {
                println!("bdv {}", op_s(op));
                rest = r;
            }
            [7, op, r @ ..] => {
                println!("cdv {}", op_s(op));
                rest = r;
            }
            _ => {
                panic!("uneven")
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    inst_p: usize,
}

impl State {
    fn get_op(self: &Self, op: &u64) -> u64 {
        match op {
            0..4 => *op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unknown operand"),
        }
    }
}

fn run_program(
    init_reg_a: u64,
    init_reg_b: u64,
    init_reg_c: u64,
    program: &Vec<u64>,
    check_for_similarity: bool,
) -> (State, Vec<u64>) {
    let mut output: Vec<u64> = Vec::new();

    let mut state = State {
        a: init_reg_a,
        b: init_reg_b,
        c: init_reg_c,
        inst_p: 0,
    };

    let mut visited_states: HashSet<State> = HashSet::new();

    while state.inst_p < program.len() {
        if visited_states.contains(&state) {
            break;
        }
        visited_states.insert(state.clone());
        let [inst, op] = &program[state.inst_p..state.inst_p + 2] else {
            panic!("Can't unpack")
        };

        match inst {
            0 => {
                state.a = state.a >> state.get_op(op);
                state.inst_p += 2;
            }
            1 => {
                state.b = state.b ^ op;
                state.inst_p += 2;
            }
            2 => {
                state.b = state.get_op(op) & 7;
                state.inst_p += 2;
            }
            3 => {
                if state.a != 0 {
                    state.inst_p = *op as usize;
                } else {
                    state.inst_p += 2;
                }
            }
            4 => {
                state.b = state.b ^ state.c;
                state.inst_p += 2;
            }
            5 => {
                let out = state.get_op(op) & 7;

                let target = program.get(output.len());
                if check_for_similarity && target != Some(&out) {
                    break;
                }
                output.push(state.get_op(op) & 7);
                state.inst_p += 2;
            }
            6 => {
                state.b = state.a >> state.get_op(op);
                state.inst_p += 2;
            }
            7 => {
                state.c = state.a >> state.get_op(op);
                state.inst_p += 2;
            }
            _ => panic!("Unknown opcode"),
        }
    }

    return (state, output);
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let reg_a = lines.next().map(read_reg).unwrap();
    let reg_b = lines.next().map(read_reg).unwrap();
    let reg_c = lines.next().map(read_reg).unwrap();
    _ = lines.next();

    let program: Vec<_> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    let (_, output) = run_program(reg_a, reg_b, reg_c, &program, false);

    Some(
        output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn to_triple(num: u64) -> String {
    let mut res = vec![];
    let mut num = num;
    while num > 0 {
        res.push((num & 7).to_string());
        num = num >> 3;
    }
    res.reverse();
    res.join("_")
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let reg_a = lines.next().map(read_reg).unwrap();
    let reg_b = lines.next().map(read_reg).unwrap();
    let reg_c = lines.next().map(read_reg).unwrap();
    _ = lines.next();

    let program: Vec<_> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    let mut max_len = 0;

    for a in 0.. {
        if a % 1000 == 0 {
            // println!("Reg a: {:?}", a);
        }
        let (_, output) = run_program(a, reg_b, reg_c, &program, true);
        if output.len() == program.len() {
            return Some(a.to_string());
        } else {
            if output.len() >= max_len {
                println!(
                    "a={}; {}",
                    to_triple(a),
                    output
                        .iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
                max_len = output.len();
            }
        }
    }

    return Some("NOT_FOUND".to_string());
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        );
        assert_eq!(result, Some("117440".to_string()));
    }

    #[test]
    fn test_bst() {
        let (state, _) = run_program(0, 0, 9, &vec![2, 6], false);
        assert_eq!(state.b, 1);
    }
    #[test]
    fn test_out() {
        let (_, output) = run_program(10, 0, 9, &vec![5, 0, 5, 1, 5, 4], false);
        assert_eq!(output, vec![0, 1, 2]);
    }
    #[test]
    fn test_prog1() {
        let (state, output) = run_program(2024, 0, 0, &vec![0, 1, 5, 4, 3, 0], false);
        assert_eq!(state.a, 0);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }
    #[test]
    fn test_prog2() {
        let (state, _) = run_program(0, 0, 9, &vec![2, 6], false);
        assert_eq!(state.b, 1);
    }
    #[test]
    fn test_bxl() {
        let (state, _) = run_program(0, 29, 0, &vec![1, 7], false);
        assert_eq!(state.b, 26);
    }
    #[test]
    fn test_bxc() {
        let (state, _) = run_program(0, 2024, 43690, &vec![4, 0], false);
        assert_eq!(state.b, 44354);
    }
}
