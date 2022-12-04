use crate::prelude::*;
use crate::std_iter;

fn parse_range(i: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_u64, is_a("-"), parse_u64)(i)
}

fn parse_line(i: &str) -> IResult<&str, ((u64, u64), (u64, u64))> {
    separated_pair(parse_range, is_a(","), parse_range)(i)
}

pub fn part1() {
    let count = std_iter!(Lines)
        .map(|line| parse_line(&line).expect("Can't parse").1)
        .filter(|((l1, r1), (l2, r2))| (l1 >= l2 && r1 <= r2) || (l2 >= l1 && r2 <= r1))
        .count();
    println!("{}", count);
}

pub fn part2() {
    let count = std_iter!(Lines)
        .map(|line| parse_line(&line).expect("Can't parse").1)
        .filter(|((l1, r1), (l2, r2))| !(r1 < l2 || l1 > r2))
        .count();
    println!("{}", count);
}
