use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{
        complete::{tag, take_while, take_while1},
        streaming::tag_no_case,
        take_until,
    },
    character::complete::digit1,
    combinator::{complete, map, opt, peek, recognize, value},
    error::{Error, ParseError},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use std::f32::{self, consts::PI};

use nom::character::char;

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    alt((
        map(
            recognize((
                opt(complete(char::<&str, Error<&str>>('-'))),
                digit1,
                opt(complete(char('.'))),
                opt(complete(digit1)),
            )),
            |s| s.parse::<f32>().unwrap(),
        ),
        map(tag_no_case("pi"), |_| PI),
    ))
    .parse(input)
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
        delimited(
            tag("{"),
            many1(delimited(ws, alphanumeric_special_1, ws)),
            tag("}"),
        )
        .parse(input)
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
    let (remainder, _) =
        value((), take_while(|c: char| c.is_whitespace() || c == ',')).parse(input)?;
    let (remainder, _) = comment(remainder)?;
    let (remainder, _) =
        value((), take_while(|c: char| c.is_whitespace() || c == ',')).parse(remainder)?;
    Ok((remainder, ()))
}

pub fn whitespace_and_equals<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (), E> {
    value(
        (),
        take_while(|c: char| c.is_whitespace() || c == ',' || c == '='),
    )
    .parse(input)
}

pub fn variable(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(variable_name, whitespace_and_equals, variable_value).parse(input)
}

pub fn comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Option<&'a str>, E> {
    opt(preceded(tag("--"), terminated(take_until("\n"), tag("\n")))).parse(input)
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
