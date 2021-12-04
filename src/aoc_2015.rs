// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use std::{collections::HashMap, io::Read};

use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, AsChar};

use crate::parsers::parse_i64;

// --- Parsings ---

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
        .unwrap()
        .0
        + 1;
    println!("{}", position);
}

pub fn day2_part1(input: std::io::Stdin) {
    let total: i64 = input
        .lines()
        .map(Result::unwrap)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let (total_sa, min_sa) = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().product())
                .fold((0, i64::MAX), |(total, min), s| (total + s * 2, min.min(s)));
            total_sa + min_sa
        })
        .sum();
    println!("{}", total);
}

pub fn day2_part2(input: std::io::Stdin) {
    let total: i64 = input
        .lines()
        .map(Result::unwrap)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let volume: i64 = numbers.iter().product();
            let shortest_waist: i64 = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().sum::<i64>())
                .min()
                .unwrap()
                * 2i64;
            shortest_waist + volume
        })
        .sum();
    println!("{}", total);
}

pub fn day3_part1(input: std::io::Stdin) {
    let count = input
        .bytes()
        .map(Result::unwrap)
        .scan((0, 0), |(x, y), step| {
            match step {
                b'>' => *x += 1,
                b'<' => *x -= 1,
                b'^' => *y += 1,
                b'v' => *y -= 1,
                _ => unreachable!(),
            };
            Some((*x, *y))
        })
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();
    println!("{}", count);
}

pub fn day3_part2(input: std::io::Stdin) {
    let (script_santa, script_robot) = input.bytes().map(Result::unwrap).tee();

    let count = vec![script_santa.skip(0), script_robot.skip(1)]
        .into_iter()
        .map(|instructions| {
            instructions.step_by(2).scan((0, 0), |(x, y), step| {
                match step {
                    b'>' => *x += 1,
                    b'<' => *x -= 1,
                    b'^' => *y += 1,
                    b'v' => *y -= 1,
                    _ => unreachable!(),
                };
                Some((*x, *y))
            })
        })
        .flatten()
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();

    println!("{}", count);
}

pub fn day4_part1(input: std::io::Stdin) {
    let key: Vec<u8> = input
        .bytes()
        .map(Result::unwrap)
        .filter(|c| c.is_alphanum())
        .collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("00000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day4_part2(input: std::io::Stdin) {
    let key: Vec<u8> = input
        .bytes()
        .map(Result::unwrap)
        .filter(|c| c.is_alphanum())
        .collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("000000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day5_part1(input: std::io::Stdin) {
    let count = input
        .lines()
        .map(Result::unwrap)
        .filter(|line| {
            let vowel_count = line
                .bytes()
                .filter(|&b| match b {
                    b'a' | b'e' | b'i' | b'o' | b'u' => true,
                    _ => false,
                })
                .count();

            let twice_in_a_row = line.bytes().tuple_windows().any(|(x, y)| x == y);

            let contains_bad_sub = line.bytes().tuple_windows().any(|(x, y)| match &[x, y] {
                b"ab" | b"cd" | b"pq" | b"xy" => true,
                _ => false,
            });

            (vowel_count >= 3) && twice_in_a_row && (!contains_bad_sub)
        })
        .count();
    println!("{}", count);
}

pub fn day5_part2(input: std::io::Stdin) {
    let count = input
        .lines()
        .map(Result::unwrap)
        .filter(|line| {
            let two_grams = line
                .bytes()
                .tuple_windows::<(u8, u8)>()
                .enumerate()
                .scan(HashMap::new(), |first_positions, (i, pair)| {
                    if !first_positions.contains_key(&pair) {
                        first_positions.insert(pair, i);
                    }
                    let first_pos = first_positions.get(&pair).unwrap();
                    Some(i - first_pos > 1)
                })
                .any(|x| x);

            let sandwiched = line.bytes().tuple_windows().any(|(x, _, z)| x == z);
            sandwiched && two_grams
        })
        .count();
    println!("{}", count);
}
pub fn day6_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day6_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day7_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day7_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day8_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day8_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day9_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day9_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day10_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day10_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day11_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day11_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day12_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day12_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day13_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day13_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day14_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day14_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day15_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day15_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day16_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day16_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day17_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day17_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day18_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day18_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day19_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day19_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day20_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day20_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day21_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day21_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day22_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day22_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day23_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day23_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day24_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day24_part2(_: std::io::Stdin) {
    todo!()
}
pub fn day25_part1(_: std::io::Stdin) {
    todo!()
}
pub fn day25_part2(_: std::io::Stdin) {
    todo!()
}
