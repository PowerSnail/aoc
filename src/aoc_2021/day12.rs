use crate::{prelude::*, std_iter};

fn visit_submarine_graph<'a>(
    node: &'a str,
    edges: &HashMap<&str, Vec<&str>>,
    mut visited: Vec<&'a str>,
) -> usize {
    if node == "end" {
        return 1;
    }
    visited.push(node);
    edges[node]
        .iter()
        .filter(|&neighbor| {
            !(neighbor.chars().next().unwrap().is_lowercase() && visited.contains(neighbor))
        })
        .map(|&destination| visit_submarine_graph(destination, edges, visited.clone()))
        .sum()
}

fn visit_submarine_graph2<'a>(
    node: &'a str,
    edges: &HashMap<&str, Vec<&str>>,
    mut visited: Vec<&'a str>,
) -> usize {
    if node == "end" {
        return 1;
    }
    visited.push(node);
    edges[node]
        .iter()
        .filter(|&neighbor| *neighbor != "start")
        .map(|&destination| {
            if destination.chars().next().unwrap().is_uppercase() {
                visit_submarine_graph2(destination, edges, visited.clone())
            } else {
                match visited.iter().filter(|&n| *n == destination).count() {
                    0 => visit_submarine_graph2(destination, edges, visited.clone()),
                    1 => visit_submarine_graph(destination, edges, visited.clone()),
                    _ => 0,
                }
            }
        })
        .sum()
}

pub fn part1() {
    let lines = std_iter!(Lines).collect_vec();
    let edges: HashMap<&str, Vec<&str>> = lines
        .iter()
        .map(|line| line.split("-").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(lhs, rhs)| [(lhs, rhs), (rhs, lhs)])
        .flatten()
        .sorted_unstable()
        .group_by(|&(a, _)| a)
        .into_iter()
        .map(|(key, group)| (key, group.into_iter().map(|p| p.1).collect_vec()))
        .collect();
    println!("{}", visit_submarine_graph("start", &edges, vec![]));
}

pub fn part2() {
    let lines = std_iter!(Lines).collect_vec();
    let edges: HashMap<&str, Vec<&str>> = lines
        .iter()
        .map(|line| line.split("-").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(lhs, rhs)| [(lhs, rhs), (rhs, lhs)])
        .flatten()
        .sorted_unstable()
        .group_by(|&(a, _)| a)
        .into_iter()
        .map(|(key, group)| (key, group.into_iter().map(|p| p.1).collect_vec()))
        .collect();
    println!("{}", visit_submarine_graph2("start", &edges, vec![]));
}
