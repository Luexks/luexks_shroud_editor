use egui::{ComboBox, Pos2, Ui};
use luexks_reassembly::shapes::{shape_id::ShapeId, shapes::Shapes};

use crate::{restructure_vertices::restructure_vertices, shroud_layer_container::ShroudLayerContainer};

pub fn shroud_layer_shape_combo_box(
    ui: &mut Ui,
    shape_id: &str,
    shroud: &mut Vec<ShroudLayerContainer>,
    index: usize,
    // shape: &mut Option<ShapeId>,
    // shape_id: &mut String,
    // vertices: &mut Vec<Pos2>,
    loaded_shapes: &Shapes,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        ComboBox::from_id_salt(shape_id.to_string())
            .selected_text(shroud[index].shape_id.as_str())
            .show_ui(ui, |ui| {
                for selectable_shape in &loaded_shapes.0 {
                    let selectable_shape_id = selectable_shape.get_id().unwrap().to_string();
                    let response = ui.selectable_value(
                        &mut shroud[index].shape_id,
                        selectable_shape_id.clone(),
                        &selectable_shape_id,
                    );
                    if response.clicked() {
                        shroud[index].vertices =
                            restructure_vertices(selectable_shape.get_first_scale_vertices());
                        shroud[index].shroud_layer.shape = selectable_shape.get_id();
                        if let Some(mirror_index) = shroud[index].mirror_index_option {
                            shroud[mirror_index].vertices = shroud[index].vertices.clone();
                            shroud[mirror_index].shroud_layer.shape = shroud[index].shroud_layer.shape.clone();

                            shroud[mirror_index].shape_id = selectable_shape_id;
                        }
                    }
                }
            });
    });
}
