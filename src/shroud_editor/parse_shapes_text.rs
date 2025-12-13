use std::num::ParseIntError;

use luexks_reassembly::{
    shapes::{
        scale::Scale, shape::Shape, shape_id::ShapeId, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take_until},
    character::complete::digit1,
    combinator::{complete, map, map_res},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair},
};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;
use thiserror::Error;

use crate::{
    mirror_pairs::MirrorPairs,
    shroud_editor::parsing::{
        brackets_around, parse_number_expression, whitespace_and_equals, ws, ws_around,
    },
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

    #[error("For shape {0}, its mirror_of shape with ID {1} could not be found :(")]
    MirrorOfNotFound(String, String),

    #[error("For shape {0}, its mirror_of shape with ID {1} is another mirror :(")]
    MirrorOfIsAMirror(String, String),

    #[error("Parse error for the number {0}. It's probably out of range :(")]
    NumberParse(String),
}

impl FromExternalError<&str, ParseIntError> for ShapesParseResult {
    fn from_external_error(input: &str, _kind: nom::error::ErrorKind, _e: ParseIntError) -> Self {
        ShapesParseResult::NumberParse(input.to_string())
    }
}

impl<'a> ParseError<&'a str> for ShapesParseResult {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        ShapesParseResult::Debug(format!("Cry {kind:?} {input}"))
    }

    fn append(_input: &'a str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse_shapes_text(input: &str) -> Result<(Shapes, MirrorPairs), ShapesParseResult> {
    match shapes(input) {
        Ok((_, (shapes, mirror_pairs))) => Ok((shapes, mirror_pairs)),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ShapesParseResult::Incomplete),
    }
}

fn shapes(input: &str) -> IResult<&str, (Shapes, MirrorPairs), ShapesParseResult> {
    let (remainder, mut shapes) = preceded(ws_around(tag("{")), many1(ws_around(shape)))
        .parse(input)
        .map_err(|e| match e {
            nom::Err::Error(ShapesParseResult::Debug(_)) => {
                nom::Err::Error(ShapesParseResult::Shapes("".to_string()))
            }
            _ => e,
        })?;
    let mut mirror_pairs = MirrorPairs::new();
    for shape_idx in 0..shapes.len() {
        if let Shape::Mirror { id, mirror_of, .. } = &shapes[shape_idx] {
            if let Some(mirror_of_idx) = shapes
                .iter()
                .position(|shape| *mirror_of == shape.get_id().unwrap())
            {
                if let Shape::Standard {
                    id: _mirror_id,
                    scales,
                } = &shapes[mirror_of_idx]
                {
                    shapes[shape_idx] = Shape::Standard {
                        id: id.clone(),
                        scales: scales
                            .to_vec()
                            .into_iter()
                            .map(|scale| Scale {
                                verts: scale.verts.mirror(),
                                ..scale
                            })
                            .collect(),
                    };
                    mirror_pairs.push((
                        VANILLA_SHAPE_COUNT + shape_idx,
                        VANILLA_SHAPE_COUNT + mirror_of_idx,
                    ));
                } else {
                    return Err(nom::Err::Error(ShapesParseResult::MirrorOfIsAMirror(
                        id.to_string(),
                        mirror_of.to_string(),
                    )));
                }
            } else {
                return Err(nom::Err::Error(ShapesParseResult::MirrorOfNotFound(
                    id.to_string(),
                    mirror_of.to_string(),
                )));
            }
        }
    }
    Ok((remainder, (Shapes(shapes), mirror_pairs)))
}

enum ScalesOrMirrorOf {
    Scales(Vec<Scale>),
    MirrorOf(u32),
}

fn shape(input: &str) -> IResult<&str, Shape, ShapesParseResult> {
    let (remainder, (id_str, scales_or_mirror_of, _)) = brackets_around((
        ws_around(digit1),
        alt((
            brackets_around(map(many1(ws_around(scale)), |scales| {
                ScalesOrMirrorOf::Scales(scales)
            })),
            map(
                preceded(
                    (
                        ws_around(brackets_around(ws)),
                        tag("mirror_of"),
                        whitespace_and_equals,
                    ),
                    map_res(digit1, str::parse::<u32>),
                ),
                |mirror_of| ScalesOrMirrorOf::MirrorOf(mirror_of),
            ),
        )),
        ws,
    ))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesParseResult::Shape(input.to_string())))?;
    let id = match id_str.parse::<u32>() {
        Ok(id) => ShapeId::Number(id),
        Err(..) => {
            return Err(nom::Err::Error(ShapesParseResult::NumberParse(
                id_str.to_string(),
            )));
        }
    };
    match scales_or_mirror_of {
        ScalesOrMirrorOf::Scales(scales) => Ok((remainder, Shape::Standard { scales, id })),
        ScalesOrMirrorOf::MirrorOf(mirror_of) => Ok((
            remainder,
            Shape::Mirror {
                id,
                mirror_of: ShapeId::Number(mirror_of),
                scale_count: 0,
                scale_names: Vec::new(),
            },
        )),
    }
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
