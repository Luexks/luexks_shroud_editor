use std::f32;
use std::f32::consts::SQRT_2;
use std::time::Duration;

use crate::block_container::BlockContainer;
use crate::color_type_conversion::rgba_to_color;
use crate::key_tracker::KeyTracker;
use crate::pos_in_polygon::is_pos_in_polygon;
use crate::render_polygon::render_polygon_fill;
use crate::render_polygon::render_polygon_outline;
use crate::restructure_vertices::restructure_vertices;
use crate::selection_type::SelectionType;
use crate::shroud_layer_container::ShroudLayerContainer;
use crate::shroud_layer_interaction::ShroudLayerInteraction;
use arboard::Clipboard;
use egui::Checkbox;
use egui::Color32;
use egui::ComboBox;
use egui::Context;
use egui::DragValue;
use egui::Frame;
use egui::Grid;
use egui::Key;
use egui::Pos2;
use egui::Rect;
use egui::Rgba;
use egui::ScrollArea;
use egui::Slider;
use egui::Stroke;
use egui::Ui;
use egui::color_picker::Alpha;
use egui::color_picker::color_edit_button_rgba;
use egui::pos2;
use egui::scroll_area::ScrollBarVisibility;
use egui::UiBuilder;
use egui_knob::Knob;
use egui_knob::KnobStyle;
use itertools::Itertools;
use luexks_reassembly::blocks::shroud::Shroud;
use luexks_reassembly::blocks::shroud_layer::ShroudLayerColor;
use luexks_reassembly::shapes::shape_id::ShapeId;
use luexks_reassembly::shapes::shapes::Shapes;
use luexks_reassembly::utility::angle::Angle;
use luexks_reassembly::utility::component_formatting::format_component;
use luexks_reassembly::utility::display_oriented_math::DisplayOriented3D;
use luexks_reassembly::utility::display_oriented_math::do2d_float_from;
use luexks_reassembly::utility::display_oriented_math::do3d_float_from;
use luexks_reassembly::utility::display_oriented_math::don_float_from;
use parse_vanilla_shapes::get_vanilla_shapes;

const FILL_COLOR_GRADIENT_TIME: f32 = 4.0;

pub struct ShroudEditor {
    pub block_container: BlockContainer,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_layer_interaction: ShroudLayerInteraction,
    pub zoom: f32,
    pub grid_size: f32,
    pub grid_enabled: bool,
    pub snap_to_grid: bool,
    pub angle_snap: f32,
    pub angle_snap_enabled: bool,
    pub pan: Pos2,
    pub key_tracker: KeyTracker,
    pub loaded_shapes: Shapes,
    pub just_exported_to_clipboard_success_option: Option<bool>,
    pub fill_color_gradient: f32,
    fill_color_gradient_increasing: bool,
    fill_color_gradient_delta_enabled: bool,
    last_frame_time: f64,
    dt: f64,
    only_show_selected_shroud_layers: bool,
}

impl ShroudEditor {
    pub fn world_pos_to_screen_pos(&self, position: Pos2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2 {
            x: center.x + (position.x + self.pan.x) * self.zoom,
            y: center.y + (position.y + self.pan.y) * self.zoom,
        }
    }

    pub fn screen_pos_to_world_pos(&self, position: Pos2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2 {
            x: (position.x - center.x) / self.zoom - self.pan.x,
            y: (position.y - center.y) / self.zoom - self.pan.y,
        }
    }

    pub fn positions_to_screen_positions(&self, positions: &Vec<Pos2>, rect: Rect) -> Vec<Pos2> {
        positions
            .iter()
            .map(|position| self.world_pos_to_screen_pos(*position, rect))
            .collect()
    }

