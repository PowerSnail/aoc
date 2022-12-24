use std::cmp::Ordering;

use nom::combinator::map;

use crate::{prelude::*, std_iter};

#[derive(Clone)]
enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

fn parse_list(s: &str) -> ParseResult<Vec<Packet>> {
    delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))(s)
}

fn parse_packet(i: &str) -> ParseResult<Packet> {
    alt((
        map(parse_list, |l| Packet::List(l)),
        map(parse_u64, |n| Packet::Number(n)),
    ))(i)
}

fn parse_pair(i: &str) -> ParseResult<(Packet, Packet)> {
    separated_pair(parse_packet, tag("\n"), parse_packet)(i)
}

fn parse_packet_pair_list(i: &str) -> ParseResult<Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), parse_pair)(i)
}

impl Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l
                .iter()
                .zip(r.iter())
                .map(|(l, r)| l.cmp(r))
                .find(|order| *order != Ordering::Equal)
                .unwrap_or(l.len().cmp(&r.len())),
            (Packet::List(_), Packet::Number(r)) => {
                self.cmp(&Packet::List(vec![Packet::Number(*r)]))
            }
            _ => other.cmp(self).reverse(),
        }
    }
}

pub fn part1() {
    let packets = parse_packet_pair_list(std_iter!(OneString).as_str())
        .unwrap()
        .1;

    let result: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("{}", result);
}

fn parse_packet_list(i: &str) -> ParserResult<Vec<Packet>> {
    separated_list1(many1(tag("\n")), parse_packet)(i)
}

pub fn part2() {
    let distress_a = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let distress_b = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    let mut packets = parse_packet_list(std_iter!(OneString).as_str()).unwrap().1;
    packets.push(distress_a.clone());
    packets.push(distress_b.clone());
    packets.sort_by(|a, b| a.cmp(b));

    let a = packets
        .binary_search_by(|probe| probe.cmp(&distress_a))
        .unwrap();
    let b = packets
        .binary_search_by(|probe| probe.cmp(&distress_b))
        .unwrap();
    println!("{}", (a + 1) * (b + 1));
}
