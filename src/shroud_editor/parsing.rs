use nom::{
    AsChar, IResult, Parser,
    branch::alt,
    bytes::{
        complete::{tag, take_while1},
        streaming::tag_no_case,
        take_until,
    },
    character::complete::satisfy,
    combinator::{complete, map, opt, peek, recognize, value},
    error::{Error, ParseError},
    multi::{many0, many1},
    number::complete::float,
    sequence::{delimited, pair, separated_pair},
};
use std::f32::{
    self,
    consts::{PI, TAU},
};

use nom::character::char;

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    alt((float, pi())).parse(input)
}

fn pi<'a>() -> impl Parser<&'a str, Output = f32, Error = Error<&'a str>> {
    map(tag_no_case("pi"), |_| PI)
}

fn tau<'a>() -> impl Parser<&'a str, Output = f32, Error = Error<&'a str>> {
    map(tag_no_case("tau"), |_| TAU)
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
        alphanumeric_special_1
            .parse(input)
            .map(|x| (x.0, vec![x.1]))
    }
}

pub fn alphanumeric_special_1(input: &str) -> IResult<&str, &str> {
    if peek(alt((
        value(
            (),
            satisfy::<_, &str, Error<_>>(|c| c.is_dec_digit() || ['-', '.'].contains(&c)),
        ),
        value((), pi()),
        value((), tau()),
    )))
    .parse(input)
    .is_ok()
    {
        recognize(many1(alt((
            tag_no_case("pi"),
            take_while1(|c: char| c.is_dec_digit() || ['-', '.', '/', '*'].contains(&c)),
        ))))
        .parse(input)
    } else {
        take_while1(|c: char| c.is_alphanumeric() || c == '_').parse(input)
    }
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