    pub fn pan_controls(&mut self) {
        let speed: f32 = 1000.0 * self.dt as f32 / self.zoom;
        let mut delta = Pos2::default();
        if self.key_tracker.is_held(Key::W) {
            delta.y += speed;
        }
        if self.key_tracker.is_held(Key::S) {
            delta.y -= speed;
        }
        if self.key_tracker.is_held(Key::D) {
            delta.x -= speed;
        }
        if self.key_tracker.is_held(Key::A) {
            delta.x += speed;
        }
        if delta.x != 0.0 && delta.y != 0.0 {
            delta *= SQRT_2 * 0.5;
        }
        self.pan = pos2(self.pan.x + delta.x, self.pan.y + delta.y);

        if let ShroudLayerInteraction::Dragging { selection, .. } = &self.shroud_layer_interaction {
            selection.iter().for_each(|index| {
                let old_offset = self.shroud[*index].shroud_layer.offset.clone().unwrap();
                self.shroud[*index].shroud_layer.offset = Some(do3d_float_from(
                    old_offset.x.to_f32() - delta.x,
                    old_offset.y.to_f32() - delta.y,
                    old_offset.z.to_f32()
                ));
            });
        }
    }

    pub fn zoom_at_position(&mut self, screen_pos: Pos2, rect: Rect, delta: f32) {
        let delta = delta * 5.0;
        let old_zoom = self.zoom;

        self.zoom = (self.zoom * (1.0 + delta * 0.1)).clamp(0.1, 40.0);

        // Calculate world position before zoom
        let center = rect.center();
        let before_x = (screen_pos.x - center.x) / old_zoom;
        let before_y = (screen_pos.y - center.y) / old_zoom;

        // Calculate world position after zoom
        let after_x = (screen_pos.x - center.x) / self.zoom;
        let after_y = (screen_pos.y - center.y) / self.zoom;

        // Adjust panning to keep the world position constant under cursor
        self.pan.x += after_x - before_x;
        self.pan.y += after_y - before_y;
    }

    fn draw_grid(&self, ui: &mut Ui, rect: Rect) {
        let stroke = Stroke::new(1.0, Color32::from_rgb(0, 0, 150));
        let axis_stroke = Stroke::new(1.0, Color32::from_rgb(255, 0, 255));

        let grid_size = self.grid_size * 2.0_f32.powi((4.5 / (self.zoom + 1.0)).round() as i32);

        let world_min = self.screen_pos_to_world_pos(rect.min, rect);
        let world_max = self.screen_pos_to_world_pos(rect.max, rect);
        let first_vertical_line_x = (world_min.x / grid_size).ceil() * grid_size;
        let first_horizontal_line_y = (world_min.y / grid_size).ceil() * grid_size;
        let vertical_grid_line_count = ((-world_min.x + world_max.x) / grid_size).ceil() as usize;
        let horizontal_grid_line_count = ((-world_min.y + world_max.y) / grid_size).ceil() as usize;

        let mut y_axis_x_option = None;
        let mut x_axis_y_option = None;

        let y_top = world_min.y;
        let y_bottom = world_max.y;
        let x_left = world_min.x;
        let x_right = world_max.x;

        (0..vertical_grid_line_count).for_each(|index| {
            let x = first_vertical_line_x + grid_size * index as f32;
            let pos_top = self.world_pos_to_screen_pos(pos2(x, y_top), rect);
            let pos_bottom = self.world_pos_to_screen_pos(pos2(x, y_bottom), rect);
            if x.abs() < f32::EPSILON {
                y_axis_x_option = Some(x);
            } else {
                ui.painter().line_segment([pos_top, pos_bottom], stroke);
            }
        });

        (0..horizontal_grid_line_count).for_each(|index| {
            let y = first_horizontal_line_y + grid_size * index as f32;
            let pos_left = self.world_pos_to_screen_pos(pos2(x_left, y), rect);
            let pos_right = self.world_pos_to_screen_pos(pos2(x_right, y), rect);
            if y.abs() < f32::EPSILON {
                x_axis_y_option = Some(y)
            } else {
                ui.painter().line_segment([pos_left, pos_right], stroke);
            }
        });

        if let Some(y_axis_x) = y_axis_x_option {
            let pos_top = self.world_pos_to_screen_pos(pos2(y_axis_x, y_top), rect);
            let pos_bottom = self.world_pos_to_screen_pos(pos2(y_axis_x, y_bottom), rect);
            ui.painter()
                .line_segment([pos_top, pos_bottom], axis_stroke);
        }
        if let Some(x_axis_y) = x_axis_y_option {
            let pos_left = self.world_pos_to_screen_pos(pos2(x_left, x_axis_y), rect);
            let pos_right = self.world_pos_to_screen_pos(pos2(x_right, x_axis_y), rect);
            ui.painter()
                .line_segment([pos_left, pos_right], axis_stroke);
        }
        ui.painter().line_segment(
            [
                self.world_pos_to_screen_pos(pos2(50.0, 0.0), rect),
                self.world_pos_to_screen_pos(pos2(40.0, 10.0), rect),
            ],
            axis_stroke,
        );
        ui.painter().line_segment(
            [
                self.world_pos_to_screen_pos(pos2(50.0, 0.0), rect),
                self.world_pos_to_screen_pos(pos2(40.0, -10.0), rect),
            ],
            axis_stroke,
        );
    }

