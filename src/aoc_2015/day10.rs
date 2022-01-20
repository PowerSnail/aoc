use crate::{prelude::*, std_iter};

pub fn part1() {
    let init = std_iter!(Bytes).map(|b| (b - b'0') as u64).collect_vec();
    let n = (0..40).fold(init, |v, _| {
        let mut numbers = vec![v[0]];
        let mut counts = vec![1u64];
        for n in v.into_iter().skip(1) {
            let last = numbers.len() - 1;
            if n == numbers[last] {
                counts[last] += 1;
            } else {
                numbers.push(n);
                counts.push(1);
            }
        }
        counts
            .into_iter()
            .interleave(numbers.into_iter())
            .collect_vec()
    });
    println!("{}", n.len());
}

pub fn part2() {
    let init = std_iter!(Bytes).map(|b| (b - b'0') as u64).collect_vec();
    let n = (0..50).fold(init, |v, _| {
        let mut numbers = vec![v[0]];
        let mut counts = vec![1u64];
        for n in v.into_iter().skip(1) {
            let last = numbers.len() - 1;
            if n == numbers[last] {
                counts[last] += 1;
            } else {
                numbers.push(n);
                counts.push(1);
            }
        }
        counts
            .into_iter()
            .interleave(numbers.into_iter())
            .collect_vec()
    });
    println!("{}", n.len());
}
