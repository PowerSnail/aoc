use crate::{prelude::*, std_iter};

fn make_neighbors2(
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x_min = if x > 0 { x - 1 } else { 0 };
    let x_max = if x < width - 1 { x + 2 } else { width };
    let y_min = if y > 0 { y - 1 } else { 0 };
    let y_max = if y < height - 1 { y + 2 } else { height };
    (y_min..y_max)
        .cartesian_product(x_min..x_max)
        .filter(move |&coord| coord != (y, x))
}

fn octopus_step(grid: &mut Vec<Vec<u8>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut stack: VecDeque<(usize, usize)> = iproduct!(0..height, 0..width).collect();
    while let Some((y, x)) = stack.pop_front() {
        match grid[y][x] {
            10 /* Already flashed */ => (),
            9 => {
                grid[y][x] += 1;
                stack.extend(make_neighbors2(y, x, width, height));
            },
            n => {
                grid[y][x] = n + 1;
            }
        }
    }
    grid.iter_mut()
        .map(|row| row.iter_mut())
        .flatten()
        .filter_map(|x| if *x == 10 { Some(*x = 0) } else { None })
        .count()
}

pub fn part1() {
    let mut grid = std_iter!(Lines)
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let result = (0..100).map(|_| octopus_step(&mut grid)).sum::<usize>();
    println!("{}", result);
}

pub fn part2() {
    let mut grid = std_iter!(Lines)
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let step = (1..usize::MAX)
        .find(|_| octopus_step(&mut grid) == grid.len() * grid[0].len())
        .unwrap();
    println!("{}", step);
}
