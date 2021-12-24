// Copyright (c) 2021 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use nom::{bytes::complete::take_until, sequence::delimited};

use crate::{prelude::*, std_iter, v_add, v_max, v_times};
use std::str::FromStr;

// --- Parsings ---

fn char_to_step(c: u8) -> Option<i64> {
    match c {
        b'(' => Some(1),
        b')' => Some(-1),
        _ => None,
    }
}

pub fn day1_part1() {
    let total: i64 = std_iter!(Bytes).filter_map(char_to_step).sum();
    println!("{}", total);
}

pub fn day1_part2() {
    let position: usize = std_iter!(Bytes)
        .filter_map(char_to_step)
        .scan(0, |current, step| {
            *current += step;
            Some(*current)
        })
        .find_position(|c| *c == -1)
        .unwrap()
        .0
        + 1;
    println!("{}", position);
}

pub fn day2_part1() {
    let total: i64 = std_iter!(Lines)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let (total_sa, min_sa) = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().product())
                .fold((0, i64::MAX), |(total, min), s| (total + s * 2, min.min(s)));
            total_sa + min_sa
        })
        .sum();
    println!("{}", total);
}

pub fn day2_part2() {
    let total: i64 = std_iter!(Lines)
        .map(|line| separated_list0(tag("x"), parse_i64)(&line).unwrap().1)
        .map(|numbers| {
            let volume: i64 = numbers.iter().product();
            let shortest_waist: i64 = numbers
                .into_iter()
                .combinations(2)
                .map(|v| v.into_iter().sum::<i64>())
                .min()
                .unwrap()
                * 2i64;
            shortest_waist + volume
        })
        .sum();
    println!("{}", total);
}

pub fn day3_part1() {
    let count = std_iter!(Bytes)
        .scan((0, 0), |(x, y), step| {
            match step {
                b'>' => *x += 1,
                b'<' => *x -= 1,
                b'^' => *y += 1,
                b'v' => *y -= 1,
                _ => unreachable!(),
            };
            Some((*x, *y))
        })
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();
    println!("{}", count);
}

pub fn day3_part2() {
    let (script_santa, script_robot) = std_iter!(Bytes).tee();

    let count = vec![script_santa.skip(0), script_robot.skip(1)]
        .into_iter()
        .map(|instructions| {
            instructions.step_by(2).scan((0, 0), |(x, y), step| {
                match step {
                    b'>' => *x += 1,
                    b'<' => *x -= 1,
                    b'^' => *y += 1,
                    b'v' => *y -= 1,
                    _ => unreachable!(),
                };
                Some((*x, *y))
            })
        })
        .flatten()
        .chain(vec![(0, 0)].into_iter())
        .unique()
        .count();

    println!("{}", count);
}

pub fn day4_part1() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("00000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day4_part2() {
    let key: Vec<u8> = std_iter!(Bytes).filter(|c| c.is_alphanum()).collect();

    let i = (1..i32::MAX)
        .filter(|i| {
            let crack = [&key, i.to_string().as_bytes()].concat();
            let digest = md5::compute(&crack);
            format!("{:x}", digest).starts_with("000000")
        })
        .next()
        .unwrap();

    println!("{}", i);
}

pub fn day5_part1() {
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

pub fn day5_part2() {
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

type Rect = ((usize, usize), (usize, usize));

#[derive(Debug)]
enum Instruction {
    TurnOn(Rect),
    Toggle(Rect),
    TurnOff(Rect),
}

fn parse_instruction(line: &str) -> IResult<&str, Instruction> {
    let (input, ins_type) = alt((tag("turn on "), tag("toggle "), tag("turn off ")))(line)?;
    let mut point_parser = separated_pair(parse_usize, is_a(","), parse_usize);
    let (input, coord_1) = point_parser(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, coord_2) = point_parser(input)?;

    let instruction = match ins_type {
        "turn on " => Instruction::TurnOn((coord_1, coord_2)),
        "turn off " => Instruction::TurnOff((coord_1, coord_2)),
        "toggle " => Instruction::Toggle((coord_1, coord_2)),
        _ => unreachable!(),
    };
    Ok((input, instruction))
}

macro_rules! rect_coords {
    ($rect:expr) => {
        (($rect.0 .0)..($rect.1 .0 + 1)).cartesian_product(($rect.0 .1)..($rect.1 .1 + 1))
    };
}

pub fn day6_part1() {
    let count: u32 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0u32; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] ^= 1)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 1)
                }
                Instruction::TurnOff(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] = 0)
                }
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}

