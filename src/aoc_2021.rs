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
