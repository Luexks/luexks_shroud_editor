use egui::{Color32, DragValue, Grid, Stroke, Ui};
use egui_knob::{Knob, KnobStyle};
use luexks_reassembly::blocks::shroud_layer::{ShroudLayer, ShroudLayerColor};

use crate::{
    restructure_vertices::restructure_vertices,
    right_tri_angle_edge_case::{RIGHT_TRI, rotate_right_tri_shroud_layer_mirror},
    rotation_edgecase::{RotationEdgecase, rotation_edgecase_logic_degrees},
    shape_container::ShapeContainer,
    shroud_editor::{
        ShroudEditor,
        add_mirror::{add_mirror, get_mirrored_shape_data},
        shape_combo_box::shroud_layer_shape_combo_box,
    },
    shroud_interaction::ShroudInteraction,
    shroud_layer_container::ShroudLayerContainer,
    styles::SHROUD_LAYER_SETTINGS_COLOUR,
};

pub trait ShroudLayerSettingsTarget {
    fn get_main_layer(&self) -> &ShroudLayer;
    fn get_main_layer_mut(&mut self) -> &mut ShroudLayer;
    fn egui_id(&self) -> String;
    fn on_shape_changed(
        &mut self,
        shape: &ShapeContainer,
        loaded_shapes: &[ShapeContainer],
        loaded_shapes_mirror_pairs: &[(usize, usize)],
    );
    fn on_x_changed(&mut self, x: f32);
    fn on_y_changed(&mut self, y: f32);
    fn on_z_changed(&mut self, z: f32);
    fn on_width_changed(&mut self, width: f32);
    fn on_height_changed(&mut self, height: f32);
    fn on_angle_changed(&mut self, angle: f32);
    fn on_color_1_changed(&mut self, color_1: ShroudLayerColor);
    fn on_color_2_changed(&mut self, color_2: ShroudLayerColor);
    fn on_line_color_changed(&mut self, line_color: ShroudLayerColor);
    fn on_taper_changed(&mut self, taper: f32);
    fn get_shape_id_string(&self) -> String;
    fn get_shape_id_str(&self) -> &str;
    fn get_shape_id_mut(&mut self) -> &mut String;
    fn set_shape_id(&mut self, shape_id: String);
}

pub struct SingleSettingsTarget<'a> {
    pub shroud: &'a mut Vec<ShroudLayerContainer>,
    pub idx: usize,
}

