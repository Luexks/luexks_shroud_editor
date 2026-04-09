use egui::{
    LayerId, Popup, PopupAnchor, PopupCloseBehavior, Response, ScrollArea, TextEdit, Ui, Vec2,
};
use luexks_reassembly::{
    blocks::shroud_layer::{ShroudLayer, ShroudLayerColor},
    utility::angle::Angle,
};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    invert_y::invert_y_of_pos2,
    pos_and_display_oriented_number_conversion::{pos2_to_do2d, pos2_to_do3d},
    restructure_vertices::restructure_vertices,
    shroud_editor::{ShroudEditor, tools::get_scaled_default_proportion_size},
    shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection, ShroudInteraction},
    shroud_layer_container::ShroudLayerContainer,
};

impl ShroudEditor {
    pub fn right_click_shroud_add(&mut self, ui: &mut Ui, response: &Response) {
        if let ShroudInteraction::Inaction { .. } = self.shroud_interaction {
            if response.secondary_clicked() {
                self.show_right_click_shroud_add = true;
                if let Some(pos) = response.interact_pointer_pos() {
                    self.right_click_shroud_screen_pos = pos;
                }
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: Vec::new(),
                };
            }
            if self.show_right_click_shroud_add {
                if response.clicked() {
                    self.show_right_click_shroud_add = false;
                }
                Popup::new(
                    "right_click_add".into(),
                    ui.ctx().to_owned(),
                    PopupAnchor::Position(self.right_click_shroud_screen_pos),
                    LayerId::new(egui::Order::Foreground, "right_click_add_layer_id".into()),
                )
                // Popup::menu(response)
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
                            self.right_click_shroud_add_body(ui);
                        });
                });
            }
        } else {
            self.show_right_click_shroud_add = false;
        }
    }

    fn right_click_shroud_add_body(&mut self, ui: &mut Ui) {
        for selectable_shape in if self.shape_search_show_vanilla {
            &self.loaded_shapes
        } else {
            &self.loaded_shapes[VANILLA_SHAPE_COUNT..]
        } {
            let selectable_shape_id = selectable_shape.s.get_id().unwrap().to_string();
            if self.shape_search_buf.is_empty()
                || selectable_shape_id
                    .to_lowercase()
                    .contains(&self.shape_search_buf.to_lowercase())
            {
                let response = ui.selectable_value(
                    &mut "",
                    &selectable_shape_id.clone(),
                    &selectable_shape_id,
                );
                if response.clicked() {
                    let world_mouse_pos_inverted_y = invert_y_of_pos2(self.world_mouse_pos);
                    self.add_undo_history = true;
                    let offset = pos2_to_do3d(&self.world_mouse_pos, 0.01);
                    let verts = restructure_vertices(selectable_shape.s.get_first_scale_vertices());
                    let mut size = get_scaled_default_proportion_size(&verts, 1.);
                    if &selectable_shape_id == "SQUARE" {
                        size.y /= 2.;
                    }
                    let size = pos2_to_do2d(&size);
                    let shroud = ShroudLayerContainer {
                        shroud_layer: ShroudLayer {
                            shape: selectable_shape.s.get_id(),
                            size: Some(size),
                            offset: Some(offset),
                            color_1: Some(ShroudLayerColor::Color1),
                            color_2: Some(ShroudLayerColor::Color2),
                            line_color: Some(ShroudLayerColor::LineColor),
                            angle: Some(Angle::Degree(0.)),
                            taper: Some(1.),
                        },
                        vertices: restructure_vertices(
                            selectable_shape.s.get_first_scale_vertices(),
                        ),
                        shape_id: selectable_shape_id,
                        delete_next_frame: false,
                        mirror_index_option: None,
                        group_idx_option: None,
                        invert_height_of_mirror: selectable_shape.invert_height_of_mirror,
                    };
                    let idx = self.shroud.len();
                    self.shroud.push(shroud);
                    self.shroud_interaction = ShroudInteraction::Placing {
                        drag_pos: world_mouse_pos_inverted_y,
                        potentially_snapped_drag_pos: world_mouse_pos_inverted_y,
                        selection: MovingShroudSelection(Vec::from([
                            MovingShroudLayerInteraction {
                                idx,
                                relative_pos: Vec2::ZERO,
                            },
                        ])),
                    };
                    self.show_right_click_shroud_add = false;
                }
            }
        }
    }
}

// pub fn shroud_layer_shape_combo_box(
//     ui: &mut Ui,
//     shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
//     shape_search_buf: &mut String,
//     shape_search_show_vanilla: &mut bool,
//     loaded_shapes: &Vec<ShapeContainer>,
//     loaded_shapes_mirror_pairs: &[(usize, usize)],
//     add_undo_history: &mut bool,
//     visual_panel_key_bindings_enabled: &mut bool,
// ) {
//     ui.horizontal(|ui| {
//         ui.label("shape=");
//         Popup::from_toggle_button_response(
//             &ui.button(shroud_layer_settings_target.get_shape_id_str()),
//         )
//         .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
//         .show(|ui| {
//         });
//     });
// }
