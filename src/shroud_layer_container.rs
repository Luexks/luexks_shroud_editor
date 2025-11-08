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

#[derive(Clone, Debug)]
pub struct ShroudLayerContainer {
    pub shroud_layer: ShroudLayer,
    pub vertices: Vec<Pos2>,
    pub shape_id: String,
    pub delete_next_frame: bool,
    pub drag_pos: Option<Pos2>,
    pub mirror_index_option: Option<usize>,
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
            drag_pos: None,
            mirror_index_option: None,
        }
    }
}

impl ShroudLayerContainer {
    pub fn get_shroud_layer_vertices(&self) -> Vec<Pos2> {
        let shape_id = self.shape_id.as_str();
        let mut verts = self.vertices.clone();
        match shape_id {
            "CANNON" => {
                verts = [6, 2, 3, 7, 4, 5, 0, 1]
                    .into_iter()
                    .map(|i| verts[i])
                    .collect();
            }
            "CANNON2" => {
                verts = [0, 1, 6, 4, 5, 7, 2, 3]
                    .into_iter()
                    .map(|i| verts[i])
                    .collect();
            }
            "SENSOR" => {
                verts = [4, 2, 3, 0, 1].into_iter().map(|i| verts[i]).collect();
            }
            _ => {}
        }
        verts.iter_mut().for_each(|vert| vert.y *= -1.0);
        let avg_vert_pos = match shape_id {
            "SQUARE" => pos2(-5.0, 0.0),
            "COMMAND" | "CANNON" | "CANNON2" | "MISSILE_LAUNCHER" | "MISSILE_SHORT" => {
                pos2(0.0, 0.0)
            }
            _ => {
                verts.iter().fold(Pos2::default(), |pos, vert| {
                    pos2(pos.x + vert.x, pos.y + vert.y)
                }) / verts.len() as f32
            }
        };
        verts
            .iter_mut()
            .for_each(|vert| *vert = pos2(vert.x - avg_vert_pos.x, vert.y - avg_vert_pos.y));
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

        if shape_id == "SQUARE" {
            let shroud_size = do2d_float_from(shroud_size.x.to_f32(), shroud_size.y.to_f32() * 2.0);
            apply_size_to_verts(&mut verts, shroud_size, shape_size);
            if let Some(taper) = self.shroud_layer.taper {
                verts = if taper >= 0.0 {
                    vec![
                        pos2(verts[0].x, verts[0].y * taper),
                        verts[1],
                        verts[2],
                        pos2(verts[3].x, verts[3].y * taper),
                    ]
                } else {
                    vec![
                        verts[0],
                        pos2(verts[2].x, verts[2].y * taper),
                        pos2(verts[1].x, verts[1].y * taper),
                        verts[3],
                    ]
                };
            }
            apply_angle_to_verts(&mut verts, angle_option);
        } else {
            apply_angle_to_verts(&mut verts, angle_option);
            apply_size_to_verts(&mut verts, shroud_size, shape_size)
        }
        verts
    }
}

fn apply_angle_to_verts(verts: &mut [Pos2], angle_option: &Option<Angle>) {
    if let Some(angle) = angle_option {
        let angle = -angle.as_radians().get_value();
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();
        verts.iter_mut().for_each(|vert| {
            let new_x = vert.x * cos_angle - vert.y * sin_angle;
            let new_y = vert.x * sin_angle + vert.y * cos_angle;
            *vert = pos2(new_x, new_y);
        });
    }
}

fn apply_size_to_verts(verts: &mut [Pos2], size: DisplayOriented2D, shape_size: Pos2) {
    verts.iter_mut().for_each(|vert| {
        *vert = pos2(
            vert.x * size.x.to_f32() / shape_size.x,
            vert.y * size.y.to_f32() / shape_size.y,
        );
    });
}