impl ShroudLayerSettingsTarget for SingleSettingsTarget<'_> {
    fn get_main_layer(&self) -> &ShroudLayer {
        &self.shroud[self.idx].shroud_layer
    }
    fn get_main_layer_mut(&mut self) -> &mut ShroudLayer {
        &mut self.shroud[self.idx].shroud_layer
    }
    fn egui_id(&self) -> String {
        self.idx.to_string()
    }
    fn on_shape_changed(
        &mut self,
        shape: &ShapeContainer,
        loaded_shapes: &[ShapeContainer],
        loaded_shapes_mirror_pairs: &[(usize, usize)],
    ) {
        self.shroud[self.idx].vertices = restructure_vertices(shape.s.get_first_scale_vertices());
        self.shroud[self.idx].invert_height_of_mirror = shape.invert_height_of_mirror;
        self.shroud[self.idx].shroud_layer.shape = shape.s.get_id();
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            self.shroud[mirror_idx].invert_height_of_mirror = shape.invert_height_of_mirror;
            if shape.invert_height_of_mirror {
                *self.shroud[mirror_idx]
                    .shroud_layer
                    .size
                    .as_mut()
                    .unwrap()
                    .y
                    .to_f32_mut() = -self.shroud[self.idx]
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
                    .get_value_mut() = self.shroud[self.idx]
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
                    .to_f32_mut() = self.shroud[self.idx]
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
                    .get_value_mut() = -self.shroud[self.idx]
                    .shroud_layer
                    .angle
                    .as_ref()
                    .unwrap()
                    .as_degrees()
                    .get_value();
            }
            let (shape, shape_id, vertices) = get_mirrored_shape_data(
                self.shroud,
                self.idx,
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
    }
    fn on_x_changed(&mut self, x: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .x
                .to_f32_mut() = x;
        }
    }
    fn on_y_changed(&mut self, y: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = -y;
        }
    }
    fn on_z_changed(&mut self, z: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .offset
                .as_mut()
                .unwrap()
                .z
                .to_f32_mut() = z;
        }
    }
    fn on_width_changed(&mut self, width: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .size
                .as_mut()
                .unwrap()
                .x
                .to_f32_mut() = width;
        }
    }
    fn on_height_changed(&mut self, mut height: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            if self.shroud[mirror_idx].invert_height_of_mirror {
                height *= -1.0;
            }
            *self.shroud[mirror_idx]
                .shroud_layer
                .size
                .as_mut()
                .unwrap()
                .y
                .to_f32_mut() = height;
        }
    }
    fn on_angle_changed(&mut self, mut angle: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            if self.shroud[mirror_idx].invert_height_of_mirror {
                angle *= -1.0;
            }
            *self.shroud[mirror_idx]
                .shroud_layer
                .angle
                .as_mut()
                .unwrap()
                .get_value_mut() = -angle;
            if *self.shroud[self.idx].shape_id == *RIGHT_TRI {
                rotate_right_tri_shroud_layer_mirror(&mut self.shroud[mirror_idx]);
            }
        }
    }
    fn on_color_1_changed(&mut self, color_1: ShroudLayerColor) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .color_1
                .as_mut()
                .unwrap() = color_1;
        }
    }
    fn on_color_2_changed(&mut self, color_2: ShroudLayerColor) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .color_2
                .as_mut()
                .unwrap() = color_2;
        }
    }
    fn on_line_color_changed(&mut self, line_color: ShroudLayerColor) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx]
                .shroud_layer
                .line_color
                .as_mut()
                .unwrap() = line_color;
        }
    }
    fn on_taper_changed(&mut self, taper: f32) {
        if let Some(mirror_idx) = self.shroud[self.idx].mirror_index_option {
            *self.shroud[mirror_idx].shroud_layer.taper.as_mut().unwrap() = taper;
        }
    }
    fn get_shape_id_mut(&mut self) -> &mut String {
        &mut self.shroud[self.idx].shape_id
    }
    fn get_shape_id_str(&self) -> &str {
        &self.shroud[self.idx].shape_id
    }
    fn get_shape_id_string(&self) -> String {
        self.shroud[self.idx].shape_id.clone()
    }
    fn set_shape_id(&mut self, shape_id: String) {
        self.shroud[self.idx].shape_id = shape_id;
    }
}

