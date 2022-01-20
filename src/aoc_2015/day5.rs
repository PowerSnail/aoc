use crate::{prelude::*, std_iter};

pub fn part1() {
    let count = std_iter!(Lines)
        .filter(|line| {
            let vowel_count = line
                .bytes()
                .filter(|&b| match b {
                    b'a' | b'e' | b'i' | b'o' | b'u' => true,
                    _ => false,
                })
                .count();

            let twice_in_a_row = line.bytes().tuple_windows().any(|(x, y)| x == y);

            let contains_bad_sub = line.bytes().tuple_windows().any(|(x, y)| match &[x, y] {
                b"ab" | b"cd" | b"pq" | b"xy" => true,
                _ => false,
            });

            (vowel_count >= 3) && twice_in_a_row && (!contains_bad_sub)
        })
        .count();
    println!("{}", count);
}

pub fn part2() {
    let count = std_iter!(Lines)
        .filter(|line| {
            let two_grams = line
                .bytes()
                .tuple_windows::<(u8, u8)>()
                .enumerate()
                .scan(HashMap::new(), |first_positions, (i, pair)| {
                    if !first_positions.contains_key(&pair) {
                        first_positions.insert(pair, i);
                    }
                    let first_pos = first_positions.get(&pair).unwrap();
                    Some(i - first_pos > 1)
                })
                .any(|x| x);

            let sandwiched = line.bytes().tuple_windows().any(|(x, _, z)| x == z);
            sandwiched && two_grams
        })
        .count();
    println!("{}", count);
}
