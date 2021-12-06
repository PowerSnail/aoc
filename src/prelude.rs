pub use itertools::Itertools;
pub use std::collections::HashMap;
pub use std::io::Read;

pub use nom::branch::alt;
pub use nom::bytes::complete::is_a;
pub use nom::bytes::complete::tag;
pub use nom::bytes::complete::take_while;
pub use nom::character::complete::alpha0;
pub use nom::character::complete::alpha1;
pub use nom::character::complete::alphanumeric0;
pub use nom::character::complete::char;
pub use nom::character::complete::digit0;
pub use nom::character::is_alphabetic;
pub use nom::character::is_alphanumeric;
pub use nom::combinator::fail;
pub use nom::multi::separated_list0;
pub use nom::number::complete::i64;
pub use nom::number::complete::u64;
pub use nom::sequence::preceded;
pub use nom::sequence::separated_pair;
pub use nom::sequence::tuple;
pub use nom::AsChar;
pub use nom::IResult;

pub use crate::parsers::parse_dec;
pub use crate::parsers::parse_i64;
pub use crate::parsers::parse_usize;

#[macro_export]
macro_rules! std_iter {
    (Lines) => {
        std::io::stdin().lines().map(Result::unwrap)
    };
    (Bytes) => {
        std::io::stdin().bytes().map(Result::unwrap)
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
