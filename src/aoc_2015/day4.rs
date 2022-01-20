use crate::{prelude::*, std_iter};

fn collision(key: &[u8], target: &[u8]) -> i32 {
    let l = (target.len() + 1) / 2;
    (1..i32::MAX)
        .find(|i| {
            let crack = [key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            let prefix = &digest[..l];
            let mut prefix_hex = vec![];
            for i in prefix {
                prefix_hex.push(i & 0xF0);
                prefix_hex.push(i & 0x0F);
            }
            prefix_hex.starts_with(target)
        })
        .unwrap()
}

pub fn part1() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();
    let i = collision(&key, &[0, 0, 0, 0, 0]);
    println!("{}", i);
}

pub fn part2() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();
    let i = collision(&key, &[0, 0, 0, 0, 0, 0]);
    println!("{}", i);
}
