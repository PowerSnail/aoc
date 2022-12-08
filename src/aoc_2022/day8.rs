use std::collections::HashSet;

use itertools::Itertools;

use crate::std_iter;

fn input() -> Vec<Vec<u8>> {
    std_iter!(Lines)
        .map(|l| l.bytes().collect_vec())
        .collect_vec()
}

fn count_visible(mut numbers: impl Iterator<Item = u8>) -> Vec<usize> {
    let mut max = numbers.next().unwrap();
    let mut i = 0;
    let mut visible = vec![0];
    for h in numbers {
        i += 1;
        if h > max {
            max = h;
            visible.push(i);
        }
    }
    visible
}

pub fn part1() {
    let grid = input();
    let mut visible = HashSet::new();

    for r in 0..grid.len() {
        for c in count_visible(grid[r].iter().copied()) {
            visible.insert((r, c));
        }
        for c in count_visible(grid[r].iter().rev().copied()) {
            visible.insert((r, grid[0].len() - 1 - c));
        }
    }

    for c in 0..grid[0].len() {
        for r in count_visible((0..grid.len()).map(|r| grid[r][c])) {
            visible.insert((r, c));
        }
        for r in count_visible((0..grid.len()).rev().map(|r| grid[r][c])) {
            visible.insert((grid.len() - 1 - r, c));
        }
    }

    println!("{}", visible.len());
}

fn search_obstructed(trees: impl Iterator<Item = u8>, target: u8) -> usize {
    let mut i = 0;
    for n in trees {
        i += 1;
        if n >= target {
            break;
        }
    }
    return i;
}

pub fn part2() {
    let grid = input();
    let mut max = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            let height = grid[r][c];
            let n = search_obstructed((0..r).rev().map(|r| grid[r][c]), height);
            let s = search_obstructed((r + 1..grid.len()).map(|r| grid[r][c]), height);
            let w = search_obstructed((0..c).rev().map(|c| grid[r][c]), height);
            let e = search_obstructed((c + 1..grid[0].len()).map(|c| grid[r][c]), height);
            max = max.max(n * s * w * e);
        }
    }
    println!("{}", max);
}
