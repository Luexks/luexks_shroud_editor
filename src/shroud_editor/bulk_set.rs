use egui::{Color32, Stroke, Ui};
use luexks_reassembly::blocks::shroud_layer::{ShroudLayer, ShroudLayerColor};

use crate::{
    restructure_vertices::restructure_vertices,
    right_tri_angle_edge_case::{RIGHT_TRI, rotate_right_tri_shroud_layer_mirror},
    shape_container::ShapeContainer,
    shroud_editor::{
        ShroudEditor,
        add_mirror::get_mirrored_shape_data,
        shape_combo_box::shroud_layer_shape_combo_box,
        shroud_settings::{
            ShroudLayerSettingsTarget, colour_settings, full_angle_settings, offset_settings,
            size_settings, taper_setting,
        },
    },
    shroud_layer_container::ShroudLayerContainer,
    styles::SHROUD_LAYER_SETTINGS_COLOUR,
};

struct BulkSettingsTarget<'a> {
    shroud: &'a mut Vec<ShroudLayerContainer>,
    bulk_layer: &'a mut ShroudLayer,
    bulk_shape_id: &'a mut String,
    whole_selection: Vec<usize>,
    selection: Vec<usize>,
    mirrors: Vec<usize>,
}

impl ShroudLayerSettingsTarget for BulkSettingsTarget<'_> {
    fn get_main_layer(&self) -> &ShroudLayer {
        self.bulk_layer
    }

    fn get_main_layer_mut(&mut self) -> &mut ShroudLayer {
        self.bulk_layer
    }

    fn egui_id(&self) -> String {
        "bulk".to_string()
    }

    fn on_shape_changed(
        &mut self,
        shape: &ShapeContainer,
        loaded_shapes: &[ShapeContainer],
        loaded_shapes_mirror_pairs: &[(usize, usize)],
    ) {
        self.selection.clone().into_iter().for_each(|idx| {
            self.shroud[idx].shape_id = shape.s.get_id().unwrap().to_string();
            self.shroud[idx].vertices = restructure_vertices(shape.s.get_first_scale_vertices());
            self.shroud[idx].invert_height_of_mirror = shape.invert_height_of_mirror;
            self.shroud[idx].shroud_layer.shape = shape.s.get_id();
            if let Some(mirror_idx) = self.shroud[idx].mirror_index_option {
                self.shroud[mirror_idx].invert_height_of_mirror = shape.invert_height_of_mirror;
                if shape.invert_height_of_mirror {
                    *self.shroud[mirror_idx]
                        .shroud_layer
                        .size
                        .as_mut()
                        .unwrap()
                        .y
                        .to_f32_mut() = -self.shroud[idx]
                        .shroud_layer
                        .size
                        .as_ref()
                        .unwrap()
                        .y
                        .to_f32();
                    *self.shroud[mirror_idx]
                        .shroud_layer
                        .angle
                        .as_mut()
                        .unwrap()
                        .as_degrees_mut()
                        .get_value_mut() = self.shroud[idx]
                        .shroud_layer
                        .angle
                        .as_ref()
                        .unwrap()
                        .as_degrees()
                        .get_value();
                } else {
                    *self.shroud[mirror_idx]
                        .shroud_layer
                        .size
                        .as_mut()
                        .unwrap()
                        .y
                        .to_f32_mut() = self.shroud[idx]
                        .shroud_layer
                        .size
                        .as_ref()
                        .unwrap()
                        .y
                        .to_f32();
                    *self.shroud[mirror_idx]
                        .shroud_layer
                        .angle
                        .as_mut()
                        .unwrap()
                        .as_degrees_mut()
                        .get_value_mut() = -self.shroud[idx]
                        .shroud_layer
                        .angle
                        .as_ref()
                        .unwrap()
                        .as_degrees()
                        .get_value();
                }
                self.shroud[mirror_idx].shape_id = self.get_shape_id_string();
                let (shape, shape_id, vertices) = get_mirrored_shape_data(
                    self.shroud,
                    idx,
                    loaded_shapes,
                    loaded_shapes_mirror_pairs,
                );
                self.shroud[mirror_idx].vertices = vertices;
                self.shroud[mirror_idx].shroud_layer.shape = Some(shape);
                if shape_id == RIGHT_TRI {
                    rotate_right_tri_shroud_layer_mirror(&mut self.shroud[mirror_idx]);
                }
                self.shroud[mirror_idx].shape_id = shape_id;
            }
        });
    }

    fn on_x_changed(&mut self, x: f32) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .x
                .to_f32_mut() = x;
        });
    }

    fn on_y_changed(&mut self, y: f32) {
        self.selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = y;
        });
        self.mirrors.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = -y;
        });
    }

    fn on_z_changed(&mut self, z: f32) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .z
                .to_f32_mut() = z;
        });
    }

    fn on_width_changed(&mut self, width: f32) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .size
                .as_mut()
                .unwrap()
                .x
                .to_f32_mut() = width;
        });
    }

    fn on_height_changed(&mut self, height: f32) {
        self.selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .size
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = height;
        });
        self.mirrors.iter().for_each(|idx| {
            let mut height = height;
            if self.shroud[*idx].invert_height_of_mirror {
                height *= -1.0;
            }
            *self.shroud[*idx]
                .shroud_layer
                .size
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = height;
        });
    }

    fn on_angle_changed(&mut self, angle: f32) {
        self.selection.iter().for_each(|idx| {
            *self.shroud[*idx]
                .shroud_layer
                .angle
                .as_mut()
                .unwrap()
                .get_value_mut() = angle;
        });
        self.mirrors.iter().for_each(|idx| {
            let mut angle = angle;
            if self.shroud[*idx].invert_height_of_mirror {
                angle *= -1.0;
            }
            *self.shroud[*idx]
                .shroud_layer
                .angle
                .as_mut()
                .unwrap()
                .get_value_mut() = -angle;
            if *self.shroud[*idx].shape_id == *RIGHT_TRI {
                rotate_right_tri_shroud_layer_mirror(&mut self.shroud[*idx]);
            }
        });
    }

    fn on_color_1_changed(&mut self, color_1: ShroudLayerColor) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx].shroud_layer.color_1.as_mut().unwrap() = color_1;
        });
    }

    fn on_color_2_changed(&mut self, color_2: ShroudLayerColor) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx].shroud_layer.color_2.as_mut().unwrap() = color_2;
        });
    }

    fn on_line_color_changed(&mut self, line_color: ShroudLayerColor) {
        self.whole_selection.iter().for_each(|idx| {
            *self.shroud[*idx].shroud_layer.line_color.as_mut().unwrap() = line_color;
        });
    }

    fn on_taper_changed(&mut self, taper: f32) {
        self.whole_selection.iter().for_each(|idx| {
            if self.shroud[*idx].shape_id == "SQUARE" {
                *self.shroud[*idx].shroud_layer.taper.as_mut().unwrap() = taper;
            }
        });
    }

    fn get_shape_id_string(&self) -> String {
        self.bulk_shape_id.clone()
    }

    fn get_shape_id_str(&self) -> &str {
        self.bulk_shape_id
    }

    fn get_shape_id_mut(&mut self) -> &mut String {
        self.bulk_shape_id
    }

    fn set_shape_id(&mut self, shape_id: String) {
        *self.bulk_shape_id = shape_id;
    }
}

