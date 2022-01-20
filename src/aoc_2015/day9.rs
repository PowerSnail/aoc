use crate::{prelude::*, std_iter};

pub fn part1() {
    let lines = std_iter!(Lines).collect_vec();
    let tokens = lines
        .iter()
        .map(|l| l.split(" ").collect_vec())
        .collect_vec();

    let nodes: HashMap<&str, usize> = tokens
        .iter()
        .map(|l| [l[0], l[2]])
        .flatten()
        .unique()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();

    let mut graph = vec![vec![0; nodes.len()]; nodes.len()];

    for line in tokens.iter() {
        let c1 = nodes.get(line[0]).unwrap();
        let c2 = nodes.get(line[2]).unwrap();
        graph[*c1][*c2] = line[4].parse().unwrap();
        graph[*c2][*c1] = line[4].parse().unwrap();
    }

    let min_distance = (0..nodes.len())
        .permutations(nodes.len())
        .map(|x| {
            x.iter()
                .zip(x.iter().skip(1))
                .map(|(&a, &b)| graph[a][b])
                .sum::<u64>()
        })
        .min()
        .unwrap();
    println!("{}", min_distance);
}

pub fn part2() {
    let lines = std_iter!(Lines).collect_vec();
    let tokens = lines
        .iter()
        .map(|l| l.split(" ").collect_vec())
        .collect_vec();

    let nodes: HashMap<&str, usize> = tokens
        .iter()
        .map(|l| [l[0], l[2]])
        .flatten()
        .unique()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();

    let mut graph = vec![vec![0; nodes.len()]; nodes.len()];

    for line in tokens.iter() {
        let c1 = nodes.get(line[0]).unwrap();
        let c2 = nodes.get(line[2]).unwrap();
        graph[*c1][*c2] = line[4].parse().unwrap();
        graph[*c2][*c1] = line[4].parse().unwrap();
    }

    let max_distance = (0..nodes.len())
        .permutations(nodes.len())
        .map(|x| {
            x.iter()
                .zip(x.iter().skip(1))
                .map(|(&a, &b)| graph[a][b])
                .sum::<u64>()
        })
        .max()
        .unwrap();
    println!("{}", max_distance);
}
