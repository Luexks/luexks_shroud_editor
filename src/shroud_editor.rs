use std::f32::consts::SQRT_2;
use std::time::Duration;

use crate::block_container::BlockContainer;
use crate::key_tracker::KeyTracker;
use crate::pos_in_polygon::is_pos_in_polygon;
use crate::restructure_vertices::restructure_vertices;
use crate::selection_type::SelectionType;
use crate::shroud_layer_container;
use crate::shroud_layer_container::ShroudLayerContainer;
use crate::shroud_layer_interaction::ShroudLayerInteraction;
use crate::render_polygon::render_polygon;
use arboard::Clipboard;
use egui::Checkbox;
use egui::Color32;
use egui::ComboBox;
use egui::Context;
use egui::DragValue;
use egui::Frame;
use egui::Grid;
use egui::Key;
use egui::Label;
use egui::Pos2;
use egui::Rect;
use egui::Rgba;
use egui::Slider;
use egui::Stroke;
use egui::Ui;
use egui::color_picker::Alpha;
use egui::color_picker::color_edit_button_rgba;
use egui::pos2;
use egui_knob::Knob;
use egui_knob::KnobStyle;
use luexks_reassembly::blocks::block::Block;
use luexks_reassembly::blocks::shroud::Shroud;
use luexks_reassembly::blocks::shroud_layer::ShroudLayerColor;
use luexks_reassembly::shapes::shape_id::ShapeId;
use luexks_reassembly::shapes::shapes::Shapes;
use luexks_reassembly::utility::angle::Angle;
use luexks_reassembly::utility::color::Color;
use luexks_reassembly::utility::component_formatting::format_component;
use luexks_reassembly::utility::display_oriented_math::DisplayOriented3D;
use luexks_reassembly::utility::display_oriented_math::do2d_float_from;
use luexks_reassembly::utility::display_oriented_math::do3d_float_from;
use luexks_reassembly::utility::display_oriented_math::don_float_from;
use parse_vanilla_shapes::get_vanilla_shapes;
use crate::color_type_conversion::color_to_rgba;
use crate::color_type_conversion::rgba_to_color;

const FILL_COLOR_GRADIENT_TIME: f32 = 4.0;

pub struct ShroudEditor {
    pub block_container: BlockContainer,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_layer_interaction: ShroudLayerInteraction,
    pub zoom: f32,
    pub grid_size: f32,
    pub grid_enabled: bool,
    pub snap_to_grid: bool,
    pub pan: Pos2,
    pub key_tracker: KeyTracker,
    pub loaded_shapes: Shapes,
    pub just_exported_to_clipboard_success_option: Option<bool>,
    pub fill_color_gradient: f32,
    fill_color_gradient_increasing: bool,
    fill_color_gradient_delta_enabled: bool,
    last_frame_time: f64,
    dt: f64,
}

