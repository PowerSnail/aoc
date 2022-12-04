use std::collections::HashSet;

use itertools::Itertools;

use crate::std_iter;

fn score(c: u8) -> u64 {
    if c.is_ascii_lowercase() {
        (c - b'a' + 1) as u64
    } else {
        (c - b'A' + 27) as u64
    }
}

pub fn part1() {
    let total: u64 = std_iter!(Lines)
        .map(|line| {
            let buffer = line.as_bytes();
            let first: HashSet<u8> = (&buffer[..buffer.len() / 2]).iter().map(|&c| c).collect();
            let second: HashSet<u8> = (&buffer[buffer.len() / 2..]).iter().map(|&c| c).collect();
            first.intersection(&second).map(|&s| s).next().unwrap()
        })
        .map(score)
        .sum();
    println!("{}", total);
}

pub fn part2() {
    let total: u64 = std_iter!(Lines)
        .chunks(3)
        .into_iter()
        .map(|lines| {
            lines
                .map(|l| l.bytes().collect::<HashSet<u8>>())
                .reduce(|s1, s2| s1.intersection(&s2).copied().collect())
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .map(score)
        .sum();
    println!("{}", total);
}
