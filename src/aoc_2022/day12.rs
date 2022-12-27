use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    hash::Hash,
};

use crate::std_iter;
use itertools::{iproduct, Itertools};

type Grid = Vec<Vec<i64>>;
type Coord = (usize, usize);

fn input() -> (Grid, Coord, Coord) {
    let mut grid = std_iter!(GridOf | b | b as i64);
    let start = iproduct!(0..grid.len(), 0..grid[0].len())
        .find(|&(row, col)| grid[row][col] == b'S' as i64)
        .unwrap();
    let end = iproduct!(0..grid.len(), 0..grid[0].len())
        .find(|&(row, col)| grid[row][col] == b'E' as i64)
        .unwrap();
    grid[start.0][start.1] = b'a' as i64;
    grid[end.0][end.1] = b'z' as i64;
    (grid, start, end)
}

fn make_neighbors(grid: &Grid, point: Coord) -> Vec<Coord> {
    let mut points = vec![];
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    }
    if point.0 < grid.len() - 1 {
        points.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    }
    if point.1 < grid[0].len() - 1 {
        points.push((point.0, point.1 + 1));
    }
    points
}

fn bfs<F>(grid: &Grid, start: Coord, distance: F) -> Vec<Vec<u64>>
where
    F: Fn(&Grid, Coord, Coord) -> Option<u64>,
{
    let mut distances = vec![vec![u64::MAX; grid[0].len()]; grid.len()];
    distances[start.0][start.1] = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut queue = VecDeque::from([start]);

    while let Some(point) = queue.pop_front() {
        if visited[point.0][point.1] {
            continue;
        }
        visited[point.0][point.1] = true;
        for neighbor in make_neighbors(&grid, point).into_iter() {
            if visited[neighbor.0][neighbor.1] {
                continue;
            }
            if let Some(delta) = distance(grid, point, neighbor) {
                if distances[neighbor.0][neighbor.1] > distances[point.0][point.1] + delta {
                    distances[neighbor.0][neighbor.1] = distances[point.0][point.1] + delta;
                }
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

fn dijkstra<T, FN, FD>(start: T, get_neighbors: FN, get_distance: FD) -> HashMap<T, u64>
where
    T: Copy + Hash + Eq + Ord,
    FN: Fn(&T) -> Vec<T>,
    FD: Fn(&T, &T) -> u64,
{
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut queue = BinaryHeap::new();
    queue.push((0, start));

    while let Some((distance, point)) = queue.pop() {
        if distances.contains_key(&point) && distance > distances[&point] {
            continue;
        }
        for neighbor in get_neighbors(&point) {
            let distance = distance + get_distance(&point, &neighbor);
            if !distances.contains_key(&neighbor) || distances[&neighbor] > distance {
                distances.insert(neighbor, distance);
                queue.push((distance, neighbor));
            }
        }
    }

    distances
}

pub fn part1() {
    let (grid, start, end) = input();

    let distances = bfs(&grid, start, |g, from, to| {
        if g[to.0][to.1] - grid[from.0][from.1] <= 1 {
            return Some(1);
        } else {
            return None;
        }
    });

    println!("{}", distances[end.0][end.1]);
}

pub fn part2() {
    let (grid, _, end) = input();

    // let distances = bfs(&grid, end, |g, to, from| {
    //     if g[to.0][to.1] - grid[from.0][from.1] <= 1 {
    //         return Some(1);
    //     } else {
    //         return None;
    //     }
    // });

    // let min_distance = iproduct!(0..grid.len(), 0..grid[0].len())
    //     .filter_map(|(row, col)| {
    //         if grid[row][col] == b'a' as i64 {
    //             Some(distances[row][col])
    //         } else {
    //             None
    //         }
    //     })
    //     .min()
    //     .unwrap();
    let distances = dijkstra(
        end,
        |point| {
            make_neighbors(&grid, *point)
                .into_iter()
                .filter(|n| grid[point.0][point.1] - grid[n.0][n.1] <= 1)
                .collect_vec()
        },
        |_, _| 1,
    );

    let min_distance = iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] == b'a' as i64)
        .map(|(r, c)| distances[&(r, c)])
        .min()
        .unwrap();

    println!("{}", min_distance);
}
