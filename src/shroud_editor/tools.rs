use std::f32::consts::TAU;

use crate::{
    shroud_editor::{ShroudEditor, shroud_settings::angle_knob_settings},
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};
use egui::{DragValue, Ui, collapsing_header::CollapsingState};
use itertools::{Itertools, sorted};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer,
    utility::{angle::Angle, display_oriented_math::do3d_float_from},
};

#[derive(Default)]
pub struct ToolSettings {
    move_selection_by_distance: f32,
    move_selection_by_angle: f32,
    radial_by_distance: f32,
    radial_by_count: usize,
    radial_by_angle: f32,
}

impl ShroudEditor {
    pub fn tools(&mut self, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ui.ctx(), "tools".into(), true)
            .show_header(ui, |ui| ui.heading("Tools"))
            .body_unindented(|ui| {
                ui.horizontal(|ui| {
                    let distance = &mut self.tool_settings.move_selection_by_distance;
                    let angle = self.tool_settings.move_selection_by_angle;
                    if ui.button("Move by").clicked() {
                        self.shroud_layer_interaction.selection().iter().for_each(
                            |shroud_layer_index| {
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
                                self.shroud[*shroud_layer_index].shroud_layer.offset =
                                    Some(new_offset);
                                // self.shroud[*shroud_layer_index].shroud_layer.angle =
                                //     Some(Angle::Degree(
                                //         self.shroud[*shroud_layer_index]
                                //             .shroud_layer
                                //             .angle
                                //             .as_ref()
                                //             .unwrap()
                                //             .as_degrees()
                                //             .get_value()
                                //             + self.tool_settings.move_selection_by_angle,
                                //     ));
                            },
                        );
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
                    *angle =
                        angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
                });
                ui.horizontal(|ui| {
                    let distance = &mut self.tool_settings.radial_by_distance;
                    let count = &mut self.tool_settings.radial_by_count;
                    let angle = self.tool_settings.radial_by_angle;
                    if ui.button("Radial of").clicked() {
                        let selection = self.shroud_layer_interaction.selection();
                        if !selection.is_empty() {
                            let new_selection_len = *count * selection.len();
                            let centre = self.shroud[selection[0]]
                                .shroud_layer
                                .offset
                                .clone()
                                .unwrap();
                            selection.iter().for_each(|shroud_layer_index| {
                                (0..*count).for_each(|i| {
                                    let original_shroud_layer_container =
                                        &self.shroud[*shroud_layer_index];
                                    let old_offset = original_shroud_layer_container
                                        .shroud_layer
                                        .offset
                                        .as_ref()
                                        .unwrap();
                                    let distance_from_centre =
                                        ((old_offset.x.to_f32() - centre.x.to_f32()).powi(2)
                                            + (old_offset.y.to_f32() - centre.y.to_f32()).powi(2))
                                        .sqrt();
                                    let new_offset = do3d_float_from(
                                        (distance_from_centre + *distance)
                                            * (TAU / *count as f32 * i as f32 + angle.to_radians())
                                                .cos()
                                            + centre.x.to_f32(),
                                        -(distance_from_centre + *distance)
                                            * (TAU / *count as f32 * i as f32 + angle.to_radians())
                                                .sin()
                                            + centre.y.to_f32(),
                                        old_offset.z.to_f32(),
                                    );
                                    let new_angle = Angle::Radian(
                                        original_shroud_layer_container
                                            .shroud_layer
                                            .angle
                                            .as_ref()
                                            .unwrap()
                                            .as_radians()
                                            .get_value()
                                            + TAU / *count as f32 * i as f32
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
                            let sorted_selection =
                                selection.iter().copied().sorted().collect::<Vec<_>>();
                            sorted_selection.iter().rev().for_each(|i| {
                                self.shroud.remove(*i);
                            });
                            self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                selection: (self.shroud.len() - new_selection_len
                                    ..self.shroud.len())
                                    .collect(),
                            }
                        }
                    }
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
                    *angle =
                        angle_knob_settings(ui, *angle, self.angle_snap, self.angle_snap_enabled);
                });
            });
    }
}
