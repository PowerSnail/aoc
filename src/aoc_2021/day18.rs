use crate::{prelude::*, std_iter};

fn add_snailfish(fish_1: &[(u64, u64)], fish_2: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let sum = fish_1
        .iter()
        .map(|&(v, l)| (v, l + 1))
        .chain(fish_2.iter().map(|&(v, l)| (v, l + 1)))
        .collect();
    reduce_snailfish(sum)
}

fn reduce_snailfish(fish: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if let Some((i, (_, l))) = fish.iter().find_position(|&(_, l)| *l > 4) {
        let mut new_fish = vec![];
        new_fish.extend_from_slice(&fish[..i]);
        new_fish.push((0, l - 1));
        new_fish.extend_from_slice(&fish[i + 2..]);
        if i > 0 {
            new_fish[i - 1].0 += fish[i].0;
        }
        if i < new_fish.len() - 1 {
            new_fish[i + 1].0 += fish[i + 1].0;
        }
        reduce_snailfish(new_fish)
    } else if let Some((i, (v, l))) = fish.iter().find_position(|&(v, _)| *v >= 10) {
        let half = *v as f64 / 2.;
        let mut new_fish = vec![];
        new_fish.extend_from_slice(&fish[..i]);
        new_fish.push((half.floor() as u64, *l + 1));
        new_fish.push((half.ceil() as u64, *l + 1));
        new_fish.extend_from_slice(&fish[i + 1..]);
        reduce_snailfish(new_fish)
    } else {
        fish
    }
}

fn magnitude_snailfish(fish: &[(u64, u64)], level: u64) -> (&[(u64, u64)], u64) {
    let (fish, lhs) = if fish[0].1 > level {
        magnitude_snailfish(fish, level + 1)
    } else {
        (&fish[1..], fish[0].0)
    };
    let (fish, rhs) = if fish[0].1 > level {
        magnitude_snailfish(fish, level + 1)
    } else {
        (&fish[1..], fish[0].0)
    };
    (fish, lhs * 3 + rhs * 2)
}

pub fn part1() {
    let fish_sum = std_iter!(Lines)
        .map(|l| {
            let mut numbers = vec![];
            let mut level = 0u64;
            for b in l.chars() {
                match b {
                    '[' => level += 1,
                    ']' => level -= 1,
                    c @ '0'..='9' => numbers.push((c as u64 - '0' as u64, level)),
                    _ => (),
                }
            }
            reduce_snailfish(numbers)
        })
        .reduce(|f1, f2| add_snailfish(&f1, &f2))
        .unwrap();
    let (_, magnitude) = magnitude_snailfish(&fish_sum, 1);
    println!("{}", magnitude);
}

pub fn part2() {
    let fishes = std_iter!(Lines)
        .map(|l| {
            let mut numbers = vec![];
            let mut level = 0u64;
            for b in l.chars() {
                match b {
                    '[' => level += 1,
                    ']' => level -= 1,
                    c @ '0'..='9' => numbers.push((c as u64 - '0' as u64, level)),
                    _ => (),
                }
            }
            reduce_snailfish(numbers)
        })
        .collect_vec();
    let max = fishes
        .iter()
        .cartesian_product(fishes.iter())
        .map(|(f1, f2)| magnitude_snailfish(&add_snailfish(f1, f2), 1).1)
        .max()
        .unwrap();
    println!("{}", max);
}
