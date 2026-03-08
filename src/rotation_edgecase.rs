use std::f32::consts::PI;

use crate::shroud_layer_container::ShroudLayerContainer;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum RotationEdgecase {
    NegativeWidth,
    NegativeHeight,
    NegativeWidthAndHeight,
}

impl From<&ShroudLayerContainer> for Option<RotationEdgecase> {
    fn from(shroud_layer_container: &ShroudLayerContainer) -> Self {
        RotationEdgecase::new(
            &shroud_layer_container.shape_id,
            shroud_layer_container
                .shroud_layer
                .size
                .as_ref()
                .unwrap()
                .x
                .to_f32(),
            shroud_layer_container
                .shroud_layer
                .size
                .as_ref()
                .unwrap()
                .y
                .to_f32(),
        )
    }
}

impl RotationEdgecase {
    fn new(shape_id: &str, width: f32, height: f32) -> Option<RotationEdgecase> {
        if shape_id == "SQUARE" {
            return None;
        }
        let negative_width = width < 0.0;
        let negative_height = height < 0.0;
        match (negative_width, negative_height) {
            (true, true) => Some(RotationEdgecase::NegativeWidthAndHeight),
            (true, false) => Some(RotationEdgecase::NegativeWidth),
            (false, true) => Some(RotationEdgecase::NegativeHeight),
            _ => None,
        }
    }
}

pub fn rotation_edgecase_logic_radians(
    rotation_edgecase_option: Option<RotationEdgecase>,
    radians: f32,
) -> f32 {
    let Some(rotation_edgecase) = rotation_edgecase_option else {
        return radians;
    };
    use RotationEdgecase;
    match rotation_edgecase {
        RotationEdgecase::NegativeWidth => PI - radians,
        RotationEdgecase::NegativeHeight => -radians,
        RotationEdgecase::NegativeWidthAndHeight => PI + radians,
    }
}

pub fn rotation_edgecase_logic_degrees(
    rotation_edgecase_option: Option<RotationEdgecase>,
    degrees: f32,
) -> f32 {
    let Some(rotation_edgecase) = rotation_edgecase_option else {
        return degrees;
    };
    match rotation_edgecase {
        RotationEdgecase::NegativeWidth => (180.0 - degrees).rem_euclid(360.0),
        RotationEdgecase::NegativeHeight => (360.0 - degrees).rem_euclid(360.0),
        RotationEdgecase::NegativeWidthAndHeight => (180.0 + degrees).rem_euclid(360.0),
    }
}
