use egui::{
    Color32, DragValue, Grid, Key, ScrollArea, Stroke, Ui, scroll_area::ScrollBarVisibility,
};
use egui_knob::{Knob, KnobStyle};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayerColor,
    shapes::shapes::Shapes,
    utility::{
        angle::Angle,
        display_oriented_math::{do2d_float_from, do3d_float_from},
    },
};

use crate::{
    shroud_editor::{
        ShroudEditor, add_mirror::add_mirror, shape_combo_box::shroud_layer_shape_combo_box,
    },
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

impl ShroudEditor {
    pub fn shroud_list(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink(false)
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {
                if self.shroud.is_empty() {
                    ui.label("No shrouds :(");
                } else {
                    (0..self.shroud.len()).for_each(|index| {
                        let is_selected = self
                            .shroud_layer_interaction
                            .is_shroud_layer_index_selected(index);
                        if !self.only_show_selected_shroud_layers || is_selected {
                            shroud_layer_settings(
                                is_selected,
                                ui,
                                index,
                                &mut self.shroud_layer_interaction,
                                self.snap_to_grid_enabled,
                                self.grid_size,
                                self.angle_snap,
                                self.angle_snap_enabled,
                                &mut self.shroud,
                                &self.loaded_shapes,
                                &self.loaded_shapes_mirror_pairs,
                            );
                        }
                    });
                }

                ui.add_space(4.0);
            });
    }
}

fn shroud_layer_settings(
    is_selected: bool,
    ui: &mut Ui,
    index: usize,
    shroud_layer_interaction: &mut ShroudLayerInteraction,
    snap_to_grid_enabled: bool,
    grid_size: f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
    shroud: &mut Vec<ShroudLayerContainer>,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
) {
    ui.vertical(|ui| {
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
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
                select_deselect_and_delete_buttons(
                    ui,
                    index,
                    &mut shroud[index],
                    shroud_layer_interaction,
                );
                shroud_layer_mirror_settings(
                    ui,
                    shroud,
                    index,
                    shroud_layer_interaction,
                    loaded_shapes,
                    loaded_shapes_mirror_pairs,
                );
                shroud_layer_shape_combo_box(
                    ui,
                    &index.to_string(),
                    shroud,
                    index,
                    loaded_shapes,
                    loaded_shapes_mirror_pairs,
                );

                let xy_speed = if snap_to_grid_enabled {
                    grid_size / 2.0
                } else {
                    0.05
                };
                ui.horizontal(|ui| {
                    let offset = shroud[index].shroud_layer.offset.clone().unwrap();
                    let mut x = offset.x.to_f32();
                    let mut y = offset.y.to_f32();
                    let mut z = offset.z.to_f32();
                    let original_offset = (x, y, z);
                    ui.label("offset={");
                    ui.add(DragValue::new(&mut x).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut y).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut z).speed(0.005));
                    ui.label("}");
                    shroud[index].shroud_layer.offset = Some(do3d_float_from(x, y, z));
                    if original_offset != (x, y, z)
                        && let Some(mirror_index) = shroud[index].mirror_index_option
                    {
                        shroud[mirror_index].shroud_layer.offset = Some(do3d_float_from(x, -y, z));
                    }
                });
                ui.horizontal(|ui| {
                    let size = shroud[index].shroud_layer.size.clone().unwrap();
                    let mut width = size.x.to_f32();
                    let mut height = size.y.to_f32();
                    let original_size = (width, height);
                    ui.label("size={");
                    ui.add(DragValue::new(&mut width).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut height).speed(xy_speed));
                    ui.label("}");
                    shroud[index].shroud_layer.size = Some(do2d_float_from(width, height));
                    if original_size != (width, height)
                        && let Some(mirror_index) = shroud[index].mirror_index_option
                    {
                        shroud[mirror_index].shroud_layer.size =
                            Some(do2d_float_from(width, height));
                        // if shroud[index].shape_id == "SQUARE" {
                        //     shroud[mirror_index].shroud_layer.size =
                        //         Some(do2d_float_from(width, -height));
                        // } else {
                        //     shroud[mirror_index].shroud_layer.size =
                        //         Some(do2d_float_from(width, height));
                        // }
                    }
                });
                ui.horizontal(|ui| {
                    full_angle_settings(ui, shroud, index, angle_snap, angle_snap_enabled);
                });

                let shroud_layer_container = &mut shroud[index];
                let mut color_1 = shroud_layer_container.shroud_layer.color_1.clone().unwrap();
                let mut color_2 = shroud_layer_container.shroud_layer.color_2.clone().unwrap();
                let mut line_color = shroud_layer_container
                    .shroud_layer
                    .line_color
                    .clone()
                    .unwrap();
                let original_color_1 = color_1.clone();
                let original_color_2 = color_2.clone();
                let original_line_color = line_color.clone();
                Grid::new(index.to_string()).show(ui, |ui| {
                    shroud_color_setting(ui, &mut color_1, "tri_color_id=");
                    shroud_color_setting(ui, &mut color_2, "tri_color1_id=");
                    shroud_color_setting(ui, &mut line_color, "line_color_id=");
                });
                shroud_layer_container.shroud_layer.color_1 = Some(color_1.clone());
                shroud_layer_container.shroud_layer.color_2 = Some(color_2.clone());
                shroud_layer_container.shroud_layer.line_color = Some(line_color.clone());
                if let Some(mirror_index) = shroud[index].mirror_index_option {
                    if original_color_1 != color_1 {
                        shroud[mirror_index].shroud_layer.color_1 = Some(color_1);
                    }
                    if original_color_2 != color_2 {
                        shroud[mirror_index].shroud_layer.color_2 = Some(color_2);
                    }
                    if original_line_color != line_color {
                        shroud[mirror_index].shroud_layer.line_color = Some(line_color);
                    }
                }

                if shroud[index].shape_id == "SQUARE" {
                    ui.horizontal(|ui| {
                        let mut taper = shroud[index].shroud_layer.taper.unwrap_or(1.0);
                        let original_taper = taper;
                        ui.label("taper=");
                        ui.add(DragValue::new(&mut taper).speed(0.025));
                        shroud[index].shroud_layer.taper = Some(taper);
                        if original_taper != taper
                            && let Some(mirror_index) = shroud[index].mirror_index_option
                        {
                            shroud[mirror_index].shroud_layer.taper = Some(taper);
                        }
                    });
                }
            });
    });
}

