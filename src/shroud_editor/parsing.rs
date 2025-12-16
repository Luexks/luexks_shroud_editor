use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{
        complete::{tag, take_while1},
        streaming::tag_no_case,
        take_until,
    },
    combinator::{complete, map, opt, peek, value},
    error::{Error, ParseError},
    multi::{many0, many1},
    number::complete::float,
    sequence::{delimited, pair, separated_pair},
};
use std::f32::{self, consts::PI};

use nom::character::char;

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    alt((float, map(tag_no_case("pi"), |_| PI))).parse(input)
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Div,
    Mul,
}

pub fn parse_number_expression(input: &str) -> IResult<&str, f32> {
    let (remainder, first_number) = parse_number(input)?;
    let (remainder, operator_number_pairs) = many0(pair(
        alt((
            value(Operator::Mul, complete(pair(opt(complete(ws)), char('*')))),
            value(Operator::Div, complete(pair(opt(complete(ws)), char('/')))),
        )),
        parse_number,
    ))
    .parse(remainder)?;
    Ok((
        remainder,
        operator_number_pairs
            .iter()
            .fold(first_number, |acc, (operator, number)| match operator {
                Operator::Mul => acc * number,
                Operator::Div => acc / number,
            }),
    ))
}

pub fn variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

pub fn variable_value(input: &str) -> IResult<&str, Vec<&str>> {
    if peek(char::<&str, Error<_>>('{')).parse(input).is_ok() {
        brackets_around(many1(ws_around(alphanumeric_special_1))).parse(input)
    } else {
        many1(alphanumeric_special_1).parse(input)
    }
}

pub fn alphanumeric_special_1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '/' || c == '*'
    })
    .parse(input)
}

pub fn ws<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    let (remainder, _) = many0(alt((
        value((), take_while1(|c: char| c.is_whitespace() || c == ',')),
        value((), comment),
    )))
    .parse(input)?;
    Ok((remainder, ()))
}

pub fn ws_and_equals<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), (ws, value((), tag("=")), ws)).parse(input)
}

pub fn variable(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(variable_name, ws_and_equals, variable_value).parse(input)
}

pub fn comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), (tag("--"), take_until("\n"))).parse(input)
}

pub fn ws_around<'a, O, F, E>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
    E: nom::error::ParseError<&'a str>,
{
    delimited(ws, inner, ws)
}

pub fn brackets_around<'a, O, F, E>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
    E: nom::error::ParseError<&'a str>,
{
    delimited(char('{'), inner, char('}'))
}
