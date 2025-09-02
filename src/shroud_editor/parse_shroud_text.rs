use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayerColor,
    shapes::{shape::Shape, shape_id::ShapeId, shapes::Shapes},
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
    restructure_vertices::restructure_vertices, shroud_layer_container::ShroudLayerContainer,
};

use nom::character::char;

#[derive(Error, Debug)]
pub enum ShroudParseResult {
    #[error("YES!")]
    Success,

    #[error("Failed to parse shroud={{ part of it")]
    Shroud,

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

#[rustfmt::skip]
pub fn parse_shroud_text(shroud_text: &str, loaded_shapes: &Shapes) -> Result<Vec<ShroudLayerContainer>, ShroudParseResult> {
    let (_, shroud_data) = shroud(shroud_text)
        .map_err(|_| ShroudParseResult::Shroud)?;
    shroud_data.iter().map(|shroud_layer_data| {
        let mut shroud_layer_container = ShroudLayerContainer::default();
        for variable_data in shroud_layer_data {
            match variable_data {
                ("tri_color_id", variable_value_data) => {
                    if let Some(tri_color_id_data) = variable_value_data.first() {
                        match *tri_color_id_data {
                            "0" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err(ShroudParseResult::Color1(tri_color_id_data.to_string())) },
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("tri_color_id".to_string()));
                    }
                },
                ("tri_color1_id", variable_value_data) => {
                    if let Some(tri_color1_id_data) = variable_value_data.first() {
                        match *tri_color1_id_data {
                            "0" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err(ShroudParseResult::Color2(tri_color1_id_data.to_string())) },
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("tri_color1_id".to_string()));
                    }
                },
                ("line_color_id", variable_value_data) => {
                    if let Some(line_color_id_data) = variable_value_data.first() {
                        match *line_color_id_data {
                            "0" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err(ShroudParseResult::LineColor(line_color_id_data.to_string())) },
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("line_color_id".to_string()));
                    }
                },
                ("shape", variable_value_data) => {
                    if let Some(shape_data) = variable_value_data.first() {
                        if shape_data.chars().all(|c| c.is_ascii_digit()) {
                            let shape_name = shape_data.parse::<u32>().unwrap();
                            let shape_name_string = shape_name.to_string();
                            if let Some(matched_shape) = match_shape(loaded_shapes, &shape_name_string) {
                                shroud_layer_container.shroud_layer.shape = Some(ShapeId::Number(shape_name));
                                shroud_layer_container.vertices = restructure_vertices(matched_shape.get_first_scale_vertices());
                                shroud_layer_container.shape_id = shape_name_string;
                            } else {
                                return Err(ShroudParseResult::ShapeCustom(shape_name_string));
                            }
                        } else {
                            let shape_name_string = shape_data.to_string();
                            shroud_layer_container.shroud_layer.shape = Some(ShapeId::Vanilla(shape_name_string.clone()));
                            if let Some(matched_shape) = match_shape(loaded_shapes, &shape_name_string) {
                                shroud_layer_container.vertices = restructure_vertices(matched_shape.get_first_scale_vertices());
                                shroud_layer_container.shape_id = shape_name_string;
                            } else {
                                return Err(ShroudParseResult::ShapeVanilla(shape_name_string));
                            }
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("shape".to_string()));
                    }
                },
                ("angle", variable_value_data) => {
                    if let Some(angle_data) = variable_value_data.first() {
                        if let Ok((_, angle)) = parse_number_expression(angle_data) {
                            shroud_layer_container.shroud_layer.angle = Some(Angle::Radian(angle).as_degrees());
                        } else {
                            return Err(ShroudParseResult::Angle(angle_data.to_string()));
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("angle".to_string()));
                    }
                },
                ("offset", variable_value_data) => {
                    if let (
                        Some(x_data),
                        Some(y_data),
                        Some(z_data),
                    ) = (
                        variable_value_data.get(0),
                        variable_value_data.get(1),
                        variable_value_data.get(2),
                    ) {
                        if let (
                            Ok((_, x)),
                            Ok((_, y)),
                            Ok((_, z)),
                        ) = (
                            parse_number_expression(x_data),
                            parse_number_expression(y_data),
                            parse_number_expression(z_data),
                        ) {
                            shroud_layer_container.shroud_layer.offset = Some(do3d_float_from(x, y, z));
                        } else {
                            return Err(ShroudParseResult::Offset(x_data.to_string(), y_data.to_string(), z_data.to_string()));
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("offset".to_string())) ;
                    }
                },
                ("size", variable_value_data) => {
                    if let (
                        Some(width_data),
                        Some(height_data),
                    ) = (
                        variable_value_data.get(0),
                        variable_value_data.get(1),
                    ) {
                        if let (
                            Ok((_, width)),
                            Ok((_, height)),
                        ) = (
                            parse_number_expression(width_data),
                            parse_number_expression(height_data),
                        ) {
                            shroud_layer_container.shroud_layer.size = Some(do2d_float_from(width, height));
                        } else {
                            return Err(ShroudParseResult::Size(width_data.to_string(), height_data.to_string()));
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("size".to_string()));
                    }
                },
                ("taper", variable_value_data) => {
                    if let Some(taper_data) = variable_value_data.first() {
                        if let Ok((_, taper)) = parse_number_expression(taper_data) {
                            shroud_layer_container.shroud_layer.taper = Some(taper);
                        } else {
                            return Err(ShroudParseResult::Taper(taper_data.to_string()));
                        }
                    } else {
                        return Err(ShroudParseResult::VariableValueData("taper".to_string())) ;
                    }
                },
                _ => { return Err(ShroudParseResult::VariableNameData(variable_data.0.to_string())) },
            }
        }
        Ok(shroud_layer_container)
    }).collect()
}

