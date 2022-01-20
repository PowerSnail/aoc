use crate::{prelude::*, std_iter};

fn binary_to_dec(bits: &[u8]) -> u32 {
    bits.iter().fold(0u32, |n, b| (n << 1) + *b as u32)
}

fn parse_row_bits(row: &str) -> Vec<u8> {
    row.bytes().map(|c| c - b'0').collect_vec()
}

pub fn part1() {
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

pub fn part2() {
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
