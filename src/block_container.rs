use egui::{Color32, Pos2, Rgba};
use luexks_reassembly::{blocks::{block::Block, shroud_layer::ShroudLayerColor}, shapes::shape_id::ShapeId, utility::color::Color};

use crate::DEFAULT_SQUARE;

pub struct BlockContainer {
    pub block: Block,
    pub vertices: Vec<Pos2>,
    pub shape_id: String,
    pub color_1: Rgba,
    pub color_2: Rgba,
    pub line_color: Rgba,
}

impl Default for BlockContainer {
    fn default() -> Self {
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
            // fill_color_1: Rgba::from_rgba_unmultiplied(127, 127, 127, 255),
            // fill_color_2: Rgba::from_rgba_unmultiplied(170, 170, 170, 255),
            // line_color: Rgba::from_rgba_unmultiplied(255, 255, 255, 255),
            // color_1: Rgba::from_rgba_unmultiplied(0.5, 0.5, 0.5, 1.0),
            // color_2: Rgba::from_rgba_unmultiplied(2.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0, 1.0),
            color_1: Rgba::from_rgba_unmultiplied(0.2, 0.2, 0.2, 1.0),
            color_2: Rgba::from_rgba_unmultiplied(0.4, 0.4, 0.4, 1.0),
            line_color: Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl BlockContainer {
    pub fn get_shroud_color(&self, color_id: ShroudLayerColor) -> Rgba {
        let color = match color_id {
            // ShroudLayerColor::Color1 => self.block.color_1.clone().unwrap(),
            // ShroudLayerColor::Color2 => self.block.color_2.clone().unwrap(),
            // ShroudLayerColor::LineColor => self.block.line_color.clone().unwrap(),
            ShroudLayerColor::Color1 => self.color_1.clone(),
            ShroudLayerColor::Color2 => self.color_2.clone(),
            ShroudLayerColor::LineColor => self.line_color.clone(),
        };
        // let color = Rgba::from_rgba_unmultiplied(
        //     color.rr() as f32 / 255.0,
        //     color.gg() as f32 / 255.0,
        //     color.bb() as f32 / 255.0,
        //     color.aa().unwrap() as f32 / 255.0,
        // );
        color
    }
}
