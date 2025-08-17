use egui::Pos2;
use luexks_reassembly::{blocks::shroud_layer::ShroudLayer, shapes::vertices::Vertices, vert};

pub struct ShroudLayerContainer {
    pub shroud_layer: ShroudLayer,
    pub vertices: Vec<Pos2>,
    pub shape_id: String,
}

impl Default for ShroudLayerContainer {
    fn default() -> Self {
        Self {
            shroud_layer: Default::default(),
            vertices: vec![
                Pos2::new(-5.0, -5.0),
                Pos2::new(-5.0, 5.0),
                Pos2::new(5.0, 5.0),
                Pos2::new(5.0, -5.0),
            ],
            shape_id: String::new(),
        }
    }
}
