use crate::{prelude::*, std_iter};

pub fn part1() {
    let mut grid = std_iter!(Lines)
        .map(|l| {
            [false]
                .into_iter()
                .chain(l.chars().map(|c| c == '#'))
                .chain([false])
                .collect_vec()
        })
        .collect_vec();

    grid.insert(0, vec![false; grid[0].len()]);
    grid.push(vec![false; grid[0].len()]);

    for _step in 0..100 {
        let mut counts = vec![vec![0; grid[0].len()]; grid.len()];
        for (y, x) in iproduct!(1..grid.len() - 1, 1..grid[0].len() - 1) {
            counts[y][x] = iproduct!([y - 1, y, y + 1], [x - 1, x, x + 1])
                .filter(|&(ny, nx)| (nx != x || ny != y) && grid[ny][nx])
                .count();
        }
        for (y, x) in iproduct!(1..grid.len() - 1, 1..grid[0].len() - 1) {
            grid[y][x] = match (grid[y][x], counts[y][x]) {
                (true, 2) | (_, 3) => true,
                _ => false,
            }
        }
    }
    let count = iproduct!(1..grid.len() - 1, 1..grid[0].len() - 1)
        .filter(|&(y, x)| grid[y][x])
        .count();
    println!("{}", count);
}

pub fn part2() {
    let mut grid = std_iter!(Lines)
        .map(|l| {
            [false]
                .into_iter()
                .chain(l.chars().map(|c| c == '#'))
                .chain([false])
                .collect_vec()
        })
        .collect_vec();

    grid.insert(0, vec![false; grid[0].len()]);
    grid.push(vec![false; grid[0].len()]);

    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    for _step in 0..100 {
        let mut counts = vec![vec![0; grid[0].len()]; grid.len()];
        for (y, x) in iproduct!(1..max_y, 1..max_x) {
            counts[y][x] = iproduct!([y - 1, y, y + 1], [x - 1, x, x + 1])
                .filter(|&(ny, nx)| (nx != x || ny != y) && grid[ny][nx])
                .count();
        }
        for (y, x) in iproduct!(1..max_y, 1..max_x) {
            grid[y][x] = match (grid[y][x], counts[y][x]) {
                (true, 2) | (_, 3) => true,
                _ => false,
            }
        }
        for (y, x) in iproduct!([1, max_y - 1], [1, max_x - 1]) {
            grid[y][x] = true;
        }
    }
    let count = iproduct!(1..max_y, 1..max_x)
        .filter(|&(y, x)| grid[y][x])
        .count();
    println!("{}", count);
}
