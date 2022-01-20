use crate::{prelude::*, std_iter};

fn parse_deer(s: &str) -> IResult<&str, (u64, u64, u64)> {
    // Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
    let (s, speed) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    let (s, fly_time) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    let (s, rest_time) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    Ok((s, (speed, fly_time, rest_time)))
}

fn distance_traveled(deer: &(u64, u64, u64), time: u64) -> u64 {
    let &(speed, fly_time, rest_time) = deer;
    let cycle = fly_time + rest_time;
    let n_cycle = time / cycle;
    let extra_time = time % cycle;
    n_cycle * fly_time * speed + extra_time.min(fly_time) * speed
}

pub fn part1() {
    let max_distance = std_iter!(Lines)
        .filter_map(|l| parse_deer(&l).ok().and_then(|(_, r)| Some(r)))
        .map(|deer| distance_traveled(&deer, 2503))
        .max()
        .unwrap();
    println!("{}", max_distance);
}

pub fn part2() {
    let deers = std_iter!(Lines)
        .filter_map(|l| parse_deer(&l).ok().and_then(|(_, r)| Some(r)))
        .collect_vec();
    let winner = (1..=2503)
        .map(|time| {
            let distances = deers
                .iter()
                .map(|deer| distance_traveled(deer, time))
                .collect_vec();
            let max = distances.iter().max().unwrap();
            distances
                .iter()
                .positions(|x| x == max)
                .collect_vec()
                .into_iter()
        })
        .flatten()
        .counts()
        .into_values()
        .max()
        .unwrap();

    println!("{}", winner);
}
