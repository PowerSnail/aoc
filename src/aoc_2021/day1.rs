use crate::{prelude::*, std_iter};

pub fn part1() {
    let count = std_iter!(Lines)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}

pub fn part2() {
    let count = std_iter!(Lines)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}
