use crate::std_iter;

fn input() -> Vec<u64> {
    std_iter!(Lines)
        .chain([String::from("")].into_iter())
        .fold(vec![0u64], |mut queue, line| {
            if line.is_empty() {
                queue.push(0);
            } else {
                *queue.last_mut().unwrap() += line.parse::<u64>().unwrap();
            }
            queue
        })
}

pub fn part1() {
    let max = input().into_iter().max().unwrap();
    println!("{}", max);
}

pub fn part2() {
    let mut sums = input();
    sums.sort();
    let three = &sums[sums.len() - 3..];

    println!("{}", three.iter().sum::<u64>());
}
