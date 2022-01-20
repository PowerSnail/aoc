use crate::{prelude::*, std_iter};

fn count_partitions(containers: &[i64], remaining_count: i64) -> usize {
    if remaining_count < 0 {
        return 0;
    }
    if remaining_count == 0 {
        return 1;
    }
    match containers {
        [] => 0,
        [head, tail @ ..] => {
            count_partitions(tail, remaining_count)
                + count_partitions(tail, remaining_count - *head)
        }
    }
}

pub fn part1() {
    let containers: Vec<i64> = std_iter!(Lines)
        .map(|l| l.parse().unwrap())
        .sorted()
        .collect_vec();
    let count = count_partitions(&containers[..], 150);
    println!("{}", count);
}

fn accumulate_partitions(
    containers: &[i64],
    remaining_count: i64,
    container_count: i64,
) -> Vec<i64> {
    if remaining_count < 0 {
        return vec![];
    }
    if remaining_count == 0 {
        return vec![container_count];
    }
    match containers {
        [] => vec![],
        [head, tail @ ..] => {
            let mut partitions = accumulate_partitions(tail, remaining_count, container_count);
            partitions.append(&mut accumulate_partitions(
                tail,
                remaining_count - *head,
                container_count + 1,
            ));
            partitions
        }
    }
}

pub fn part2() {
    let containers: Vec<i64> = std_iter!(Lines)
        .map(|l| l.parse().unwrap())
        .sorted()
        .collect_vec();
    let (_, count) = accumulate_partitions(&containers[..], 150, 0)
        .into_iter()
        .counts()
        .into_iter()
        .min()
        .unwrap();

    println!("{}", count);
}
