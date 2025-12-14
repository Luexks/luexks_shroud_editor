use egui::{Color32, DragValue, Grid, Stroke, Ui, collapsing_header::CollapsingState};
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
    shroud_interaction::ShroudInteraction,
    shroud_layer_container::ShroudLayerContainer,
};

impl ShroudEditor {
    pub fn shroud_list(&mut self, ui: &mut Ui) {
        if self.shroud.is_empty() {
            ui.label("No shrouds :(");
        } else {
            // let start = ui.cursor().min.y;
            // println!("{}", start);
            // (0..self.shroud.len()).for_each(|index| {
            for index in 0..self.shroud.len() {
                let is_selected = self
                    .shroud_interaction
                    .is_shroud_layer_index_selected(index);
                if !self.only_show_selected_shroud_layers || is_selected {
                    shroud_layer_settings(
                        is_selected,
                        ui,
                        index,
                        &mut self.shroud_interaction,
                        self.grid_snap_enabled,
                        self.grid_size,
                        self.angle_snap,
                        self.angle_snap_enabled,
                        &mut self.shroud,
                        &self.loaded_shapes,
                        &self.loaded_shapes_mirror_pairs,
                        &mut self.shape_search_show_vanilla,
                        &mut self.shape_search_buf,
                    );
                    // }
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
}

fn shroud_layer_settings(
    is_selected: bool,
    ui: &mut Ui,
    index: usize,
    shroud_interaction: &mut ShroudInteraction,
    grid_snap_enabled: bool,
    grid_size: f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
    shroud: &mut Vec<ShroudLayerContainer>,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
    show_vanilla: &mut bool,
    shape_search_buf: &mut String,
) {
    // let start_y = ui.cursor().min.y;
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
                // let response = ui.label("h");
                // let visible_rect = ui.clip_rect();
                // println!("{}\t{}", response.rect.min.y, visible_rect.min.y);
                // let start_y = ui.cursor().min.y;
                ui.spacing_mut().item_spacing.y = 2.0;
                CollapsingState::load_with_default_open(ui.ctx(), index.to_string().into(), true)
                    .show_header(ui, |ui| {
                        select_deselect_and_delete_buttons(
                            ui,
                            index,
                            &mut shroud[index],
                            shroud_interaction,
                        );
                    })
                    .body_unindented(|ui| {
                        let top_of_shroud_layer_settings_y = ui.cursor().min.y;
                        let shroud_layer_settings_height = if shroud[index].shape_id == "SQUARE" {
                            207.0
                            // 222.0
                        } else {
                            187.0
                            // 202.0
                        };
                        let window_bottom_y = ui.clip_rect().max.y;
                        // println!("{}\t{}", top_of_shroud_layer_settings_y, window_bottom_y);
                        let shroud_layer_settings_are_off_screen =
                            top_of_shroud_layer_settings_y + shroud_layer_settings_height < 0.0
                                || top_of_shroud_layer_settings_y > window_bottom_y;
                        if shroud_layer_settings_are_off_screen {
                            ui.add_space(shroud_layer_settings_height);
                            // println!("Culling shroud layer of ID: {}", index);
                        } else {
                            shroud_layer_mirror_settings(
                                ui,
                                shroud,
                                index,
                                shroud_interaction,
                                loaded_shapes,
                                loaded_shapes_mirror_pairs,
                            );
                            shroud_layer_shape_combo_box(
                                ui,
                                shroud,
                                index,
                                loaded_shapes,
                                loaded_shapes_mirror_pairs,
                                show_vanilla,
                                shape_search_buf,
                            );

                            let xy_speed = if grid_snap_enabled {
                                grid_size / 2.0
                            } else {
                                0.05
                            };
                            ui.horizontal(|ui| {
                                let offset = shroud[index].shroud_layer.offset.as_mut().unwrap();
                                let x = offset.x.to_f32_mut();
                                let y = offset.y.to_f32_mut();
                                let z = offset.z.to_f32_mut();
                                let original_offset = (*x, *y, *z);
                                ui.label("offset={");
                                ui.add(DragValue::new(x).speed(xy_speed));
                                ui.label(",");
                                ui.add(DragValue::new(y).speed(xy_speed));
                                ui.label(",");
                                ui.add(DragValue::new(z).speed(0.005));
                                ui.label("}");
                                let (x, y, z) = (*x, *y, *z);
                                if original_offset != (x, y, z)
                                    && let Some(mirror_index) = shroud[index].mirror_index_option
                                {
                                    shroud[mirror_index].shroud_layer.offset =
                                        Some(do3d_float_from(x, -y, z));
                                }
                            });
                            ui.horizontal(|ui| {
                                let size = shroud[index].shroud_layer.size.as_mut().unwrap();
                                let width = size.x.to_f32_mut();
                                let height = size.y.to_f32_mut();
                                let original_size = (*width, *height);
                                ui.label("size={");
                                ui.add(DragValue::new(width).speed(xy_speed));
                                ui.label(",");
                                ui.add(DragValue::new(height).speed(xy_speed));
                                ui.label("}");
                                let (width, height) = (*width, *height);
                                if original_size != (width, height)
                                    && let Some(mirror_index) = shroud[index].mirror_index_option
                                {
                                    shroud[mirror_index].shroud_layer.size =
                                        Some(do2d_float_from(width, height));
                                }
                            });
                            ui.horizontal(|ui| {
                                full_angle_settings(
                                    ui,
                                    shroud,
                                    index,
                                    angle_snap,
                                    angle_snap_enabled,
                                );
                            });

                            let shroud_layer_container = &mut shroud[index];
                            let color_1 = shroud_layer_container
                                .shroud_layer
                                .color_1
                                .as_mut()
                                .unwrap();
                            let color_2 = shroud_layer_container
                                .shroud_layer
                                .color_2
                                .as_mut()
                                .unwrap();
                            let line_color = shroud_layer_container
                                .shroud_layer
                                .line_color
                                .as_mut()
                                .unwrap();
                            let original_color_1 = *color_1;
                            let original_color_2 = *color_2;
                            let original_line_color = *line_color;
                            Grid::new(index.to_string()).show(ui, |ui| {
                                shroud_color_setting(ui, color_1, "tri_color_id=");
                                shroud_color_setting(ui, color_2, "tri_color1_id=");
                                shroud_color_setting(ui, line_color, "line_color_id=");
                            });
                            let (color_1, color_2, line_color) = (*color_1, *color_2, *line_color);
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
                                        && let Some(mirror_index) =
                                            shroud[index].mirror_index_option
                                    {
                                        shroud[mirror_index].shroud_layer.taper = Some(taper);
                                    }
                                });
                            }
                        }
                    });
            });
    });
    // }
    // let end_y = ui.cursor().min.y;
    // println!("Height: {}", end_y - start_y);
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
    shroud_interaction: &mut ShroudInteraction,
) {
    ui.horizontal(|ui| {
        if !shroud_interaction.selection().contains(&index) {
            if ui.button("Select").clicked() {
                *shroud_interaction = ShroudInteraction::Inaction {
                    selection: shroud_interaction
                        .selection()
                        .into_iter()
                        .chain(std::iter::once(index))
                        .collect(),
                };
            }
        } else if ui.button("Deselect").clicked() {
            *shroud_interaction = ShroudInteraction::Inaction {
                selection: shroud_interaction
                    .selection()
                    .into_iter()
                    .filter(|selection_index| *selection_index != index)
                    .collect(),
            };
        }
        if ui.button("Delete (Double Click)").double_clicked() {
            shroud_layer_container.delete_next_frame = true;
            *shroud_interaction = ShroudInteraction::Inaction {
                selection: shroud_interaction
                    .selection()
                    .into_iter()
                    .filter(|selection_index| *selection_index != index)
                    .collect(),
            };
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
    let original_knob_angle = 360.0 - angle + 90.0;
    let mut knob_angle = original_knob_angle;
    ui.add(
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
    let mut angle = (knob_angle - 90.0) % 360.0;
    if angle != 0.0 {
        angle = 360.0 - angle;
    }
    // println!("{}\t{}", original_knob_angle, angle);
    angle
    // (angle - 90.0).rem_euclid(360.0)

    // let pre_knob_angle = 360.0 - angle + 90.0;
    // let mut angle = 360.0 - angle + 90.0;
    // // println!("Before{}", angle);
    // ui.add(
    //     Knob::new(&mut angle, 0.0, 450.0, KnobStyle::Wiper)
    //         .with_size(20.0)
    //         .with_sweep_range(0.5, 1.25)
    //         .with_background_arc(false),
    // );
    // if angle_snap_enabled && pre_knob_angle != angle {
    //     angle = (angle / angle_snap).round() * angle_snap;
    // }
    // // println!(" After{}", angle);
    // if angle < 90.0 {
    //     angle = 360.0 + angle;
    // }
    // let angle = (angle - 90.0) % 360.0;
    // if angle < 0.0 { angle } else { 360.0 - angle }
}

fn shroud_layer_mirror_settings(
    ui: &mut Ui,
    shroud: &mut Vec<ShroudLayerContainer>,
    index: usize,
    shroud_interaction: &mut ShroudInteraction,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
) {
    ui.horizontal(|ui| {
        if let Some(mirror_index) = shroud[index].mirror_index_option {
            if !shroud_interaction.selection().contains(&mirror_index) {
                if ui.button("Select Mirror").clicked() {
                    *shroud_interaction = ShroudInteraction::Inaction {
                        selection: shroud_interaction
                            .selection()
                            .into_iter()
                            .chain(std::iter::once(mirror_index))
                            .collect(),
                    };
                }
            } else if ui.button("Deselect Mirror").clicked() {
                *shroud_interaction = ShroudInteraction::Inaction {
                    selection: shroud_interaction
                        .selection()
                        .into_iter()
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
                *shroud_interaction = ShroudInteraction::Inaction {
                    selection: shroud_interaction
                        .selection()
                        .into_iter()
                        .filter(|selection_index| *selection_index != mirror_index)
                        .collect(),
                };
            }
        } else if ui.button("Add Mirror").clicked() {
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
