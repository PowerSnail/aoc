use crate::{prelude::*, std_iter};

pub fn part1() {
    let grid = std_iter!(GridOf | b | (b - b'0') as usize);
    let width = grid[0].len();
    let height = grid.len();

    let mut costs = vec![vec![usize::MAX; width]; height];
    costs[0][0] = 0;

    let mut parents = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0 as usize), 0, 0));
    while let Some((Reverse(_), y, x)) = heap.pop() {
        if x == width - 1 && y == height - 1 {
            break;
        }
        for (ny, nx) in make_neighbors(y, x, width, height) {
            let next_cost = costs[y][x] + grid[ny][nx];
            if next_cost < costs[ny][nx] {
                costs[ny][nx] = next_cost;
                heap.push((Reverse(next_cost), ny, nx));
                parents.insert((ny, nx), (y, x));
            }
        }
    }
    println!("{}", costs[height - 1][width - 1]);
}

fn enlarge_grid(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut large_grid = vec![vec![0; width * 5]; height * 5];
    for y in 0..height * 5 {
        for x in 0..width * 5 {
            large_grid[y][x] = (grid[y % height][x % width] - 1 + y / height + x / height) % 9 + 1;
        }
    }
    large_grid
}

pub fn part2() {
    let grid = enlarge_grid(std_iter!(GridOf | b | (b - b'0') as usize));
    let width = grid[0].len();
    let height = grid.len();

    let mut costs = vec![vec![usize::MAX; width]; height];
    costs[0][0] = 0;

    let mut parents = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0 as usize), 0, 0));

    while let Some((Reverse(_), y, x)) = heap.pop() {
        if x == width - 1 && y == height - 1 {
            break;
        }
        for (ny, nx) in make_neighbors(y, x, width, height) {
            let next_cost = costs[y][x] + grid[ny][nx];
            if next_cost < costs[ny][nx] {
                costs[ny][nx] = next_cost;
                heap.push((Reverse(next_cost), ny, nx));
                parents.insert((ny, nx), (y, x));
            }
        }
    }
    println!("{}", costs[height - 1][width - 1]);
}
