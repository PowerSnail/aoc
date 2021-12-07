// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{prelude::*, std_iter};
use std::str::FromStr;

// --- Parsings ---

fn char_to_step(c: u8) -> Option<i64> {
    match c {
        b'(' => Some(1),
        b')' => Some(-1),
        _ => None,
    }
}

pub fn day1_part1() {
    let total: i64 = std_iter!(Bytes).filter_map(char_to_step).sum();
    println!("{}", total);
}

pub fn day1_part2() {
    let position: usize = std_iter!(Bytes)
        .filter_map(char_to_step)
        .scan(0, |current, step| {
            *current += step;
            Some(*current)
        })
        .find_position(|c| *c == -1)
        .unwrap()
        .0
        + 1;
    println!("{}", position);
}

pub fn day2_part1() {
    let total: i64 = std_iter!(Lines)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let (total_sa, min_sa) = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().product())
                .fold((0, i64::MAX), |(total, min), s| (total + s * 2, min.min(s)));
            total_sa + min_sa
        })
        .sum();
    println!("{}", total);
}

pub fn day2_part2() {
    let total: i64 = std_iter!(Lines)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let volume: i64 = numbers.iter().product();
            let shortest_waist: i64 = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().sum::<i64>())
                .min()
                .unwrap()
                * 2i64;
            shortest_waist + volume
        })
        .sum();
    println!("{}", total);
}

pub fn day3_part1() {
    let count = std_iter!(Bytes)
        .scan((0, 0), |(x, y), step| {
            match step {
                b'>' => *x += 1,
                b'<' => *x -= 1,
                b'^' => *y += 1,
                b'v' => *y -= 1,
                _ => unreachable!(),
            };
            Some((*x, *y))
        })
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();
    println!("{}", count);
}

pub fn day3_part2() {
    let (script_santa, script_robot) = std_iter!(Bytes).tee();

    let count = vec![script_santa.skip(0), script_robot.skip(1)]
        .into_iter()
        .map(|instructions| {
            instructions.step_by(2).scan((0, 0), |(x, y), step| {
                match step {
                    b'>' => *x += 1,
                    b'<' => *x -= 1,
                    b'^' => *y += 1,
                    b'v' => *y -= 1,
                    _ => unreachable!(),
                };
                Some((*x, *y))
            })
        })
        .flatten()
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();

    println!("{}", count);
}

pub fn day4_part1() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("00000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day4_part2() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("000000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day5_part1() {
    let count = std_iter!(Lines)
        .filter(|line| {
            let vowel_count = line
                .bytes()
                .filter(|&b| match b {
                    b'a' | b'e' | b'i' | b'o' | b'u' => true,
                    _ => false,
                })
                .count();

            let twice_in_a_row = line.bytes().tuple_windows().any(|(x, y)| x == y);

            let contains_bad_sub = line.bytes().tuple_windows().any(|(x, y)| match &[x, y] {
                b"ab" | b"cd" | b"pq" | b"xy" => true,
                _ => false,
            });

            (vowel_count >= 3) && twice_in_a_row && (!contains_bad_sub)
        })
        .count();
    println!("{}", count);
}

pub fn day5_part2() {
    let count = std_iter!(Lines)
        .filter(|line| {
            let two_grams = line
                .bytes()
                .tuple_windows::<(u8, u8)>()
                .enumerate()
                .scan(HashMap::new(), |first_positions, (i, pair)| {
                    if !first_positions.contains_key(&pair) {
                        first_positions.insert(pair, i);
                    }
                    let first_pos = first_positions.get(&pair).unwrap();
                    Some(i - first_pos > 1)
                })
                .any(|x| x);

            let sandwiched = line.bytes().tuple_windows().any(|(x, _, z)| x == z);
            sandwiched && two_grams
        })
        .count();
    println!("{}", count);
}

type Rect = ((usize, usize), (usize, usize));

#[derive(Debug)]
enum Instruction {
    TurnOn(Rect),
    Toggle(Rect),
    TurnOff(Rect),
}

fn parse_instruction(line: &str) -> IResult<&str, Instruction> {
    let (input, ins_type) = alt((tag("turn on "), tag("toggle "), tag("turn off ")))(line)?;
    let mut point_parser = separated_pair(parse_usize, is_a(","), parse_usize);
    let (input, coord_1) = point_parser(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, coord_2) = point_parser(input)?;

    let instruction = match ins_type {
        "turn on " => Instruction::TurnOn((coord_1, coord_2)),
        "turn off " => Instruction::TurnOff((coord_1, coord_2)),
        "toggle " => Instruction::Toggle((coord_1, coord_2)),
        _ => unreachable!(),
    };
    Ok((input, instruction))
}

macro_rules! rect_coords {
    ($rect:expr) => {
        (($rect.0 .0)..($rect.1 .0 + 1)).cartesian_product(($rect.0 .1)..($rect.1 .1 + 1))
    };
}

pub fn day6_part1() {
    let count: u32 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0u32; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] ^= 1)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 1)
                }
                Instruction::TurnOff(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 0)
                }
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}

