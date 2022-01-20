use crate::{prelude::*, std_iter};

pub fn part1() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("00000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn part2() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("000000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}
