use egui::{Pos2, pos2};

use crate::{right_tri_angle_edge_case::RIGHT_TRI, shape_container::ShapeContainer};

const ASYM_VERT_TOLERANCE: f32 = 0.02;

pub const fn invert_y_of_pos2(position: Pos2) -> Pos2 {
    pos2(position.x, -position.y)
}

impl ShapeContainer {
    pub fn set_invert_height_of_mirror(&mut self) {
        if self.s.get_id().unwrap().to_string() == RIGHT_TRI {
            self.invert_height_of_mirror = false;
            return;
        }
        let mut verts = self
            .s
            .get_first_scale_vertices()
            .0
            .into_iter()
            .map(|vert| pos2(vert.0.x.to_f32(), vert.0.y.to_f32()))
            .collect::<Vec<_>>();
        let avg_vert_pos = verts.iter().fold(Pos2::default(), |pos, vert| {
            pos2(pos.x + vert.x, pos.y + vert.y)
        }) / verts.len() as f32;
        verts.iter_mut().for_each(|vert| {
            vert.x -= avg_vert_pos.x;
            vert.y -= avg_vert_pos.y;
        });
        for v in &verts {
            if !verts.iter().any(|u| {
                (v.x - u.x).abs() < ASYM_VERT_TOLERANCE && (v.y + u.y).abs() < ASYM_VERT_TOLERANCE
            }) {
                self.invert_height_of_mirror = true;
                return;
            }
        }
        // verts.retain(|vert| vert.y.abs() >= f32::EPSILON);
        // verts.retain(|vert| vert.y.abs() >= ASYM_VERT_TOLERANCE);
        // if verts.len() % 2 != 0 {
        //     self.invert_height_of_mirror = true;
        //     return;
        // }
        // while let Some(first) = verts.first() {
        //     let first_x_range = (first.x - ASYM_VERT_TOLERANCE..first.x + ASYM_VERT_TOLERANCE);
        //     let first_y_range = (first.y - ASYM_VERT_TOLERANCE..first.y + ASYM_VERT_TOLERANCE);
        //     if let Some(mirror) = verts.iter().position(|vert| {
        //         first_x_range.contains(&vert.x) && first_y_range.contains(&-vert.y)
        //     }) {
        //         verts.remove(mirror);
        //         verts.remove(0);
        //     } else {
        //         self.invert_height_of_mirror = true;
        //         return;
        //     }
        // }
    }
}
