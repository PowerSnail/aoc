use clap::Parser;
use itertools::Itertools;
use std::io::{self, prelude::*};

#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    day: u32,
    part: u32,
}

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

fn main() {
    let opts: Opts = Opts::parse();
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    match opts {
        Opts { day: 1, part: 1 } => println!("{}", day1_part1(input)),
        Opts { day: 1, part: 2 } => println!("{}", day1_part2(input)),
        _ => println!("Unimplemented: day{}, part{}", opts.day, opts.part),
    }
}
