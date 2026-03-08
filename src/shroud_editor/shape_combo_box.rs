use egui::{Popup, PopupCloseBehavior, ScrollArea, TextEdit, Ui};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    shape_container::ShapeContainer, shroud_editor::shroud_settings::ShroudLayerSettingsTarget,
};

pub fn shroud_layer_shape_combo_box(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    shape_search_buf: &mut String,
    shape_search_show_vanilla: &mut bool,
    loaded_shapes: &Vec<ShapeContainer>,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
    add_undo_history: &mut bool,
    visual_panel_key_bindings_enabled: &mut bool,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        Popup::from_toggle_button_response(
            &ui.button(shroud_layer_settings_target.get_shape_id_str()),
        )
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            *visual_panel_key_bindings_enabled = false;
            let search = ui.add(
                TextEdit::singleline(shape_search_buf)
                    .code_editor()
                    .desired_width(120.0)
                    .hint_text("Search (:"),
            );
            search.request_focus();
            ui.horizontal(|ui| {
                ui.label("Show Vanilla:");
                ui.checkbox(shape_search_show_vanilla, "");
            });
            ScrollArea::vertical()
                .min_scrolled_height(500.0)
                .max_height(500.0)
                .min_scrolled_width(250.0)
                .max_width(250.0)
                .show(ui, |ui| {
                    for selectable_shape in if *shape_search_show_vanilla {
                        &loaded_shapes
                    } else {
                        &loaded_shapes[VANILLA_SHAPE_COUNT..]
                    } {
                        let selectable_shape_id = selectable_shape.s.get_id().unwrap().to_string();
                        if shape_search_buf.is_empty()
                            || selectable_shape_id
                                .to_lowercase()
                                .contains(&shape_search_buf.to_lowercase())
                        {
                            let response = ui.selectable_value(
                                shroud_layer_settings_target.get_shape_id_mut(),
                                selectable_shape_id.clone(),
                                selectable_shape_id,
                            );
                            if response.clicked() {
                                *add_undo_history = true;
                                shroud_layer_settings_target.on_shape_changed(
                                    selectable_shape,
                                    loaded_shapes,
                                    loaded_shapes_mirror_pairs,
                                );
                            }
                        }
                    }
                });
        });
    });
}
