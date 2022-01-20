use crate::{prelude::*, std_iter};
fn hex_to_binary(b: u8) -> String {
    let n = match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        _ => unreachable!(),
    };
    format!("{:04b}", n)
}

#[derive(Debug)]
enum Packet {
    Literal(u64, u64),
    Operator(u64, u64, Vec<Packet>),
}

fn parse_literal_list(s: &str) -> (&str, u64) {
    let mut out = String::new();
    for i in (0..s.len()).step_by(5) {
        out.push_str(&s[i + 1..i + 5]);
        if s.as_bytes()[i] == b'0' {
            return (&s[i + 5..], u64::from_str_radix(&out, 2).unwrap());
        }
    }
    ("", u64::from_str_radix(&out, 2).unwrap())
}

fn parse_number(s: &str, n_bit: usize) -> (&str, u64) {
    (&s[n_bit..], u64::from_str_radix(&s[..n_bit], 2).unwrap())
}

fn take_bits(s: &str, n_bit: usize) -> (&str, &str) {
    (&s[n_bit..], &s[..n_bit])
}

fn parse_sub_packet(s: &str) -> (&str, Vec<Packet>) {
    let (s, length_type) = parse_number(s, 1);
    match length_type {
        0 => {
            let mut packets = Vec::new();
            let (s, bit_length) = parse_number(s, 15);
            let (s, mut s_packets) = take_bits(s, bit_length as usize);
            while s_packets.len() > 0 {
                let (s_packets_next, packet) = parse_packet(s_packets);
                s_packets = s_packets_next;
                packets.push(packet);
            }
            (s, packets)
        }
        1 => {
            let (s, packet_count) = parse_number(s, 11);
            (0..packet_count).fold((s, vec![]), |(s, mut v), _| {
                let (s, packet) = parse_packet(s);
                v.push(packet);
                (s, v)
            })
        }
        _ => unreachable!(),
    }
}

fn parse_packet(s: &str) -> (&str, Packet) {
    let (s, version) = parse_number(s, 3);
    let (s, type_id) = parse_number(s, 3);
    match type_id {
        4 => {
            let (s, value) = parse_literal_list(s);
            (s, Packet::Literal(version, value as u64))
        }
        op => {
            let (s, sub_list) = parse_sub_packet(s);
            (s, Packet::Operator(version, op, sub_list))
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(version, _) => *version,
        Packet::Operator(version, _, children) => {
            version + children.iter().map(version_sum).sum::<u64>()
        }
    }
}

fn packet_value(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, op, children) => {
            let child_values = children.iter().map(packet_value);
            match *op {
                0 => child_values.sum(),
                1 => child_values.product(),
                2 => child_values.min().unwrap(),
                3 => child_values.max().unwrap(),
                5 => child_values
                    .collect_tuple()
                    .and_then(|(p1, p2)| Some((p1 > p2) as u64))
                    .unwrap(),
                6 => child_values
                    .collect_tuple()
                    .and_then(|(p1, p2)| Some((p1 < p2) as u64))
                    .unwrap(),
                7 => child_values
                    .collect_tuple()
                    .and_then(|(p1, p2)| Some((p1 == p2) as u64))
                    .unwrap(),
                _ => unreachable!(),
            }
        }
    }
}

pub fn part1() {
    let input: String = std_iter!(Bytes).map(hex_to_binary).collect();
    let (_, root) = parse_packet(&input);
    println!("{:?}", version_sum(&root));
}

pub fn part2() {
    let input: String = std_iter!(Bytes).map(hex_to_binary).collect();
    let (_, root) = parse_packet(&input);
    println!("{:?}", packet_value(&root));
}
