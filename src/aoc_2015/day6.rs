use crate::{prelude::*, std_iter};

type Rect = ((usize, usize), (usize, usize));

#[derive(Debug)]
enum Instruction {
    TurnOn(Rect),
    Toggle(Rect),
    TurnOff(Rect),
}

fn parse_instruction(line: &str) -> IResult<&str, Instruction> {
    let (input, ins_type) = alt((tag("turn on "), tag("toggle "), tag("turn off ")))(line)?;
    let mut point_parser = separated_pair(parse_usize, is_a(","), parse_usize);
    let (input, coord_1) = point_parser(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, coord_2) = point_parser(input)?;

    let instruction = match ins_type {
        "turn on " => Instruction::TurnOn((coord_1, coord_2)),
        "turn off " => Instruction::TurnOff((coord_1, coord_2)),
        "toggle " => Instruction::Toggle((coord_1, coord_2)),
        _ => unreachable!(),
    };
    Ok((input, instruction))
}

macro_rules! rect_coords {
    ($rect:expr) => {
        (($rect.0 .0)..($rect.1 .0 + 1)).cartesian_product(($rect.0 .1)..($rect.1 .1 + 1))
    };
}

pub fn part1() {
    let count: u32 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0u32; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] ^= 1)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 1)
                }
                Instruction::TurnOff(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 0)
                }
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}

pub fn part2() {
    let count: i64 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0i64; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 2)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 1)
                }
                Instruction::TurnOff(rect) => rect_coords!(rect)
                    .for_each(|(x, y)| grid[x * 1000 + y] = (grid[x * 1000 + y] - 1).max(0)),
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}
