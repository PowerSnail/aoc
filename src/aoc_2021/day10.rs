use crate::{prelude::*, std_iter};

enum LineProcessResult {
    IllegalChar(u8),
    UnmatchedChars(Vec<u8>),
}

fn left_bracket_of(c: u8) -> u8 {
    match c {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => unreachable!(),
    }
}

fn process_line(s: &str) -> LineProcessResult {
    let mut stack = vec![];
    for c in s.bytes() {
        match c {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(c);
            }
            _ => {
                if stack.pop().unwrap() != left_bracket_of(c) {
                    return LineProcessResult::IllegalChar(c);
                }
            }
        }
    }
    LineProcessResult::UnmatchedChars(stack)
}

pub fn part1() {
    let score = std_iter!(Lines)
        .filter_map(|l| match process_line(&l) {
            LineProcessResult::IllegalChar(c) => Some(c),
            _ => None,
        })
        .map(|char| match char {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum::<u64>();
    println!("{}", score);
}

pub fn part2() {
    let mut scores = std_iter!(Lines)
        .filter_map(|l| match process_line(&l) {
            LineProcessResult::UnmatchedChars(chars) => Some(chars),
            _ => None,
        })
        .map(|remaining_chars| {
            remaining_chars
                .into_iter()
                .rev()
                .map(|char| match char {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                })
                .fold(0u64, |sum, x| sum * 5 + x)
        })
        .collect_vec();

    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}
