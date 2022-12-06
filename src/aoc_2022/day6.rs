use std::collections::HashSet;

use crate::std_iter;

pub fn part1() {
    let i = std_iter!(Lines)
        .next()
        .unwrap()
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter(|&(_, w)| HashSet::<u8>::from_iter(w.iter().copied()).len() == 4)
        .next()
        .unwrap()
        .0;

    println!("{}", i + 4);
}

pub fn part2() {
    let i = std_iter!(Lines)
        .next()
        .unwrap()
        .as_bytes()
        .windows(14)
        .enumerate()
        .filter(|&(_, w)| HashSet::<u8>::from_iter(w.iter().copied()).len() == 14)
        .next()
        .unwrap()
        .0;

    println!("{}", i + 14);
}
