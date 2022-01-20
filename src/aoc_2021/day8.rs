use crate::{prelude::*, std_iter};

pub fn part1() {
    let count = std_iter!(Lines)
        .map(|l| {
            let (_, rhs) = l.split_once(" | ").unwrap();
            rhs.split_whitespace()
                .filter(|l| match l.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>();
    println!("{}", count);
}

fn encode_char(s: u8) -> u8 {
    1 << (s - b'a')
}

fn encode_lcd(s: &[u8]) -> (usize, u8) {
    (s.len(), s.iter().fold(0, |n, b| n | encode_char(*b)))
}

fn pop_if<T, F>(vec: &mut Vec<T>, f: F) -> T
where
    F: Fn(&T) -> bool,
{
    let (x, _) = vec.iter().find_position(|&x| f(x)).expect("Not found");
    vec.remove(x)
}

pub fn part2() {
    let result = std_iter!(Lines)
        .map(|l| {
            let (lhs, rhs) = l.split_once(" | ").unwrap();

            let mut patterns = lhs
                .split_whitespace()
                .map(|l| encode_lcd(l.as_bytes()))
                .fold(vec![vec![]; 8], |mut v, (count, pattern)| {
                    v[count].push(pattern);
                    v
                });
            let mut representations = [0; 10];

            representations[1] = patterns[2][0];
            representations[7] = patterns[3][0];
            representations[4] = patterns[4][0];
            representations[8] = patterns[7][0];

            let four_angle = representations[4] - representations[1];
            let one = representations[1];

            representations[5] = pop_if(&mut patterns[5], |x| x & four_angle == four_angle);
            representations[3] = pop_if(&mut patterns[5], |x| x & one == one);
            representations[2] = patterns[5][0];
            representations[0] = pop_if(&mut patterns[6], |x| x & four_angle != four_angle);
            representations[9] = pop_if(&mut patterns[6], |x| x & one == one);
            representations[6] = patterns[6][0];

            rhs.split_whitespace()
                .map(|token| {
                    let (_, pattern) = encode_lcd(token.as_bytes());
                    let (i, _) = representations
                        .iter()
                        .find_position(|&&x| x == pattern)
                        .unwrap();
                    i
                })
                .reduce(|sum, i| sum * 10 + i)
                .unwrap()
        })
        .sum::<usize>();

    println!("{}", result);
}