pub fn day6_part2() {
    let count: i64 = std_iter!(Lines)
        .map(|l| parse_instruction(&l).unwrap().1)
        .fold(vec![0i64; 1000 * 1000], |mut grid, instruction| {
            match instruction {
                Instruction::Toggle(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 2)
                }
                Instruction::TurnOn(rect) => {
                    rect_coords!(rect).for_each(|(x, y)| grid[x * 1000 + y] += 1)
                }
                Instruction::TurnOff(rect) => rect_coords!(rect)
                    .for_each(|(x, y)| grid[x * 1000 + y] = (grid[x * 1000 + y] - 1).max(0)),
            };
            grid
        })
        .into_iter()
        .sum();
    println!("{}", count);
}

#[derive(Debug)]
enum CircuitNode {
    Value(u16),
    Node(String),
}

impl FromStr for CircuitNode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse() {
            Ok(v) => Self::Value(v),
            Err(_) => Self::Node(s.to_string()),
        })
    }
}

enum CircuitConnection {
    Assign(CircuitNode),
    Not(CircuitNode),
    And(CircuitNode, CircuitNode),
    Or(CircuitNode, CircuitNode),
    RightShift(CircuitNode, CircuitNode),
    LeftShift(CircuitNode, CircuitNode),
}

fn parse_circuit_instruction(l: &str) -> (String, CircuitConnection) {
    let (lhs, rhs) = l.split(" -> ").collect_tuple().unwrap();
    let rhs = rhs.to_string();
    let tokens = lhs.split(" ").collect_vec();

    let connection = match tokens[..] {
        [x] => CircuitConnection::Assign(x.parse().unwrap()),
        ["NOT", x] => CircuitConnection::Not(x.parse().unwrap()),
        [x, "AND", y] => CircuitConnection::And(x.parse().unwrap(), y.parse().unwrap()),
        [x, "OR", y] => CircuitConnection::Or(x.parse().unwrap(), y.parse().unwrap()),
        [x, "RSHIFT", y] => CircuitConnection::RightShift(x.parse().unwrap(), y.parse().unwrap()),
        [x, "LSHIFT", y] => CircuitConnection::LeftShift(x.parse().unwrap(), y.parse().unwrap()),
        _ => unreachable!(),
    };

    return (rhs, connection);
}

fn visit(
    node: &CircuitNode,
    circuit: &HashMap<String, CircuitConnection>,
    memory: &mut HashMap<String, u16>,
) -> u16 {
    match node {
        CircuitNode::Value(v) => *v,
        CircuitNode::Node(n) => {
            if !memory.contains_key(n) {
                let value = match &circuit[n] {
                    CircuitConnection::Assign(x) => visit(x, circuit, memory),
                    CircuitConnection::Not(x) => !visit(x, circuit, memory),
                    CircuitConnection::And(x, y) => {
                        visit(x, circuit, memory) & visit(y, circuit, memory)
                    }
                    CircuitConnection::Or(x, y) => {
                        visit(x, circuit, memory) | visit(y, circuit, memory)
                    }
                    CircuitConnection::RightShift(x, b) => {
                        visit(x, circuit, memory) >> visit(b, circuit, memory)
                    }
                    CircuitConnection::LeftShift(x, b) => {
                        visit(x, circuit, memory) << visit(b, circuit, memory)
                    }
                };
                memory.insert(n.to_string(), value);
            }
            memory[n]
        }
    }
}

pub fn day7_part1() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory: HashMap<String, u16> = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);
    println!("{}", wire_a);
}

