use egui::{Popup, PopupCloseBehavior, ScrollArea, TextEdit, Ui};
use luexks_reassembly::shapes::shapes::Shapes;
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    restructure_vertices::restructure_vertices, shroud_editor::add_mirror::get_mirrored_shape_data,
    shroud_layer_container::ShroudLayerContainer,
};

pub fn shroud_layer_shape_combo_box(
    ui: &mut Ui,
    shroud: &mut [ShroudLayerContainer],
    index: usize,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
    show_vanilla: &mut bool,
    search_buf: &mut String,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        Popup::from_toggle_button_response(&ui.button(&shroud[index].shape_id))
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                let search = ui.add(
                    TextEdit::singleline(search_buf)
                        .code_editor()
                        .desired_width(120.0)
                        .hint_text("Search (:"),
                );
                search.request_focus();
                ui.horizontal(|ui| {
                    ui.label("Show Vanilla:");
                    ui.checkbox(show_vanilla, "");
                });
                ScrollArea::vertical()
                    .min_scrolled_height(500.0)
                    .max_height(500.0)
                    .min_scrolled_width(250.0)
                    .max_width(250.0)
                    .show(ui, |ui| {
                        for selectable_shape in if *show_vanilla {
                            &loaded_shapes.0
                        } else {
                            &loaded_shapes.0[VANILLA_SHAPE_COUNT..]
                        } {
                            let selectable_shape_id =
                                selectable_shape.get_id().unwrap().to_string();
                            if search_buf.is_empty()
                                || selectable_shape_id
                                    .to_lowercase()
                                    .contains(&*search_buf.to_lowercase())
                            {
                                let response = ui.selectable_value(
                                    &mut shroud[index].shape_id,
                                    selectable_shape_id.clone(),
                                    selectable_shape_id,
                                );
                                if response.clicked() {
                                    shroud[index].vertices = restructure_vertices(
                                        selectable_shape.get_first_scale_vertices(),
                                    );
                                    shroud[index].shroud_layer.shape = selectable_shape.get_id();
                                    if let Some(mirror_index) = shroud[index].mirror_index_option {
                                        let (shape, shape_id, vertices) = get_mirrored_shape_data(
                                            shroud,
                                            index,
                                            loaded_shapes,
                                            loaded_shapes_mirror_pairs,
                                        );
                                        shroud[mirror_index].vertices = vertices;
                                        shroud[mirror_index].shroud_layer.shape = Some(shape);
                                        shroud[mirror_index].shape_id = shape_id;
                                    }
                                }
                            }
                        }
                    });
            });
    });
}
