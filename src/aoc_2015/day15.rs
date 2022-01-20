use crate::{prelude::*, std_iter};

fn parse_recipe(s: &str) -> IResult<&str, Vec<i64>> {
    // Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
    let (s, _) = take_after(": ")(s)?;
    separated_list0(tag(", "), preceded(take_after(" "), parse_i64))(s)
}

fn score_recipe(r: &Vec<i64>) -> i64 {
    r[..4].iter().map(|&v| v.max(0)).product()
}

fn add_recipe(x: Vec<i64>, y: Vec<i64>) -> Vec<i64> {
    x.into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| x + y)
        .collect_vec()
}

fn scale_recipe(x: &Vec<i64>, factor: i64) -> Vec<i64> {
    x.iter().map(|&v| v * factor).collect_vec()
}

struct IntegerPartition {
    _state: Vec<usize>,
}

impl IntegerPartition {
    pub fn new(total: usize, count: usize) -> IntegerPartition {
        assert!(count > 0);
        let mut _state = vec![0; count];
        _state[0] = total;
        IntegerPartition { _state }
    }
}

impl Iterator for IntegerPartition {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self._state.is_empty() {
            return None;
        }
        let result = self
            ._state
            .iter()
            .chain([0].iter())
            .tuple_windows()
            .map(|(&x, &y)| x - y)
            .collect_vec();

        for i in (1..self._state.len()).rev() {
            if self._state[i] < self._state[i - 1] {
                self._state[i] += 1;
                for j in (i + 1)..self._state.len() {
                    self._state[j] = 0;
                }
                return Some(result);
            }
        }
        self._state.clear();
        Some(result)
    }
}

pub fn part1() {
    let l = std_iter!(Lines)
        .map(|l| parse_recipe(&l).expect("Parser Error").1)
        .collect_vec();

    let m = IntegerPartition::new(100, l.len())
        .map(|n| {
            let r = l
                .iter()
                .zip(n.iter())
                .map(|(ingredient, scale)| scale_recipe(ingredient, *scale as i64))
                .reduce(add_recipe)
                .unwrap();
            score_recipe(&r)
        })
        .max()
        .unwrap();

    println!("{:?}", m);
}

pub fn part2() {
    let l = std_iter!(Lines)
        .map(|l| parse_recipe(&l).expect("Parser Error").1)
        .collect_vec();

    let m = IntegerPartition::new(100, l.len())
        .filter_map(|n| {
            let r = l
                .iter()
                .zip(n.iter())
                .map(|(ingredient, scale)| scale_recipe(ingredient, *scale as i64))
                .reduce(add_recipe)
                .unwrap();
            if r[4] == 500 {
                Some(score_recipe(&r))
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!("{:?}", m);
}