pub fn day7_part2() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    let mut memory = HashMap::from_iter([("b".to_string(), wire_a)]);
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    println!("{}", wire_a);
}

mod literals {

    pub fn decode_string(s: &[u8]) -> Option<Vec<u8>> {
        match s {
            [b'"', inner @ .., b'"'] => decode_inner_string(inner, vec![]),
            _ => None,
        }
    }

    fn decode_inner_string(s: &[u8], mut so_far: Vec<u8>) -> Option<Vec<u8>> {
        match s {
            [] => Some(so_far),
            [b'\\', c @ b'"' | c @ b'\\', tail @ ..] => {
                so_far.push(*c);
                decode_inner_string(tail, so_far)
            }
            [b'\\', b'x', a @ b'0'..=b'9' | a @ b'a'..=b'f', b @ b'0'..=b'9' | b @ b'a'..=b'f', tail @ ..] =>
            {
                let num = u8::from_str_radix(std::str::from_utf8(&[*a, *b]).unwrap(), 16).unwrap();
                so_far.push(num);
                decode_inner_string(tail, so_far)
            }
            [c, tail @ ..] => {
                so_far.push(*c);
                decode_inner_string(tail, so_far)
            }
        }
    }

    pub fn encode_string(s: &[u8]) -> Vec<u8> {
        let mut builder = vec![b'"'];
        for c in s {
            match *c {
                b'"' => builder.extend([b'\\', b'"']),
                b'\\' => builder.extend([b'\\', b'\\']),
                c => builder.push(c),
            };
        }
        builder.push(b'"');
        builder
    }
}

pub fn day8_part1() {
    let result: usize = std_iter!(Lines)
        .map(|l| l.len() - literals::decode_string(l.as_bytes()).unwrap().len())
        .sum();
    println!("{}", result);
}

pub fn day8_part2() {
    let result: usize = std_iter!(Lines)
        .map(|l| literals::encode_string(l.as_bytes()).len() - l.len())
        .sum();
    println!("{}", result);
}

