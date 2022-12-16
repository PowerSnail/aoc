use nom::character::complete::one_of;

use crate::prelude::*;

#[derive(Debug)]
enum Operand {
    Number(u64),
    This,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operator: char,
    operand: Operand,
    predicate: u64,
    target_true: usize,
    target_false: usize,
}

impl Monkey {
    fn calculate_target(&self, anxiety: u64) -> usize {
        if anxiety % self.predicate == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }

    fn calculate_anxiety(&self, n: u64) -> u64 {
        let operand = match self.operand {
            Operand::Number(v) => v,
            Operand::This => n,
        };
        match self.operator {
            '+' => n + operand,
            '*' => n * operand,
            _ => unreachable!(),
        }
    }
}

fn parse_operand(i: &str) -> ParseResult<Operand> {
    parse_u64(i)
        .and_then(|(i, n)| Ok((i, Operand::Number(n))))
        .or_else(|_| tag("old")(i).and_then(|(i, _)| Ok((i, Operand::This))))
}

fn parse_monkey(i: &str) -> ParseResult<Monkey> {
    let (i, _) = tuple((tag("Monkey "), digit1, tag(":\n")))(i)?;
    let (i, items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), parse_u64),
    )(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, (operator, operand)) = preceded(
        tag("  Operation: new = old "),
        separated_pair(one_of("+-*/"), tag(" "), parse_operand),
    )(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, predicate) = preceded(tag("  Test: divisible by "), parse_u64)(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, target_true) = preceded(tag("    If true: throw to monkey "), parse_usize)(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, target_false) = preceded(tag("    If false: throw to monkey "), parse_usize)(i)?;
    let (i, _) = take_while(|c| c == '\n')(i)?;
    Ok((
        i,
        Monkey {
            items,
            operator,
            operand,
            predicate,
            target_true,
            target_false,
        },
    ))
}

fn input() -> Vec<Monkey> {
    let mut line = String::new();
    std::io::stdin()
        .read_to_string(&mut line)
        .expect("Failed to read");
    let mut monkeys = vec![];
    let mut i = line.as_str();

    while let Ok((next_i, monkey)) = parse_monkey(i) {
        i = next_i;
        monkeys.push(monkey);
    }
    monkeys
}

pub fn part1() {
    let mut monkeys = input();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            counts[i] += monkeys[i].items.len();
            std::mem::take(&mut monkeys[i].items)
                .into_iter()
                .for_each(|n| {
                    let anxiety = monkeys[i].calculate_anxiety(n);
                    let anxiety = anxiety / 3;
                    let target = monkeys[i].calculate_target(anxiety);
                    monkeys[target].items.push(anxiety);
                })
        }
    }
    counts.sort();
    println!("{}", counts[counts.len() - 1] * counts[counts.len() - 2]);
}

pub fn part2() {
    let mut monkeys = input();
    let mut counts = vec![0; monkeys.len()];
    let multiple : u64 = monkeys.iter().map(|m| m.predicate).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            counts[i] += monkeys[i].items.len();
            std::mem::take(&mut monkeys[i].items)
                .into_iter()
                .for_each(|n| {
                    let anxiety = monkeys[i].calculate_anxiety(n);
                    let anxiety = anxiety % multiple;
                    let target = monkeys[i].calculate_target(anxiety);
                    monkeys[target].items.push(anxiety);
                })
        }
    }
    counts.sort();
    println!("{}", counts[counts.len() - 1] * counts[counts.len() - 2]);
}
