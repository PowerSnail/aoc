use crate::{prelude::*, std_iter};

fn fishes_reproduction(fishes: Vec<usize>, days: usize) -> usize {
    (0..days)
        .fold(fishes, |fishes, _| {
            vec![
                fishes[1],
                fishes[2],
                fishes[3],
                fishes[4],
                fishes[5],
                fishes[6],
                fishes[0] + fishes[7],
                fishes[8],
                fishes[0],
            ]
        })
        .into_iter()
        .sum()
}

pub fn part1() {
    let fishes = std_iter!(Bytes)
        .filter(|c| c.is_dec_digit())
        .map(|c| c - b'0')
        .fold(vec![0; 9], |mut v, d| {
            v[d as usize] += 1;
            v
        });
    println!("{}", fishes_reproduction(fishes, 80));
}

pub fn part2() {
    let fishes = std_iter!(Bytes)
        .filter(|c| c.is_dec_digit())
        .map(|c| c - b'0')
        .fold(vec![0; 9], |mut v, d| {
            v[d as usize] += 1;
            v
        });
    println!("{}", fishes_reproduction(fishes, 256));
}
