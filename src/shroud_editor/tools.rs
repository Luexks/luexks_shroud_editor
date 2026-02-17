use std::f32::consts::TAU;

use crate::{
    pos_and_display_oriented_number_conversion::do3d_to_pos2,
    shroud_editor::{ShroudEditor, add_mirror::add_mirror, shroud_settings::angle_knob_settings},
    shroud_interaction::ShroudInteraction,
    shroud_layer_container::ShroudLayerContainer,
};
use egui::{DragValue, Ui, collapsing_header::CollapsingState, pos2};
use itertools::Itertools;
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer,
    utility::{
        angle::Angle,
        display_oriented_math::{do2d_float_from, do3d_float_from},
    },
};

pub struct ToolSettings {
    move_selection_by_distance: f32,
    move_selection_by_angle: f32,
    move_selection_by_x: f32,
    move_selection_by_y: f32,
    move_selection_by_z: f32,
    scale_by_scale_factor: f32,
    scale_by_about_x: f32,
    scale_by_about_y: f32,
    scale_by_2_x_scale_factor: f32,
    scale_by_2_y_scale_factor: f32,
    scale_by_2_about_x: f32,
    scale_by_2_about_y: f32,
    radial_about_x: f32,
    radial_about_y: f32,
    radial_by_count: usize,
    radial_by_angle: f32,
    default_proportions_scale: f32,
}

impl Default for ToolSettings {
    fn default() -> Self {
        ToolSettings {
            move_selection_by_distance: 10.0,
            move_selection_by_angle: 0.0,
            move_selection_by_x: 10.0,
            move_selection_by_y: 10.0,
            move_selection_by_z: 0.0,
            scale_by_scale_factor: 1.0,
            scale_by_about_x: 0.0,
            scale_by_about_y: 0.0,
            scale_by_2_x_scale_factor: 1.0,
            scale_by_2_y_scale_factor: 1.0,
            scale_by_2_about_x: 0.0,
            scale_by_2_about_y: 0.0,
            radial_about_x: 0.0,
            radial_about_y: 0.0,
            radial_by_count: 3,
            radial_by_angle: 0.0,
            default_proportions_scale: 1.0,
        }
    }
}

