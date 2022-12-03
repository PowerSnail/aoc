use std::fmt::Display;

use crate::chain;
use crate::prelude::*;
use crate::std_iter;

fn parse_rule(s: &str) -> ParseResult<(u32, Vec<u32>)> {
    let (s, (lhs, rhs)) = separated_pair(parse_element, tag(" => "), many1(parse_element))(s)?;
    Ok((s, (lhs, rhs)))
}

fn parse_element(s: &str) -> ParseResult<u32> {
    match s.as_bytes() {
        &[i, j, ..] if b'A' <= i && i <= b'Z' && b'a' <= j && j <= b'z' => {
            Ok((&s[2..], ((i as u32) << 8) + (j as u32)))
        }
        &[i, ..] if is_alphabetic(i) => Ok((&s[1..], i as u32)),
        _ => fail(s),
    }
}

fn element_name(e: u32) -> String {
    if e > 0xFF {
        String::from_utf8(vec![((e >> 8) & 0xFF) as u8, (e & 0xFF) as u8]).unwrap()
    } else {
        String::from_utf8(vec![(e & 0xFF) as u8]).unwrap()
    }
}

fn parse_input(s: &str) -> ParseResult<Vec<u32>> {
    let (s, elements) = many1(parse_element)(s)?;
    Ok((s, elements))
}

fn get_input() -> (Vec<u32>, Vec<(u32, Vec<u32>)>) {
    let mut lines = std_iter!(Lines).collect_vec();
    let input = parse_input(&lines.pop().unwrap()).unwrap().1;
    lines.pop();

    let rules = lines
        .into_iter()
        .map(|l| parse_rule(&l).unwrap().1)
        .collect_vec();
    (input, rules)
}

pub fn part1() {
    let (input, rules) = get_input();
    let rules = rules.into_iter().into_group_map();
    let strings: HashSet<_> = input
        .iter()
        .enumerate()
        .filter_map(|(i, element)| rules.get(element).and_then(|r| Some((i, r))))
        .map(|(i, replacements)| {
            let left = &input[..i];
            let right = &input[i + 1..];
            replacements.iter().map(|r| {
                chain![left.iter(), r.iter(), right.iter()]
                    .map(|x| *x)
                    .collect_vec()
            })
        })
        .flatten()
        .collect();
    println!("{}", strings.len());
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct EarleyState<'a> {
    rule: &'a (u32, Vec<u32>),
    rule_i: usize,
    line_i: usize,
}

impl<'a> Display for EarleyState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left_seg = self.rule.1.iter().take(self.rule_i).map(|c| element_name(*c)).collect::<String>();
        let right_seg = self.rule.1.iter().skip(self.rule_i).map(|c| element_name(*c)).collect::<String>();
        write!(f, "{} -> {}*{}, {}", element_name(self.rule.0), left_seg, right_seg, self.line_i)
    }
}

impl<'a> EarleyState<'a> {
    pub fn new(rule: &'a (u32, Vec<u32>), rule_i: usize, line_i: usize) -> EarleyState {
        EarleyState {
            rule,
            rule_i,
            line_i,
        }
    }

    pub fn lhs(&self) -> u32 {
        self.rule.0
    }

    pub fn rhs(&self) -> &Vec<u32> {
        &self.rule.1
    }

    pub fn is_end(&self) -> bool {
        return self.rule_i >= self.rhs().len();
    }

    pub fn next_rhs(&self) -> Option<u32> {
        if self.is_end() {
            None
        } else {
            Some(self.rhs()[self.rule_i])
        }
    }
}

const ROOT : u32 = ((b'<' as u32) << 8) + b'>' as u32;

pub fn part2() {
    let (input, rules) = get_input();
    let rules = rules
        .into_iter()
        .collect_vec();
    let top_level_rule = (ROOT, vec!['e' as u32]);
    let first_state = EarleyState::new(&top_level_rule, 0, 0);
    
    let mut states = vec![HashSet::new(); input.len() + 1];
    states[0].insert(first_state);
    
    for k in 0..=input.len() {
        let mut queue = VecDeque::from_iter(states[k].iter().cloned());
        while let Some(state) = queue.pop_front() {
            // completer
            if state.is_end() {
                let prev_states = states[state.line_i]
                    .iter()
                    .filter(|state_j| state_j.next_rhs().map_or(false, |s| s == state.lhs()))
                    .cloned()
                    .collect_vec();

                for prev_state in prev_states {
                    let new_state = EarleyState::new(prev_state.rule, prev_state.rule_i + 1, prev_state.line_i);
                    if states[k].insert(new_state.clone()) {
                        queue.push_back(new_state);
                    }
                }
            } else {
                // predictor
                let new_states = rules
                    .iter()
                    .filter(|rule| rule.0 == state.next_rhs().unwrap())
                    .map(|rule| EarleyState::new(rule, 0, k))
                    .filter(|s| !states[k].contains(s))
                    .collect_vec();
                
                for s in new_states {
                    if states[k].insert(s.clone()) {
                        queue.push_back(s);
                    }
                }

                // scanner (a non-terminal always produces a terminal with the same value)
                if k < input.len() && state.next_rhs().unwrap() == input[k] {
                    let new_state = EarleyState::new(
                        state.rule,
                        state.rule_i + 1,
                        state.line_i,
                    );
                    states[k + 1].insert(new_state);
                }
            }
        }
    }

    for i in 0..=input.len() {
        println!("{}", i);
        for s in states[i].iter().filter(|s| s.is_end()).sorted_by_key(|r| r.lhs()) {
            println!("{}", s);
        }
        println!("------");
    }
    if states[states.len() - 1].contains(&EarleyState::new(&top_level_rule, top_level_rule.1.len(), 0)) {
        println!("Accepted");
    } else {
        println!("Not Accepted");
    }

}
