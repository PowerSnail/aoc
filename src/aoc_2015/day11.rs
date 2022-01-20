use crate::{prelude::*, std_iter};

fn radix_26_parse(s: &[u8]) -> u64 {
    match s {
        [head @ .., x] => radix_26_parse(head) * 26 + (x - b'a') as u64,
        [] => 0,
    }
}

fn radix_26_encode(value: u64) -> Vec<u8> {
    match (value / 26, value % 26) {
        (0, 0) => vec![],
        (q, r) => {
            let mut v = radix_26_encode(q);
            v.push(r as u8 + b'a');
            v
        }
    }
}

fn password_is_ok(value: u64) -> bool {
    let forbidden_digits = [b'i', b'o', b'l'];
    let digits = radix_26_encode(value);

    let contains_bad_char = digits.iter().any(|x| forbidden_digits.contains(x));

    let pairs = digits
        .iter()
        .zip(digits.iter().skip(1))
        .enumerate()
        .filter_map(|(i, (x, y))| if x == y { Some(i) } else { None })
        .collect_vec();
    let has_2_pairs = pairs.len() > 3 || (pairs.len() == 2 && pairs[1] - pairs[0] > 1);

    let has_consecutive = digits
        .iter()
        .zip(digits.iter().skip(1))
        .zip(digits.iter().skip(2))
        .find(|((x, y), z)| *y - *x == 1 && *z - *y == 1)
        .is_some();

    has_consecutive && !contains_bad_char && has_2_pairs
}

pub fn part1() {
    let original = std_iter!(Bytes).collect_vec();
    let mut password = radix_26_parse(&original[..]);
    while !password_is_ok(password) {
        password += 1;
    }
    let string = radix_26_encode(password);
    println!("{}", String::from_utf8(string).unwrap());
}

pub fn part2() {
    let original = std_iter!(Bytes).collect_vec();
    let mut password = radix_26_parse(&original[..]);
    while !password_is_ok(password) {
        password += 1;
    }
    password += 1;
    while !password_is_ok(password) {
        password += 1;
    }
    let string = radix_26_encode(password);
    println!("{}", String::from_utf8(string).unwrap());
}
