use crate::{prelude::*, std_iter};

fn target_sue(key: &str) -> i64 {
    match key {
        "children" => 3,
        "cats" => 7,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 0,
        "vizslas" => 0,
        "goldfish" => 5,
        "trees" => 3,
        "cars" => 2,
        "perfumes" => 1,
        _ => unreachable!(),
    }
}

fn target_sue2(key: &str, v: i64) -> bool {
    match key {
        "children" => v == 3,
        "cats" => v > 7,
        "samoyeds" => v == 2,
        "pomeranians" => v < 3,
        "akitas" => v == 0,
        "vizslas" => v == 0,
        "goldfish" => v < 5,
        "trees" => v > 3,
        "cars" => v == 2,
        "perfumes" => v == 1,
        _ => unreachable!(),
    }
}

fn parse_sue(s: &str) -> IResult<&str, Vec<(String, i64)>> {
    let (s, _) = take_after(": ")(s)?;
    let (s, values) = separated_list0(tag(", "), separated_pair(alpha1, tag(": "), parse_i64))(s)?;
    let values = values
        .into_iter()
        .map(|(key, v)| (key.to_string(), v))
        .collect_vec();
    Ok((s, values))
}

pub fn part1() {
    let (sue_id, _) = std_iter!(Lines)
        .map(|l| parse_sue(&l).unwrap().1)
        .find_position(|sue| sue.iter().all(|(key, v)| target_sue(key) == *v))
        .unwrap();
    println!("{}", sue_id + 1);
}

pub fn part2() {
    let (sue_id, _) = std_iter!(Lines)
        .map(|l| parse_sue(&l).unwrap().1)
        .find_position(|sue| sue.iter().all(|(key, v)| target_sue2(key, *v)))
        .unwrap();
    println!("{}", sue_id + 1);
}