pub fn day9_part1() {
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

pub fn day9_part2() {
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

pub fn day10_part1() {
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

pub fn day10_part2() {
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

pub fn day11_part1() {
    let original = std_iter!(Bytes).collect_vec();
    let mut password = radix_26_parse(&original[..]);
    while !password_is_ok(password) {
        password += 1;
    }
    let string = radix_26_encode(password);
    println!("{}", String::from_utf8(string).unwrap());
}

pub fn day11_part2() {
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

pub fn day12_part1() {
    let data = std_iter!(Lines).next().unwrap();
    let value: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut sum = 0.0;
    let mut stack = vec![&value];

    while let Some(v) = stack.pop() {
        match v {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
                ()
            }
            serde_json::Value::Number(n) => sum += n.as_f64().unwrap(),
            serde_json::Value::Array(arr) => arr.iter().for_each(|v| stack.push(v)),
            serde_json::Value::Object(obj) => obj.values().for_each(|v| stack.push(v)),
        }
    }
    println!("{}", sum);
}

pub fn day12_part2() {
    let data = std_iter!(Lines).next().unwrap();
    let value: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut sum = 0.0;
    let mut stack = vec![&value];

    while let Some(v) = stack.pop() {
        match v {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
                ()
            }
            serde_json::Value::Number(n) => sum += n.as_f64().unwrap(),
            serde_json::Value::Array(arr) => arr.iter().for_each(|v| stack.push(v)),
            serde_json::Value::Object(obj) => {
                if !obj.values().filter_map(|v| v.as_str()).any(|v| v == "red") {
                    obj.values().for_each(|v| stack.push(v))
                }
            }
        }
    }
    println!("{}", sum);
}

fn parse_day13_line(input: &str) -> IResult<&str, (&str, &str, i64)> {
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

pub fn day13_part1() {
    let lines = std_iter!(Lines).collect_vec();
    let rules = lines
        .iter()
        .map(|l| parse_day13_line(l).expect("Parser Error").1)
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

pub fn day13_part2() {
    let lines = std_iter!(Lines).collect_vec();
    let rules = lines
        .iter()
        .map(|l| parse_day13_line(l).expect("Parser Error").1)
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

fn parse_deer(s: &str) -> IResult<&str, (u64, u64, u64)> {
    // Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
    let (s, speed) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    let (s, fly_time) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    let (s, rest_time) = preceded(take_till(|c: char| c.is_digit(10)), parse_u64)(s)?;
    Ok((s, (speed, fly_time, rest_time)))
}

fn distance_traveled(deer: &(u64, u64, u64), time: u64) -> u64 {
    let &(speed, fly_time, rest_time) = deer;
    let cycle = fly_time + rest_time;
    let n_cycle = time / cycle;
    let extra_time = time % cycle;
    n_cycle * fly_time * speed + extra_time.min(fly_time) * speed
}

pub fn day14_part1() {
    let max_distance = std_iter!(Lines)
        .filter_map(|l| parse_deer(&l).ok().and_then(|(_, r)| Some(r)))
        .map(|deer| distance_traveled(&deer, 2503))
        .max()
        .unwrap();
    println!("{}", max_distance);
}

pub fn day14_part2() {
    let deers = std_iter!(Lines)
        .filter_map(|l| parse_deer(&l).ok().and_then(|(_, r)| Some(r)))
        .collect_vec();
    let winner = (1..=2503)
        .map(|time| {
            let distances = deers
                .iter()
                .map(|deer| distance_traveled(deer, time))
                .collect_vec();
            let max = distances.iter().max().unwrap();
            distances
                .iter()
                .positions(|x| x == max)
                .collect_vec()
                .into_iter()
        })
        .flatten()
        .counts()
        .into_values()
        .max()
        .unwrap();

    println!("{}", winner);
}

fn parse_recipe(s: &str) -> IResult<&str, Recipe> {
    // Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
    let (s, _) = take_until(": ")(s)?;
    let (s, capacity) = delimited(tag("capacity "), parse_i64, tag(", "))(s)?;
    let (s, durability) = delimited(tag("durability "), parse_i64, tag(", "))(s)?;
    let (s, flavor) = delimited(tag("flavor "), parse_i64, tag(", "))(s)?;
    let (s, texture) = delimited(tag("texture "), parse_i64, tag(", "))(s)?;
    let (s, calories) = preceded(tag("calories "), parse_i64)(s)?;
    Ok((
        s,
        Recipe {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    ))
}

struct Recipe {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Recipe {
    pub fn score(&self) -> i64 {
        vec![self.capacity, self.durability, self.flavor, self.texture]
            .into_iter()
            .map(|v| v.clamp(0, i64::MAX))
            .product()
    }
}

impl std::ops::Add for Recipe {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

impl std::ops::Mul<i64> for Recipe {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

fn max_recipe_score(ingredients: &[Recipe], teaspoons: usize) -> i64 {
    todo!()
}

pub fn day15_part1() {
    std_iter!(Lines)
        .map(|l| parse_recipe(&l).expect("Parser Error").1)
        .map(|r| {
        });
    todo!()
}

pub fn day15_part2() {
    todo!()
}

pub fn day16_part1() {
    todo!()
}
pub fn day16_part2() {
    todo!()
}
pub fn day17_part1() {
    todo!()
}
pub fn day17_part2() {
    todo!()
}
pub fn day18_part1() {
    todo!()
}
pub fn day18_part2() {
    todo!()
}
pub fn day19_part1() {
    todo!()
}
pub fn day19_part2() {
    todo!()
}
pub fn day20_part1() {
    todo!()
}
pub fn day20_part2() {
    todo!()
}
pub fn day21_part1() {
    todo!()
}
pub fn day21_part2() {
    todo!()
}
pub fn day22_part1() {
    todo!()
}
pub fn day22_part2() {
    todo!()
}
pub fn day23_part1() {
    todo!()
}
pub fn day23_part2() {
    todo!()
}
pub fn day24_part1() {
    todo!()
}
pub fn day24_part2() {
    todo!()
}
pub fn day25_part1() {
    todo!()
}
pub fn day25_part2() {
    todo!()
}