impl ShroudEditor {
    pub fn tools(&mut self, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ui.ctx(), "tools".into(), false)
            .show_header(ui, |ui| ui.heading("Tools"))
            .body_unindented(|ui| {
                self.move_tool(ui);
                ui.separator();
                self.move_by_x_y_z_tool(ui);
                ui.separator();
                self.scale_by(ui);
                ui.separator();
                self.scale_by_2(ui);
                ui.separator();
                self.radial_tool(ui);
                ui.separator();
                self.default_proportions_tool(ui);
            });
    }

    fn default_proportions_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .button("Set to default proportions with size multiplier")
                .clicked()
            {
                self.add_undo_history = true;
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|shroud_layer_index| {
                        let shroud_layer = &mut self.shroud[*shroud_layer_index];
                        let verts = &shroud_layer.vertices;
                        let (min_x, max_x, min_y, max_y) = verts.iter().fold(
                            (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
                            |(min_x, max_x, min_y, max_y), vert| {
                                (
                                    vert.x.min(min_x),
                                    vert.x.max(max_x),
                                    vert.y.min(min_y),
                                    vert.y.max(max_y),
                                )
                            },
                        );
                        let shape_size = (-min_x + max_x, -min_y + max_y);
                        let scaled_default_proportion_size = do2d_float_from(
                            shape_size.0 * self.tool_settings.default_proportions_scale,
                            shape_size.1 * self.tool_settings.default_proportions_scale,
                        );
                        if let Some(mirror_index) = shroud_layer.mirror_index_option {
                            shroud_layer.shroud_layer.size =
                                Some(scaled_default_proportion_size.clone());
                            self.shroud[mirror_index].shroud_layer.size =
                                Some(scaled_default_proportion_size);
                        } else {
                            shroud_layer.shroud_layer.size = Some(scaled_default_proportion_size);
                        }
                    });
            }
            ui.add(DragValue::new(
                &mut self.tool_settings.default_proportions_scale,
            ));
        });
    }

    fn radial_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let about_x = self.tool_settings.radial_about_x;
            let about_y = self.tool_settings.radial_about_y;
            let count = self.tool_settings.radial_by_count;
            let angle = self.tool_settings.radial_by_angle;
            if ui.button("Radial of").clicked() {
                self.radial(about_x, about_y, count, angle);
            }
            let about_x = &mut self.tool_settings.radial_about_x;
            let about_y = &mut self.tool_settings.radial_about_y;
            let count = &mut self.tool_settings.radial_by_count;
            ui.add(DragValue::new(count).range(2..=360));
            ui.label("about X:");
            ui.add(DragValue::new(about_x));
            ui.label("about Y:");
            ui.add(DragValue::new(about_y));
        });
        ui.horizontal(|ui| {
            let angle = &mut self.tool_settings.radial_by_angle;
            ui.label("plus angle");
            let angle_speed = if self.angle_snap_enabled {
                self.angle_snap
            } else {
                1.0
            };
            ui.add(DragValue::new(angle).speed(angle_speed));
            (*angle, _) = angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
        });
    }

    fn radial(&mut self, about_x: f32, about_y: f32, count: usize, angle: f32) {
        let selection = self.shroud_interaction.selection();
        if selection.is_empty() {
            return;
        }
        self.add_undo_history = true;
        let new_selection_len = count * selection.len();
        let centre = pos2(about_x, about_y);
        let angle_increment = TAU / count as f32;
        let originals = selection
            .iter()
            .map(|shroud_layer_index| self.shroud[*shroud_layer_index].clone())
            .collect::<Vec<_>>();
        self.shroud.reserve(new_selection_len);
        originals.into_iter().for_each(|mut original| {
            if let Some(mirror_index) = original.mirror_index_option
                && !selection.contains(&mirror_index)
            {
                self.shroud[mirror_index].mirror_index_option = None;
                original.mirror_index_option = None;
            }
            (0..count).for_each(|i| {
                let old_offset = original.shroud_layer.offset.as_ref().unwrap();
                let relative_offset = do3d_to_pos2(old_offset) - centre;
                let radial_angle = angle_increment * i as f32 + angle.to_radians();
                let (sin, cos) = radial_angle.sin_cos();
                let new_offset = do3d_float_from(
                    centre.x + relative_offset.x * cos - relative_offset.y * sin,
                    centre.y + relative_offset.x * sin + relative_offset.y * cos,
                    old_offset.z.to_f32(),
                );
                let new_angle = Angle::Radian(
                    original
                        .shroud_layer
                        .angle
                        .as_ref()
                        .unwrap()
                        .as_radians()
                        .get_value()
                        + radial_angle,
                );
                let mut radial_shroud_layer_container = original.clone();
                radial_shroud_layer_container.shroud_layer.offset = Some(new_offset);
                radial_shroud_layer_container.shroud_layer.angle = Some(new_angle);
                self.shroud.push(radial_shroud_layer_container);
            });
        });
        let sorted_selection = selection.into_iter().sorted().collect::<Vec<_>>();
        sorted_selection.iter().rev().for_each(|i| {
            self.shroud.remove(*i);
        });
        self.shroud_interaction = ShroudInteraction::Inaction {
            selection: (self.shroud.len() - new_selection_len..self.shroud.len()).collect(),
        };
    }

    fn move_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let distance = &mut self.tool_settings.move_selection_by_distance;
            let angle = self.tool_settings.move_selection_by_angle;
            if ui.button("Move by").clicked() {
                self.add_undo_history = true;
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|shroud_layer_index| {
                        let offset = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .offset
                            .as_ref()
                            .unwrap();
                        let new_offset = do3d_float_from(
                            offset.x.to_f32() + *distance * angle.to_radians().cos(),
                            offset.y.to_f32() + *distance * angle.to_radians().sin(),
                            offset.z.to_f32(),
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.offset = Some(new_offset);
                        if let Some(mirror_index) =
                            self.shroud[*shroud_layer_index].mirror_index_option
                        {
                            let offset = self.shroud[*shroud_layer_index]
                                .shroud_layer
                                .offset
                                .as_ref()
                                .unwrap();
                            let new_mirror_offset = do3d_float_from(
                                offset.x.to_f32(),
                                -offset.y.to_f32(),
                                offset.z.to_f32(),
                            );
                            self.shroud[mirror_index].shroud_layer.offset = Some(new_mirror_offset);
                        }
                    });
            }
            let angle = &mut self.tool_settings.move_selection_by_angle;
            ui.add(DragValue::new(distance));
            ui.label("at angle");
            let angle_speed = if self.angle_snap_enabled {
                self.angle_snap
            } else {
                1.0
            };
            ui.add(DragValue::new(angle).speed(angle_speed));
            (*angle, _) = angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
        });
    }

    fn move_by_x_y_z_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let xy_speed = self.get_xy_speed();
            let x = &mut self.tool_settings.move_selection_by_x;
            let y = &mut self.tool_settings.move_selection_by_y;
            let z = &mut self.tool_settings.move_selection_by_z;
            if ui.button("Move by").clicked() {
                self.add_undo_history = true;
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|shroud_layer_index| {
                        let offset = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .offset
                            .as_ref()
                            .unwrap();
                        let new_offset = do3d_float_from(
                            offset.x.to_f32() + *x,
                            offset.y.to_f32() + *y,
                            offset.z.to_f32() + *z,
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.offset = Some(new_offset);
                        if let Some(mirror_index) =
                            self.shroud[*shroud_layer_index].mirror_index_option
                        {
                            let offset = self.shroud[*shroud_layer_index]
                                .shroud_layer
                                .offset
                                .as_ref()
                                .unwrap();
                            let new_mirror_offset = do3d_float_from(
                                offset.x.to_f32(),
                                -offset.y.to_f32(),
                                offset.z.to_f32(),
                            );
                            self.shroud[mirror_index].shroud_layer.offset = Some(new_mirror_offset);
                        }
                    });
            }
            ui.label("X:");
            ui.add(DragValue::new(x).speed(xy_speed));
            ui.label("Y:");
            ui.add(DragValue::new(y).speed(xy_speed));
            ui.label("Z:");
            ui.add(DragValue::new(z).speed(0.005));
        });
    }

    fn scale_by(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let xy_speed = self.get_xy_speed();
            let scale_factor = &mut self.tool_settings.scale_by_scale_factor;
            let about_x = &mut self.tool_settings.scale_by_about_x;
            let about_y = &mut self.tool_settings.scale_by_about_y;
            if ui.button("Scale by").clicked() {
                self.add_undo_history = true;
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|shroud_layer_index| {
                        let offset = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .offset
                            .as_ref()
                            .unwrap();
                        let new_offset = do3d_float_from(
                            *scale_factor * (offset.x.to_f32() - *about_x) + *about_x,
                            *scale_factor * (offset.y.to_f32() - *about_y) + *about_y,
                            offset.z.to_f32(),
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.offset = Some(new_offset);
                        let size = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .size
                            .as_ref()
                            .unwrap();
                        let new_size = do2d_float_from(
                            size.x.to_f32() * *scale_factor,
                            size.y.to_f32() * *scale_factor,
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.size = Some(new_size);
                        if let Some(mirror_index) =
                            self.shroud[*shroud_layer_index].mirror_index_option
                        {
                            let offset = self.shroud[*shroud_layer_index]
                                .shroud_layer
                                .offset
                                .as_ref()
                                .unwrap();
                            let new_mirror_offset = do3d_float_from(
                                offset.x.to_f32(),
                                -offset.y.to_f32(),
                                offset.z.to_f32(),
                            );
                            self.shroud[mirror_index].shroud_layer.offset = Some(new_mirror_offset);
                            self.shroud[mirror_index].shroud_layer.size =
                                self.shroud[*shroud_layer_index].shroud_layer.size.clone();
                        }
                    });
            }
            ui.label("scale factor:");
            ui.add(DragValue::new(scale_factor).speed(xy_speed));
            ui.label("about X:");
            ui.add(DragValue::new(about_x).speed(xy_speed));
            ui.label("Y:");
            ui.add(DragValue::new(about_y).speed(xy_speed));
        });
    }

    fn scale_by_2(&mut self, ui: &mut Ui) {
        let xy_speed = self.get_xy_speed();
        let x_scale_factor = &mut self.tool_settings.scale_by_2_x_scale_factor;
        let y_scale_factor = &mut self.tool_settings.scale_by_2_y_scale_factor;
        let about_x = &mut self.tool_settings.scale_by_2_about_x;
        let about_y = &mut self.tool_settings.scale_by_2_about_y;
        ui.horizontal(|ui| {
            if ui.button("Scale by").clicked() {
                self.add_undo_history = true;
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|shroud_layer_index| {
                        let offset = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .offset
                            .as_ref()
                            .unwrap();
                        let new_offset = do3d_float_from(
                            *x_scale_factor * (offset.x.to_f32() - *about_x) + *about_x,
                            *y_scale_factor * (offset.y.to_f32() - *about_y) + *about_y,
                            offset.z.to_f32(),
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.offset = Some(new_offset);
                        let size = self.shroud[*shroud_layer_index]
                            .shroud_layer
                            .size
                            .as_ref()
                            .unwrap();
                        let new_size = do2d_float_from(
                            size.x.to_f32() * *x_scale_factor,
                            size.y.to_f32() * *y_scale_factor,
                        );
                        self.shroud[*shroud_layer_index].shroud_layer.size = Some(new_size);
                        if let Some(mirror_index) =
                            self.shroud[*shroud_layer_index].mirror_index_option
                        {
                            let offset = self.shroud[*shroud_layer_index]
                                .shroud_layer
                                .offset
                                .as_ref()
                                .unwrap();
                            let new_mirror_offset = do3d_float_from(
                                offset.x.to_f32(),
                                -offset.y.to_f32(),
                                offset.z.to_f32(),
                            );
                            self.shroud[mirror_index].shroud_layer.offset = Some(new_mirror_offset);
                            self.shroud[mirror_index].shroud_layer.size =
                                self.shroud[*shroud_layer_index].shroud_layer.size.clone();
                        }
                    });
            }
            ui.label("X scale factor:");
            ui.add(DragValue::new(x_scale_factor).speed(xy_speed));
            ui.label("Y scale factor:");
            ui.add(DragValue::new(y_scale_factor).speed(xy_speed));
        });
        ui.horizontal(|ui| {
            ui.label("about X:");
            ui.add(DragValue::new(about_x).speed(xy_speed));
            ui.label("Y:");
            ui.add(DragValue::new(about_y).speed(xy_speed));
        });
    }
}
