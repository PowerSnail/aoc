use std::str::FromStr;

use nom::error::ErrorKind;
use nom::{bytes::complete::take_while, combinator::fail, IResult, branch::alt};
use nom::character::complete::char;

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn parse_u64(line: &str) -> IResult<&str, u64> {
    let (input, digits) = take_while(is_digit)(line)?;
    match digits.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
}

pub fn parse_i64(line: &str) -> IResult<&str, i64> {
    if let Ok((line, sign)) = alt::<_, _, (_, ErrorKind), _>((char('+'), char('-')))(line) {
        let (line, number) = parse_u64(line)?;
        match sign {
            '+' => Ok((line, number as i64)),
            '-' => Ok((line, -(number as i64))),
            _ => unreachable!(),
        }
    } else {
        let (line, number) = parse_u64(line)?;
        Ok((line, number as i64))
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
