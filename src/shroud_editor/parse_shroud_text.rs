use std::f32::{self, consts::PI};

use luexks_reassembly::{blocks::shroud_layer::ShroudLayerColor, shapes::{shape_id::ShapeId, shapes::Shapes}, utility::{angle::Angle, display_oriented_math::{do2d_float_from, do3d_float_from}}};
use nom::{
    bytes::complete::{tag, take_while, take_while1}, combinator::{peek, value}, error::Error, multi::{many0, many1}, sequence::{delimited, separated_pair}, IResult, Parser
};

use crate::{restructure_vertices::restructure_vertices, shroud_layer_container::ShroudLayerContainer};

use nom::character::char;

pub fn parse_shroud_text(shroud_text: &str, loaded_shapes: &Shapes) -> Result<Vec<ShroudLayerContainer>, String> {
    // let shroud_data_result = shroud(shroud_text);
    let (_, shroud_data) = shroud(shroud_text)
        .map_err(|_| "Failed to parse shroud :(".to_string())?;
    shroud_data.iter().map(|shroud_layer_data| {
        let mut shroud_layer_container = ShroudLayerContainer::default();
        for variable_data in shroud_layer_data {
            match variable_data {
                ("tri_color_id", variable_value_data) => {
                    if let Some(tri_color_id_data) = variable_value_data.get(0) {
                        match *tri_color_id_data {
                            "0" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err("Failed to parse tri_color_id :(".to_string()) },
                            _ => {},
                        }
                    } else {
                        return Err("Failed to parse tri_color_id :(".to_string()) ;
                    }
                },
                ("tri_color1_id", variable_value_data) => {
                    if let Some(tri_color1_id_data) = variable_value_data.get(0) {
                        match *tri_color1_id_data {
                            "0" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.color_2 = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err("Failed to parse tri_color1_id :(".to_string()) },
                            _ => {},
                        }
                    } else {
                        return Err("Failed to parse tri_color1_id :(".to_string()) ;
                    }
                },
                ("line_color_id", variable_value_data) => {
                    if let Some(line_color_id_data) = variable_value_data.get(0) {
                        match *line_color_id_data {
                            "0" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::Color1); },
                            "1" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::Color2); },
                            "2" => { shroud_layer_container.shroud_layer.line_color = Some(ShroudLayerColor::LineColor); },
                            _ => { return Err("Failed to parse line_color_id :(".to_string()) },
                            _ => {},
                        }
                    } else {
                        return Err("Failed to parse line_color_id :(".to_string()) ;
                    }
                },
                ("shape", variable_value_data) => {
                    // let shape = variable_value_data.get(0);
                    if let Some(shape_data) = variable_value_data.get(0) {
                        if shape_data.chars().any(|c| c.is_alphabetic()) {
                            let shape_name = shape_data.to_string();
                            shroud_layer_container.shroud_layer.shape = Some(ShapeId::Vanilla(shape_name.clone()));
                            if let Some(matched_shape) = loaded_shapes.0.iter().find(|loaded_shape| loaded_shape.get_id().unwrap().to_string() == shape_name) {
                                shroud_layer_container.vertices = restructure_vertices(matched_shape.get_first_scale_vertices());
                                shroud_layer_container.shape_id = shape_name;
                            } else {
                                return Err("No match of defined shape :(".to_string());
                            }
                        } else if shape_data.chars().any(|c| !c.is_numeric()) {
                            let shape_name = shape_data.parse::<u32>().unwrap();
                            let shape_name_string = shape_name.to_string();
                            if let Some(matched_shape) = loaded_shapes.0.iter().find(|loaded_shapes| loaded_shapes.get_id().unwrap().to_string() == shape_name_string) {
                                shroud_layer_container.shroud_layer.shape = Some(ShapeId::Number(shape_name));
                                shroud_layer_container.vertices = restructure_vertices(matched_shape.get_first_scale_vertices());
                                shroud_layer_container.shape_id = shape_name_string;
                            } else {
                                return Err("No match of defined shape :(".to_string());
                            }
                        } else {
                            return Err("Failed to parse shape :(".to_string()) ;
                        }
                    } else {
                        return Err("Failed to parse shape :(".to_string()) ;
                    }
                },
                ("angle", variable_value_data) => {
                    if let Some(angle_data) = variable_value_data.get(0) {
                        if let Ok(angle) = angle_data.parse::<f32>() {
                            shroud_layer_container.shroud_layer.angle = Some(Angle::Radian(angle).as_degrees());
                        } else {
                            return Err("Failed to parse angle :(".to_string()) ;
                        }
                    } else {
                        return Err("Failed to parse angle :(".to_string()) ;
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
                            Ok(x),
                            Ok(y),
                            Ok(z),
                        ) = (
                            x_data.parse::<f32>(),
                            y_data.parse::<f32>(),
                            z_data.parse::<f32>(),
                        ) {
                            shroud_layer_container.shroud_layer.offset = Some(do3d_float_from(x, y, z));
                        } else {
                            return Err("Failed to parse offset :(".to_string()) ;
                        }
                    } else {
                        return Err("Failed to parse offset :(".to_string()) ;
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
                            Ok(width),
                            Ok(height),
                        ) = (
                            width_data.parse::<f32>(),
                            height_data.parse::<f32>(),
                        ) {
                            shroud_layer_container.shroud_layer.size = Some(do2d_float_from(width, height));
                        } else {
                            return Err("Failed to parse size :(".to_string()) ;
                        }
                    } else {
                        return Err("Failed to parse size :(".to_string()) ;
                    }
                },
                ("taper", variable_value_data) => {
                    if let Some(taper_data) = variable_value_data.get(0) {
                        if let Ok(taper) = taper_data.parse::<f32>() {
                            shroud_layer_container.shroud_layer.taper = Some(taper);
                        } else {
                            return Err("Failed to parse taper :(".to_string()) ;
                        }
                    } else {
                        return Err("Failed to parse taper :(".to_string()) ;
                    }
                },
                _ => { return Err("Failed to parse variable data :(".to_string()) },
            }
        }
        Ok(shroud_layer_container)
    }).collect()
    // let shroud_result = match shroud_data_result {
    //     Err(_) => Err("Failed to parse shroud :(".to_string()),
    //     Ok((_, shroud_data)) => Ok(shroud_data.iter().map(|shroud_layer_data| {
    //         let mut shroud_layer_container = ShroudLayerContainer::default();
    //         for variable_data in shroud_layer_data {
    //             match variable_data {
    //                 ("tri_color_id", variable_value_data) => {
    //                     if let Some(tri_color_id_data) = variable_value_data.get(0) {
    //                         match *tri_color_id_data {
    //                             "0" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
    //                             "1" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
    //                             "2" => { shroud_layer_container.shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
    //                             _ => { return Err("Failed to parse tri_color_id :(".to_string()) },
    //                             _ => {},
    //                         }
    //                     } else {
    //                         return Err("Failed to parse shape :(".to_string()) ;
    //                     }
    //                 },
    //                 ("shape", variable_value_data) => {
    //                     // let shape = variable_value_data.get(0);
    //                     if let Some(shape_data) = variable_value_data.get(0) {
    //                         if shape_data.chars().any(|c| c.is_alphabetic()) {
    //                             shroud_layer_container.shroud_layer.shape = Some(ShapeId::Vanilla(shape_data.to_string()));
    //                         } else if shape_data.chars().any(|c| !c.is_numeric()) {
    //                             shroud_layer_container.shroud_layer.shape = Some(ShapeId::Number(shape_data.parse::<u32>().unwrap()));
    //                         } else {
    //                             return Err("Failed to parse shape :(".to_string()) ;
    //                         }
    //                     } else {
    //                         return Err("Failed to parse shape :(".to_string()) ;
    //                     }
    //                 },
    //                 _ => { return Err("Failed to parse variable data :(".to_string()) },
    //                 _ => { }
    //             }
    //         }
    //         // todo!()
    //         // shroud_layer_container
    //         Ok(shroud_layer_container)
    //     // }).collect::<Vec<ShroudLayerContainer>>()),
    //     }).collect::<Result<Vec<_>, _>>()),
    // };
    // if let Err(s) = shroud_result {
    //     return Err(s);
    // } else if let Ok(shroud) = shroud_result {
    //     // shroud.into_iter().filter_map(|shroud_layer_result| match shroud_layer_result {
    //     //     Ok(shroud_layer) => Some(shroud_layer),
    //     //     Err(s) => {return s;}
    //     shroud.into_iter().filter(|shroud_layer_result| shroud_layer_result).collect();
    // }
    // shroud_result
    // todo!()
    // let shroud = match shroud_data_result {
    //     Ok((_remainder, shroud_data)) => {
    //         shroud_data.iter().map(|shroud_layer_data| {
    //             let mut shroud_layer = ShroudLayer::default();
    //             // shroud_layer_data.iter().for_each(|variable_data| {
    //             for variable_data in shroud_layer_data {
    //                 match variable_data {
    //                     ("tri_color_id", variable_value_data) => {
    //                         if let Some(tri_color_id_data) = variable_value_data.get(0) {
    //                             match *tri_color_id_data {
    //                                 "0" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
    //                                 "1" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
    //                                 "2" => { shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
    //                                 _ => { return Err("Failed to parse tri_color_id :(".to_string()) },
    //                             }
    //                         } else {
    //                             return Err("Failed to parse shape :(".to_string()) ;
    //                         }
    //                     },
    //                     ("shape", variable_value_data) => {
    //                         // let shape = variable_value_data.get(0);
    //                         if let Some(shape_data) = variable_value_data.get(0) {
    //                             if shape_data.chars().any(|c| c.is_alphabetic()) {
    //                                 shroud_layer.shape = Some(ShapeId::Vanilla(shape_data.to_string()));
    //                             } else if shape_data.chars().any(|c| !c.is_numeric()) {
    //                                 shroud_layer.shape = Some(ShapeId::Number(shape_data.parse::<u32>().unwrap()));
    //                             } else {
    //                                 return Err("Failed to parse shape :(".to_string()) ;
    //                             }
    //                         } else {
    //                             return Err("Failed to parse shape :(".to_string()) ;
    //                         }
    //                     },
    //                     // _ => { return Err("Failed to parse variable data :(".to_string()) },
    //                     _ => { }
    //                 }
    //             }
    //             // });
    //             ShroudLayerContainer {
    //                 shroud_layer,
    //                 ..Default::default()
    //             }
    //         }).collect::<Vec<_>>()
    //         // let shroud = shroud.into_iter().filter_map(Result::ok).collect::<Vec<_>>();
    //         // todo!()
    //     },
    //     // Err(_e) => Err("Failed to parse shroud :(".to_string()),
    //     Err(e) => Err(e.to_string()),
    // };
    // shroud
    // todo!();
    // let shroud_data_result = shroud(shroud_text);
    // match shroud_data_result {
    //     Ok((_remainder, shroud_data)) => {
    //         dbg!(&shroud_data);
    //         let shroud = shroud_data.iter().map(|shroud_layer_data| {
    //             let mut shroud_layer = ShroudLayer::default();
    //             // shroud_layer_data.iter().for_each(|variable_data| {
    //             for variable_data in shroud_layer_data {
    //                 match variable_data {
    //                     ("tri_color_id", variable_value_data) => {
    //                         if let Some(tri_color_id_data) = variable_value_data.get(0) {
    //                             match *tri_color_id_data {
    //                                 "0" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color1); },
    //                                 "1" => { shroud_layer.color_1 = Some(ShroudLayerColor::Color2); },
    //                                 "2" => { shroud_layer.color_1 = Some(ShroudLayerColor::LineColor); },
    //                                 _ => { return Err("Failed to parse tri_color_id :(".to_string()) },
    //                             }
    //                         } else {
    //                             return Err("Failed to parse shape :(".to_string()) ;
    //                         }
    //                     },
    //                     ("shape", variable_value_data) => {
    //                         // let shape = variable_value_data.get(0);
    //                         if let Some(shape_data) = variable_value_data.get(0) {
    //                             if shape_data.chars().any(|c| c.is_alphabetic()) {
    //                                 shroud_layer.shape = Some(ShapeId::Vanilla(shape_data.to_string()));
    //                             } else if shape_data.chars().any(|c| !c.is_numeric()) {
    //                                 shroud_layer.shape = Some(ShapeId::Number(shape_data.parse::<u32>().unwrap()));
    //                             } else {
    //                                 return Err("Failed to parse shape :(".to_string()) ;
    //                             }
    //                         } else {
    //                             return Err("Failed to parse shape :(".to_string()) ;
    //                         }
    //                     },
    //                     // _ => { return Err("Failed to parse variable data :(".to_string()) },
    //                     _ => { }
    //                 }
    //             }
    //             // });
    //             Ok(ShroudLayerContainer {
    //                 shroud_layer,
    //                 ..Default::default()
    //             })
    //         }).collect::<Vec<_>>();
    //         dbg!(&shroud);
    //         let shroud = shroud.into_iter().filter_map(Result::ok).collect::<Vec<_>>();
    //         dbg!(shroud);
    //         todo!()
    //     },
    //     // Err(_e) => Err("Failed to parse shroud :(".to_string()),
    //     Err(e) => Err(e.to_string()),
    // }
    // // todo!();
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn variable_value(input: &str) -> IResult<&str, Vec<&str>> {
    if peek(char::<&str, Error<_>>('{')).parse(input).is_ok() {
        // println!("Starts with {{");
        delimited(tag("{"), many1(delimited(whitespace, alphanumeric_dash_1, whitespace)), tag("}")).parse(input)
    } else {
        many1(alphanumeric_dash_1).parse(input)
    }
    // todo!()
}

fn alphanumeric_dash_1(input: &str) -> IResult<&str, &str> {
    take_while1(|s: char| s.is_alphanumeric() || s == '-' || s == '_' || s == '.').parse(input)
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
    // dbg!(separated_pair(variable_name, whitespace_and_equals, variable_value).parse(input))
}

fn shroud_layer_container(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    // println!("1/many0");
    // many0(delimited(whitespace, variable, whitespace)).parse(input)
    // dbg!(delimited(tag("{"), many0(delimited(whitespace, variable, whitespace)), tag("}")).parse(input))
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
        many0(delimited(whitespace, shroud_layer_container, whitespace)).parse(remainder);
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
