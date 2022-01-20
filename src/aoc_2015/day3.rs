use crate::{prelude::*, std_iter};

pub fn part1() {
    let count = std_iter!(Bytes)
        .scan((0, 0), |(x, y), step| {
            match step {
                b'>' => *x += 1,
                b'<' => *x -= 1,
                b'^' => *y += 1,
                b'v' => *y -= 1,
                _ => unreachable!(),
            };
            Some((*x, *y))
        })
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();
    println!("{}", count);
}

pub fn part2() {
    let (script_santa, script_robot) = std_iter!(Bytes).tee();

    let count = vec![script_santa.skip(0), script_robot.skip(1)]
        .into_iter()
        .map(|instructions| {
            instructions.step_by(2).scan((0, 0), |(x, y), step| {
                match step {
                    b'>' => *x += 1,
                    b'<' => *x -= 1,
                    b'^' => *y += 1,
                    b'v' => *y -= 1,
                    _ => unreachable!(),
                };
                Some((*x, *y))
            })
        })
        .flatten()
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();

    println!("{}", count);
}
