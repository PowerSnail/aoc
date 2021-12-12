// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::{HashSet, VecDeque};

use itertools::iproduct;

use crate::{prelude::*, std_iter};

// --- Parsings ---

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    aim: i64,
}

impl Ship {
    fn new(x: i64, y: i64, aim: i64) -> Ship {
        Ship { x, y, aim }
    }

    fn default() -> Ship {
        Ship { x: 0, y: 0, aim: 0 }
    }

    fn move_x(&self, n: i64) -> Ship {
        Ship::new(self.x + n, self.y, self.aim)
    }

    fn move_y(&self, n: i64) -> Ship {
        Ship::new(self.x, self.y + n, self.aim)
    }

    fn move_aim(&self, n: i64) -> Ship {
        Ship::new(self.x, self.y, self.aim + n)
    }
}

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn parse_instruction(line: &str) -> Instruction {
    let (_, (instruction, step)) = separated_pair(
        alt((tag("forward"), tag("down"), tag("up"))),
        char(' '),
        parse_i64,
    )(line)
    .expect("Parser Error");

    match instruction {
        "forward" => Instruction::Forward(step),
        "up" => Instruction::Up(step),
        "down" => Instruction::Down(step),
        _ => unreachable!(),
    }
}

// --- Solutions ---

pub fn day1_part1() {
    let count = std_iter!(Lines)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}

pub fn day1_part2() {
    let count = std_iter!(Lines)
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count();
    println!("{}", count);
}

pub fn day2_part1() {
    let p = std_iter!(Lines).map(|s| parse_instruction(&s)).fold(
        Ship::default(),
        |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_y(-n),
            Instruction::Down(n) => ship.move_y(n),
            Instruction::Forward(n) => ship.move_x(n),
        },
    );
    println!("{}", p.x * p.y);
}

pub fn day2_part2() {
    let p = std_iter!(Lines).map(|s| parse_instruction(&s)).fold(
        Ship::default(),
        |ship, instruction| match instruction {
            Instruction::Up(n) => ship.move_aim(-n),
            Instruction::Down(n) => ship.move_aim(n),
            Instruction::Forward(n) => ship.move_x(n).move_y(ship.aim * n),
        },
    );
    println!("{}", p.x * p.y);
}

fn binary_to_dec(bits: &[u8]) -> u32 {
    bits.iter().fold(0u32, |n, b| (n << 1) + *b as u32)
}

fn parse_row_bits(row: &str) -> Vec<u8> {
    row.bytes().map(|c| c - b'0').collect_vec()
}

pub fn day3_part1() {
    let numbers = std_iter!(Lines)
        .map(|s| parse_row_bits(&s))
        .collect::<Vec<_>>();
    let width = numbers[0].len();
    let height = numbers.len();

    let gamma = (0..width)
        .map(|c| numbers.iter().filter(|row| row[c] == 1).count() * 2 > height)
        .map(|has_more_1| if has_more_1 { 1 } else { 0 })
        .fold(0u32, |x, b| (x << 1) + b);

    let epsilon = (!gamma) & ((1 << width) - 1);

    println!("{}", gamma * epsilon)
}

fn filter_grid(
    grid: &Vec<Vec<u8>>,
    remaining_rows: Vec<usize>,
    column: usize,
    filter_function: fn(usize, usize) -> u8,
) -> usize {
    if remaining_rows.len() == 1 {
        return remaining_rows[0];
    }

    let count_1 = remaining_rows
        .iter()
        .map(|&row| grid[row][column])
        .filter(|&n| n == 1)
        .count();
    let count_0 = remaining_rows.len() - count_1;

    let filter_b = filter_function(count_1, count_0);

    let remaining_rows = remaining_rows
        .into_iter()
        .filter(|&r| grid[r][column] == filter_b)
        .collect_vec();
    filter_grid(grid, remaining_rows, column + 1, filter_function)
}

pub fn day3_part2() {
    let grid = std_iter!(Lines)
        .map(|line| parse_row_bits(&line))
        .collect_vec();

    let o2_row = filter_grid(
        &grid,
        (0..grid.len()).collect_vec(),
        0,
        |count_1, count_0| if count_1 >= count_0 { 1 } else { 0 },
    );
    let co2_row = filter_grid(
        &grid,
        (0..grid.len()).collect_vec(),
        0,
        |count_1, count_0| if count_1 >= count_0 { 0 } else { 1 },
    );

    let o2_number = binary_to_dec(&grid[o2_row]);
    let co2_number = binary_to_dec(&grid[co2_row]);
    println!("{}", o2_number * co2_number);
}

fn has_bingo(board: &[u32]) -> bool {
    if board
        .iter()
        .chunks(5)
        .into_iter()
        .any(|mut row| row.all(|n| *n == 0))
    {
        return true;
    }
    if (0usize..5).any(|c| (0..5).all(|r| board[r * 5 + c] == 0)) {
        return true;
    }
    return false;
}