fn shroud_color_setting(ui: &mut Ui, color: &mut ShroudLayerColor, text: &str) {
    ui.label(text);
    ui.selectable_value(color, ShroudLayerColor::Color1, "0");
    ui.selectable_value(color, ShroudLayerColor::Color2, "1");
    ui.selectable_value(color, ShroudLayerColor::LineColor, "2");
    ui.end_row();
}

fn select_deselect_and_delete_buttons(
    ui: &mut Ui,
    index: usize,
    shroud_layer_container: &mut ShroudLayerContainer,
    shroud_layer_interaction: &mut ShroudLayerInteraction,
) {
    ui.horizontal(|ui| {
        if !shroud_layer_interaction.selection().contains(&index) {
            if ui.button("Select").clicked() {
                *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                    selection: shroud_layer_interaction
                        .selection()
                        .iter()
                        .copied()
                        .chain(std::iter::once(index))
                        .collect(),
                };
            }
        } else if ui.button("Deselect").clicked() {
            *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                selection: shroud_layer_interaction
                    .selection()
                    .iter()
                    .copied()
                    .filter(|selection_index| *selection_index != index)
                    .collect(),
            };
        }
        if ui.button("Delete (Double Click)").double_clicked() {
            shroud_layer_container.delete_next_frame = true;
        }
    });
}

fn full_angle_settings(
    ui: &mut Ui,
    shroud: &mut [ShroudLayerContainer],
    index: usize,
    angle_snap: f32,
    angle_snap_enabled: bool,
) {
    let mut angle = shroud[index]
        .shroud_layer
        .angle
        .clone()
        .unwrap()
        .as_degrees()
        .get_value();
    let original_angle = angle;
    let angle_speed = if angle_snap_enabled { angle_snap } else { 1.0 };
    ui.label("angle=");
    ui.add(DragValue::new(&mut angle).speed(angle_speed));
    ui.label("*pi/180");
    let angle = angle_knob_settings(ui, angle, angle_snap, angle_snap_enabled);
    shroud[index].shroud_layer.angle = Some(Angle::Degree(angle));
    if original_angle != angle
        && let Some(mirror_index) = shroud[index].mirror_index_option
    {
        shroud[mirror_index].shroud_layer.angle = Some(Angle::Degree(-angle));
    }
}

pub fn angle_knob_settings(
    ui: &mut Ui,
    angle: f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
) -> f32 {
    let pre_knob_angle = angle;
    let mut angle = angle + 90.0;
    ui.add(
        Knob::new(&mut angle, 0.0, 360.0 * 1.5, KnobStyle::Wiper)
            .with_size(20.0)
            .with_sweep_range(0.5, 1.5)
            .with_background_arc(false),
    );
    if angle_snap_enabled && pre_knob_angle != angle {
        angle = (angle / angle_snap).round() * angle_snap;
    }
    let angle = (angle - 90.0) % 360.0;
    if angle < 0.0 { angle + 360.0 } else { angle }
}

fn shroud_layer_mirror_settings(
    ui: &mut Ui,
    shroud: &mut Vec<ShroudLayerContainer>,
    index: usize,
    shroud_layer_interaction: &mut ShroudLayerInteraction,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
) {
    ui.horizontal(|ui| {
        if let Some(mirror_index) = shroud[index].mirror_index_option {
            if !shroud_layer_interaction.selection().contains(&mirror_index) {
                if ui.button("Select Mirror").clicked() {
                    *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                        selection: shroud_layer_interaction
                            .selection()
                            .iter()
                            .copied()
                            .chain(std::iter::once(mirror_index))
                            .collect(),
                    };
                }
            } else if ui.button("Deselect Mirror").clicked() {
                *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                    selection: shroud_layer_interaction
                        .selection()
                        .iter()
                        .copied()
                        .filter(|selection_index| *selection_index != mirror_index)
                        .collect(),
                };
            }
            if ui.button("Unlink").clicked() {
                shroud[index].mirror_index_option = None;
                shroud[mirror_index].mirror_index_option = None;
            }
            if ui.button("Delete Mirror").clicked() {
                shroud[mirror_index].delete_next_frame = true;
            }
        } else if ui.button("Add Mirror").clicked()
            || (shroud_layer_interaction.selection().contains(&index)
                && ui.input(|i| i.key_pressed(Key::F)))
        {
            add_mirror(
                shroud,
                index,
                false,
                loaded_shapes,
                loaded_shapes_mirror_pairs,
            );
        }
    });
}
