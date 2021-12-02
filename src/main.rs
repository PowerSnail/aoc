use clap::Parser;
use itertools::Itertools;
use nom::character::complete::char;
use nom::combinator::fail;
use nom::sequence::separated_pair;
use std::io::{self, prelude::*};

pub use nom::branch::alt;
pub use nom::bytes::complete::{tag, take_while};
pub use nom::IResult;

#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    day: u32,
    part: u32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    match opts {
        Opts { day: 1, part: 1 } => println!("{}", day1_part1(input)),
        Opts { day: 1, part: 2 } => println!("{}", day1_part2(input)),
        Opts { day: 2, part: 1 } => println!("{}", day2_part1(input)),
        Opts { day: 2, part: 2 } => println!("{}", day2_part2(input)),
        _ => println!("Unimplemented: day{}, part{}", opts.day, opts.part),
    }
}

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

fn is_hex_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_i64(line: &str) -> IResult<&str, i64> {
    let (input, digits) = take_while(is_hex_digit)(line)?;
    match digits.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
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

fn day1_part1(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

fn day1_part2(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

fn day2_part1(lines: impl Iterator<Item = String>) -> i64 {
    let p = lines
        .map(|s| parse_instruction(&s))
        .fold(Ship::default(), |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_y(-n),
            Instruction::Down(n) => ship.move_y(n),
            Instruction::Forward(n) => ship.move_x(n),
        });
    p.x * p.y
}

fn day2_part2(lines: impl Iterator<Item = String>) -> i64 {
    let p = lines
        .map(|s| parse_instruction(&s))
        .fold(Ship::default(), |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_aim(-n),
            Instruction::Down(n) => ship.move_aim(n),
            Instruction::Forward(n) => ship.move_x(n).move_y(ship.aim * n),
        });
    p.x * p.y
}
