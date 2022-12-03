use std::str::FromStr;

use nom::bytes::complete::{take, take_until};
use nom::character::complete::char;
use nom::error::{ErrorKind, ParseError};
use nom::{branch::alt, bytes::complete::take_while, combinator::fail, IResult};
use nom::{FindSubstring, InputIter, InputLength, InputTake};

pub fn take_after<T, Input, Error: ParseError<Input>>(
    tag: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputTake + FindSubstring<T> + InputIter,
    T: InputLength + Clone,
{
    move |line: Input| {
        let tag = tag.clone();
        let tag_len = tag.input_len();
        let (s, _) = take_until(tag)(line)?;
        take(tag_len)(s)
    }
}

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

