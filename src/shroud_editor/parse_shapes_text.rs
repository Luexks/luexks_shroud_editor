use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayerColor,
    shapes::{
        scale::Scale, shape::Shape, shape_id::ShapeId, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::{
        angle::Angle,
        display_oriented_math::{do2d_float_from, do3d_float_from},
    },
};
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
    error::Error,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use std::f32::{self, consts::PI};
use thiserror::Error;

use crate::{
    restructure_vertices::restructure_vertices,
    shroud_editor::parsing::{
        brackets_around, parse_number_expression, variable, whitespace_and_equals, ws, ws_around,
    },
    shroud_layer_container::ShroudLayerContainer,
};

use nom::character::char;

#[derive(Error, Debug)]
pub enum ShapesParseResult {
    #[error("YES!")]
    Success,

    #[error("Failed to parse the brackets around everything :(")]
    Shapes,

    #[error("Failed to get variable value data for: `{0}` :(")]
    VariableValueData(String),

    #[error("No know variable name: `{0}` :(")]
    VariableNameData(String),

    #[error("Failed to parse tri_color_id: `{0}` :(")]
    Color1(String),

    #[error("Failed to parse tri_color1_id: `{0}` :(")]
    Color2(String),

    #[error("Failed to parse line_color_id: `{0}` :(")]
    LineColor(String),

    #[error("Failed to find vanilla shape in loaded shape list: `{0}` :(")]
    ShapeVanilla(String),

    #[error("Failed to find custom shape in loaded shape list: `{0}` :(")]
    ShapeCustom(String),

    #[error("Failed to parse angle: `{0}` :(")]
    Angle(String),

    #[error("Failed to parse offset: `x: {0} y: {1} z: {2}` :(")]
    Offset(String, String, String),

    #[error("Failed to parse size: `width: {0} height: {1}` :(")]
    Size(String, String),

    #[error("Failed to parse taper: `{0}` :(")]
    Taper(String),
}

pub fn parse_shapes_text(input: &str) -> Result<Shapes, ShapesParseResult> {
    // shapes(input).unwrap_or_else(Err(ShapesParseResult::Shapes))
    match shapes(input) {
        Ok((_, shapes)) => Ok(shapes),
        Err(_) => Err(ShapesParseResult::Shapes),
    }
}

fn shapes(input: &str) -> IResult<&str, Shapes> {
    let (remainder, _) = ws_around(tag("{")).parse(input)?;
    let (_, shapes) = many0(ws_around(shape)).parse(remainder)?;
    Ok((remainder, Shapes(shapes)))
}

fn shape(input: &str) -> IResult<&str, Shape> {
    let (remainder, (id, scales, _)) = brackets_around((
        ws_around(digit1),
        brackets_around(many1(ws_around(scale))),
        ws,
    ))
    .parse(input)?;
    Ok((
        remainder,
        Shape::Standard {
            scales,
            id: ShapeId::Number(id.parse::<u32>().unwrap()),
        },
    ))
}

fn scale(input: &str) -> IResult<&str, Scale> {
    let (remainder, verts) = brackets_around(preceded(take_until("verts="), verts)).parse(input)?;
    Ok((
        remainder,
        Scale {
            verts,
            ..Default::default()
        },
    ))
}

fn verts(input: &str) -> IResult<&str, Vertices> {
    let (remainder, verts) = many1(ws_around(vert)).parse(input)?;
    Ok((remainder, Vertices(verts)))
}

fn vert(input: &str) -> IResult<&str, Vertex> {
    let (remainder, (x, y)) = brackets_around(ws_around(separated_pair(
        parse_number_expression,
        ws,
        parse_number_expression,
    )))
    .parse(input)?;
    Ok((remainder, Vertex(do2d_float_from(x, y))))
}
