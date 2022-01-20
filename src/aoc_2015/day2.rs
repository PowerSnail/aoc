use crate::{prelude::*, std_iter};

pub fn part1() {
    let total: i64 = std_iter!(Lines)
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

pub fn part2() {
    let total: i64 = std_iter!(Lines)
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
