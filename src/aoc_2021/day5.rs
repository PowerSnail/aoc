use crate::{prelude::*, std_iter};

pub fn part1() {
    let overlapped_count = std_iter!(Lines)
        .map(|l| {
            let numbers = l
                .split(" -> ")
                .map(|l| l.split(",").map(|x| x.parse().unwrap()))
                .flatten()
                .collect_vec();
            LineSegment::new(
                Point::new(numbers[0], numbers[1]),
                Point::new(numbers[2], numbers[3]),
            )
        })
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .map(|l| l.scan_line())
        .flatten()
        .counts()
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .count();

    println!("{:?}", overlapped_count);
}

pub fn part2() {
    let overlapped_count = std_iter!(Lines)
        .map(|l| {
            let numbers = l
                .split(" -> ")
                .map(|l| l.split(",").map(|x| x.parse().unwrap()))
                .flatten()
                .collect_vec();
            LineSegment::new(
                Point::new(numbers[0], numbers[1]),
                Point::new(numbers[2], numbers[3]),
            )
        })
        .map(|l| l.scan_line())
        .flatten()
        .counts()
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .count();

    println!("{:?}", overlapped_count);
}
