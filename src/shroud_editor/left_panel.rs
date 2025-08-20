use arboard::Clipboard;
use egui::{
    Checkbox, Color32, Context, DragValue, Grid, Rgba, Slider, Ui,
    color_picker::{Alpha, color_edit_button_rgba},
};
use luexks_reassembly::{
    blocks::{shroud::Shroud, shroud_layer::ShroudLayer},
    shapes::shape_id::ShapeId,
    utility::{
        angle::Angle,
        component_formatting::format_component,
        display_oriented_math::{do2d_float_from, do3d_float_from},
    },
};

use crate::{
    color_type_conversion::rgba_to_color,
    shroud_editor::{FILL_COLOR_GRADIENT_TIME, ShroudEditor, shape_combo_box::shape_combo_box},
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

impl ShroudEditor {
    pub fn left_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("side_panel")
            .exact_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Luexks Shroud Editor");
                ui.horizontal(|ui| {
                    let export_to_clipboard_button = ui.button("Export to Clipboard");
                    if export_to_clipboard_button.clicked() {
                        let mut clipboard = Clipboard::new().unwrap();
                        let shroud = format_component(
                            Shroud(
                                self.shroud
                                    .iter()
                                    .map(|shroud_layer_container| {
                                        shroud_layer_container.shroud_layer.clone()
                                    })
                                    .collect(),
                            ),
                            "shroud",
                        );
                        let shroud_export = shroud.to_string();
                        let just_exported_to_clipboard_status =
                            clipboard.set_text(shroud_export).is_ok();
                        self.just_exported_to_clipboard_success_option =
                            Some(just_exported_to_clipboard_status)
                    }
                    if let Some(just_exported_to_clipboard_success) =
                        self.just_exported_to_clipboard_success_option
                    {
                        if export_to_clipboard_button.contains_pointer() {
                            if just_exported_to_clipboard_success {
                                ui.label("Copied to clipboard.");
                            } else {
                                ui.label("Failed :(");
                            }
                        } else {
                            self.just_exported_to_clipboard_success_option = None
                        }
                    }
                });
                self.background_grid_settings(ui);
                self.angle_snap_settings(ui);
                self.fill_color_gradient_setting(ui);
                self.block_settings(ui);
                ui.heading("Shroud Layers:");
                if ui.button("Add Shroud Layer").clicked() {
                    self.add_shroud_layer()
                }
                ui.horizontal(|ui| {
                    ui.label("Only Show Selected:");
                    ui.checkbox(&mut self.only_show_selected_shroud_layers, "");
                });
                self.shroud_list(ui);
            });
    }

    fn background_grid_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Grid Enabled:");
            ui.add(Checkbox::new(&mut self.grid_enabled, ""));
            if self.grid_enabled {
                ui.label("Size:");
                ui.add(DragValue::new(&mut self.grid_size).speed(0.05));
                self.grid_size = self.grid_size.max(0.1);
                ui.label("Snap:");
                ui.add(Checkbox::new(&mut self.snap_to_grid, ""));
            }
        });
    }

    fn angle_snap_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Angle Snap Enabled:");
            ui.add(Checkbox::new(&mut self.angle_snap_enabled, ""));
            if self.angle_snap_enabled {
                ui.add(DragValue::new(&mut self.angle_snap).speed(2.0));
                self.angle_snap = self.angle_snap.clamp(1.0, 90.0);
            }
        });
    }

    fn block_settings(&mut self, ui: &mut Ui) {
        ui.heading("Block Settings");
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
            .inner_margin(6.0)
            .corner_radius(0.0)
            .show(ui, |ui| {
                shape_combo_box(
                    ui,
                    "",
                    &mut self.block_container.block.shape,
                    &mut self.block_container.shape_id,
                    &mut self.block_container.vertices,
                    &self.loaded_shapes,
                );
                Grid::new("").show(ui, |ui| {
                    ui.label(format!(
                        "fillColor={}",
                        self.block_container
                            .block
                            .color_1
                            .clone()
                            .unwrap()
                            .to_string()
                    ));
                    block_color_setting(ui, &mut self.block_container.color_1);
                    self.block_container.block.color_1 =
                        Some(rgba_to_color(self.block_container.color_1));
                    ui.end_row();

                    ui.label(format!(
                        "fillColor1={}",
                        self.block_container
                            .block
                            .color_2
                            .clone()
                            .unwrap()
                            .to_string()
                    ));
                    block_color_setting(ui, &mut self.block_container.color_2);
                    self.block_container.block.color_2 =
                        Some(rgba_to_color(self.block_container.color_2));
                    ui.end_row();

                    ui.label(format!(
                        "lineColor={}",
                        self.block_container
                            .block
                            .line_color
                            .clone()
                            .unwrap()
                            .to_string()
                    ));
                    block_color_setting(ui, &mut self.block_container.line_color);
                    self.block_container.block.line_color =
                        Some(rgba_to_color(self.block_container.line_color));
                    ui.end_row();
                });

                ui.add_space(4.0);
            });
    }

    fn fill_color_gradient_setting(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if self.fill_color_gradient_delta_enabled {
                if self.fill_color_gradient_increasing {
                    self.fill_color_gradient += 1.0 / FILL_COLOR_GRADIENT_TIME * self.dt as f32;
                    if self.fill_color_gradient >= 1.0 {
                        self.fill_color_gradient = 1.0;
                        self.fill_color_gradient_increasing = false;
                    }
                } else {
                    self.fill_color_gradient -= 1.0 / FILL_COLOR_GRADIENT_TIME * self.dt as f32;
                    if self.fill_color_gradient <= 0.0 {
                        self.fill_color_gradient = 0.0;
                        self.fill_color_gradient_increasing = true;
                    }
                }
            }
            ui.label("Gradient:");
            ui.checkbox(&mut self.fill_color_gradient_delta_enabled, "");
            ui.add(Slider::new(&mut self.fill_color_gradient, 0.0..=1.0));
        });
    }

    fn add_shroud_layer(&mut self) {
        let new_shroud_offset =
            do3d_float_from(self.world_mouse_pos.x, self.world_mouse_pos.y, 0.01);
        self.shroud.push(ShroudLayerContainer {
            shroud_layer: ShroudLayer {
                offset: Some(new_shroud_offset),
                shape: Some(ShapeId::Vanilla("SQUARE".to_string())),
                size: Some(do2d_float_from(10.0, 5.0)),
                angle: Some(Angle::Radian(0.0)),
                ..Default::default()
            },
            ..Default::default()
        });
        self.shroud_layer_interaction = ShroudLayerInteraction::Placing {
            selection: vec![self.shroud.len() - 1],
        };
    }
}

fn block_color_setting(ui: &mut Ui, color: &mut Rgba) {
    color_edit_button_rgba(ui, color, Alpha::OnlyBlend);
}
