use crate::{prelude::*, std_iter};

fn char_to_step(c: u8) -> Option<i64> {
    match c {
        b'(' => Some(1),
        b')' => Some(-1),
        _ => None,
    }
}

pub fn part1() {
    let total: i64 = std_iter!(Bytes).filter_map(char_to_step).sum();
    println!("{}", total);
}

pub fn part2() {
    let position: usize = std_iter!(Bytes)
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