pub fn offset_settings(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    add_undo_history: &mut bool,
    xy_speed: f32,
) {
    ui.horizontal(|ui| {
        let x = shroud_layer_settings_target
            .get_main_layer_mut()
            .offset
            .as_mut()
            .unwrap()
            .x
            .to_f32_mut();
        ui.label("offset={");
        let response = ui.add(DragValue::new(x).speed(xy_speed));
        let x = *x;
        if response.changed() {
            shroud_layer_settings_target.on_x_changed(x);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
        ui.label(",");
        let y = shroud_layer_settings_target
            .get_main_layer_mut()
            .offset
            .as_mut()
            .unwrap()
            .y
            .to_f32_mut();
        let response = ui.add(DragValue::new(y).speed(xy_speed));
        let y = *y;
        if response.changed() {
            shroud_layer_settings_target.on_y_changed(y);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
        ui.label(",");
        let z = shroud_layer_settings_target
            .get_main_layer_mut()
            .offset
            .as_mut()
            .unwrap()
            .z
            .to_f32_mut();
        let response = ui.add(DragValue::new(z).speed(0.005));
        let z = *z;
        if response.changed() {
            shroud_layer_settings_target.on_z_changed(z);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
        ui.label("}");
    });
}

pub fn size_settings(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    add_undo_history: &mut bool,
    xy_speed: f32,
) {
    ui.horizontal(|ui| {
        let width = shroud_layer_settings_target
            .get_main_layer_mut()
            .size
            .as_mut()
            .unwrap()
            .x
            .to_f32_mut();
        ui.label("size={");
        let response = ui.add(DragValue::new(width).speed(xy_speed));
        let width = *width;
        if response.changed() {
            shroud_layer_settings_target.on_width_changed(width);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
        ui.label(",");
        let height = shroud_layer_settings_target
            .get_main_layer_mut()
            .size
            .as_mut()
            .unwrap()
            .y
            .to_f32_mut();
        let response = ui.add(DragValue::new(height).speed(xy_speed));
        let height = *height;
        if response.changed() {
            shroud_layer_settings_target.on_height_changed(height);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
        ui.label("}");
    });
}

pub fn colour_settings(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    add_undo_history: &mut bool,
) {
    Grid::new(shroud_layer_settings_target.egui_id()).show(ui, |ui| {
        let color_1 = shroud_layer_settings_target
            .get_main_layer_mut()
            .color_1
            .as_mut()
            .unwrap();
        if shroud_color_setting_and_if_changed(ui, color_1, "tri_color_id=", add_undo_history) {
            let color_1 = *color_1;
            shroud_layer_settings_target.on_color_1_changed(color_1);
        }
        let color_2 = shroud_layer_settings_target
            .get_main_layer_mut()
            .color_2
            .as_mut()
            .unwrap();
        if shroud_color_setting_and_if_changed(ui, color_2, "tri_color1_id=", add_undo_history) {
            let color_2 = *color_2;
            shroud_layer_settings_target.on_color_2_changed(color_2);
        }
        let line_color = shroud_layer_settings_target
            .get_main_layer_mut()
            .line_color
            .as_mut()
            .unwrap();
        if shroud_color_setting_and_if_changed(ui, line_color, "line_color_id=", add_undo_history) {
            let line_color = *line_color;
            shroud_layer_settings_target.on_line_color_changed(line_color);
        }
    });
}

pub fn shroud_color_setting_and_if_changed(
    ui: &mut Ui,
    color: &mut ShroudLayerColor,
    text: &str,
    add_undo_history: &mut bool,
) -> bool {
    ui.label(text);
    let changed = ui
        .selectable_value(color, ShroudLayerColor::Color1, "0")
        .clicked()
        || ui
            .selectable_value(color, ShroudLayerColor::Color2, "1")
            .clicked()
        || ui
            .selectable_value(color, ShroudLayerColor::LineColor, "2")
            .clicked();
    if changed {
        *add_undo_history = true;
    }
    ui.end_row();
    changed
}

pub fn taper_setting(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    add_undo_history: &mut bool,
) {
    ui.horizontal(|ui| {
        let taper = shroud_layer_settings_target
            .get_main_layer_mut()
            .taper
            .as_mut()
            .unwrap();
        ui.label("taper=");
        let response = ui.add(DragValue::new(taper).speed(0.025));
        if response.changed() {
            let taper = *taper;
            shroud_layer_settings_target.on_taper_changed(taper);
        }
        if response.drag_stopped() || response.lost_focus() {
            *add_undo_history = true;
        }
    });
}

pub fn full_angle_settings(
    ui: &mut Ui,
    shroud_layer_settings_target: &mut impl ShroudLayerSettingsTarget,
    add_undo_history: &mut bool,
    angle_snap: f32,
    angle_snap_enabled: bool,
    rotation_edgecase_option: Option<RotationEdgecase>,
) {
    let angle_speed = if angle_snap_enabled { angle_snap } else { 1.0 };
    ui.horizontal(|ui| {
        let angle = shroud_layer_settings_target
            .get_main_layer_mut()
            .angle
            .as_mut()
            .unwrap()
            .as_degrees_mut()
            .get_value_mut();
        ui.label("angle=");
        let response = ui.add(DragValue::new(angle).speed(angle_speed));
        ui.label("*pi/180");
        *angle = rotation_edgecase_logic_degrees(rotation_edgecase_option, *angle);
        let (knob_changed, knob_drag_stopped) =
            angle_knob_settings(ui, angle, angle_snap, angle_snap_enabled);
        *angle = rotation_edgecase_logic_degrees(rotation_edgecase_option, *angle);
        if response.changed() || knob_changed {
            let angle = *angle;
            shroud_layer_settings_target.on_angle_changed(angle);
        }
        if response.drag_stopped() || response.lost_focus() || knob_drag_stopped {
            *add_undo_history = true;
        }
    });
}

impl ShroudEditor {
    pub fn shroud_list(&mut self, ui: &mut Ui) {
        if self.shroud.is_empty() {
            ui.label("No shrouds :(");
        } else {
            // let start = ui.cursor().min.y;
            // println!("{}", start);
            // (0..self.shroud.len()).for_each(|index| {
            let are_multiple_selected = self.shroud_interaction.selection_len() >= 2;
            for index in 0..self.shroud.len() {
                let is_selected = self
                    .shroud_interaction
                    .is_shroud_layer_index_selected(index);
                if !self.only_show_selected_shroud_layers || is_selected {
                    self.shroud_layer_settings(is_selected, ui, index, are_multiple_selected);
                }
                // });
                // let top_of_shroud_layer_settings_y = ui.cursor().min.y;
                // let window_bottom_y = ui.clip_rect().max.y;
                // if top_of_shroud_layer_settings_y > window_bottom_y {
                //     break;
                // }
            }
        }

        ui.add_space(4.0);
    }

    fn shroud_layer_settings(
        &mut self,
        is_selected: bool,
        ui: &mut Ui,
        index: usize,
        are_multiple_selected: bool,
    ) {
        // let start_y = ui.cursor().min.y;
        ui.vertical(|ui| {
            egui::Frame::new()
                .fill(SHROUD_LAYER_SETTINGS_COLOUR)
                .inner_margin(6.0)
                .corner_radius(4.0)
                .stroke(Stroke::new(
                    1.0,
                    if is_selected {
                        Color32::BLACK
                    } else {
                        Color32::TRANSPARENT
                    },
                ))
                .show(ui, |ui| {
                    self.shroud_layer_settings_body(is_selected, ui, index, are_multiple_selected);
                });
        });
    }

    fn shroud_layer_settings_body(
        &mut self,
        is_selected: bool,
        ui: &mut Ui,
        idx: usize,
        are_multiple_selected: bool,
    ) {
        // let response = ui.label("h");
        // let visible_rect = ui.clip_rect();
        // println!("{}\t{}", response.rect.min.y, visible_rect.min.y);
        // let start_y = ui.cursor().min.y;
        let xy_speed = self.get_xy_speed();
        let rotation_edgecase_option = Into::<Option<RotationEdgecase>>::into(&self.shroud[idx]);
        ui.spacing_mut().item_spacing.y = 2.0;
        let mut shroud_layer_settings_height = 187.0;
        if self.shroud[idx].shape_id == "SQUARE" {
            shroud_layer_settings_height += 20.0;
        }
        if self.shroud[idx].group_idx_option.is_some() {
            shroud_layer_settings_height += 20.0;
        }
        let window_bottom_y = ui.clip_rect().max.y;
        let top_of_shroud_layer_settings_y = ui.cursor().min.y;
        let shroud_layer_settings_are_off_screen =
            top_of_shroud_layer_settings_y + shroud_layer_settings_height < 0.0
                || top_of_shroud_layer_settings_y > window_bottom_y;
        if shroud_layer_settings_are_off_screen {
            ui.add_space(20.0);
        } else {
            self.select_deselect_and_delete_buttons(ui, idx, is_selected, are_multiple_selected);
        };
        if !(is_selected && self.shroud_interaction.selection_len() == 1) {
            return;
        }
        // println!("{}\t{}", top_of_shroud_layer_settings_y, window_bottom_y);
        if shroud_layer_settings_are_off_screen {
            ui.add_space(shroud_layer_settings_height);
            // println!("Culling shroud layer of ID: {}", index);
            return;
        }
        self.shroud_layer_mirror_settings(ui, idx);
        self.individual_shroud_layer_group_settings(ui, idx);

        let show_taper_setting = self.shroud[idx].shape_id == "SQUARE";

        let add_undo_history = &mut self.add_undo_history;
        let shroud_layer_settings_target = &mut SingleSettingsTarget {
            shroud: &mut self.shroud,
            idx,
        };
        shroud_layer_shape_combo_box(
            ui,
            shroud_layer_settings_target,
            &mut self.shape_search_buf,
            &mut self.shape_search_show_vanilla,
            &self.loaded_shapes,
            &self.loaded_shapes_mirror_pairs,
            add_undo_history,
            &mut self.visual_panel_key_bindings_enabled,
        );

        offset_settings(ui, shroud_layer_settings_target, add_undo_history, xy_speed);
        size_settings(ui, shroud_layer_settings_target, add_undo_history, xy_speed);
        full_angle_settings(
            ui,
            shroud_layer_settings_target,
            add_undo_history,
            self.angle_snap,
            self.angle_snap_enabled,
            rotation_edgecase_option,
        );
        colour_settings(ui, shroud_layer_settings_target, add_undo_history);

        if show_taper_setting {
            taper_setting(ui, shroud_layer_settings_target, add_undo_history);
        }
        // let end_y = ui.cursor().min.y;
        // println!("Height: {}", end_y - start_y);
    }

    fn shroud_layer_mirror_settings(&mut self, ui: &mut Ui, index: usize) {
        ui.horizontal(|ui| {
            if let Some(mirror_index) = self.shroud[index].mirror_index_option {
                if !self.shroud_interaction.selection().contains(&mirror_index) {
                    if ui.button("Select Mirror").clicked() {
                        self.shroud_interaction = ShroudInteraction::Inaction {
                            selection: self
                                .shroud_interaction
                                .selection()
                                .into_iter()
                                .chain(std::iter::once(mirror_index))
                                .collect(),
                        };
                    }
                } else if ui.button("Deselect Mirror").clicked() {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: self
                            .shroud_interaction
                            .selection()
                            .into_iter()
                            .filter(|selection_index| *selection_index != mirror_index)
                            .collect(),
                    };
                }
                if ui.button("Unlink").clicked() {
                    self.shroud[index].mirror_index_option = None;
                    self.shroud[mirror_index].mirror_index_option = None;
                    self.add_undo_history = true;
                }
                if ui.button("Delete Mirror").clicked() {
                    self.shroud[mirror_index].delete_next_frame = true;
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: self
                            .shroud_interaction
                            .selection()
                            .into_iter()
                            .filter(|selection_index| *selection_index != mirror_index)
                            .collect(),
                    };
                    self.add_undo_history = true;
                }
            } else if ui.button("Add Mirror").clicked() {
                add_mirror(
                    &mut self.shroud,
                    index,
                    false,
                    &self.loaded_shapes,
                    &self.loaded_shapes_mirror_pairs,
                );
                self.add_undo_history = true;
            }
        });
    }

    fn select_deselect_and_delete_buttons(
        &mut self,
        ui: &mut Ui,
        index: usize,
        is_selected: bool,
        are_multiple_selected: bool,
    ) {
        ui.horizontal(|ui| {
            ui.label(index.to_string());
            if !is_selected {
                if ui.button("Select").clicked() {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: self
                            .shroud_interaction
                            .selection()
                            .into_iter()
                            .chain(std::iter::once(index))
                            .collect(),
                    };
                }
            } else {
                if ui.button("Deselect").clicked() {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: self
                            .shroud_interaction
                            .selection()
                            .into_iter()
                            .filter(|selection_index| *selection_index != index)
                            .collect(),
                    };
                }
                if are_multiple_selected && ui.button("Solo Select").clicked() {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: [index].into(),
                    };
                }
            }
            if ui.button("Delete (Double Click)").double_clicked() {
                self.add_undo_history = true;
                self.shroud[index].delete_next_frame = true;
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: self
                        .shroud_interaction
                        .selection()
                        .into_iter()
                        .filter(|selection_index| *selection_index != index)
                        .collect(),
                };
            }
        });
    }
}

pub fn angle_knob_settings(
    ui: &mut Ui,
    angle: &mut f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
) -> (bool, bool) {
    let original_knob_angle = 360.0 - *angle + 90.0;
    let mut knob_angle = original_knob_angle;
    let response = ui.add(
        Knob::new(&mut knob_angle, 0.0, 720.0, KnobStyle::Wiper)
            .with_size(20.0)
            .with_sweep_range(0.5, 2.0)
            .with_background_arc(false),
    );
    if angle_snap_enabled && original_knob_angle != knob_angle {
        knob_angle = (knob_angle / angle_snap).round() * angle_snap;
    }
    if knob_angle < 90.0 {
        knob_angle += 360.0;
    }
    *angle = (knob_angle - 90.0) % 360.0;
    if *angle != 0.0 {
        *angle = 360.0 - *angle;
    }
    (response.changed(), response.drag_stopped())
}
