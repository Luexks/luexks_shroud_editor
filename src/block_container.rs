use egui::{Pos2, Rgba};
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
        BlockContainer {
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
        }
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
}
