use crate::{prelude::*, std_iter};

fn make_volcano_grid() -> Vec<Vec<u8>> {
    std_iter!(Lines)
        .map(|l| l.as_bytes().into_iter().map(|&b| b - b'0').collect_vec())
        .collect_vec()
}

pub fn part1() {
    let grid = make_volcano_grid();
    let height = grid.len();
    let width = grid[0].len();
    let total = (0..height)
        .cartesian_product(0..width)
        .filter(|&(y, x)| {
            make_neighbors(y, x, width, height).all(|(ny, nx)| grid[ny][nx] > grid[y][x])
        })
        .map(|(x, y)| grid[x][y] as u32 + 1)
        .sum::<u32>();
    println!("{}", total);
}

pub fn part2() {
    let grid = make_volcano_grid();
    let height = grid.len();
    let width = grid[0].len();
    let low_points = (0..height)
        .cartesian_product(0..width)
        .filter(|&(y, x)| {
            make_neighbors(y, x, width, height).all(|(ny, nx)| grid[ny][nx] > grid[y][x])
        })
        .collect_vec();

    let product = low_points
        .into_iter()
        .map(|(y, x)| {
            let mut stack = vec![(y, x)];
            let mut visited = HashSet::new();
            while let Some((y, x)) = stack.pop() {
                if !visited.insert((y, x)) {
                    continue;
                }
                make_neighbors(y, x, width, height)
                    .filter(|&(ny, nx)| grid[ny][nx] != 9 && grid[ny][nx] > grid[y][x])
                    .for_each(|node| {
                        stack.push(node);
                    });
            }
            visited
        })
        .map(|basin| basin.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    println!("{}", product);
}
