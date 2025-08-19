use core::f32;

use egui::{Pos2, pos2};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer,
    shapes::shape_id::ShapeId,
    utility::{
        angle::Angle,
        display_oriented_math::{DisplayOriented2D, do2d_float_from},
    },
};

use crate::DEFAULT_SQUARE;

#[derive(Clone)]
pub struct ShroudLayerContainer {
    pub shroud_layer: ShroudLayer,
    pub vertices: Vec<Pos2>,
    pub shape_id: String,
    pub delete_next_frame: bool,
}

impl Default for ShroudLayerContainer {
    fn default() -> Self {
        Self {
            shroud_layer: ShroudLayer {
                shape: Some(ShapeId::Vanilla("SQUARE".to_string())),
                size: Some(do2d_float_from(10.0, 5.0)),
                angle: Some(Angle::Radian(0.0)),
                ..Default::default()
            },
            vertices: DEFAULT_SQUARE.into(),
            shape_id: "SQUARE".to_string(),
            delete_next_frame: false,
        }
    }
}

impl ShroudLayerContainer {
    pub fn get_shroud_layer_vertices(&self) -> Vec<Pos2> {
        let shape_id = self.shape_id.as_str();
        // println!("{}", shape_id);
        // println!("{}", self.shroud_layer.shape.clone().unwrap().get_name());
        let verts = self.vertices.clone();
        let avg_vert_pos = match shape_id {
            "SQUARE" => pos2(-5.0, 0.0),
            "COMMAND" => pos2(0.0, 0.0),
            _ => {
                verts.iter().fold(Pos2::default(), |pos, vert| {
                    pos2(pos.x + vert.x, pos.y + vert.y)
                }) / verts.len() as f32
            }
        };
        let verts = verts
            .iter()
            .map(|vert| pos2(vert.x - avg_vert_pos.x, vert.y - avg_vert_pos.y))
            .collect::<Vec<_>>();
        // println!("{:?}", verts);
        let (min_x, max_x, min_y, max_y) = verts.iter().fold(
            (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
            |(min_x, max_x, min_y, max_y), vert| {
                (
                    vert.x.min(min_x),
                    vert.x.max(max_x),
                    vert.y.min(min_y),
                    vert.y.max(max_y),
                )
            },
        );
        let angle_option = &self.shroud_layer.angle;
        let shape_size = pos2(-min_x + max_x, -min_y + max_y);
        let shroud_size = self.shroud_layer.size.clone().unwrap();

        let verts = if shape_id == "SQUARE" {
            let shroud_size = do2d_float_from(shroud_size.x.to_f32(), shroud_size.y.to_f32() * 2.0);
            let verts = apply_size_to_verts(verts, shroud_size, shape_size);
            let verts = if let Some(taper) = self.shroud_layer.taper {
                let verts = vec![
                    pos2(verts[0].x, verts[0].y * taper),
                    verts[1],
                    verts[2],
                    pos2(verts[3].x, verts[3].y * taper),
                ];
                verts
            } else {
                verts
            };
            let verts = apply_angle_to_verts(verts, angle_option);
            verts
        } else {
            let verts = apply_angle_to_verts(verts, angle_option);
            let verts = apply_post_angle_application_resize(verts, shape_size, angle_option);
            let verts = apply_size_to_verts(verts, shroud_size, shape_size);
            verts
        };
        verts
    }
}

fn apply_angle_to_verts(verts: Vec<Pos2>, angle_option: &Option<Angle>) -> Vec<Pos2> {
    if let Some(angle) = angle_option {
        let angle = angle.as_radians().get_value();
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();
        verts
            .iter()
            .map(|vert| {
                let new_x = vert.x * cos_angle - vert.y * sin_angle;
                let new_y = vert.x * sin_angle + vert.y * cos_angle;
                pos2(new_x, new_y)
            })
            .collect()
    } else {
        verts
    }
}

fn apply_post_angle_application_resize(
    verts: Vec<Pos2>,
    shape_size: Pos2,
    angle_option: &Option<Angle>,
) -> Vec<Pos2> {
    if angle_option.is_some() {
        let (min_x, max_x, min_y, max_y) = verts.iter().fold(
            (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
            |(min_x, max_x, min_y, max_y), vert| {
                (
                    vert.x.min(min_x),
                    vert.x.max(max_x),
                    vert.y.min(min_y),
                    vert.y.max(max_y),
                )
            },
        );
        let rotated_shape_size = pos2(-min_x + max_x, -min_y + max_y);
        let verts = if shape_size != rotated_shape_size {
            let resized_verts = verts
                .iter()
                .map(|vert| {
                    pos2(
                        vert.x * shape_size.x
                            / if rotated_shape_size.x.abs() < f32::EPSILON {
                                1.0
                            } else {
                                rotated_shape_size.x
                            },
                        vert.y * shape_size.y
                            / if rotated_shape_size.y.abs() < f32::EPSILON {
                                1.0
                            } else {
                                rotated_shape_size.y
                            },
                    )
                })
                .collect();
            resized_verts
        } else {
            verts
        };
        verts
    } else {
        verts
    }
}

fn apply_size_to_verts(verts: Vec<Pos2>, size: DisplayOriented2D, shape_size: Pos2) -> Vec<Pos2> {
    let verts = verts
        .iter()
        .map(|vert| {
            pos2(
                vert.x * size.x.to_f32() / shape_size.x,
                vert.y * size.y.to_f32() / shape_size.y,
            )
        })
        .collect();
    verts
}
