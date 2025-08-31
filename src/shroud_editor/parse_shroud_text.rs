use std::f32::consts::PI;

use egui::IntoAtoms;
use luexks_reassembly::{blocks::{shroud::Shroud, shroud_layer::{ShroudLayer, ShroudLayerColor}}, shapes::shape_id::ShapeId};
use nom::{
    branch::alt, bytes::complete::{tag, take_till, take_while, take_while1}, character::{
        complete::{alphanumeric1, multispace1},
        multispace0,
    }, combinator::{peek, value}, error::Error, multi::{many0, many1}, sequence::{delimited, separated_pair}, Err, IResult, Parser
};

use crate::shroud_layer_container::ShroudLayerContainer;

use nom::character::char;

pub fn parse_shroud_text(shroud_text: &str) -> Result<Vec<ShroudLayerContainer>, String> {
    let shroud_data_result = shroud(shroud_text);
    match shroud_data_result {
        Ok((_remainder, shroud_data)) => {
            dbg!(&shroud_data);
            let shroud = shroud_data.iter().map(|shroud_layer_data| {
                let mut shroud_layer = ShroudLayer::default();
                // shroud_layer_data.iter().for_each(|variable_data| {
                for variable_data in shroud_layer_data {
                    match variable_data {
                        ("tri_color_id", variable_value_data) => {
                            if let Some(tri_color_id_data) = variable_value_data.get(0) {
                                match *tri_color_id_data {
                                    "0" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
                                    "1" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
                                    "2" => { shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
                                    _ => { return Err("Failed to parse tri_color_id :(".to_string()) },
                                }
                            } else {
                                return Err("Failed to parse shape :(".to_string()) ;
                            }
                        },
                        ("shape", variable_value_data) => {
                            // let shape = variable_value_data.get(0);
                            if let Some(shape_data) = variable_value_data.get(0) {
                                if shape_data.chars().any(|c| c.is_alphabetic()) {
                                    shroud_layer.shape = Some(ShapeId::Vanilla(shape_data.to_string()));
                                } else if shape_data.chars().any(|c| !c.is_numeric()) {
                                    shroud_layer.shape = Some(ShapeId::Number(shape_data.parse::<u32>().unwrap()));
                                } else {
                                    return Err("Failed to parse shape :(".to_string()) ;
                                }
                            } else {
                                return Err("Failed to parse shape :(".to_string()) ;
                            }
                        },
                        _ => { return Err("Failed to parse variable data :(".to_string()) },
                    }
                }
                // });
                Ok(ShroudLayerContainer {
                    shroud_layer,
                    ..Default::default()
                })
            }).collect::<Vec<_>>();
            dbg!(shroud);
            todo!()
        },
        // Err(_e) => Err("Failed to parse shroud :(".to_string()),
        Err(e) => Err(e.to_string()),
    }
    // todo!();
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn variable_value(input: &str) -> IResult<&str, Vec<&str>> {
    if peek(char::<&str, Error<_>>('{')).parse(input).is_ok() {
        many1(delimited(whitespace, alphanumeric1, whitespace)).parse(input)
    } else {
        many1(take_while1(|c: char| c.is_alphanumeric() || c == '_')).parse(input)
    }
    // todo!()
}

fn whitespace(input: &str) -> IResult<&str, ()> {
    value((), take_while(|c: char| c.is_whitespace() || c == ',')).parse(input)
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

fn shroud_layer(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    // println!("1/many0");
    // many0(delimited(whitespace, variable, whitespace)).parse(input)
    delimited(tag("{"), many0(delimited(whitespace, variable, whitespace)), tag("}")).parse(input)
}

// fn shroud_layer(input: &str) -> IResult<&str, ()> {
//     delimited(tag("{"), shroud_layer, tag("}"))
// }

fn shroud(input: &str) -> IResult<&str, Vec<Vec<(&str, Vec<&str>)>>> {
    // let mut shroud = Shroud::default();
    // dbg!(input);
    let (remainder, _) = whitespace(input)?;
    let (remainder, _) = tag("shroud")(remainder)?;
    let (remainder, _) = whitespace_and_equals(remainder)?;
    // let (remainder, _) = whitespace(remainder)?;
    // let (remainder, _) = tag("=")(remainder)?;
    // let (remainder, _) = whitespace(remainder)?;
    let (remainder, _) = tag("{")(remainder)?;
    let (remainder, _) = whitespace(remainder)?;
    // let (remainder, _) = delimited(tag("{"), any, tag("}")).parse(remainder)?;
    // println!("2/many0");
    let shroud_data_result =
        many0(delimited(whitespace, shroud_layer, whitespace)).parse(remainder);
    shroud_data_result
    // let (remainder, shroud_data) =
    //     many0(delimited(whitespace, shroud_layer, whitespace)).parse(remainder)?;
    // shroud_data

    // loop {
    //     let next_shroud_layer_result = char('{').parse(remainder);
    //     match next_shroud_layer_result {
    //         Err =>
    //     }
    // }

    // let (remainder, _) = tag("}")(remainder)?;
    // let (remainder, _ ) = whitespace(remainder)?;

    // loop {
    // }
    // todo!();
}

// fn parse_shroud(input: &str) -> IResult<&str, &str> {
//     delimited("{", take_till(cond), "}")
// }

// fn ws_or_comma(input: &str) -> IResult<&str, ()> {
//     value((), many0(alt((
//         value((), multispace0),
//         value((), tag(",")),
//     ))))(input)
// }

// fn parse_whitespace(input: &str) -> IResult<&str, ()> {
//     value((), many0(alt((
//         value((), multispace1),
//         value((), tag(",")),
//     ))))(input)
//     // value((), many0(alt((
//     //     value((), multispace1),
//     //     value((), tag(",")),
//     // ))))
// }

// fn parse_shroud(input: &str) -> IResult<&str, &str> {
//     value((), tag("shroud")).and(
//     value((), many0(
//         alt(
//             value((), multispace1),
//             value((), tag(",")),
//         )
//     ))).and(
//         delimited("{", &str, "}")
//     )
//     (input);
// }
