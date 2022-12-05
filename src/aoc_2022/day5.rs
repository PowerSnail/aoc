use nom::character::complete::{anychar, one_of};

use crate::{prelude::*, std_iter};

fn parse_box(i: &str) -> IResult<&str, char> {
    alt((
        delimited(char('['), anychar, char(']')),
        delimited(char(' '), char(' '), char(' ')),
    ))(i)
}

fn parse_row(i: &str) -> IResult<&str, Vec<char>> {
    separated_list1(char(' '), parse_box)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, (u64, u64, u64)> {
    let (i, count) = preceded(tag("move "), parse_u64)(i)?;
    let (i, from) = preceded(tag(" from "), parse_u64)(i)?;
    let (i, to) = preceded(tag(" to "), parse_u64)(i)?;

    Ok((i, (count, from - 1, to - 1)))
}

fn input() -> (Vec<std::collections::VecDeque<char>>, Vec<(u64, u64, u64)>) {
    let mut grid = vec![];
    let mut instructions = vec![];
    for line in std_iter!(Lines) {
        if let Ok((_, s)) = parse_row(&line) {
            if grid.len() == 0 {
                for _ in 0..s.len() {
                    grid.push(VecDeque::new());
                }
            }
            for (i, c) in s.into_iter().enumerate() {
                if c != ' ' {
                    grid[i].push_front(c);
                }
            }
        } else if let Ok((_, v)) = parse_instruction(&line) {
            instructions.push(v);
        }
    }
    return (grid, instructions);
}

fn print_top(grid: &Vec<VecDeque<char>>) {
    for stack in grid {
        print!("{}", stack.back().unwrap())
    }
    println!("");
}

pub fn part1() {
    let (mut grid, instructions) = input();
    for (count, from, to) in instructions {
        for _ in 0..count {
            let c = grid[from as usize].pop_back().unwrap();
            grid[to as usize].push_back(c);
        }
    }
    print_top(&grid);
}

pub fn part2() {
    let (mut grid, instructions) = input();
    for (count, from, to) in instructions {
        let mut temp = vec![];
        for _ in 0..count {
            let c = grid[from as usize].pop_back().unwrap();
            temp.push(c);
        }
        temp.reverse();
        grid[to as usize].extend(temp);
    }
    print_top(&grid);
}
