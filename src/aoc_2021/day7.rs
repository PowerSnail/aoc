use crate::{prelude::*, std_iter};

pub fn part1() {
    let numbers = std_iter!(SplitBy ",")
        .map(|x| x.parse::<i64>().unwrap())
        .sorted()
        .collect_vec();
    let median = numbers[numbers.len() / 2];
    let total_diff: i64 = numbers.iter().map(|&x| (x - median).abs()).sum();
    println!("{}", total_diff);
}

pub fn part2() {
    let numbers = std_iter!(SplitBy ",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let mean = numbers.iter().sum::<i64>() as f64 / numbers.len() as f64;

    let total_diff: i64 = [mean.floor() as i64, mean.ceil() as i64]
        .iter()
        .map(|i| numbers.iter().map(|&n| sum_to_1((n - i).abs())).sum())
        .min()
        .unwrap();
    println!("{}", total_diff);
}
