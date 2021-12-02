// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use std::io::Read;

use itertools::Itertools;

fn char_to_step(c: u8) -> Option<i64> {
    match c {
        b'(' => Some(1),
        b')' => Some(-1),
        _ => None,
    }
}

pub fn day1_part1(input: std::io::Stdin) {
    let total: i64 = input
        .bytes()
        .map(Result::unwrap)
        .filter_map(char_to_step)
        .sum();
    println!("{}", total);
}


pub fn day1_part2(input: std::io::Stdin) {
    let position: usize = input
        .bytes()
        .map(Result::unwrap)
        .filter_map(char_to_step)
        .scan(0, |current, step| { 
            *current += step; 
            Some(*current)
        })
        .find_position(|c| *c == -1)
        .unwrap().0 + 1;
    println!("{}", position);
}