use std::str::FromStr;

use crate::{prelude::*, std_iter};

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

pub fn part1() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory: HashMap<String, u16> = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);
    println!("{}", wire_a);
}

pub fn part2() {
    let circuit: HashMap<String, CircuitConnection> = std_iter!(Lines)
        .map(|l| parse_circuit_instruction(&l))
        .collect();

    let mut memory = HashMap::new();
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    let mut memory = HashMap::from_iter([("b".to_string(), wire_a)]);
    let wire_a = visit(&CircuitNode::Node(String::from("a")), &circuit, &mut memory);

    println!("{}", wire_a);
}