pub fn day6_part2() {
    let count: i64 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0i64; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 2)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 1)
                }
                Instruction::TurnOff(rect) => rect_coords!(rect)
                    .for_each(|(x, y)| grid[x * 1000 + y] = (grid[x * 1000 + y] - 1).max(0)),
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}

#[derive(Debug)]
enum CircuitNode {
    Value(u16),
    Node(String),
}

impl FromStr for CircuitNode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse() {
            Ok(v) => Self::Value(v),
            Err(_) => Self::Node(s.to_string()),
        })
    }
}

enum CircuitConnection {
    Assign(CircuitNode),
    Not(CircuitNode),
    And(CircuitNode, CircuitNode),
    Or(CircuitNode, CircuitNode),
    RightShift(CircuitNode, CircuitNode),
    LeftShift(CircuitNode, CircuitNode),
}

fn parse_circuit_instruction(l: &str) -> (String, CircuitConnection) {
    let (lhs, rhs) = l.split(" -> ").collect_tuple().unwrap();
    let rhs = rhs.to_string();
    let tokens = lhs.split(" ").collect_vec();

    let connection = match tokens[..] {
        [x] => CircuitConnection::Assign(x.parse().unwrap()),
        ["NOT", x] => CircuitConnection::Not(x.parse().unwrap()),
        [x, "AND", y] => CircuitConnection::And(x.parse().unwrap(), y.parse().unwrap()),
        [x, "OR", y] => CircuitConnection::Or(x.parse().unwrap(), y.parse().unwrap()),
        [x, "RSHIFT", y] => CircuitConnection::RightShift(x.parse().unwrap(), y.parse().unwrap()),
        [x, "LSHIFT", y] => CircuitConnection::LeftShift(x.parse().unwrap(), y.parse().unwrap()),
        _ => unreachable!(),
    };

    return (rhs, connection);
}

fn visit(node: &CircuitNode, circuit: &HashMap<String, CircuitConnection>, memory: &mut HashMap<String, u16>) -> u16 {
    match node {
        CircuitNode::Value(v) => *v,
        CircuitNode::Node(n) => {
            if !memory.contains_key(n) {
                let value = match &circuit[n] {
                    CircuitConnection::Assign(x) => visit(x, circuit, memory),
                    CircuitConnection::Not(x) => !visit(x, circuit, memory),
                    CircuitConnection::And(x, y) => visit(x, circuit, memory) & visit(y, circuit, memory),
                    CircuitConnection::Or(x, y) => visit(x, circuit, memory) | visit(y, circuit, memory),
                    CircuitConnection::RightShift(x, b) => visit(x, circuit, memory) >> visit(b, circuit, memory),
                    CircuitConnection::LeftShift(x, b) => visit(x, circuit, memory) << visit(b, circuit, memory),
                };
                memory.insert(n.to_string(), value);
            }
            memory[n]
        }
    }
}

pub fn day7_part1() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory : HashMap<String, u16> = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);
    println!("{}", wire_a);
}

pub fn day7_part2() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    let mut memory = HashMap::from_iter([("b".to_string(), wire_a)]);
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    println!("{}", wire_a);
}
pub fn day8_part1() {
    todo!()
}
pub fn day8_part2() {
    todo!()
}
pub fn day9_part1() {
    todo!()
}
pub fn day9_part2() {
    todo!()
}
pub fn day10_part1() {
    todo!()
}
pub fn day10_part2() {
    todo!()
}
pub fn day11_part1() {
    todo!()
}
pub fn day11_part2() {
    todo!()
}
pub fn day12_part1() {
    todo!()
}
pub fn day12_part2() {
    todo!()
}
pub fn day13_part1() {
    todo!()
}
pub fn day13_part2() {
    todo!()
}
pub fn day14_part1() {
    todo!()
}
pub fn day14_part2() {
    todo!()
}
pub fn day15_part1() {
    todo!()
}
pub fn day15_part2() {
    todo!()
}
pub fn day16_part1() {
    todo!()
}
pub fn day16_part2() {
    todo!()
}
pub fn day17_part1() {
    todo!()
}
pub fn day17_part2() {
    todo!()
}
pub fn day18_part1() {
    todo!()
}
pub fn day18_part2() {
    todo!()
}
pub fn day19_part1() {
    todo!()
}
pub fn day19_part2() {
    todo!()
}
pub fn day20_part1() {
    todo!()
}
pub fn day20_part2() {
    todo!()
}
pub fn day21_part1() {
    todo!()
}
pub fn day21_part2() {
    todo!()
}
pub fn day22_part1() {
    todo!()
}
pub fn day22_part2() {
    todo!()
}
pub fn day23_part1() {
    todo!()
}
pub fn day23_part2() {
    todo!()
}
pub fn day24_part1() {
    todo!()
}
pub fn day24_part2() {
    todo!()
}
pub fn day25_part1() {
    todo!()
}
pub fn day25_part2() {
    todo!()
}
