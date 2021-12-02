// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use std::io::Read;

pub fn day1_part1(input: std::io::Stdin) {
    let total: i64 = input
        .bytes()
        .map(Result::unwrap)
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => unreachable!(),
        })
        .sum();
    println!("{}", total);
}

