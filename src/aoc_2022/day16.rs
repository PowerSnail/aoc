use crate::prelude::*;
use std::hash::Hash;

struct Node<'a> {
    name: &'a str,
    children: Vec<&'a str>,
    rate: u64,
}

struct Graph<'a> {
    valves: HashMap<&'a str, Node<'a>>,
    distances: HashMap<(&'a str, &'a str), u64>,
}

impl<'a> Graph<'a> {
    fn get_valve(&self, name: &'a str) -> &'a Node {
        self.valves.get(name)
            .expect(&format!("Has no valve named {}", name))
    }

    fn get_distance(&self, x: &str, y: &str) -> u64 {
        *self.distances.get(&(x, y))
            .expect(&format!("Cannot get distance between {}, and {}", x, y))
    }
}

fn parse_line(i: &str) -> ParseResult<Node> {
    let (i, name) = preceded(tag("Valve "), alpha1)(i)?;
    let (i, rate) = preceded(tag(" has flow rate="), parse_u64)(i)?;
    let (i, children) = preceded(
        tag("; tunnels lead to valves "),
        separated_list0(tag(", "), alpha1),
    )(i)?;
    Ok((
        i,
        Node {
            name,
            children,
            rate,
        },
    ))
}

fn parse_input(i: &str) -> ParserResult<Vec<Node>> {
    separated_list1(tag("\n"), parse_line)(i)
}

fn generate_graph(valves: Vec<Node>) -> Graph {
    let valves = HashMap::from_iter(valves.into_iter().map(|v| (v.name, v)));

    let mut distances = HashMap::new();
    for v in valves.values() {
        for (n, d) in dijkstra(v.name, |n| valves[n].children.clone(), |_, _| 1) {
            distances.insert((v.name, n), d);
        }
    }

    Graph { valves, distances }
}

fn dijkstra<T, FN, FD>(start: T, get_neighbors: FN, get_distance: FD) -> HashMap<T, u64>
where
    T: Copy + Hash + Eq + Ord,
    FN: Fn(&T) -> Vec<T>,
    FD: Fn(&T, &T) -> u64,
{
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut queue = BinaryHeap::new();
    queue.push((0, start));

    while let Some((distance, point)) = queue.pop() {
        if distances.contains_key(&point) && distance > distances[&point] {
            continue;
        }
        for neighbor in get_neighbors(&point) {
            let distance = distance + get_distance(&point, &neighbor);
            if !distances.contains_key(&neighbor) || distances[&neighbor] > distance {
                distances.insert(neighbor, distance);
                queue.push((distance, neighbor));
            }
        }
    }

    distances
}

fn maximize_flow(graph: &Graph, current: &str, time: u64, closed: HashSet<&str>) -> u64 {
    if time < 2 {
        return 0;
    }
    let flow = graph.valves[current].rate * (time - 1);
    let max = graph.valves[current]
        .children
        .iter()
        .map(|n| {
            let distance = graph.distances[&(current, *n)];
            let ignore_current = if time > distance {
                maximize_flow(graph, current, time - distance, closed.clone())
            } else {
                0
            };
            let use_current = if flow > 0 && time - 1 > distance {
                let closed = closed.difference(&HashSet::from([current])).cloned().collect();
                maximize_flow(graph, current, time - distance - 1, closed)
            } else {
                0
            };
            ignore_current.max(use_current)
        })
        .max()
        .unwrap();
    return max;
}

pub fn part1() {
    let input = stdio_string();
    let valves = parse_input(&input).unwrap().1;
    let graph = generate_graph(valves);
    let all_nodes = HashSet::from_iter(graph.valves.keys().cloned());
    let max = maximize_flow(&graph, "AA", 30, all_nodes);
    println!("{}", max);
}

pub fn part2() {
    todo!()
}
