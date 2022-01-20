use crate::{prelude::*, std_iter};

fn parse_line(input: &str) -> IResult<&str, (&str, &str, i64)> {
    // Alice would gain 54 happiness units by sitting next to Bob.
    let (input, name1) = alpha1(input)?;
    let (input, _) = tag(" would ")(input)?;
    let (input, direction) = alt((tag("lose "), tag("gain ")))(input)?;
    let (input, n) = digit1(input)?;
    let (input, _) = tag(" happiness units by sitting next to ")(input)?;
    let (input, name2) = alpha1(input)?;

    let happiness = match direction {
        "lose " => -1i64 * n.parse::<i64>().unwrap(),
        "gain " => n.parse().unwrap(),
        _ => unreachable!(),
    };

    Ok((input, (name1, name2, happiness)))
}

pub fn part1() {
    let lines = std_iter!(Lines).collect_vec();
    let rules = lines
        .iter()
        .map(|l| parse_line(l).expect("Parser Error").1)
        .collect_vec();
    let nodes: NodeRegistration = rules
        .iter()
        .map(|&(n1, n2, _)| [n1, n2])
        .flatten()
        .collect();
    let mut graph = vec![vec![0; nodes.len()]; nodes.len()];

    for (n1, n2, value) in rules {
        graph[nodes.get_id(n1).unwrap()][nodes.get_id(n2).unwrap()] = value;
    }

    let total: i64 = (0..nodes.len())
        .permutations(nodes.len())
        .map(|sittings| {
            (0..sittings.len())
                .map(|i| (sittings[i], sittings[(i + 1) % sittings.len()]))
                .map(|(prev, next)| graph[prev][next] + graph[next][prev])
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", total);
}

pub fn part2() {
    let lines = std_iter!(Lines).collect_vec();
    let rules = lines
        .iter()
        .map(|l| parse_line(l).expect("Parser Error").1)
        .collect_vec();
    let nodes: NodeRegistration = rules
        .iter()
        .map(|&(n1, n2, _)| [n1, n2])
        .flatten()
        .collect();
    let mut graph = vec![vec![0; nodes.len() + 1]; nodes.len() + 1];

    for (n1, n2, value) in rules {
        graph[nodes.get_id(n1).unwrap()][nodes.get_id(n2).unwrap()] = value;
    }

    let total: i64 = (0..graph.len())
        .permutations(graph.len())
        .map(|sittings| {
            (0..sittings.len())
                .map(|i| (sittings[i], sittings[(i + 1) % sittings.len()]))
                .map(|(prev, next)| graph[prev][next] + graph[next][prev])
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", total);
}
