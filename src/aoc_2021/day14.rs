use crate::{prelude::*, std_iter};

fn polymer_grow(
    chain: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new_chain: HashMap<(u8, u8), usize> = HashMap::new();
    for ((c1, c2), count) in chain {
        if let Some(&c_mid) = rules.get(&(c1, c2)) {
            *new_chain.entry((c1, c_mid)).or_insert(0) += count;
            *new_chain.entry((c_mid, c2)).or_insert(0) += count;
        } else {
            *new_chain.entry((c1, c2)).or_insert(0) += count;
        }
    }
    new_chain
}

fn input() -> (HashMap<(u8, u8), u8>, HashMap<(u8, u8), usize>) {
    let lines = std_iter!(Lines).collect_vec();
    let rules: HashMap<(u8, u8), u8> = lines[2..]
        .iter()
        .filter_map(|l| l.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(lhs, rhs)| ((lhs.as_bytes()[0], lhs.as_bytes()[1]), rhs.as_bytes()[0]))
        .collect();
    let template = format!("a{}a", &lines[0]).bytes().tuple_windows().counts();
    (rules, template)
}

pub fn part1() {
    let (rules, template) = input();
    let final_chain = (0..10).fold(template, |chain, _| polymer_grow(chain, &rules));
    let mut counts = HashMap::new();
    for ((c1, c2), count) in final_chain {
        *counts.entry(c1).or_insert(0) += count;
        *counts.entry(c2).or_insert(0) += count;
    }
    counts.remove(&b'a');
    if let MinMaxResult::MinMax(min, max) = counts.values().minmax() {
        println!("{}", (*max - *min) / 2);
    }
}

pub fn part2() {
    let (rules, template) = input();
    let final_chain = (0..40).fold(template, |chain, _| polymer_grow(chain, &rules));
    let mut counts = HashMap::new();
    for ((c1, c2), count) in final_chain {
        *counts.entry(c1).or_insert(0) += count;
        *counts.entry(c2).or_insert(0) += count;
    }
    counts.remove(&b'a');
    if let MinMaxResult::MinMax(min, max) = counts.values().minmax() {
        println!("{}", (*max - *min) / 2);
    }
}