impl ShroudEditor {
    pub fn bulk_set(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Apply to entire selection:");
            egui::Frame::new()
                .fill(SHROUD_LAYER_SETTINGS_COLOUR)
                .inner_margin(6.0)
                .corner_radius(4.0)
                .stroke(Stroke::new(1.0, Color32::BLUE))
                .show(ui, |ui| {
                    self.bulk_set_body(ui);
                });
        });
    }

    fn bulk_set_body(&mut self, ui: &mut Ui) {
        let xy_speed = self.get_xy_speed();
        let (whole_selection, selection, mirrors) = self.get_selection_mirror_split();
        let add_undo_history = &mut self.add_undo_history;
        let bulk_settings_target = &mut BulkSettingsTarget {
            shroud: &mut self.shroud,
            bulk_layer: &mut self.tool_settings.bulk_layer,
            bulk_shape_id: &mut self.tool_settings.bulk_shape_id,
            whole_selection,
            selection,
            mirrors,
        };
        ui.spacing_mut().item_spacing.y = 2.0;
        shroud_layer_shape_combo_box(
            ui,
            bulk_settings_target,
            &mut self.shape_search_buf,
            &mut self.shape_search_show_vanilla,
            &self.loaded_shapes,
            &self.loaded_shapes_mirror_pairs,
            add_undo_history,
            &mut self.visual_panel_key_bindings_enabled,
        );
        offset_settings(ui, bulk_settings_target, add_undo_history, xy_speed);
        size_settings(ui, bulk_settings_target, add_undo_history, xy_speed);
        full_angle_settings(
            ui,
            bulk_settings_target,
            add_undo_history,
            self.angle_snap,
            self.angle_snap_enabled,
            None,
        );
        colour_settings(ui, bulk_settings_target, add_undo_history);
        taper_setting(ui, bulk_settings_target, add_undo_history);
    }
}
