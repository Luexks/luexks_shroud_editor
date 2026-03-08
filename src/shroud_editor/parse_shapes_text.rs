use std::num::ParseIntError;

use luexks_reassembly::{
    shapes::{scale::Scale, shape::Shape, shape_id::ShapeId, vertex::Vertex, vertices::Vertices},
    utility::display_oriented_math::do2d_float_from,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take_until},
    character::complete::{alphanumeric0, digit1},
    combinator::{complete, map, map_res, opt},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair},
};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;
use thiserror::Error;

use crate::{
    mirror_pairs::MirrorPairs,
    shape_container::ShapeContainer,
    shroud_editor::parsing::{
        brackets_around, parse_number_expression, ws, ws_and_equals, ws_around,
    },
};

#[derive(Error, Debug)]
pub enum ShapesMessage {
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

    #[error("Could not open file :(")]
    CouldNotOpenFile,
}

impl FromExternalError<&str, ParseIntError> for ShapesMessage {
    fn from_external_error(input: &str, _kind: nom::error::ErrorKind, _e: ParseIntError) -> Self {
        ShapesMessage::NumberParse(input.to_string())
    }
}

impl<'a> ParseError<&'a str> for ShapesMessage {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        ShapesMessage::Debug(format!("Cry {kind:?} {input}"))
    }

    fn append(_input: &'a str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse_shapes_text(
    input: &str,
) -> Result<(Vec<ShapeContainer>, MirrorPairs, Vec<usize>), ShapesMessage> {
    match shapes(input) {
        Ok((_, (shapes, mirror_pairs, non_mirrors))) => Ok((shapes, mirror_pairs, non_mirrors)),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ShapesMessage::Incomplete),
    }
}

fn shapes(
    input: &str,
) -> IResult<&str, (Vec<ShapeContainer>, MirrorPairs, Vec<usize>), ShapesMessage> {
    let (remainder, mut shapes) = preceded(ws_around(tag("{")), many1(ws_around(shape)))
        .parse(input)
        .map_err(|e| match e {
            nom::Err::Error(ShapesMessage::Debug(_)) => {
                nom::Err::Error(ShapesMessage::Shapes("".to_string()))
            }
            _ => e,
        })?;
    shapes.retain(|shape| {
        matches!(shape.s, Shape::Mirror { .. }) || shape.s.get_first_scale_vertices().0.len() >= 3
    });
    let mut non_mirrors =
        (VANILLA_SHAPE_COUNT..VANILLA_SHAPE_COUNT + shapes.len()).collect::<Vec<_>>();
    let mut mirror_pairs = MirrorPairs::new();
    for shape_idx in 0..shapes.len() {
        if let Shape::Mirror { id, mirror_of, .. } = &shapes[shape_idx].s {
            if let Some(mirror_of_idx) = shapes
                .iter()
                .position(|shape| *mirror_of == shape.s.get_id().unwrap())
            {
                if let Shape::Standard {
                    id: _mirror_id,
                    scales,
                } = &shapes[mirror_of_idx].s
                {
                    shapes[shape_idx].s = Shape::Standard {
                        id: id.clone(),
                        scales: scales
                            .iter()
                            .cloned()
                            .map(|scale| Scale {
                                verts: scale.verts.mirror(),
                                ..scale
                            })
                            .collect(),
                    };
                    let shape_idx = VANILLA_SHAPE_COUNT + shape_idx;
                    let mirror_of_idx = VANILLA_SHAPE_COUNT + mirror_of_idx;
                    mirror_pairs.push((shape_idx, mirror_of_idx));
                    non_mirrors.retain(|non_mirror_idx| {
                        *non_mirror_idx != shape_idx && *non_mirror_idx != mirror_of_idx
                    });
                } else {
                    return Err(nom::Err::Error(ShapesMessage::MirrorOfIsAMirror(
                        id.to_string(),
                        mirror_of.to_string(),
                    )));
                }
            } else {
                return Err(nom::Err::Error(ShapesMessage::MirrorOfNotFound(
                    id.to_string(),
                    mirror_of.to_string(),
                )));
            }
        }
    }
    Ok((remainder, (shapes, mirror_pairs, non_mirrors)))
}

enum ScalesOrMirrorOf {
    Scales(Vec<Scale>),
    MirrorOf(u32),
}

fn shape(input: &str) -> IResult<&str, ShapeContainer, ShapesMessage> {
    let (remainder, (id_str, _, scales_or_mirror_of, _)) = brackets_around((
        ws_around(digit1),
        opt((radial_launcher, ws)),
        alt((
            brackets_around(map(many1(ws_around(scale)), |scales| {
                ScalesOrMirrorOf::Scales(scales)
            })),
            map(
                preceded(
                    (
                        ws_around(brackets_around(ws)),
                        tag("mirror_of"),
                        ws_and_equals,
                    ),
                    map_res(digit1, str::parse::<u32>),
                ),
                ScalesOrMirrorOf::MirrorOf,
            ),
        )),
        (opt((ws, radial_launcher)), ws),
    ))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesMessage::Shape(input.to_string())))?;
    let id = match id_str.parse::<u32>() {
        Ok(id) => ShapeId::Number(id),
        Err(..) => {
            return Err(nom::Err::Error(ShapesMessage::NumberParse(
                id_str.to_string(),
            )));
        }
    };
    match scales_or_mirror_of {
        ScalesOrMirrorOf::Scales(scales) => Ok((
            remainder,
            ShapeContainer::new(Shape::Standard { scales, id }),
        )),
        ScalesOrMirrorOf::MirrorOf(mirror_of) => Ok((
            remainder,
            ShapeContainer::new(Shape::Mirror {
                id,
                mirror_of: ShapeId::Number(mirror_of),
                scale_count: 0,
                scale_names: Vec::new(),
            }),
        )),
    }
}

fn scale(input: &str) -> IResult<&str, Scale, ShapesMessage> {
    let (remainder, verts) = brackets_around(ws_around(delimited(
        (take_until("verts"), tag("verts"), ws_and_equals),
        verts,
        (
            ws,
            opt(complete((
                take_until("ports"),
                tag("ports"),
                ws_and_equals,
                brackets_around(many0(ws_around(brackets_around(take_until("}"))))),
            ))),
        ),
    )))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesMessage::Scale(input.to_string())))?;
    Ok((
        remainder,
        Scale {
            verts,
            ..Default::default()
        },
    ))
}

fn verts(input: &str) -> IResult<&str, Vertices, ShapesMessage> {
    let (remainder, verts) = brackets_around(many1(ws_around(vert)))
        .parse(input)
        .map_err(|_| nom::Err::Error(ShapesMessage::Verts(input.to_string())))?;
    Ok((remainder, Vertices(verts)))
}

fn vert(input: &str) -> IResult<&str, Vertex, ShapesMessage> {
    let (remainder, (x, y)) = brackets_around(ws_around(separated_pair(
        parse_number_expression,
        ws,
        parse_number_expression,
    )))
    .parse(input)
    .map_err(|_| nom::Err::Error(ShapesMessage::Vert(input.to_string())))?;
    Ok((remainder, Vertex(do2d_float_from(x, y))))
}

fn radial_launcher(input: &str) -> IResult<&str, (), ShapesMessage> {
    let (remainder, _) = (tag("radial_launcher")).parse(input)?;
    let (remainder, _) = ws_and_equals(remainder)?;
    let (remainder, _) = alphanumeric0.parse(remainder)?;
    Ok((remainder, ()))
}
