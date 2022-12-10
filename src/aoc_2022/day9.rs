use nom::character::complete::one_of;

use crate::{prelude::*, std_iter};

fn parse(i: &str) -> IResult<&str, (char, u64)> {
    separated_pair(one_of("LRUD"), tag(" "), parse_u64)(i)
}

fn step(initial: (i64, i64), direction: char) -> (i64, i64) {
    match direction {
        'L' => (initial.0 - 1, initial.1),
        'R' => (initial.0 + 1, initial.1),
        'U' => (initial.0, initial.1 + 1),
        'D' => (initial.0, initial.1 - 1),
        _ => unreachable!(),
    }
}

fn follow((hx, hy): (i64, i64), (tx, ty): (i64, i64)) -> (i64, i64) {
    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        return (tx, ty);
    }
    let newx = if hx > tx { tx + 1 } else if hx < tx { tx - 1 } else { tx };
    let newy = if hy > ty { ty + 1 } else if hy < ty { ty - 1 } else { ty };
    return (newx, newy);
}

pub fn part1() {
    let visited: HashSet<(i64, i64)> = std_iter!(Lines)
        .map(|l| parse(&l).unwrap().1)
        .map(|(direction, steps)| vec![direction; steps as usize].into_iter())
        .flatten()
        .scan((0, 0), |head, direction| {
            *head = step(*head, direction);
            Some(*head)
        })
        .scan((0, 0), |tail, head| {
            *tail = follow(head, *tail);
            Some(*tail)
        })
        .collect();

    println!("{}", visited.len());
}

pub fn part2() {
    let visited: HashSet<(i64, i64)> = std_iter!(Lines)
        .map(|l| parse(&l).unwrap().1)
        .flat_map(|(direction, steps)| vec![direction; steps as usize].into_iter())
        .scan(vec![(0, 0); 10], |points, direction| {
            points[0] = step(points[0], direction);
            for i in 1..10 {
                points[i] = follow(points[i - 1], points[i]);
            }
            Some(points[9])
        })
        .collect();
    println!("{}", visited.len());
}
