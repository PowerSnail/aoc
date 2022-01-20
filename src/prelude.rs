pub use itertools::Itertools;
pub use std::collections::HashMap;
pub use std::collections::VecDeque;
use std::fmt::Debug;
pub use std::io::Read;

pub use itertools::iproduct;
pub use ndarray::prelude::*;
pub use ndarray::Array;
pub use nom::branch::alt;
pub use nom::bytes::complete::is_a;
pub use nom::bytes::complete::tag;
pub use nom::bytes::complete::take_till;
pub use nom::bytes::complete::take_while;
pub use nom::character::complete::alpha0;
pub use nom::character::complete::alpha1;
pub use nom::character::complete::alphanumeric0;
pub use nom::character::complete::char;
pub use nom::character::complete::digit0;
pub use nom::character::complete::digit1;
pub use nom::character::complete::space0;
pub use nom::character::complete::space1;
pub use nom::character::is_alphabetic;
pub use nom::character::is_alphanumeric;
pub use nom::character::is_digit;
pub use nom::combinator::fail;
pub use nom::multi::separated_list0;
pub use nom::number::complete::i64;
pub use nom::number::complete::u64;
pub use nom::sequence::delimited;
pub use nom::sequence::pair;
pub use nom::sequence::preceded;
pub use nom::sequence::separated_pair;
pub use nom::sequence::tuple;
pub use nom::AsChar;
pub use nom::IResult;

pub use crate::parsers::parse_dec;
pub use crate::parsers::parse_i64;
pub use crate::parsers::parse_u64;
pub use crate::parsers::parse_usize;
pub use crate::parsers::take_after;

#[macro_export]
macro_rules! std_iter {
    (Lines) => {
        std::io::stdin().lines().map(Result::unwrap)
    };
    (Bytes) => {
        std::io::stdin().bytes().map(Result::unwrap)
    };
    (SplitBy $c:expr) => {
        std::io::stdin().lines().next().unwrap().unwrap().split($c)
    };
    (Grid) => {
        std_iter!(Lines)
            .map(|l| l.bytes().collect_vec())
            .collect_vec()
    };
    (GridOf $f:expr) => {
        std_iter!(Lines)
            .map(|l| l.bytes().map($f).collect_vec())
            .collect_vec()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn transposed(&self) -> Point {
        Point::new(self.y, self.x)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    p1: Point,
    p2: Point,
}

impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> LineSegment {
        LineSegment { p1, p2 }
    }

    pub fn is_horizontal(&self) -> bool {
        return self.p1.y == self.p2.y;
    }

    pub fn is_vertical(&self) -> bool {
        return self.p1.x == self.p2.x;
    }

    pub fn dx(&self) -> i64 {
        self.p2.x - self.p1.x
    }

    pub fn dy(&self) -> i64 {
        self.p2.y - self.p1.y
    }

    pub fn scan_line(&self) -> Vec<Point> {
        if self.dx().abs() >= self.dy().abs() {
            self.scan_line_x()
        } else {
            LineSegment::new(self.p1.transposed(), self.p2.transposed())
                .scan_line_x()
                .into_iter()
                .map(|p| p.transposed())
                .collect_vec()
        }
    }

    fn scan_line_x(&self) -> Vec<Point> {
        if self.dx() >= 0 {
            self.scan_line_x_normalized()
        } else {
            LineSegment::new(self.p2, self.p1).scan_line_x_normalized()
        }
    }

    fn scan_line_x_normalized(&self) -> Vec<Point> {
        let step_y = self.dy() / self.dx();
        (self.p1.x..(self.p2.x + 1))
            .enumerate()
            .map(|(i, x)| Point::new(x, self.p1.y + (i as i64) * step_y))
            .collect_vec()
    }
}

pub fn sum_to_1(n: i64) -> i64 {
    (n + 1) * n / 2
}

pub fn eprint_and_return<T>(x: T) -> T
where
    T: Debug,
{
    eprintln!("{:?}", x);
    x
}

pub struct NodeRegistration {
    names: Vec<String>,
    ids: HashMap<String, usize>,
}

impl NodeRegistration {
    pub fn add(&mut self, name: &str) -> usize {
        if self.ids.contains_key(name) {
            return self.ids[name];
        }
        let id = self.names.len();
        self.names.push(name.to_string());
        self.ids.insert(name.to_string(), id);
        id
    }

    pub fn get_name(&self, id: usize) -> Option<&str> {
        if id < self.names.len() {
            Some(&self.names[id])
        } else {
            None
        }
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.ids.get(name).and_then(|x| Some(*x))
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

impl<'a> FromIterator<&'a str> for NodeRegistration {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut reg = NodeRegistration {
            names: vec![],
            ids: HashMap::new(),
        };
        for s in iter {
            reg.add(s);
        }
        reg
    }
}

#[macro_export]
macro_rules! v_max {
    ($t:ident, $e:expr) => {
        $t.iter().map(|&x| x.max($e)).collect()
    };
}

#[macro_export]
macro_rules! v_times {
    ($t:ident, $e:expr) => {
        $t.iter().map(|&x| x * $e).collect()
    };
}

#[macro_export]
macro_rules! v_add {
    ($x:ident, $y:ident) => {
        $x.iter().zip($y.iter()).map(|(&x1, &x2)| x1 + x2).collect()
    };
}