pub fn day4_part1() {
    let mut lines = std_iter!(Lines);
    let num_sequence = lines
        .next()
        .unwrap()
        .split(",")
        .map(|string| string.parse::<u32>().unwrap())
        .collect_vec();
    let mut boards: Vec<u32> = lines
        .map(|l| {
            l.split(" ")
                .filter_map(|l| l.parse::<u32>().ok())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let board_count = boards.len() / 25;
    for n in num_sequence.into_iter() {
        for x in boards.iter_mut() {
            if *x == n {
                *x = 0;
            }
        }

        for board in 0..board_count {
            if has_bingo(&boards[board * 25..(board + 1) * 25]) {
                println!(
                    "{}",
                    boards[board * 25..(board + 1) * 25].iter().sum::<u32>() * n
                );
                return;
            }
        }
    }
}

pub fn day4_part2() {
    let mut lines = std_iter!(Lines);
    let num_sequence = lines
        .next()
        .unwrap()
        .split(",")
        .map(|string| string.parse::<u32>().unwrap())
        .collect_vec();
    let mut boards: Vec<u32> = lines
        .map(|l| {
            l.split(" ")
                .filter_map(|l| l.parse::<u32>().ok())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let board_count = boards.len() / 25;
    let mut remaining_boards = (0..board_count).collect_vec();

    for n in num_sequence.into_iter() {
        for x in boards.iter_mut() {
            if *x == n {
                *x = 0;
            }
        }

        let boards_left = remaining_boards
            .iter()
            .map(|board| *board)
            .filter(|board| !has_bingo(&boards[board * 25..(board + 1) * 25]))
            .collect_vec();

        if boards_left.len() == 0 {
            let board = remaining_boards[0];
            println!(
                "{}",
                boards[board * 25..(board + 1) * 25].iter().sum::<u32>() * n
            );
            return;
        }

        remaining_boards = boards_left;
    }
}

pub fn day5_part1() {
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

pub fn day5_part2() {
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

pub fn day6_part1() {
    let fishes = std_iter!(Bytes)
        .filter(|c| c.is_dec_digit())
        .map(|c| c - b'0')
        .fold(vec![0; 9], |mut v, d| {
            v[d as usize] += 1;
            v
        });
    println!("{}", fishes_reproduction(fishes, 80));
}

pub fn day6_part2() {
    let fishes = std_iter!(Bytes)
        .filter(|c| c.is_dec_digit())
        .map(|c| c - b'0')
        .fold(vec![0; 9], |mut v, d| {
            v[d as usize] += 1;
            v
        });
    println!("{}", fishes_reproduction(fishes, 256));
}

pub fn day7_part1() {
    let numbers = std_iter!(SplitBy ",")
        .map(|x| x.parse::<i64>().unwrap())
        .sorted()
        .collect_vec();
    let median = numbers[numbers.len() / 2];
    let total_diff: i64 = numbers.iter().map(|&x| (x - median).abs()).sum();
    println!("{}", total_diff);
}

pub fn day7_part2() {
    let numbers = std_iter!(SplitBy ",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let mean = numbers.iter().sum::<i64>() as f64 / numbers.len() as f64;

    let total_diff: i64 = [mean.floor() as i64, mean.ceil() as i64]
        .iter()
        .map(|i| numbers.iter().map(|&n| sum_to_1((n - i).abs())).sum())
        .min()
        .unwrap();
    println!("{}", total_diff);
}

pub fn day8_part1() {
    let count = std_iter!(Lines)
        .map(|l| {
            let (_, rhs) = l.split_once(" | ").unwrap();
            rhs.split_whitespace()
                .filter(|l| match l.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>();
    println!("{}", count);
}

fn encode_char(s: u8) -> u8 {
    1 << (s - b'a')
}

fn encode_lcd(s: &[u8]) -> (usize, u8) {
    (s.len(), s.iter().fold(0, |n, b| n | encode_char(*b)))
}

fn pop_if<T, F>(vec: &mut Vec<T>, f: F) -> T
where
    F: Fn(&T) -> bool,
{
    let (x, _) = vec.iter().find_position(|&x| f(x)).expect("Not found");
    vec.remove(x)
}

pub fn day8_part2() {
    let result = std_iter!(Lines)
        .map(|l| {
            let (lhs, rhs) = l.split_once(" | ").unwrap();

            let mut patterns = lhs
                .split_whitespace()
                .map(|l| encode_lcd(l.as_bytes()))
                .fold(vec![vec![]; 8], |mut v, (count, pattern)| {
                    v[count].push(pattern);
                    v
                });
            let mut representations = [0; 10];

            representations[1] = patterns[2][0];
            representations[7] = patterns[3][0];
            representations[4] = patterns[4][0];
            representations[8] = patterns[7][0];

            let four_angle = representations[4] - representations[1];
            let one = representations[1];

            representations[5] = pop_if(&mut patterns[5], |x| x & four_angle == four_angle);
            representations[3] = pop_if(&mut patterns[5], |x| x & one == one);
            representations[2] = patterns[5][0];
            representations[0] = pop_if(&mut patterns[6], |x| x & four_angle != four_angle);
            representations[9] = pop_if(&mut patterns[6], |x| x & one == one);
            representations[6] = patterns[6][0];

            rhs.split_whitespace()
                .map(|token| {
                    let (_, pattern) = encode_lcd(token.as_bytes());
                    let (i, _) = representations
                        .iter()
                        .find_position(|&&x| x == pattern)
                        .unwrap();
                    i
                })
                .reduce(|sum, i| sum * 10 + i)
                .unwrap()
        })
        .sum::<usize>();

    println!("{}", result);
}

fn make_volcano_grid() -> Vec<Vec<u8>> {
    std_iter!(Lines)
        .map(|l| l.as_bytes().into_iter().map(|&b| b - b'0').collect_vec())
        .collect_vec()
}

fn make_neighbors(
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (y > 0, (y - 1, x)),
        (x > 0, (y, x - 1)),
        (y < height - 1, (y + 1, x)),
        (x < width - 1, (y, x + 1)),
    ]
    .into_iter()
    .filter_map(|(cond, value)| if cond { Some(value) } else { None })
}

pub fn day9_part1() {
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

pub fn day9_part2() {
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

enum LineProcessResult {
    IllegalChar(u8),
    UnmatchedChars(Vec<u8>),
}

fn left_bracket_of(c: u8) -> u8 {
    match c {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => unreachable!(),
    }
}

fn process_line(s: &str) -> LineProcessResult {
    let mut stack = vec![];
    for c in s.bytes() {
        match c {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(c);
            }
            _ => {
                if stack.pop().unwrap() != left_bracket_of(c) {
                    return LineProcessResult::IllegalChar(c);
                }
            }
        }
    }
    LineProcessResult::UnmatchedChars(stack)
}

pub fn day10_part1() {
    let score = std_iter!(Lines)
        .filter_map(|l| match process_line(&l) {
            LineProcessResult::IllegalChar(c) => Some(c),
            _ => None,
        })
        .map(|char| match char {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum::<u64>();
    println!("{}", score);
}

pub fn day10_part2() {
    let mut scores = std_iter!(Lines)
        .filter_map(|l| match process_line(&l) {
            LineProcessResult::UnmatchedChars(chars) => Some(chars),
            _ => None,
        })
        .map(|remaining_chars| {
            remaining_chars
                .into_iter()
                .rev()
                .map(|char| match char {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                })
                .fold(0u64, |sum, x| sum * 5 + x)
        })
        .collect_vec();

    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}

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

pub fn day11_part1() {
    let mut grid = std_iter!(Lines)
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let result = (0..100).map(|_| octopus_step(&mut grid)).sum::<usize>();
    println!("{}", result);
}
pub fn day11_part2() {
    let mut grid = std_iter!(Lines)
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let step = (1..usize::MAX)
        .find(|_| octopus_step(&mut grid) == grid.len() * grid[0].len())
        .unwrap();
    println!("{}", step);
}
pub fn day12_part1() {
    todo!()
}
pub fn day12_part2() {
    todo!()
}
pub fn day13_part1() {
    todo!()
}
pub fn day13_part2() {
    todo!()
}
pub fn day14_part1() {
    todo!()
}
pub fn day14_part2() {
    todo!()
}
pub fn day15_part1() {
    todo!()
}
pub fn day15_part2() {
    todo!()
}
pub fn day16_part1() {
    todo!()
}
pub fn day16_part2() {
    todo!()
}
pub fn day17_part1() {
    todo!()
}
pub fn day17_part2() {
    todo!()
}
pub fn day18_part1() {
    todo!()
}
pub fn day18_part2() {
    todo!()
}
pub fn day19_part1() {
    todo!()
}
pub fn day19_part2() {
    todo!()
}
pub fn day20_part1() {
    todo!()
}
pub fn day20_part2() {
    todo!()
}
pub fn day21_part1() {
    todo!()
}
pub fn day21_part2() {
    todo!()
}
pub fn day22_part1() {
    todo!()
}
pub fn day22_part2() {
    todo!()
}
pub fn day23_part1() {
    todo!()
}
pub fn day23_part2() {
    todo!()
}
pub fn day24_part1() {
    todo!()
}
pub fn day24_part2() {
    todo!()
}
pub fn day25_part1() {
    todo!()
}
pub fn day25_part2() {
    todo!()
}