impl ShroudEditor {
    pub fn new() -> Self {
        ShroudEditor::default()
    }

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
    }

    pub fn zoom_at_position(&mut self, screen_pos: Pos2, rect: Rect, delta: f32) {
        let delta = delta * 5.0;
        let old_zoom = self.zoom;

        self.zoom = (self.zoom * (1.0 + delta * 0.1)).clamp(0.1, 10.0);

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
                self.grid_settings(ui);
                self.fill_color_gradient_setting(ui);
                self.block_settings(ui);
                ui.heading("Shroud Layers:");
                if ui.button("Add Shroud Layer").clicked() {
                    self.shroud.push(ShroudLayerContainer::default());
                }
                egui::Frame::new()
                    .inner_margin(0.0)
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        self.shroud.iter_mut().enumerate().for_each(
                            |(index, shroud_layer_container)| {
                                let is_selected = self
                                    .shroud_layer_interaction
                                    .is_shroud_layer_index_selected(index);
                                ui.vertical(|ui| {
                                    egui::Frame::new()
                                        .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                                        .inner_margin(6.0)
                                        .corner_radius(4.0)
                                        .stroke(Stroke::new(
                                            if is_selected { 1.0 } else { 0.0 },
                                            Color32::BLACK,
                                        ))
                                        .show(ui, |ui| {
                                            shape_combo_box(
                                                ui,
                                                &index.to_string(),
                                                &mut shroud_layer_container.shroud_layer.shape,
                                                &mut shroud_layer_container.shape_id,
                                                &mut shroud_layer_container.vertices,
                                                &self.loaded_shapes,
                                            );

                                            let xy_speed = if self.snap_to_grid {
                                                self.grid_size / 2.0
                                            } else {
                                                0.05
                                            };
                                            ui.horizontal(|ui| {
                                                let offset = shroud_layer_container
                                                    .shroud_layer
                                                    .offset
                                                    .clone()
                                                    .unwrap();
                                                let mut x = offset.x.to_f32();
                                                let mut y = offset.y.to_f32();
                                                let mut z = offset.z.to_f32();
                                                ui.label("offset={");
                                                ui.add(DragValue::new(&mut x).speed(xy_speed));
                                                ui.label(",");
                                                ui.add(DragValue::new(&mut y).speed(xy_speed));
                                                ui.label(",");
                                                ui.add(DragValue::new(&mut z).speed(0.05));
                                                ui.label("}");
                                                shroud_layer_container.shroud_layer.offset =
                                                    Some(do3d_float_from(x, y, z));
                                            });
                                            ui.horizontal(|ui| {
                                                let size = shroud_layer_container
                                                    .shroud_layer
                                                    .size
                                                    .clone()
                                                    .unwrap();
                                                let mut width = size.x.to_f32();
                                                let mut height = size.y.to_f32();
                                                ui.label("size={");
                                                ui.add(DragValue::new(&mut width).speed(xy_speed));
                                                ui.label(",");
                                                ui.add(DragValue::new(&mut height).speed(xy_speed));
                                                ui.label("}");
                                                shroud_layer_container.shroud_layer.size =
                                                    Some(do2d_float_from(width, height));
                                            });
                                            ui.horizontal(|ui| {
                                                let mut angle = shroud_layer_container
                                                    .shroud_layer
                                                    .angle
                                                    .clone()
                                                    .unwrap()
                                                    .as_degrees()
                                                    .get_value();

                                                ui.label("angle=");
                                                ui.add(DragValue::new(&mut angle).speed(5.0));
                                                ui.label("*pi/180");

                                                let mut angle = angle + 90.0;
                                                ui.add(
                                                    Knob::new(
                                                        &mut angle,
                                                        0.0,
                                                        360.0 * 1.5,
                                                        KnobStyle::Wiper,
                                                    )
                                                    .with_sweep_range(0.5, 1.5)
                                                    .with_background_arc(false),
                                                );
                                                let angle = (angle - 90.0) % 360.0;
                                                let angle =
                                                    if angle < 0.0 { angle + 360.0 } else { angle };
                                                shroud_layer_container.shroud_layer.angle =
                                                    Some(Angle::Degree(angle));
                                            });

                                            let mut color_1 = shroud_layer_container.shroud_layer.color_1.clone().unwrap();
                                            let mut color_2 = shroud_layer_container.shroud_layer.color_2.clone().unwrap();
                                            let mut line_color = shroud_layer_container.shroud_layer.line_color.clone().unwrap();
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
                                                    let mut taper = shroud_layer_container
                                                        .shroud_layer
                                                        .taper
                                                        .unwrap_or(1.0);
                                                    ui.label("taper=");
                                                    ui.add(DragValue::new(&mut taper).speed(0.025));
                                                    shroud_layer_container.shroud_layer.taper =
                                                        Some(taper);
                                                });
                                            }
                                        });
                                });
                            },
                        );

                        ui.add_space(4.0);
                    });
            });
    }

    fn visual_panel(&mut self, ctx: &Context) {
        let central_panel_frame = Frame::new()
            // .fill(Color32::from_rgb(0, 0, 0)) // Pure black background
            .inner_margin(0.0);

        egui::CentralPanel::default()
            .frame(central_panel_frame)
            .show(ctx, |ui| {
                let mouse_pos = ui.input(|i| i.pointer.hover_pos());
                let response =
                    ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect = response.rect;

                self.draw_grid(ui, rect);

                self.shroud.iter().enumerate().for_each(|(index, shroud_layer_container)| {
                    let is_hovered = self.is_shroud_hovered(mouse_pos, shroud_layer_container, rect);
                    let is_selected = self.is_shroud_layer_index_selected(index);

                    let selection_type_option = match (is_hovered, is_selected) {
                        (true, _) => Some(SelectionType::Hovered),
                        (false, true) => Some(SelectionType::Selected),
                        _ => None,
                    };
                    // let line_width = if let Some(mouse_pos) = mouse_pos {
                    //     if is_pos_in_polygon(
                    //         // self.position_to_screen_position(mouse_pos, rect),
                    //         // dbg!(mouse_pos),
                    //         mouse_pos,
                    //         self.positions_to_screen_positions(
                    //             &shroud_layer_container
                    //                 .get_shroud_layer_vertices()
                    //                 .iter()
                    //                 .map(|vertex| {
                    //                     Pos2::new(
                    //                         vertex.x + offset.x.to_f32(),
                    //                         vertex.y + offset.y.to_f32(),
                    //                     )
                    //                 })
                    //                 .collect::<Vec<_>>(),
                    //             rect,
                    //         ),
                    //     ) {
                    //         3.0
                    //     } else {
                    //         1.0
                    //     }
                    // } else {
                    //     1.0
                    // };
                    // let stroke = Stroke::new(1.0, Color32::RED);
                    render_polygon(
                        ui.painter(),
                        self,
                        rect,
                        shroud_layer_container.get_shroud_layer_vertices(),
                        shroud_layer_container.shroud_layer.offset.clone().unwrap(),
                        shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
                        shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
                        shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
                        selection_type_option,
                    );
                });

                // Handle mouse wheel for zooming
                if let Some(pos) = ui.ctx().pointer_interact_pos() {
                    let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                    if scroll_delta != 0.0 && rect.contains(pos) {
                        self.zoom_at_position(pos, rect, scroll_delta * 0.01);
                    }
                }

                // dbg!(mouse_pos);
                // dbg!(TEST_SQUARE);
                // dbg!(response.drag_started());

                if response.drag_started() {
                    // let dragged_on_shroud_layers = self.shroud.iter().map(|shroud_layer_container| {
                    // }).collect();
                    let mouse_pos = response.interact_pointer_pos();
                    if let Some(mouse_pos) = mouse_pos {
                        let mut dragged_on_shroud_layers: Vec<usize> = Vec::default();
                        for (index, shroud_layer_container) in self.shroud.iter().enumerate() {
                            let offset =
                                shroud_layer_container.shroud_layer.offset.clone().unwrap();
                            let vertices = self.positions_to_screen_positions(
                                &shroud_layer_container
                                    .get_shroud_layer_vertices()
                                    .iter()
                                    .map(|vertex| {
                                        Pos2::new(
                                            vertex.x + offset.x.to_f32(),
                                            vertex.y + offset.y.to_f32(),
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                                rect,
                            );
                            if is_pos_in_polygon(mouse_pos, vertices) {
                                dragged_on_shroud_layers.push(index);
                            }
                        }
                        if let Some(index) = dragged_on_shroud_layers.first() {
                            self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                                drag_start_pos: mouse_pos,
                                selection: vec![*index],
                            };
                        }
                    }
                    // let mouse_pos = response.interact_pointer_pos();
                    // if let Some(mouse_pos) = mouse_pos {
                    //     println!("God speed.");
                    //     // if is_pos_in_polygon(self.position_to_screen_position(mouse_pos, rect), self.positions_to_screen_positions(TEST_SQUARE.into(), rect)) {
                    //     if is_pos_in_polygon(
                    //         // dbg!(self.position_to_screen_position(mouse_pos, rect) / 2.0),
                    //         dbg!(mouse_pos),
                    //         dbg!(self.positions_to_screen_positions(&shroud_layer_container.vertices, rect))
                    //     ) {
                    //     // if true {
                    //         println!("Touch down.");
                    //         self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                    //             drag_start_pos: mouse_pos,
                    //             selection: vec![0],
                    //         };
                    //     }
                    // }
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
                        // let (x, y) = if self.snap_to_grid {(
                        //         don_float_from(delta.x + old_offset.x.to_f32()),
                        //         don_float_from(delta.y + old_offset.y.to_f32()),
                        //     )
                        // } else {
                        //     (
                        //         don_float_from((delta.x + old_offset.x.to_f32() / self.grid_size).round() * self.grid_size),
                        //         don_float_from((delta.y + old_offset.y.to_f32() / self.grid_size).round() * self.grid_size),
                        //     )
                        // };
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
                    // }
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

    fn is_shroud_layer_index_selected(&self, index: usize) -> bool {
        self.shroud_layer_interaction
            .is_shroud_layer_index_selected(index)
    }

    fn grid_settings(&mut self, ui: &mut Ui) {
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
                // let color_1 = self.block_container.block.color_1.clone().unwrap();
                // let color_2 = self.block_container.block.color_2.clone().unwrap();
                // let line_color = self.block_container.block.line_color.clone().unwrap();
                // ui.horizontal(|ui| {
                //     ui.add_sized([2.0, 20.0],  Label::new(format!("fillColor={}", color_1.to_string())).wrap());
                //     let color_1 = color_setting(ui, color_1);
                //     self.block_container.block.color_1 = Some(color_1);
                // });
                // ui.horizontal(|ui| {
                //     ui.add_sized([2.0, 20.0],  Label::new(format!("fillColor1={}", color_2.to_string())).wrap());
                //     let color_2 = color_setting(ui, color_2);
                //     self.block_container.block.color_2 = Some(color_2);
                // });
                // ui.horizontal(|ui| {
                //     ui.add_sized([2.0, 20.0],  Label::new(format!("lineColor={}", line_color.to_string())).wrap());
                //     let line_color = color_setting(ui, line_color);
                //     self.block_container.block.line_color = Some(line_color);
                // });
                // dbg!(&color_1, &color_2, &line_color);
                // let mut color_1 = self.block_container.color_1;
                // let mut color_2 = self.block_container.color_2;
                // let mut line_color = self.block_container.line_color;
                Grid::new("").show(ui, |ui| {
                    ui.label(format!("fillColor={}", self.block_container.block.color_1.clone().unwrap().to_string()));
                    block_color_setting(ui, &mut self.block_container.color_1);
                    self.block_container.block.color_1 = Some(rgba_to_color(self.block_container.color_1));
                    ui.end_row();

                    ui.label(format!("fillColor1={}", self.block_container.block.color_2.clone().unwrap().to_string()));
                    block_color_setting(ui, &mut self.block_container.color_2);
                    self.block_container.block.color_2 = Some(rgba_to_color(self.block_container.color_2));
                    ui.end_row();

                    ui.label(format!("lineColor={}", self.block_container.block.line_color.clone().unwrap().to_string()));
                    block_color_setting(ui, &mut self.block_container.line_color);
                    self.block_container.block.line_color = Some(rgba_to_color(self.block_container.line_color));
                    ui.end_row();
                    // ui.label(format!("fillColor={}", color_1.to_string()));
                    // let color_1 = block_color_setting(ui, color_1);
                    // self.block_container.block.color_1 = Some(color_1);
                    // ui.end_row();

                    // ui.label(format!("fillColor1={}", color_2.to_string()));
                    // let color_2 = block_color_setting(ui, color_2);
                    // self.block_container.block.color_2 = Some(color_2);
                    // ui.end_row();

                    // ui.label(format!("lineColor={}", line_color.to_string()));
                    // let line_color = block_color_setting(ui, line_color);
                    // self.block_container.block.line_color = Some(line_color);
                    // ui.end_row();
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

    fn is_shroud_hovered(&self, mouse_pos: Option<Pos2>, shroud_layer_container: &ShroudLayerContainer, rect: Rect) -> bool {
        if let Some(mouse_pos) = mouse_pos {
            let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
            if is_pos_in_polygon(
                // self.position_to_screen_position(mouse_pos, rect),
                // dbg!(mouse_pos),
                mouse_pos,
                self.positions_to_screen_positions(
                    &shroud_layer_container
                        .get_shroud_layer_vertices()
                        .iter()
                        .map(|vertex| {
                            Pos2::new(
                                vertex.x + offset.x.to_f32(),
                                vertex.y + offset.y.to_f32(),
                            )
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
            pan: Pos2::new(0.0, 0.0),
            key_tracker: KeyTracker::default(),
            loaded_shapes: get_vanilla_shapes(),
            just_exported_to_clipboard_success_option: None,
            fill_color_gradient: 0.0,
            fill_color_gradient_increasing: true,
            fill_color_gradient_delta_enabled: true,
            last_frame_time: 0.0,
            dt: 0.0,
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

// fn block_color_setting(ui: &mut Ui, color: Color) -> Color {
//     let mut color = Rgba::from_rgba_unmultiplied(
//         color.rr() as f32 / 255.0,
//         color.gg() as f32 / 255.0,
//         color.bb() as f32 / 255.0,
//         color.aa().unwrap() as f32 / 255.0,
//     );
//     color_edit_button_rgba(ui, &mut color, Alpha::OnlyBlend);
//     let color = Color::new_aarrggb_u8(
//         (color.a() * 255.0) as u8,
//         (color.r() * 255.0) as u8,
//         (color.b() * 255.0) as u8,
//         (color.g() * 255.0) as u8,
//     );
//     color
//     // let old_color = color;
//     // let mut new_color = Rgba::from_rgba_premultiplied(
//     //     old_color.rr() as f32 / 255.0,
//     //     old_color.gg() as f32 / 255.0,
//     //     old_color.bb() as f32 / 255.0,
//     //     old_color.aa().unwrap() as f32 / 255.0,
//     // );
//     // let old_alpha = new_color.a();
//     // color_edit_button_rgba(ui, &mut new_color, Alpha::OnlyBlend);
//     // let new_color = if new_color.a() == old_alpha {
//     //     Color::new_aarrggb_u8(
//     //         (new_color.a() * 255.0) as u8,
//     //         (new_color.r() * 255.0) as u8,
//     //         (new_color.g() * 255.0) as u8,
//     //         (new_color.b() * 255.0) as u8,
//     //     )
//     // } else {
//     //     Color::new_aarrggb_u8(
//     //         (new_color.a() * 255.0) as u8,
//     //         old_color.rr(),
//     //         old_color.gg(),
//     //         old_color.bb(),
//     //     )
//     // };
//     // new_color
// }

fn shroud_color_setting(ui: &mut Ui, color: &mut ShroudLayerColor, text: &str) {
    ui.label(text);
    ui.selectable_value(color, ShroudLayerColor::Color1, "0");
    ui.selectable_value(color, ShroudLayerColor::Color2, "1");
    ui.selectable_value(color, ShroudLayerColor::LineColor, "2");
    ui.end_row();
}
