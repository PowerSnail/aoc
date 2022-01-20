use crate::{prelude::*, std_iter};

fn parse_fold(l: &str) -> IResult<&str, (char, u64)> {
    preceded(
        tag("fold along "),
        separated_pair(alt((char('x'), char('y'))), char('='), digit0),
    )(l)
    .and_then(|(l, (x_y, n))| Ok((l, (x_y, n.parse().unwrap()))))
}

fn print_board(points: &Vec<(u64, u64)>) {
    let (width, height) = points.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max_x.max(x), max_y.max(y))
    });
    let mut board = vec![vec![' '; width as usize + 1]; height as usize + 1];
    for &(x, y) in points.iter() {
        board[y as usize][x as usize] = '█';
    }
    for line in board.into_iter() {
        for c in line.into_iter() {
            print!("{}", c);
        }
        println!();
    }
}

pub fn part1() {
    let lines = std_iter!(Lines).collect_vec();
    let points: Vec<(u64, u64)> = lines
        .iter()
        .filter_map(|l| l.split(",").collect_tuple::<(&str, &str)>())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect_vec();

    let final_points = lines
        .iter()
        .skip(points.len() + 1)
        .take(1)
        .map(|l| parse_fold(l.as_str()).expect("failed to parse fold").1)
        .fold(points, |points, (x_y, n)| {
            points
                .into_iter()
                .map(|(x, y)| {
                    let x = if x_y == 'x' && x > n { 2 * n - x } else { x };
                    let y = if x_y == 'y' && y > n { 2 * n - y } else { y };
                    (x, y)
                })
                .unique()
                .collect_vec()
        });

    println!("{}", final_points.len());
}

pub fn part2() {
    let lines = std_iter!(Lines).collect_vec();
    let points: Vec<(u64, u64)> = lines
        .iter()
        .filter_map(|l| l.split(",").collect_tuple::<(&str, &str)>())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect_vec();

    let final_points = lines
        .iter()
        .skip(points.len() + 1)
        .map(|l| parse_fold(l.as_str()).expect("failed to parse fold").1)
        .fold(points, |points, (x_y, n)| {
            points
                .into_iter()
                .map(|(x, y)| {
                    let x = if x_y == 'x' && x > n { 2 * n - x } else { x };
                    let y = if x_y == 'y' && y > n { 2 * n - y } else { y };
                    (x, y)
                })
                .unique()
                .collect_vec()
        });

    print_board(&final_points);
}
