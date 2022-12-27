use crate::prelude::*;

fn parse_point(i: &str) -> ParseResult<(i64, i64)> {
    separated_pair(parse_i64, tag(","), parse_i64)(i)
}

fn parse_path(i: &str) -> ParseResult<Vec<(i64, i64)>> {
    separated_list1(tag(" -> "), parse_point)(i)
}

fn parse_input(i: &str) -> ParseResult<Vec<Vec<(i64, i64)>>> {
    separated_list1(tag("\n"), parse_path)(i)
}

fn scanline(start: (i64, i64), end: (i64, i64)) -> Vec<(i64, i64)> {
    if end.0 > start.0 {
        return (start.0..=end.0).map(|x| (x, start.1)).collect();
    }
    if end.0 < start.0 {
        return (end.0..=start.0).map(|x| (x, start.1)).collect();
    }
    if end.1 > start.1 {
        return (start.1..=end.1).map(|y| (start.0, y)).collect();
    }
    if end.1 < start.1 {
        return (end.1..=start.1).map(|y| (start.0, y)).collect();
    }
    unreachable!()
}

fn sand_drop(x: i64, y: i64, bottom: i64, occupied: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    let mut x = x;
    for y in y..bottom {
        if !occupied.contains(&(x, y + 1)) {
            continue;
        }
        if !occupied.contains(&(x - 1, y + 1)) {
            x -= 1;
            continue;
        }
        if !occupied.contains(&(x + 1, y + 1)) {
            x += 1;
            continue;
        }
        return Some((x, y));
    }
    return None;
}

pub fn part1() {
    let mut occupied: HashSet<(i64, i64)> = parse_input(stdio_string().as_str())
        .unwrap()
        .1
        .into_iter()
        .flat_map(|path| {
            (0..path.len() - 1)
                .flat_map(|i| scanline(path[i], path[i + 1]).into_iter())
                .collect_vec()
                .into_iter()
        })
        .collect();

    let bottom = occupied.iter().map(|(x, y)| *y).max().unwrap();

    for count in 0.. {
        if let Some(sand) = sand_drop(500, 0, bottom, &occupied) {
            occupied.insert(sand);
        } else {
            println!("{}", count);
            break;
        }
    }
}

pub fn part2() {
    todo!()
}
