use egui::{Pos2, Rgba, pos2};
use luexks_reassembly::{
    blocks::{block::Block, shroud_layer::ShroudLayerColor},
    shapes::shape_id::ShapeId,
    utility::color::Color,
};

use crate::{DEFAULT_SQUARE, color_type_conversion::rgba_to_color};

pub struct BlockContainer {
    pub block: Block,
    pub vertices: Vec<Pos2>,
    pub shape_id: String,
    pub color_1: Rgba,
    pub color_2: Rgba,
    pub line_color: Rgba,
    pub input_color_1: String,
    pub input_color_2: String,
    pub input_line_color: String,
    // pub input_color_1: Vec<u8>,
    // pub input_color_2: Vec<u8>,
    // pub input_line_color: Vec<u8>,
    // pub search_buf: String,
    pub visible: bool,
    pub max_scale: u8,
    pub use_non_turreted_offset: bool,
    pub offset: Pos2,
}

impl Default for BlockContainer {
    fn default() -> Self {
        let (color_1, color_2, line_color) = (
            Rgba::from_rgba_unmultiplied(0.2, 0.2, 0.2, 1.0),
            Rgba::from_rgba_unmultiplied(0.4, 0.4, 0.4, 1.0),
            Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, 1.0),
        );
        let (input_color_1, input_color_2, input_line_color) = (
            rgba_to_color(color_1).to_string(),
            rgba_to_color(color_2).to_string(),
            rgba_to_color(line_color).to_string(),
        );
        // let (
        //     color_1,
        //     color_2,
        //     line_color,
        // ) = (
        //     Rgba::from_rgba_unmultiplied(0.2, 0.2, 0.2, 1.0),
        //     Rgba::from_rgba_unmultiplied(0.4, 0.4, 0.4, 1.0),
        //     Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, 1.0),
        // );
        // let (
        //     input_color_1,
        //     input_color_2,
        //     input_line_color,
        // ) = (
        //     String::from_utf8_lossy(rgba_to_color_string(color_1)),
        //     String::from_utf8_lossy(rgba_to_color_string(color_2)),
        //     String::from_utf8_lossy(rgba_to_color_string(line_color)),
        // );
        let mut block_container = BlockContainer {
            block: Block {
                shape: Some(ShapeId::Vanilla("SQUARE".to_string())),
                scale: Some(1),
                color_1: Some(Color::new_aarrggbb_str("FF7F7F7F")),
                color_2: Some(Color::new_aarrggbb_str("FFAAAAAA")),
                line_color: Some(Color::new_aarrggbb_str("FFFFFFFF")),
                ..Default::default()
            },
            vertices: DEFAULT_SQUARE.into(),
            shape_id: "SQUARE".into(),
            // color_1: Rgba::from_rgba_unmultiplied(0.2, 0.2, 0.2, 1.0),
            // color_2: Rgba::from_rgba_unmultiplied(0.4, 0.4, 0.4, 1.0),
            // line_color: Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, 1.0),
            // input_color_1: String::new(),
            // input_color_2: String::new(),
            // input_line_color: String::new(),
            // input_color_1: Vec::new(),
            // input_color_2: Vec::new(),
            // input_line_color: Vec::new(),
            color_1,
            color_2,
            line_color,
            input_color_1,
            input_color_2,
            input_line_color,
            // search_buf: String::new(),
            visible: true,
            max_scale: 10,
            use_non_turreted_offset: true,
            offset: Pos2::default(),
        };
        block_container.update_non_turreted_offset();
        block_container
    }
}

impl BlockContainer {
    pub fn get_shroud_color(&self, color_id: ShroudLayerColor) -> Rgba {
        match color_id {
            ShroudLayerColor::Color1 => self.color_1,
            ShroudLayerColor::Color2 => self.color_2,
            ShroudLayerColor::LineColor => self.line_color,
        }
    }

    pub fn update_non_turreted_offset(&mut self) {
        let avg_vert_pos = match &*self.shape_id {
            "SQUARE" => pos2(-5.0, 0.0),
            "COMMAND" | "CANNON" | "CANNON2" | "MISSILE_LAUNCHER" | "MISSILE_SHORT" => {
                pos2(0.0, 0.0)
            }
            _ => {
                self.vertices.iter().fold(Pos2::default(), |pos, vert| {
                    pos2(pos.x + vert.x, pos.y + vert.y)
                }) / self.vertices.len() as f32
            }
        };
        let mut verts = self.vertices.clone();
        verts
            .iter_mut()
            .for_each(|vert| *vert = pos2(vert.x - avg_vert_pos.x, vert.y - avg_vert_pos.y));

        let min_vert_dist = verts
            .iter()
            .map(|vert| (vert.x.powi(2) + vert.y.powi(2)).sqrt())
            .min_by(f32::total_cmp)
            .unwrap();
        let max_midpoint_dist = verts
            .iter()
            .zip(verts.iter().cycle().skip(1))
            .map(|(vert_a, vert_b)| {
                (((vert_a.x + vert_b.x) / 2.0).powi(2) + (vert_a.y + vert_b.y) / 2.0)
                    .powi(2)
                    .sqrt()
            })
            .max_by(f32::total_cmp)
            .unwrap();
        let icon_radius = min_vert_dist.min(max_midpoint_dist);
        self.offset = pos2(icon_radius * -0.5, 0.0)
    }
}
