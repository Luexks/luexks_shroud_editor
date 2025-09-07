use luexks_reassembly::{
    shapes::{
        scale::Scale, shape::Shape, shape_id::ShapeId, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};
use nom::{
    IResult, Parser,
    bytes::{complete::tag, take_until},
    character::complete::digit1,
    combinator::complete,
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair},
};
use thiserror::Error;

use crate::shroud_editor::parsing::{
    brackets_around, parse_number_expression, whitespace_and_equals, ws, ws_around,
};

#[derive(Error, Debug)]
pub enum ShapesParseResult {
    #[error("YES!")]
    Success,

    #[error("{0}")]
    Debug(String),

    #[error(
        "This is a 'the input for a part of the parser was incomplete' type error, but it shouldn't occur."
    )]
    Incomplete,

    #[error("Failed to parse the shape={{ part of it: `{0}` :(")]
    Shapes(String),

    #[error("Failed to parse shape: `{0}` :(")]
    Shape(String),

    #[error("Failed to parse scale: `{0}` :(")]
    Scale(String),

    #[error("Failed to parse verts: `{0}` :(")]
    Verts(String),

    #[error("Failed to parse vert: `{0}` :(")]
    Vert(String),
}

impl<'a> ParseError<&'a str> for ShapesParseResult {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        ShapesParseResult::Debug(format!("Cry {kind:?} {input}"))
    }

    fn append(_input: &'a str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse_shapes_text(input: &str) -> Result<Shapes, ShapesParseResult> {
    match shapes(input) {
        Ok((_, shapes)) => Ok(shapes),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ShapesParseResult::Incomplete),
    }
}

fn shapes(input: &str) -> IResult<&str, Shapes, ShapesParseResult> {
    let (remainder, shapes) = preceded(ws_around(tag("{")), many1(ws_around(shape)))
        .parse(input)
        .map_err(|_| nom::Err::Error(ShapesParseResult::Shapes(input.to_string())))?;
    Ok((remainder, Shapes(shapes)))
}

fn shape(input: &str) -> IResult<&str, Shape, ShapesParseResult> {
    let (remainder, (id, scales, _)) = brackets_around((
        ws_around(digit1),
        brackets_around(many1(ws_around(scale))),
        ws,
    ))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesParseResult::Shape(input.to_string())))?;
    Ok((
        remainder,
        Shape::Standard {
            scales,
            id: ShapeId::Number(id.parse::<u32>().unwrap()),
        },
    ))
}

fn scale(input: &str) -> IResult<&str, Scale, ShapesParseResult> {
    let (remainder, verts) = brackets_around(ws_around(delimited(
        take_until("verts="),
        preceded(tag("verts="), verts),
        ws_around((
            take_until("ports"),
            preceded(tag("ports"), whitespace_and_equals),
            complete(brackets_around(many0(ws_around(brackets_around(
                take_until("}"),
            ))))),
        )),
    )))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesParseResult::Scale(input.to_string())))?;
    Ok((
        remainder,
        Scale {
            verts,
            ..Default::default()
        },
    ))
}

fn verts(input: &str) -> IResult<&str, Vertices, ShapesParseResult> {
    let (remainder, verts) = brackets_around(many1(ws_around(vert)))
        .parse(input)
        .map_err(|_| nom::Err::Error(ShapesParseResult::Verts(input.to_string())))?;
    Ok((remainder, Vertices(verts)))
}

fn vert(input: &str) -> IResult<&str, Vertex, ShapesParseResult> {
    let (remainder, (x, y)) = brackets_around(ws_around(separated_pair(
        parse_number_expression,
        ws,
        parse_number_expression,
    )))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesParseResult::Vert(input.to_string())))?;
    Ok((remainder, Vertex(do2d_float_from(x, y))))
}
