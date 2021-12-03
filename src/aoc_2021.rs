// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use itertools::Itertools;
use nom::character::complete::char;
use nom::combinator::fail;
use nom::sequence::separated_pair;

pub use nom::branch::alt;
pub use nom::bytes::complete::{tag, take_while};
pub use nom::IResult;

use crate::parsers::parse_i64;

// --- Parsings ---

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    aim: i64,
}

impl Ship {
    fn new(x: i64, y: i64, aim: i64) -> Ship {
        Ship { x, y, aim }
    }

    fn default() -> Ship {
        Ship { x: 0, y: 0, aim: 0 }
    }

    fn move_x(&self, n: i64) -> Ship {
        Ship::new(self.x + n, self.y, self.aim)
    }

    fn move_y(&self, n: i64) -> Ship {
        Ship::new(self.x, self.y + n, self.aim)
    }

    fn move_aim(&self, n: i64) -> Ship {
        Ship::new(self.x, self.y, self.aim + n)
    }
}

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn parse_instruction(line: &str) -> Instruction {
    let (_, (instruction, step)) = separated_pair(
        alt((tag("forward"), tag("down"), tag("up"))),
        char(' '),
        parse_i64,
    )(line)
    .expect("Parser Error");

    match instruction {
        "forward" => Instruction::Forward(step),
        "up" => Instruction::Up(step),
        "down" => Instruction::Down(step),
        _ => unreachable!(),
    }
}

// --- Solutions ---

pub fn day1_part1(input: std::io::Stdin) {
    let count = input
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}

pub fn day1_part2(input: std::io::Stdin) {
    let count = input
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}

pub fn day2_part1(input: std::io::Stdin) {
    let p = input
        .lines()
        .map(Result::unwrap)
        .map(|s| parse_instruction(&s))
        .fold(Ship::default(), |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_y(-n),
            Instruction::Down(n) => ship.move_y(n),
            Instruction::Forward(n) => ship.move_x(n),
        });
    println!("{}", p.x * p.y);
}

pub fn day2_part2(input: std::io::Stdin) {
    let p = input
        .lines()
        .map(Result::unwrap)
        .map(|s| parse_instruction(&s))
        .fold(Ship::default(), |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_aim(-n),
            Instruction::Down(n) => ship.move_aim(n),
            Instruction::Forward(n) => ship.move_x(n).move_y(ship.aim * n),
        });
    println!("{}", p.x * p.y);
}

pub fn day3_part1(input: std::io::Stdin) {
    let numbers = input
        .lines()
        .map(Result::unwrap)
        .map(|s| s.bytes().collect_vec())
        .collect::<Vec<_>>();
    let width = numbers[0].len();
    let height = numbers.len();

    let gamma = (0..width)
        .map(|c| numbers.iter().filter(|row| row[c] == b'0').count())
        .map(|count_0| if count_0 > (height) / 2 { 0 } else { 1 })
        .fold(0u32, |x, b| (x << 1) + b);

    let epsilon = (!gamma) & ((1 << width) - 1);

    println!("{}", gamma * epsilon)
}

fn binary_to_dec(bits: &[u8]) -> u32 {
    bits.iter().fold(0u32, |n, &b| (n << 1) + (b - b'0') as u32)
}

pub fn day3_part2(input: std::io::Stdin) {
    let chars = input
        .lines()
        .map(Result::unwrap)
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    let o2_number = (0..chars[0].len()).fold((0..chars.len()).collect_vec(), |rows, column| {
        if rows.len() == 1 {
            return rows;
        }
        let count_1 = rows
            .iter()
            .map(|&row| chars[row][column])
            .filter(|&n| n == b'1')
            .count();
        let filter_b = match count_1 >= (rows.len() - count_1) {
            true => b'1',
            false => b'0',
        };
        rows.into_iter()
            .filter(|&row| chars[row][column] == filter_b)
            .collect_vec()
    })[0];

    let co2_number = (0..chars[0].len()).fold((0..chars.len()).collect_vec(), |rows, column| {
        if rows.len() == 1 {
            return rows;
        }
        let count_1 = rows
            .iter()
            .map(|&row| chars[row][column])
            .filter(|&n| n == b'1')
            .count();
        let filter_b = match count_1 >= (rows.len() - count_1) {
            true => b'0',
            false => b'1',
        };
        rows.into_iter()
            .filter(|&row| chars[row][column] == filter_b)
            .collect_vec()
    })[0];

    println!(
        "{}",
        binary_to_dec(&chars[o2_number]) * binary_to_dec(&chars[co2_number])
    );
}

pub fn day4_part1(input: std::io::Stdin) {
    // input.lines().map(Result::unwrap)
    //     .filter(|l|)
    todo!()
}

pub fn day4_part2(input: std::io::Stdin) {
    todo!()
}

pub fn day5_part1(input: std::io::Stdin) {
    // input.lines().map(Result::unwrap)
    //     .filter(|l|)
    todo!()
}

pub fn day5_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day6_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day6_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day7_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day7_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day8_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day8_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day9_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day9_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day10_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day10_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day11_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day11_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day12_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day12_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day13_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day13_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day14_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day14_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day15_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day15_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day16_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day16_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day17_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day17_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day18_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day18_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day19_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day19_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day20_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day20_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day21_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day21_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day22_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day22_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day23_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day23_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day24_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day24_part2(input: std::io::Stdin) {
    todo!()
}
pub fn day25_part1(input: std::io::Stdin) {
    todo!()
}
pub fn day25_part2(input: std::io::Stdin) {
    todo!()
}
