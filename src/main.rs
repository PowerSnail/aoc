use itertools::Itertools;
use std::io::{self, prelude::*};
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    day: u32,
    part: u32,
}

fn day1_part1(lines: impl Iterator<Item = String>) -> usize
{
    let (i1, i2) = lines.map(|s| s.parse::<u32>().unwrap()).tee();
    i1.zip(i2.skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

fn main() {
    let opts: Opts = Opts::parse();
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    match opts {
        Opts { day: 1, part: 1 } => println!("{}", day1_part1(input)),
        _ => println!("Unimplemented: day{}, part{}", opts.day, opts.part),
    }
}
