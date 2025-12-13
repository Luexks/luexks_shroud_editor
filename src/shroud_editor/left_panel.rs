use arboard::Clipboard;
use egui::{
    Checkbox, Color32, Context, DragValue, Grid, Popup, PopupCloseBehavior, Pos2, Rgba, ScrollArea,
    Slider, TextBuffer, TextEdit, Ui, Vec2,
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
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    color_type_conversion::{rgba_to_color, rgba_to_color_string, str_to_rgba_option},
    invert_y::invert_y_of_pos2,
    restructure_vertices::restructure_vertices,
    shroud_editor::{
        FILL_COLOR_GRADIENT_TIME, ShroudEditor, get_loaded_shapes_mirror_pairs,
        parse_shapes_text::{ShapesParseResult, parse_shapes_text},
        parse_shroud_text::{ShroudParseResult, parse_shroud_text},
    },
    shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection, ShroudInteraction},
    shroud_layer_container::ShroudLayerContainer,
};

type IsChanged = bool;

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
                        self.file_settings(ctx, ui);
                        self.editor_settings(ctx, ui);
                        self.block_settings(ui);
                        self.tools(ui);
                        self.shroud_settings(ctx, ui);
                        self.shroud_list(ui);
                    });
            });
    }

    fn file_settings(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "file".into(), false)
            .show_header(ui, |ui| ui.heading("File"))
            .body_unindented(|ui| {
                self.export_shroud_to_clipboard_button(ui);
                self.import_shroud_from_paste_box(ui);
                self.import_shapes_from_paste_box(ui);
            });
    }

    fn editor_settings(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "editor".into(), false)
            .show_header(ui, |ui| ui.heading("Editor Settings"))
            .body_unindented(|ui| {
                self.background_grid_settings(ui);
                self.angle_snap_settings(ui);
                self.fill_color_gradient_setting(ui);
                self.binding_config(ctx, ui);
            });
    }

    fn binding_config(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "bindings".into(), true)
            .show_header(ui, |ui| ui.label("Bindings"))
            .body_unindented(|ui| {
                ui.label(
                    r#"This will be a binding section in the future, but for now:
    WASD move
    Scroll zoom
    click: select
    shift + click: multiselect
    drag: drag selection
    c: copy
    v: paste
    f: add mirror to selection
    r: delete selection"#,
                );
            });
    }

    fn shroud_settings(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Shroud Layers");
        if ui.button("Add Shroud Layer").clicked() {
            self.add_shroud_layer()
        }
        if self.shroud.is_empty() {
            return;
        }
        ui.horizontal(|ui| {
            ui.label("Only Show Selected:");
            ui.checkbox(&mut self.only_show_selected_shroud_layers, "");
        });
        ui.horizontal(|ui| {
            if ui.button("Select All").clicked() {
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: (0..self.shroud.len()).collect(),
                };
            }
            if ui.button("Deselect All").clicked() {
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: Vec::new(),
                };
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Expand Selection").clicked() {
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|index| {
                        let mut drop_down =
                            CollapsingState::load(ctx, index.to_string().into()).unwrap();
                        drop_down.set_open(true);
                        drop_down.store(ctx);
                    });
            }
            if ui.button("Collapse Selection").clicked() {
                self.shroud_interaction
                    .selection()
                    .iter()
                    .for_each(|index| {
                        let mut drop_down =
                            CollapsingState::load(ctx, index.to_string().into()).unwrap();
                        drop_down.set_open(false);
                        drop_down.store(ctx);
                    });
            }
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
        CollapsingState::load_with_default_open(ui.ctx(), "block".into(), false)
            .show_header(ui, |ui| ui.heading("Block Settings"))
            .body_unindented(|ui| {
                self.block_visibility_settings(ui);
                self.block_offset_settings(ui);
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                    .inner_margin(6.0)
                    .corner_radius(0.0)
                    .show(ui, |ui| {
                        let is_shape_changed = block_shape_combo_box(
                            ui,
                            &mut self.block_container.block.shape,
                            &mut self.block_container.shape_id,
                            &mut self.block_container.vertices,
                            &self.loaded_shapes,
                            // &mut self.block_container.search_buf,
                            &mut self.shape_search_show_vanilla,
                            &mut self.shape_search_buf,
                            &mut self.block_container.max_scale,
                            self.block_container.block.scale.unwrap(),
                        );
                        let is_scale_changed = self.block_scale_settings(ui);
                        if self.block_container.use_non_turreted_offset
                            && (is_shape_changed || is_scale_changed)
                        {
                            self.block_container.update_non_turreted_offset();
                        }
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

    fn block_visibility_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Visible:");
            if ui.checkbox(&mut self.block_container.visible, "").clicked()
                && self.block_container.use_non_turreted_offset
            {
                self.block_container.update_non_turreted_offset();
            }
        });
    }

    fn block_offset_settings(&mut self, ui: &mut Ui) {
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
            .inner_margin(6.0)
            .corner_radius(0.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Use Auto (Non-Turreted) Offset:");
                    if ui
                        .checkbox(&mut self.block_container.use_non_turreted_offset, "")
                        .clicked() && self.block_container.use_non_turreted_offset {
                            self.block_container.update_non_turreted_offset();
                    }
                });
                ui.horizontal(|ui| {
                    if self.block_container.use_non_turreted_offset {
                        ui.label(format!(
                            "Offset: {:.2}, {:.2}",
                            self.block_container.offset.x, self.block_container.offset.y
                        ));
                    } else {
                        ui.label("Offset:");
                        ui.add(DragValue::new(&mut self.block_container.offset.x));
                        ui.label(",");
                        ui.add(DragValue::new(&mut self.block_container.offset.y));
                    }
                });
                ui.label("Block offset is added to all shroud layer offsets on import and subtracted from all shroud layer offsets on export; no visual change while editing.");
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
        let last = self.shroud.len();
        let new_shroud_offset =
            do3d_float_from(self.world_mouse_pos.x, -self.world_mouse_pos.y, 0.01);
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
        let drag_pos = invert_y_of_pos2(self.world_mouse_pos);
        self.shroud_interaction = ShroudInteraction::Placing {
            selection: MovingShroudSelection(
                [MovingShroudLayerInteraction {
                    idx: last,
                    relative_pos: Vec2::default(),
                }]
                .into(),
            ),
            drag_pos,
            potentially_snapped_drag_pos: drag_pos,
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
                                let pre_block_offset_offset = shroud_layer.offset.as_ref().unwrap();
                                let post_block_offset_offset = do3d_float_from(
                                    pre_block_offset_offset.x.to_f32()
                                        - self.block_container.offset.x,
                                    pre_block_offset_offset.y.to_f32()
                                        - self.block_container.offset.y,
                                    pre_block_offset_offset.z.to_f32(),
                                );
                                ShroudLayer {
                                    angle: if shroud_layer
                                        .angle
                                        .clone()
                                        .unwrap()
                                        .as_radians()
                                        .get_value()
                                        .abs()
                                        < f32::EPSILON
                                    {
                                        None
                                    } else {
                                        shroud_layer.angle.clone()
                                    },
                                    taper: if shroud_layer_container.shape_id != "SQUARE"
                                        && shroud_layer.taper.unwrap() == 1.0
                                    {
                                        None
                                    } else {
                                        shroud_layer.taper
                                    },
                                    offset: Some(post_block_offset_offset),
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
                    let response = ui.button("Import (Double Click)");
                    if response.double_clicked() {
                        match parse_shroud_text(&self.shroud_import_text, &self.loaded_shapes) {
                            Ok(imported_shroud) => {
                                self.shroud = imported_shroud;
                                self.shroud.iter_mut().for_each(|shroud_layer| {
                                    let pre_block_offset_offset =
                                        shroud_layer.shroud_layer.offset.as_ref().unwrap();
                                    let post_block_offset_offset = do3d_float_from(
                                        pre_block_offset_offset.x.to_f32()
                                            + self.block_container.offset.x,
                                        pre_block_offset_offset.y.to_f32()
                                            + self.block_container.offset.y,
                                        pre_block_offset_offset.z.to_f32(),
                                    );
                                    shroud_layer.shroud_layer.offset =
                                        Some(post_block_offset_offset);
                                });
                                self.just_imported_shroud_from_paste_box_message_option =
                                    Some(ShroudParseResult::Success);
                            }
                            Err(err) => {
                                self.just_imported_shroud_from_paste_box_message_option = Some(err);
                            }
                        }
                    }
                    if let Some(message) = &self.just_imported_shroud_from_paste_box_message_option
                    {
                        ui.label(message.to_string());
                    }
                    if !response.contains_pointer() {
                        self.just_imported_shroud_from_paste_box_message_option = None;
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
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                });
            });
    }

    fn block_scale_settings(&mut self, ui: &mut Ui) -> IsChanged {
        let mut is_changed = IsChanged::default();
        ui.horizontal(|ui| {
            ui.label("scale=");
            let mut scale = self.block_container.block.scale.unwrap();
            let original_scale = scale;
            ui.add(Slider::new(&mut scale, 1..=self.block_container.max_scale));
            if original_scale != scale {
                self.block_container.block.scale = Some(scale);
                self.block_container.vertices = restructure_vertices(
                    self.loaded_shapes
                        .0
                        .iter()
                        .find(|shape| {
                            shape.get_id().unwrap().to_string() == self.block_container.shape_id
                        })
                        .unwrap()
                        .get_nth_scale_vertices(scale as usize - 1),
                );
                is_changed = true;
            }
        });
        is_changed
    }

    fn import_shapes_from_paste_box(&mut self, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ui.ctx(), "import_shape".into(), false)
            .show_header(ui, |ui| {
                ui.strong("Import Shapes from Paste Box");
            })
            .body(|ui| {
                ui.label("READ: Keep all custom shapes in paste box.");
                ui.label("READ: If a custom shape used by a shroud layer is not reimported, nothing significant will happen, but it will just be weird.");
                ui.horizontal(|ui| {
                    let response = ui.button("Import");
                    if response.clicked() {
                        match parse_shapes_text(&self.shapes_import_text) {
                            Ok((imported_shapes, mirror_pairs)) => {
                                self.loaded_shapes = Shapes(
                                    self.loaded_shapes.0[0..VANILLA_SHAPE_COUNT]
                                        .iter()
                                        .cloned()
                                        .chain(imported_shapes.0)
                                        .collect(),
                                );
                                self.loaded_shapes_mirror_pairs = get_loaded_shapes_mirror_pairs(&self.loaded_shapes);
                                self.loaded_shapes_mirror_pairs.extend(mirror_pairs);
                                self.just_imported_shapes_from_paste_box_message_option =
                                    Some(ShapesParseResult::Success);
                            }
                            Err(err) => {
                                self.just_imported_shapes_from_paste_box_message_option = Some(err);
                            }
                        }
                    }
                    if let Some(message) = &self.just_imported_shapes_from_paste_box_message_option
                    {
                        ui.label(message.to_string());
                    }
                    if !response.contains_pointer() {
                        self.just_imported_shapes_from_paste_box_message_option = None;
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
                        TextEdit::multiline(&mut self.shapes_import_text)
                            .code_editor()
                            .desired_width(f32::INFINITY)
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
        if let Some(rgba) = rgba_option
            && response.changed()
        {
            *color = rgba;
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
    shape: &mut Option<ShapeId>,
    shape_id: &mut String,
    vertices: &mut Vec<Pos2>,
    loaded_shapes: &Shapes,
    show_vanilla: &mut bool,
    search_buf: &mut String,
    max_scale: &mut u8,
    scale: u8,
) -> IsChanged {
    let mut is_shape_changed = IsChanged::default();
    ui.horizontal(|ui| {
        ui.label("shape=");
        Popup::from_toggle_button_response(&ui.button(shape_id.as_str()))
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.add(
                    TextEdit::singleline(search_buf)
                        .code_editor()
                        .desired_width(120.0)
                        .hint_text("Search (:"),
                );
                ui.horizontal(|ui| {
                    ui.label("Show Vanilla:");
                    ui.checkbox(show_vanilla, "");
                });
                ScrollArea::vertical()
                    .min_scrolled_height(500.0)
                    .max_height(500.0)
                    .min_scrolled_width(250.0)
                    .max_width(250.0)
                    .show(ui, |ui| {
                        for selectable_shape in if *show_vanilla {
                            &loaded_shapes.0
                        } else {
                            &loaded_shapes.0[VANILLA_SHAPE_COUNT..]
                        } {
                            let selectable_shape_id =
                                selectable_shape.get_id().unwrap().to_string();
                            if search_buf.is_empty()
                                || selectable_shape_id
                                    .to_lowercase()
                                    .contains(&*search_buf.to_lowercase())
                            {
                                let response = ui.selectable_value(
                                    shape_id,
                                    selectable_shape_id.clone(),
                                    selectable_shape_id,
                                );
                                if response.clicked() {
                                    *max_scale = selectable_shape.get_scale_count() as u8;
                                    let scale = u8::min(scale, *max_scale);
                                    *vertices = restructure_vertices(
                                        selectable_shape.get_nth_scale_vertices(scale as usize - 1),
                                    );
                                    *shape = selectable_shape.get_id();
                                    is_shape_changed = true;
                                }
                            }
                        }
                    });
            });
    });
    is_shape_changed
}
