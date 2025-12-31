use egui::{Popup, PopupCloseBehavior, ScrollArea, TextEdit, Ui};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    restructure_vertices::restructure_vertices,
    shroud_editor::{ShroudEditor, add_mirror::get_mirrored_shape_data},
};

impl ShroudEditor {
    pub fn shroud_layer_shape_combo_box(&mut self, ui: &mut Ui, index: usize) {
        ui.horizontal(|ui| {
            ui.label("shape=");
            Popup::from_toggle_button_response(&ui.button(&self.shroud[index].shape_id))
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .show(|ui| {
                    self.visual_panel_key_bindings_enabled = false;
                    let search = ui.add(
                        TextEdit::singleline(&mut self.shape_search_buf)
                            .code_editor()
                            .desired_width(120.0)
                            .hint_text("Search (:"),
                    );
                    search.request_focus();
                    ui.horizontal(|ui| {
                        ui.label("Show Vanilla:");
                        ui.checkbox(&mut self.shape_search_show_vanilla, "");
                    });
                    ScrollArea::vertical()
                        .min_scrolled_height(500.0)
                        .max_height(500.0)
                        .min_scrolled_width(250.0)
                        .max_width(250.0)
                        .show(ui, |ui| {
                            for selectable_shape in if self.shape_search_show_vanilla {
                                &self.loaded_shapes.0
                            } else {
                                &self.loaded_shapes.0[VANILLA_SHAPE_COUNT..]
                            } {
                                let selectable_shape_id =
                                    selectable_shape.get_id().unwrap().to_string();
                                if self.shape_search_buf.is_empty()
                                    || selectable_shape_id
                                        .to_lowercase()
                                        .contains(&self.shape_search_buf.to_lowercase())
                                {
                                    let response = ui.selectable_value(
                                        &mut self.shroud[index].shape_id,
                                        selectable_shape_id.clone(),
                                        selectable_shape_id,
                                    );
                                    if response.clicked() {
                                        self.add_undo_history = true;
                                        self.shroud[index].vertices = restructure_vertices(
                                            selectable_shape.get_first_scale_vertices(),
                                        );
                                        self.shroud[index].shroud_layer.shape =
                                            selectable_shape.get_id();
                                        if let Some(mirror_index) =
                                            self.shroud[index].mirror_index_option
                                        {
                                            let (shape, shape_id, vertices) =
                                                get_mirrored_shape_data(
                                                    &self.shroud,
                                                    index,
                                                    &self.loaded_shapes,
                                                    &self.loaded_shapes_mirror_pairs,
                                                );
                                            self.shroud[mirror_index].vertices = vertices;
                                            self.shroud[mirror_index].shroud_layer.shape =
                                                Some(shape);
                                            self.shroud[mirror_index].shape_id = shape_id;
                                        }
                                    }
                                }
                            }
                        });
                });
        });
    }
}