fn parse_number(input: &str) -> IResult<&str, f32> {
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
enum Operator {
    Div,
    Mul,
}

fn parse_number_expression(input: &str) -> IResult<&str, f32> {
    let (remainder, first_number) = parse_number(input)?;
    let (remainder, operator_number_pairs) = many0(pair(
        alt((
            value(
                Operator::Mul,
                complete(pair(opt(complete(whitespace_and_comment)), char('*'))),
            ),
            value(
                Operator::Div,
                complete(pair(opt(complete(whitespace_and_comment)), char('/'))),
            ),
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

fn match_shape<'a>(loaded_shapes: &'a Shapes, shape_name_string: &'a str) -> Option<&'a Shape> {
    loaded_shapes
        .0
        .iter()
        .find(|loaded_shapes| loaded_shapes.get_id().unwrap().to_string() == shape_name_string)
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn variable_value(input: &str) -> IResult<&str, Vec<&str>> {
    if peek(char::<&str, Error<_>>('{')).parse(input).is_ok() {
        delimited(
            tag("{"),
            many1(delimited(
                whitespace_and_comment,
                alphanumeric_special_1,
                whitespace_and_comment,
            )),
            tag("}"),
        )
        .parse(input)
    } else {
        many1(alphanumeric_special_1).parse(input)
    }
}

fn alphanumeric_special_1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '/' || c == '*'
    })
    .parse(input)
}

fn whitespace_and_comment(input: &str) -> IResult<&str, ()> {
    let (remainder, _) =
        value((), take_while(|c: char| c.is_whitespace() || c == ',')).parse(input)?;
    let (remainder, _) = comment(remainder)?;
    let (remainder, _) =
        value((), take_while(|c: char| c.is_whitespace() || c == ',')).parse(remainder)?;
    Ok((remainder, ()))
}

fn whitespace_and_equals(input: &str) -> IResult<&str, ()> {
    value(
        (),
        take_while(|c: char| c.is_whitespace() || c == ',' || c == '='),
    )
    .parse(input)
}

fn variable(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(variable_name, whitespace_and_equals, variable_value).parse(input)
}

fn shroud_layer_container(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    delimited(
        tag("{"),
        many0(delimited(
            whitespace_and_comment,
            variable,
            whitespace_and_comment,
        )),
        tag("}"),
    )
    .parse(input)
}

fn shroud(input: &str) -> IResult<&str, Vec<Vec<(&str, Vec<&str>)>>> {
    let (remainder, _) = whitespace_and_comment(input)?;
    let (remainder, _) = tag("shroud")(remainder)?;
    let (remainder, _) = whitespace_and_equals(remainder)?;
    let (remainder, _) = tag("{")(remainder)?;
    let (remainder, _) = whitespace_and_comment(remainder)?;
    many0(delimited(
        whitespace_and_comment,
        shroud_layer_container,
        whitespace_and_comment,
    ))
    .parse(remainder)
}

fn comment(input: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("--"), terminated(take_until("\n"), tag("\n")))).parse(input)
}
