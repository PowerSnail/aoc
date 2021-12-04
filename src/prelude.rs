pub use itertools::Itertools;
pub use std::collections::HashMap;
pub use std::io::Read;

pub use nom::branch::alt;
pub use nom::bytes::complete::is_a;
pub use nom::bytes::complete::tag;
pub use nom::bytes::complete::take_while;
pub use nom::character::complete::char;
pub use nom::multi::separated_list0;
pub use nom::sequence::separated_pair;
pub use nom::AsChar;
pub use nom::IResult;

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