    fn left_panel(&mut self, ctx: &Context) {
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
                    self.shroud.push(ShroudLayerContainer::default());
                }
                ui.horizontal(|ui| {
                    ui.label("Only Show Selected:");
                    ui.checkbox(&mut self.only_show_selected_shroud_layers, "");
                });
                self.shroud_list(ui);
            });
    }

    fn visual_panel(&mut self, ctx: &Context) {
        let central_panel_frame = Frame::new().inner_margin(0.0);

        egui::CentralPanel::default()
            .frame(central_panel_frame)
            .show(ctx, |ui| {
                let mouse_pos = ui.input(|i| i.pointer.hover_pos());
                let response =
                    ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect = response.rect;

                self.draw_grid(ui, rect);

                self.render_shroud(mouse_pos, ui, rect);
                self.shroud_layer_interaction.selection().iter().for_each(|index| {
                    if *index < self.shroud.len() {
                        let offset = self.shroud[*index].shroud_layer.offset.clone().unwrap();
                        let gizmo_center = pos2(
                            offset.x.to_f32(),
                            offset.y.to_f32(),
                        );
                        let (gizmo_pos_top_left, gizmo_pos_bottom_right) = (
                            self.world_pos_to_screen_pos(gizmo_center, rect),
                            self.world_pos_to_screen_pos(gizmo_center, rect),
                        );
                        let gizmo_size = 20.0;
                        self.angle_gizmo(ui, gizmo_pos_top_left, gizmo_pos_bottom_right, gizmo_size, *index);
                        self.size_gizmo(ui, gizmo_pos_top_left, gizmo_pos_bottom_right, gizmo_size, *index, self.grid_size, self.snap_to_grid);
                    }
                });

                self.zoom(ui, rect);

                let mouse_pos = response.interact_pointer_pos();
                if let Some(mouse_pos) = mouse_pos {
                    // if response.clicked() {
                    // if ui.input(|i| i.pointer.primary_released()) {
                    if ui.input(|i| i.pointer.primary_pressed()) {
                        if let Some(shroud_that_would_be_selected_index) =
                            self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect)
                        {
                            // self.shroud_layer_interaction = ShroudLayerInteraction::Inaction { selection: self.shroud_layer_interaction.selection().iter().chain(std::iter::once(&shroud_that_would_be_selected_index)).map(|index| *index).collect() }
                            if ctx.input(|i| i.modifiers.shift) {
                                if !self
                                    .shroud_layer_interaction
                                    .selection()
                                    .contains(&shroud_that_would_be_selected_index)
                                {
                                    self.shroud_layer_interaction =
                                        ShroudLayerInteraction::Inaction {
                                            selection: self
                                                .shroud_layer_interaction
                                                .selection()
                                                .iter()
                                                .copied()
                                                .chain(std::iter::once(
                                                    shroud_that_would_be_selected_index,
                                                ))
                                                .collect(),
                                        };
                                }
                            } else {
                                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                    selection: vec![shroud_that_would_be_selected_index],
                                };
                            }
                        } else {
                            self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                selection: Vec::new(),
                            };
                        }
                    }

                    if response.drag_started() {
                        if !self.shroud_layer_interaction.selection().is_empty() {
                            self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                                drag_start_pos: mouse_pos,
                                selection: self.shroud_layer_interaction.selection(),
                            };
                        }
                        // if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                        //     self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                        //         drag_start_pos: mouse_pos,
                        //         selection: vec![shroud_that_would_be_selected_index],
                        //     };
                        // }
                    }
                }

                if let ShroudLayerInteraction::Dragging {
                    drag_start_pos: _,
                    selection,
                } = &self.shroud_layer_interaction
                {
                    let delta = ui.input(|i| i.pointer.delta()) / self.zoom;
                    selection.iter().for_each(|selected_index| {
                        let old_offset = self
                            .shroud
                            .get(*selected_index)
                            .unwrap()
                            .shroud_layer
                            .offset
                            .clone()
                            .unwrap();
                        let (x, y) = (
                            don_float_from(delta.x + old_offset.x.to_f32()),
                            don_float_from(delta.y + old_offset.y.to_f32()),
                        );
                        self.shroud[*selected_index].shroud_layer.offset =
                            Some(DisplayOriented3D {
                                x: x,
                                y: y,
                                z: old_offset.z,
                            });
                    });
                    if response.drag_stopped() {
                        selection.iter().for_each(|selected_index| {
                            let old_offset = self
                                .shroud
                                .get(*selected_index)
                                .unwrap()
                                .shroud_layer
                                .offset
                                .clone()
                                .unwrap();
                            if self.snap_to_grid {
                                let (x, y) = (
                                    don_float_from(
                                        (delta.x + old_offset.x.to_f32() / self.grid_size).round()
                                            * self.grid_size,
                                    ),
                                    don_float_from(
                                        (delta.y + old_offset.y.to_f32() / self.grid_size).round()
                                            * self.grid_size,
                                    ),
                                );
                                self.shroud[*selected_index].shroud_layer.offset =
                                    Some(DisplayOriented3D {
                                        x: x,
                                        y: y,
                                        z: old_offset.z,
                                    });
                            }
                        });
                        self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                            selection: self.shroud_layer_interaction.selection(),
                        }
                    }
                }
            });
    }

    fn angle_gizmo(&mut self, ui: &mut Ui, gizmo_pos_top_left: Pos2, gizmo_pos_bottom_right: Pos2, gizmo_size: f32, index: usize) {
        let gizmo_pos_top_left = pos2(
            gizmo_pos_top_left.x - gizmo_size,
            gizmo_pos_top_left.y - gizmo_size,
        );
        let gizmo_pos_bottom_right = pos2(
            gizmo_pos_bottom_right.x + gizmo_size,
            gizmo_pos_bottom_right.y + gizmo_size,
        );
        let gizmo_rect = Rect::from_two_pos(gizmo_pos_top_left, gizmo_pos_bottom_right);
        ui.scope_builder(
            UiBuilder::new().max_rect(gizmo_rect),
            |ui| {
            egui::Frame::new()
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    let angle = self.shroud[index].shroud_layer.angle.clone().unwrap().as_degrees().get_value();
                    let angle = angle_knob_settings(ui, angle, self.angle_snap, self.angle_snap_enabled);
                    self.shroud[index].shroud_layer.angle = Some(Angle::Degree(angle));
                });
        });
    }

    fn size_gizmo(&mut self, ui: &mut Ui, gizmo_pos_top_left: Pos2, gizmo_pos_bottom_right: Pos2, gizmo_size: f32, index: usize, grid_size: f32, snap_to_grid: bool) {
        let size = self.shroud[index].shroud_layer.size.clone().unwrap();
        let mut width = size.x.to_f32();
        let mut height = size.y.to_f32();

        let is_square = self.shroud[index].shape_id == "SQUARE";
        const GIZMO_DISTANCE: f32 = 50.0;
        let height_gizmo_pos_top_left = if is_square {
            let angle = self.shroud[index].shroud_layer.angle.clone().unwrap().as_radians().get_value() + f32::consts::PI * 0.5;
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE * angle.cos(),
                gizmo_pos_top_left.y - gizmo_size + GIZMO_DISTANCE * angle.sin(),
            )
        } else {
            pos2(
                gizmo_pos_top_left.x - gizmo_size,
                gizmo_pos_top_left.y - gizmo_size - GIZMO_DISTANCE,
            )
        };
        let height_gizmo_pos_bottom_right = if is_square {
            let angle = self.shroud[index].shroud_layer.angle.clone().unwrap().as_radians().get_value() + f32::consts::PI * 0.5;
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size + GIZMO_DISTANCE * angle.cos(),
                gizmo_pos_bottom_right.y - gizmo_size + GIZMO_DISTANCE * angle.sin(),
            )
        } else {
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size,
                gizmo_pos_bottom_right.y - gizmo_size - GIZMO_DISTANCE,
            )
        };
        let gizmo_rect = Rect::from_two_pos(height_gizmo_pos_top_left, height_gizmo_pos_bottom_right);
        ui.scope_builder(
            UiBuilder::new().max_rect(gizmo_rect),
            |ui| {
            egui::Frame::new()
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    let xy_speed = if snap_to_grid { grid_size / 2.0 } else { 0.05 };
                    ui.add(DragValue::new(&mut height).speed(xy_speed));
                });
        });
        // let angle = angle;
        // let gizmo_distance = 20.0;
        let width_gizmo_pos_top_left = if is_square {
            let angle = self.shroud[index].shroud_layer.angle.clone().unwrap().as_radians().get_value();
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE * angle.cos(),
                gizmo_pos_top_left.y - gizmo_size + GIZMO_DISTANCE * angle.sin(),
            )
        } else {
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE,
                gizmo_pos_top_left.y - gizmo_size,
            )
        };
        let width_gizmo_pos_bottom_right = if is_square {
            let angle = self.shroud[index].shroud_layer.angle.clone().unwrap().as_radians().get_value();
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size + GIZMO_DISTANCE * angle.cos(),
                gizmo_pos_bottom_right.y - gizmo_size + GIZMO_DISTANCE * angle.sin(),
            )
        } else {
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size + GIZMO_DISTANCE,
                gizmo_pos_bottom_right.y - gizmo_size,
            )
        };
        let gizmo_rect = Rect::from_two_pos(width_gizmo_pos_top_left, width_gizmo_pos_bottom_right);
        ui.scope_builder(
            UiBuilder::new().max_rect(gizmo_rect),
            |ui| {
            egui::Frame::new()
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    let xy_speed = if snap_to_grid { grid_size / 2.0 } else { 0.05 };
                    ui.add(DragValue::new(&mut width).speed(xy_speed));
                });
        });
        self.shroud[index].shroud_layer.size = Some(do2d_float_from(width, height));
    }

    fn is_shroud_layer_index_selected(&self, index: usize) -> bool {
        self.shroud_layer_interaction
            .is_shroud_layer_index_selected(index)
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

    fn update_dt(&mut self, ctx: &Context) {
        let now = ctx.input(|i| i.time);
        self.dt = now - self.last_frame_time;
        self.last_frame_time = now;
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

    fn is_shroud_hovered(
        &self,
        mouse_pos: Option<Pos2>,
        shroud_layer_container: &ShroudLayerContainer,
        rect: Rect,
    ) -> bool {
        if let Some(mouse_pos) = mouse_pos {
            let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
            if is_pos_in_polygon(
                mouse_pos,
                self.positions_to_screen_positions(
                    &shroud_layer_container
                        .get_shroud_layer_vertices()
                        .iter()
                        .map(|vertex| {
                            Pos2::new(vertex.x + offset.x.to_f32(), vertex.y + offset.y.to_f32())
                        })
                        .collect::<Vec<_>>(),
                    rect,
                ),
            ) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn zoom(&mut self, ui: &mut Ui, rect: Rect) {
        if let Some(pos) = ui.ctx().pointer_interact_pos() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 && rect.contains(pos) {
                self.zoom_at_position(pos, rect, scroll_delta * 0.01);
            }
        }
    }

    #[rustfmt::skip]
    fn render_shroud(&self, mouse_pos: Option<Pos2>, ui: &mut Ui, rect: Rect) {
        if !self.shroud.is_empty() {
            let render_pipeline = self.shroud.iter()
                .enumerate()
                .sorted_by(|(_, shroud_layer_container_1), (_, shroud_layer_container_2)| {
                    let z1 = shroud_layer_container_1.shroud_layer.offset.clone().unwrap().z.to_f32();
                    let z2 = shroud_layer_container_2.shroud_layer.offset.clone().unwrap().z.to_f32();
                    z1.partial_cmp(&z2).unwrap()
                })
                .collect::<Vec<_>>();
            let mut current_z = render_pipeline.first().unwrap().1.shroud_layer.offset.clone().unwrap().z.to_f32();
            let mut next_outline_render_start_index = usize::default();
            render_pipeline.iter()
                .enumerate()
                .for_each(|(pipeline_index, (index, shroud_layer_container))| {
                    let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();

                    let is_clipping_and_on_top = !(offset.z.to_f32() > current_z) && pipeline_index == render_pipeline.len() - 1;
                    if  is_clipping_and_on_top {
                        render_polygon_fill(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            offset.clone(),
                            shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
                            shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
                        );
                    }

                    let is_above_last = offset.z.to_f32() > current_z;
                    let is_on_top = pipeline_index == render_pipeline.len() - 1;
                    if is_above_last || is_on_top {
                        render_pipeline[next_outline_render_start_index..pipeline_index].iter()
                            .for_each(|(index, shroud_layer_container)| {
                                let is_hovered = if let Some(mouse_pos) = mouse_pos {
                                    if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                                        if *index == shroud_that_would_be_selected_index {
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };
                                let is_selected = self.is_shroud_layer_index_selected(*index);
                                let selection_type_option = match (is_hovered, is_selected) {
                                    (true, _) => Some(SelectionType::Hovered),
                                    (false, true) => Some(SelectionType::Selected),
                                    _ => None,
                                };
                                render_polygon_outline(
                                    ui.painter(),
                                    self,
                                    rect,
                                    shroud_layer_container.get_shroud_layer_vertices(),
                                    shroud_layer_container.shroud_layer.offset.clone().unwrap(),
                                    shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
                                    selection_type_option.clone(),
                                );
                            });
                        next_outline_render_start_index = pipeline_index;
                    }
 
                    let is_not_clipping_and_on_top = offset.z.to_f32() > current_z && pipeline_index == render_pipeline.len() - 1;
                    let is_below_top = pipeline_index != render_pipeline.len() - 1;
                    if is_not_clipping_and_on_top || is_below_top {
                        render_polygon_fill(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            offset.clone(),
                            shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
                            shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
                        );
                    }
                    current_z = offset.z.to_f32();

                    if is_on_top {
                        let is_hovered = if let Some(mouse_pos) = mouse_pos {
                            if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                                if *index == shroud_that_would_be_selected_index {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        let is_selected = self.is_shroud_layer_index_selected(*index);
                        let selection_type_option = match (is_hovered, is_selected) {
                            (true, _) => Some(SelectionType::Hovered),
                            (false, true) => Some(SelectionType::Selected),
                            _ => None,
                        };
                        render_polygon_outline(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            shroud_layer_container.shroud_layer.offset.clone().unwrap(),
                            shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
                            selection_type_option.clone(),
                        );
                    }
                });

            // render_pipeline.for_each(|(index, shroud_layer_container)| {
            //     let is_hovered = self.is_shroud_hovered(mouse_pos, shroud_layer_container, rect);
            //     let is_selected = self.is_shroud_layer_index_selected(index);

            //     let selection_type_option = match (is_hovered, is_selected) {
            //         (true, _) => Some(SelectionType::Hovered),
            //         (false, true) => Some(SelectionType::Selected),
            //         _ => None,
            //     };
            //     render_polygon(
            //         ui.painter(),
            //         self,
            //         rect,
            //         shroud_layer_container.get_shroud_layer_vertices(),
            //         shroud_layer_container.shroud_layer.offset.clone().unwrap(),
            //         shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
            //         shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
            //         shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
            //         selection_type_option,
            //     );
            // });
        }
    }

    fn get_shroud_that_would_be_selected_index_option(
        &self,
        mouse_pos: Pos2,
        rect: Rect,
    ) -> Option<usize> {
        let mut dragged_on_shroud_layer_data: Vec<(usize, f32)> = Vec::default();
        self.shroud
            .iter()
            .enumerate()
            .for_each(|(index, shroud_layer_container)| {
                if self.is_shroud_hovered(Some(mouse_pos), shroud_layer_container, rect) {
                    dragged_on_shroud_layer_data.push((
                        index,
                        shroud_layer_container
                            .shroud_layer
                            .offset
                            .clone()
                            .unwrap()
                            .z
                            .to_f32(),
                    ));
                }
            });
        dragged_on_shroud_layer_data.sort_by(|(_, z1), (_, z2)| z2.partial_cmp(z1).unwrap());
        if let Some((index, _)) = dragged_on_shroud_layer_data.first() {
            Some(*index)
        } else {
            None
        }
    }

    fn shroud_list(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink(false)
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {
                if self.shroud.is_empty() {
                    ui.label("No shrouds :(");
                } else {
                    self.shroud.iter_mut().enumerate().for_each(
                        |(index, shroud_layer_container)| {
                            let is_selected = self
                                .shroud_layer_interaction
                                .is_shroud_layer_index_selected(index);
                            if !self.only_show_selected_shroud_layers || is_selected {
                                shroud_layer_settings(
                                    is_selected,
                                    ui,
                                    index,
                                    shroud_layer_container,
                                    &mut self.shroud_layer_interaction,
                                    &self.loaded_shapes,
                                    self.snap_to_grid,
                                    self.grid_size,
                                    self.angle_snap,
                                    self.angle_snap_enabled,
                                );
                            }
                        },
                    );
                }

                ui.add_space(4.0);
            });
    }

    fn delete_shroud_layers(&mut self) {
        self.shroud = self
            .shroud
            .iter()
            .filter(|shroud_layer_container| !shroud_layer_container.delete_next_frame)
            .cloned()
            .collect();
    }

    fn hotkey_shroud_deletion(&mut self, ctx: &Context) {
        let shroud_delete_hotkey_pressed = ctx.input(|i| i.key_pressed(Key::R));
        // let shroud_delete_hotkey_pressed = ctx.input(|i| i.key_pressed(Key::Backspace))
        //     || ctx.input(|i| i.key_pressed(Key::Delete))
        //     || ctx.input(|i| i.key_pressed(Key::R));
        if shroud_delete_hotkey_pressed {
            let selection = self.shroud_layer_interaction.selection();
            let mut descending_selection = selection;
            descending_selection.sort_by(|index_a, index_b| index_b.cmp(index_a));
            descending_selection.iter().for_each(|index| {
                self.shroud.remove(*index);
            });
        }
    }
}

impl Default for ShroudEditor {
    fn default() -> Self {
        Self {
            block_container: Default::default(),
            shroud: Vec::default(),
            shroud_layer_interaction: ShroudLayerInteraction::Inaction {
                selection: Vec::new(),
            },
            // zoom: 1,
            zoom: 1.0,
            grid_size: 2.5,
            grid_enabled: true,
            snap_to_grid: true,
            angle_snap: 10.0,
            angle_snap_enabled: true,
            pan: Pos2::new(0.0, 0.0),
            key_tracker: KeyTracker::default(),
            loaded_shapes: get_vanilla_shapes(),
            just_exported_to_clipboard_success_option: None,
            fill_color_gradient: 0.0,
            fill_color_gradient_increasing: true,
            fill_color_gradient_delta_enabled: true,
            last_frame_time: 0.0,
            dt: 0.0,
            only_show_selected_shroud_layers: false,
        }
    }
}

impl eframe::App for ShroudEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_dt(ctx);
        self.key_tracker.update(ctx);
        self.pan_controls();

        self.left_panel(ctx);
        self.visual_panel(ctx);

        self.delete_shroud_layers();
        self.hotkey_shroud_deletion(ctx);

        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / 60.0));
    }
}

