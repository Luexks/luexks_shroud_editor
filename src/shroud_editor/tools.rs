use std::f32::consts::TAU;

use crate::{
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
    radial_by_distance: f32,
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
            radial_by_distance: 10.0,
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
            let distance = self.tool_settings.radial_by_distance;
            let count = self.tool_settings.radial_by_count;
            let angle = self.tool_settings.radial_by_angle;
            self.radial_button(ui, distance, count, angle);
            let distance = &mut self.tool_settings.radial_by_distance;
            let count = &mut self.tool_settings.radial_by_count;
            ui.add(DragValue::new(count).range(2..=360));
            let angle = &mut self.tool_settings.radial_by_angle;
            ui.label("by");
            ui.add(DragValue::new(distance));
            ui.label("at angle");
            let angle_speed = if self.angle_snap_enabled {
                self.angle_snap
            } else {
                1.0
            };
            ui.add(DragValue::new(angle).speed(angle_speed));
            *angle = angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
        });
    }

    fn radial_button(&mut self, ui: &mut Ui, distance: f32, count: usize, angle: f32) {
        if ui.button("Radial of").clicked() {
            let selection = self.shroud_interaction.selection();
            if selection.is_empty() {
                return;
            }
            let new_selection_len = count * selection.len();
            let centre = self.shroud[selection[0]]
                .shroud_layer
                .offset
                .clone()
                .unwrap();
            let mut old_mirror_indexes = Vec::new();
            selection.iter().for_each(|shroud_layer_index| {
                if let Some(old_mirror_index) = self.shroud[*shroud_layer_index].mirror_index_option
                    && !selection.contains(&old_mirror_index)
                {
                    old_mirror_indexes.push(old_mirror_index);
                }
                (0..count).for_each(|i| {
                    let original_shroud_layer_container = &self.shroud[*shroud_layer_index];
                    let old_offset = original_shroud_layer_container
                        .shroud_layer
                        .offset
                        .as_ref()
                        .unwrap();
                    let distance_from_centre = ((old_offset.x.to_f32() - centre.x.to_f32())
                        .powi(2)
                        + (old_offset.y.to_f32() - centre.y.to_f32()).powi(2))
                    .sqrt();
                    let new_drag_pos = pos2(
                        (distance_from_centre + distance)
                            * (TAU / count as f32 * i as f32 + angle.to_radians()).cos()
                            + centre.x.to_f32(),
                        -(distance_from_centre + distance)
                            * (TAU / count as f32 * i as f32 + angle.to_radians()).sin()
                            + centre.y.to_f32(),
                    );
                    // let x_from_centre = old_offset.x.to_f32() - centre.x.to_f32();
                    // let y_from_centre = old_offset.y.to_f32() - centre.y.to_f32();
                    // let angle_from_centre = (old_offset.x.to_f32() - centre.x.to_f32())
                    //     .atan2(old_offset.y.to_f32() - centre.y.to_f32());
                    // let new_drag_pos = pos2(
                    //     (x_from_centre + distance)
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).cos()
                    //         + centre.x.to_f32(),
                    //     -(y_from_centre + distance)
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).sin()
                    //         + centre.y.to_f32(),
                    // );
                    // let new_drag_pos = pos2(
                    //     distance
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).cos()
                    //     + x_from_centre
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).sin()
                    //         + centre.x.to_f32(),
                    //     -(y_from_centre + distance)
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).sin()
                    //     + y_from_centre
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).cos()
                    //         + centre.y.to_f32(),
                    // );
                    // let new_drag_pos = pos2(
                    //     distance
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).cos()
                    //     + distance_from_centre
                    //         * (TAU / count as f32 * i as f32 + angle_from_centre.to_radians()).cos()
                    //         + centre.x.to_f32(),
                    //     -distance
                    //         * (TAU / count as f32 * i as f32 + angle.to_radians()).sin()
                    //     + -distance_from_centre
                    //         * (TAU / count as f32 * i as f32 + angle_from_centre.to_radians()).sin()
                    //         + centre.y.to_f32(),
                    // );
                    // let new_drag_pos = pos2(
                    //     (distance + distance_from_centre)
                    //         * (TAU / count as f32 * i as f32
                    //             + (angle.to_radians() + angle_from_centre))
                    //             .cos()
                    //         + centre.x.to_f32(),
                    //     -(distance + distance_from_centre)
                    //         * (TAU / count as f32 * i as f32
                    //             + (angle.to_radians() + angle_from_centre))
                    //             .sin()
                    //         + centre.y.to_f32(),
                    // );
                    let new_offset =
                        do3d_float_from(new_drag_pos.x, new_drag_pos.y, old_offset.z.to_f32());
                    let new_angle = Angle::Radian(
                        original_shroud_layer_container
                            .shroud_layer
                            .angle
                            .as_ref()
                            .unwrap()
                            .as_radians()
                            .get_value()
                            + TAU / count as f32 * i as f32
                            + angle.to_radians(),
                    );
                    let radial_shroud_layer_container = ShroudLayerContainer {
                        shroud_layer: ShroudLayer {
                            offset: Some(new_offset),
                            angle: Some(new_angle),
                            ..original_shroud_layer_container.shroud_layer.clone()
                        },
                        ..original_shroud_layer_container.clone()
                    };
                    self.shroud.push(radial_shroud_layer_container);
                });
            });
            let sorted_selection = selection
                .iter()
                .copied()
                .chain(old_mirror_indexes)
                .sorted()
                .collect::<Vec<_>>();
            sorted_selection.iter().rev().for_each(|i| {
                self.shroud.remove(*i);
            });
            // let mut mirror_count = 0;
            self.shroud_interaction = ShroudInteraction::Inaction {
                selection: (self.shroud.len() - new_selection_len..self.shroud.len()).collect(),
            };
            (self.shroud.len() - new_selection_len..self.shroud.len()).for_each(
                |shroud_layer_index| {
                    if self.shroud[shroud_layer_index]
                        .mirror_index_option
                        .is_some()
                    {
                        add_mirror(
                            &mut self.shroud,
                            shroud_layer_index,
                            true,
                            &self.loaded_shapes,
                            &self.loaded_shapes_mirror_pairs,
                        );
                        // mirror_count += 1;
                    }
                },
            );
            // self.shroud_interaction = ShroudInteraction::Inaction {
            //     selection: (self.shroud.len() - new_selection_len - mirror_count..self.shroud.len()).collect(),
            // };
        }
    }

    fn move_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let distance = &mut self.tool_settings.move_selection_by_distance;
            let angle = self.tool_settings.move_selection_by_angle;
            if ui.button("Move by").clicked() {
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
            *angle = angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
        });
    }

    fn move_by_x_y_z_tool(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let x = &mut self.tool_settings.move_selection_by_x;
            let y = &mut self.tool_settings.move_selection_by_y;
            let z = &mut self.tool_settings.move_selection_by_z;
            if ui.button("Move by").clicked() {
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
            let xy_speed = if self.grid_snap_enabled {
                self.grid_size / 2.0
            } else {
                0.05
            };
            ui.label("X:");
            ui.add(DragValue::new(x).speed(xy_speed));
            ui.label("Y:");
            ui.add(DragValue::new(y).speed(xy_speed));
            ui.label("Z:");
            ui.add(DragValue::new(z).speed(0.005));
        });
    }
}
