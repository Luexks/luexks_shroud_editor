use std::f32::{EPSILON, INFINITY};

use arboard::Clipboard;
use egui::{
    Checkbox, Color32, ComboBox, Context, DragValue, Grid, Pos2, Rgba, ScrollArea, Slider,
    TextBuffer, TextEdit, Ui,
    collapsing_header::CollapsingState,
    color_picker::{Alpha, color_edit_button_rgba},
    scroll_area::ScrollBarVisibility,
    vec2,
};
use egui_extras::syntax_highlighting::{CodeTheme, highlight};
use luexks_reassembly::{
    blocks::{shroud::Shroud, shroud_layer::ShroudLayer},
    shapes::{shape_id::ShapeId, shapes::Shapes},
    utility::{
        angle::Angle,
        component_formatting::format_component,
        display_oriented_math::{do2d_float_from, do3d_float_from},
    },
};

use crate::{
    color_type_conversion::{rgba_to_color, rgba_to_color_string, str_to_rgba_option},
    restructure_vertices::restructure_vertices,
    shroud_editor::{
        FILL_COLOR_GRADIENT_TIME, ShroudEditor,
        parse_shroud_text::{ShroudParseResult, parse_shroud_text},
    },
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

impl ShroudEditor {
    pub fn left_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("side_panel")
            .min_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Luexks Shroud Editor");
                ScrollArea::vertical()
                    .auto_shrink(false)
                    .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                    .show(ui, |ui| {
                        CollapsingState::load_with_default_open(ctx, "file".into(), false)
                            .show_header(ui, |ui| ui.heading("File"))
                            .body_unindented(|ui| {
                                self.export_shroud_to_clipboard_button(ui);
                                self.import_shroud_from_paste_box(ui);
                            });
                        CollapsingState::load_with_default_open(ctx, "editor".into(), true)
                            .show_header(ui, |ui| ui.heading("Editor Settings"))
                            .body_unindented(|ui| {
                                self.background_grid_settings(ui);
                                self.angle_snap_settings(ui);
                                self.fill_color_gradient_setting(ui);
                            });
                        self.block_settings(ui);
                        ui.heading("Shroud Layers");
                        if ui.button("Add Shroud Layer").clicked() {
                            self.add_shroud_layer()
                        }
                        ui.horizontal(|ui| {
                            ui.label("Only Show Selected:");
                            ui.checkbox(&mut self.only_show_selected_shroud_layers, "");
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Select All").clicked() {
                                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                    selection: (0..self.shroud.len()).collect(),
                                };
                            }
                            if ui.button("Deselect All").clicked() {
                                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                    selection: Vec::new(),
                                };
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Expand Selection").clicked() {
                                self.shroud_layer_interaction.selection().iter().for_each(
                                    |index| {
                                        let mut drop_down =
                                            CollapsingState::load(ctx, index.to_string().into())
                                                .unwrap();
                                        drop_down.set_open(true);
                                        drop_down.store(ctx);
                                    },
                                );
                            }
                            if ui.button("Collapse Selection").clicked() {
                                self.shroud_layer_interaction.selection().iter().for_each(
                                    |index| {
                                        let mut drop_down =
                                            CollapsingState::load(ctx, index.to_string().into())
                                                .unwrap();
                                        drop_down.set_open(false);
                                        drop_down.store(ctx);
                                    },
                                );
                            }
                        });
                        self.shroud_list(ui);
                    });
            });
    }

    fn background_grid_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Grid Visible:");
            ui.add(Checkbox::new(&mut self.grid_visible, ""));
            ui.label("Size:");
            ui.add(DragValue::new(&mut self.grid_size).speed(0.05));
            self.grid_size = self.grid_size.max(0.1);
            ui.label("Snap:");
            ui.add(Checkbox::new(&mut self.grid_snap_enabled, ""));
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
        CollapsingState::load_with_default_open(ui.ctx(), "block".into(), true)
            .show_header(ui, |ui| ui.heading("Block Settings"))
            .body_unindented(|ui| {
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                    .inner_margin(6.0)
                    .corner_radius(0.0)
                    .show(ui, |ui| {
                        block_shape_combo_box(
                            ui,
                            "",
                            &mut self.block_container.block.shape,
                            &mut self.block_container.shape_id,
                            &mut self.block_container.vertices,
                            &self.loaded_shapes,
                        );
                        Grid::new("").show(ui, |ui| {
                            ui.label("fillColor=");
                            block_color_settings(
                                ui,
                                &mut self.block_container.color_1,
                                &mut self.block_container.input_color_1,
                            );
                            self.block_container.block.color_1 =
                                Some(rgba_to_color(self.block_container.color_1));
                            ui.end_row();

                            ui.label("fillColor1=");
                            block_color_settings(
                                ui,
                                &mut self.block_container.color_2,
                                &mut self.block_container.input_color_2,
                            );
                            self.block_container.block.color_2 =
                                Some(rgba_to_color(self.block_container.color_2));
                            ui.end_row();

                            ui.label("lineColor=");
                            block_color_settings(
                                ui,
                                &mut self.block_container.line_color,
                                &mut self.block_container.input_line_color,
                            );
                            self.block_container.block.line_color =
                                Some(rgba_to_color(self.block_container.line_color));
                            ui.end_row();
                        });

                        ui.add_space(4.0);
                    });
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

    fn export_shroud_to_clipboard_button(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let export_to_clipboard_button = ui.button("Export Shroud to Clipboard");
            if export_to_clipboard_button.clicked() {
                let mut clipboard = Clipboard::new().unwrap();
                let shroud = format_component(
                    Shroud(
                        self.shroud
                            .iter()
                            .map(|shroud_layer_container| {
                                let shroud_layer = shroud_layer_container.shroud_layer.clone();
                                ShroudLayer {
                                    angle: if shroud_layer
                                        .angle
                                        .clone()
                                        .unwrap()
                                        .as_radians()
                                        .get_value()
                                        .abs()
                                        < EPSILON
                                    {
                                        None
                                    } else {
                                        shroud_layer.angle.clone()
                                    },
                                    taper: if shroud_layer_container.shape_id
                                        != "SQUARE".to_string()
                                        && shroud_layer.taper.clone().unwrap() == 1.0
                                    {
                                        None
                                    } else {
                                        shroud_layer.taper.clone()
                                    },
                                    ..shroud_layer
                                }
                            })
                            .collect(),
                    ),
                    "shroud",
                );
                let shroud_export = shroud.to_string();
                let just_exported_to_clipboard_status = clipboard.set_text(shroud_export).is_ok();
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
    }

    fn import_shroud_from_paste_box(&mut self, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ui.ctx(), "import_shroud".into(), false)
            .show_header(ui, |ui| {
                ui.strong("Import Shroud from Paste Box");
            })
            .body(|ui| {
                ui.horizontal(|ui| {
                    let response = ui.button("Import");
                    if response.clicked() {
                        match parse_shroud_text(&self.shroud_import_text, &self.loaded_shapes) {
                            Ok(imported_shroud) => {
                                self.shroud = imported_shroud;
                                self.just_imported_from_paste_box_message_option =
                                    Some(ShroudParseResult::Success);
                            }
                            Err(err) => {
                                self.just_imported_from_paste_box_message_option = Some(err);
                            }
                        }
                    }
                    if let Some(message) = &self.just_imported_from_paste_box_message_option {
                        ui.label(message.to_string());
                    }
                    if !response.contains_pointer() {
                        self.just_imported_from_paste_box_message_option = None;
                    }
                });
                ScrollArea::horizontal().show(ui, |ui| {
                    let theme = CodeTheme::light(12.0);
                    let mut layouter = |ui: &Ui, buf: &dyn TextBuffer, wrap_width: f32| {
                        let mut layout_job =
                            highlight(ui.ctx(), ui.style(), &theme, buf.as_str(), "toml");
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };
                    ui.add(
                        TextEdit::multiline(&mut self.shroud_import_text)
                            .code_editor()
                            .desired_width(INFINITY)
                            .layouter(&mut layouter),
                    );
                });
            });
    }
}

fn block_color_settings(ui: &mut Ui, color: &mut Rgba, input_color: &mut String) {
    let response = ui.add(
        TextEdit::singleline(input_color)
            .code_editor()
            .min_size(vec2(100.0, 20.0))
            .hint_text("0xFFFFFFFF"),
    );
    ui.horizontal(|ui| {
        let rgba_option = str_to_rgba_option(input_color);
        if let Some(rgba) = rgba_option {
            if response.changed() {
                *color = rgba;
            }
        }
        let original_color = *color;
        color_edit_button_rgba(ui, color, Alpha::OnlyBlend);
        if !response.changed() && original_color != *color {
            *input_color = rgba_to_color_string(*color);
        }
        if !response.has_focus() && rgba_option.is_none() {
            ui.colored_label(Color32::RED, ">:(");
        }
    });
}

fn block_shape_combo_box(
    ui: &mut Ui,
    id: &str,
    shape: &mut Option<ShapeId>,
    shape_id: &mut String,
    vertices: &mut Vec<Pos2>,
    loaded_shapes: &Shapes,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        ComboBox::from_id_salt(id.to_string())
            .selected_text(shape_id.as_str())
            .show_ui(ui, |ui| {
                for selectable_shape in &loaded_shapes.0 {
                    let selectable_shape_id = selectable_shape.get_id().unwrap().to_string();
                    let response = ui.selectable_value(
                        shape_id,
                        selectable_shape_id.clone(),
                        selectable_shape_id,
                    );
                    if response.clicked() {
                        *vertices =
                            restructure_vertices(selectable_shape.get_first_scale_vertices());
                        *shape = selectable_shape.get_id();
                    }
                }
            });
    });
}