fn shape_combo_box(
    ui: &mut Ui,
    index: &str,
    shape: &mut Option<ShapeId>,
    shape_id: &mut String,
    vertices: &mut Vec<Pos2>,
    loaded_shapes: &Shapes,
) {
    ui.horizontal(|ui| {
        ui.label("shape=");
        ComboBox::from_id_salt(index.to_string())
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

fn block_color_setting(ui: &mut Ui, color: &mut Rgba) {
    color_edit_button_rgba(ui, color, Alpha::OnlyBlend);
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
        } else {
            if ui.button("Deselect").clicked() {
                *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                    selection: shroud_layer_interaction
                        .selection()
                        .iter()
                        .copied()
                        .filter(|selection_index| *selection_index != index)
                        .collect(),
                };
            }
        }
        if ui.button("Delete (Double Click)").double_clicked() {
            shroud_layer_container.delete_next_frame = true;
        }
    });
}

fn shroud_layer_settings(
    is_selected: bool,
    ui: &mut Ui,
    index: usize,
    shroud_layer_container: &mut ShroudLayerContainer,
    shroud_layer_interaction: &mut ShroudLayerInteraction,
    loaded_shapes: &Shapes,
    snap_to_grid: bool,
    grid_size: f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
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
                    shroud_layer_container,
                    shroud_layer_interaction,
                );
                shape_combo_box(
                    ui,
                    &index.to_string(),
                    &mut shroud_layer_container.shroud_layer.shape,
                    &mut shroud_layer_container.shape_id,
                    &mut shroud_layer_container.vertices,
                    loaded_shapes,
                );

                let xy_speed = if snap_to_grid { grid_size / 2.0 } else { 0.05 };
                ui.horizontal(|ui| {
                    let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
                    let mut x = offset.x.to_f32();
                    let mut y = offset.y.to_f32();
                    let mut z = offset.z.to_f32();
                    ui.label("offset={");
                    ui.add(DragValue::new(&mut x).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut y).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut z).speed(0.005));
                    ui.label("}");
                    shroud_layer_container.shroud_layer.offset = Some(do3d_float_from(x, y, z));
                });
                ui.horizontal(|ui| {
                    let size = shroud_layer_container.shroud_layer.size.clone().unwrap();
                    let mut width = size.x.to_f32();
                    let mut height = size.y.to_f32();
                    ui.label("size={");
                    ui.add(DragValue::new(&mut width).speed(xy_speed));
                    ui.label(",");
                    ui.add(DragValue::new(&mut height).speed(xy_speed));
                    ui.label("}");
                    shroud_layer_container.shroud_layer.size = Some(do2d_float_from(width, height));
                });
                ui.horizontal(|ui| {
                    full_angle_settings(ui, shroud_layer_container, angle_snap, angle_snap_enabled);
                });

                let mut color_1 = shroud_layer_container.shroud_layer.color_1.clone().unwrap();
                let mut color_2 = shroud_layer_container.shroud_layer.color_2.clone().unwrap();
                let mut line_color = shroud_layer_container
                    .shroud_layer
                    .line_color
                    .clone()
                    .unwrap();
                Grid::new(index.to_string()).show(ui, |ui| {
                    shroud_color_setting(ui, &mut color_1, "tri_color_id=");
                    shroud_color_setting(ui, &mut color_2, "tri_color1_id=");
                    shroud_color_setting(ui, &mut line_color, "line_color_id=");
                });
                shroud_layer_container.shroud_layer.color_1 = Some(color_1);
                shroud_layer_container.shroud_layer.color_2 = Some(color_2);
                shroud_layer_container.shroud_layer.line_color = Some(line_color);

                if shroud_layer_container.shape_id == "SQUARE" {
                    ui.horizontal(|ui| {
                        let mut taper = shroud_layer_container.shroud_layer.taper.unwrap_or(1.0);
                        ui.label("taper=");
                        ui.add(DragValue::new(&mut taper).speed(0.025));
                        shroud_layer_container.shroud_layer.taper = Some(taper);
                    });
                }
            });
    });
}

fn full_angle_settings(ui: &mut Ui, shroud_layer_container: &mut ShroudLayerContainer, angle_snap: f32, angle_snap_enabled: bool) {
    let mut angle = shroud_layer_container
        .shroud_layer
        .angle
        .clone()
        .unwrap()
        .as_degrees()
        .get_value();

    let angle_speed = if angle_snap_enabled { angle_snap } else { 1.0 };
    ui.label("angle=");
    ui.add(DragValue::new(&mut angle).speed(angle_speed));
    ui.label("*pi/180");
    let angle = angle_knob_settings(ui, angle, angle_snap, angle_snap_enabled);
    shroud_layer_container.shroud_layer.angle = Some(Angle::Degree(angle));
}

fn angle_knob_settings(ui: &mut Ui, angle: f32, angle_snap: f32, angle_snap_enabled: bool) -> f32 {
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
    let angle = if angle < 0.0 { angle + 360.0 } else { angle };
    angle
}