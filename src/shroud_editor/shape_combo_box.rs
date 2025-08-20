use egui::{ComboBox, Pos2, Ui};
use luexks_reassembly::shapes::{shape_id::ShapeId, shapes::Shapes};

use crate::restructure_vertices::restructure_vertices;

pub fn shape_combo_box(
    ui: &mut Ui,
    index: &str,
    shape: &mut Option<ShapeId>,
    shape_id: &mut String,
    vertices: &mut Vec<Pos2>,
    loaded_shapes: &Shapes,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        ComboBox::from_id_salt(index.to_string())
            .selected_text(shape_id.as_str())
            .show_ui(ui, |ui| {
                for selectable_shape in &loaded_shapes.0 {
                    let selectable_shape_id = selectable_shape.get_id().unwrap().to_string();
                    let response = ui.selectable_value(
                        shape_id,
                        selectable_shape_id.clone(),
                        selectable_shape_id,
                    );
                    if response.clicked() {
                        *vertices =
                            restructure_vertices(selectable_shape.get_first_scale_vertices());
                        *shape = selectable_shape.get_id();
                    }
                }
            });
    });
}
