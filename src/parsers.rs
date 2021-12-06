use std::str::FromStr;

use nom::{bytes::complete::take_while, combinator::fail, IResult};

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn parse_i64(line: &str) -> IResult<&str, i64> {
    let (input, digits) = take_while(is_digit)(line)?;
    match digits.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
}

pub fn parse_usize(line: &str) -> IResult<&str, usize> {
    let (input, digits) = take_while(is_digit)(line)?;
    match digits.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
}

pub fn parse_dec<T>(line: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    let (input, digits) = take_while(is_digit)(line)?;
    match digits.parse::<T>() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
}
