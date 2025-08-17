use crate::fonts;
use crate::fonts::replace_fonts;
use crate::key_tracker::KeyTracker;
use crate::pos_in_polygon;
use crate::pos_in_polygon::is_pos_in_polygon;
use crate::restructure_vertices::restructure_vertices;
use crate::shroud_editor;
use crate::shroud_layer_container::ShroudLayerContainer;
use crate::shroud_layer_rendering::render_shroud_layer;
use crate::styles::apply_styles;
use egui::Color32;
use egui::ComboBox;
use egui::Context;
use egui::Frame;
use egui::Key;
use egui::Pos2;
use egui::Rect;
use egui::Stroke;
use egui::Vec2;
use luexks_reassembly::blocks::{shroud::Shroud, shroud_layer::ShroudLayer};
use luexks_reassembly::shapes::shapes::Shapes;
use luexks_reassembly::utility::display_oriented_math::DisplayOriented3D;
use luexks_reassembly::utility::display_oriented_math::do3d_float_from;
use luexks_reassembly::utility::display_oriented_math::don_float_from;
use parse_vanilla_shapes::get_vanilla_shapes;

pub enum ShroudLayerInteraction {
    Inaction {
        selection: Vec<usize>,
    },
    Dragging {
        drag_start_pos: Pos2,
        selection: Vec<usize>,
    },
}

impl ShroudLayerInteraction {
    pub fn selection(&self) -> Vec<usize> {
        match self {
            ShroudLayerInteraction::Inaction { selection } => selection.clone(),
            ShroudLayerInteraction::Dragging { selection, .. } => selection.clone(),
        }
    }
}

struct BlockProperties {
    shape: String,
    scale: usize,
}

pub const TEST_SQUARE: [Pos2; 4] = [
    Pos2::new(-5.0, -5.0),
    Pos2::new(-5.0, 5.0),
    Pos2::new(5.0, 5.0),
    Pos2::new(5.0, -5.0),
];

// pub struct ShroudEditor<'a> {
pub struct ShroudEditor {
    // pub shroud: Shroud,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_layer_interaction: ShroudLayerInteraction,
    // pub selected_shroud_layer_indexes: Vec<usize>,
    // pub selected_shroud_layers: Vec<&'a ShroudLayer>,
    // pub zoom: usize,
    pub zoom: f32,
    pub grid_size: f32,
    pub grid_active: bool,
    pub snap_to_grid: bool,
    pub pan: Pos2,
    // pub dragging: bool,
    pub key_tracker: KeyTracker,
    pub loaded_shapes: Shapes,
}

impl ShroudEditor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // replace_fonts(&cc.egui_ctx);
        // apply_styles(&cc.egui_ctx);
        ShroudEditor::default()
    }

    // Преобразование координаты формы в координату экрана
    pub fn position_to_screen_position(&self, position: Pos2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2 {
            // x: center.x + (position.x + self.pan.x), // * self.zoom,
            // y: center.y + (position.y + self.pan.y), // * self.zoom,
            x: center.x + (position.x + self.pan.x) * self.zoom,
            y: center.y + (position.y + self.pan.y) * self.zoom,
        }
    }

    pub fn positions_to_screen_positions(&self, positions: &Vec<Pos2>, rect: Rect) -> Vec<Pos2> {
        positions
            .iter()
            .map(|position| self.position_to_screen_position(*position, rect))
            .collect()
    }

    pub fn pan_controls(&mut self, ctx: &Context) {
        const SPEED: f32 = 2.0;
        // let input = ctx.input(|i| i.clone());

        // if input.key_down(Key::W) {
        //     self.pan.y -= SPEED;
        // }
        // if input.key_down(Key::S) {
        //     self.pan.y += SPEED;
        // }
        // if input.key_down(Key::D) {
        //     self.pan.x += SPEED;
        // }
        // if input.key_down(Key::A) {
        //     self.pan.x -= SPEED;
        // }
        if self.key_tracker.is_held(Key::W) {
            // self.pan.y -= SPEED;
            self.pan.y += SPEED;
        }
        if self.key_tracker.is_held(Key::S) {
            // self.pan.y += SPEED;
            self.pan.y -= SPEED;
        }
        if self.key_tracker.is_held(Key::D) {
            // self.pan.x += SPEED;
            self.pan.x -= SPEED;
        }
        if self.key_tracker.is_held(Key::A) {
            // self.pan.x -= SPEED;
            self.pan.x += SPEED;
        }
    }

    pub fn zoom_at_position(&mut self, screen_pos: Pos2, rect: Rect, delta: f32) {
        let delta = delta * 5.0;
        // // Adjust zoom
        // self.zoom = (self.zoom * (1.0 + delta * 0.1)).clamp(0.1, 10.0);

        // let zoom = self.zoom as f32;
        // let old_zoom = zoom;

        // // Calculate world position before zoom
        // let center = rect.center();
        // let before_x = (screen_pos.x - center.x) / old_zoom;
        // let before_y = (screen_pos.y - center.y) / old_zoom;

        // // Calculate world position after zoom
        // let after_x = (screen_pos.x - center.x) / self.zoom;
        // let after_y = (screen_pos.y - center.y) / self.zoom;

        // // Adjust panning to keep the world position constant under cursor
        // self.pan.x += after_x - before_x;
        // self.pan.y += after_y - before_y;
        let old_zoom = self.zoom;
        // println!("{}", delta);

        // Adjust zoom
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
}

// fn is_key_down(ctx: &Context, key: Key) -> bool {
//     ctx.input(|i| i.key_down(desired_key))
// }

impl Default for ShroudEditor {
    fn default() -> Self {
        Self {
            shroud: Vec::default(),
            shroud_layer_interaction: ShroudLayerInteraction::Inaction {
                selection: Vec::new(),
            },
            // zoom: 1,
            zoom: 1.0,
            grid_size: 1.0,
            grid_active: true,
            snap_to_grid: true,
            pan: Pos2::new(0.0, 0.0),
            key_tracker: KeyTracker::default(),
            loaded_shapes: get_vanilla_shapes(),
        }
    }
}

impl eframe::App for ShroudEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.key_tracker.update(ctx);
        self.pan_controls(ctx);
        // egui::CentralPanel::default().show(ctx, |ui| {
        egui::SidePanel::left("side_panel")
            .default_width(220.0)
            .exact_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Luexks Shroud Editor");
                if ui.button("Add Shroud Layer").clicked() {
                    self.shroud.push(ShroudLayerContainer::default());
                    println!("Added a shroud");
                }
                ui.heading("Block Properties");
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                    .inner_margin(6.0)
                    .corner_radius(0.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.strong("Block Properties:");
                        });

                        ui.add_space(4.0);
                    });
                ui.heading("Shroud Layers:");
                egui::Frame::new()
                    // .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                    .inner_margin(0.0)
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        // ui.horizontal(|ui| {
                        //     ui.strong("Shroud Layers:");
                        // });
                        // ui.label("Shroud");
                        self.shroud.iter_mut().enumerate().for_each(
                            |(index, shroud_layer_container)| {
                                // ui.label("Shroud");
                                // println!("Shroud");
                                ui.vertical(|ui| {
                                    egui::Frame::new()
                                        .fill(Color32::from_rgba_unmultiplied(220, 220, 220, 255))
                                        .inner_margin(6.0)
                                        .corner_radius(4.0)
                                        .show(ui, |ui| {
                                            // ui.label("Shroud");
                                            // println!("Shroud");
                                            // let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
                                            // ui.label(
                                            //     format!("offset={{{:.3},{:.3},{:.3}}}",
                                            //     offset.x.to_f32(),
                                            //     offset.y.to_f32(),
                                            //     offset.z.to_f32(),
                                            // ));
                                            ui.horizontal(|ui| {
                                                ui.label("shape=");
                                                // ui.button(shroud_layer_container.shroud_layer.shape.clone().unwrap())
                                                ComboBox::from_label(index.to_string())
                                                    .selected_text(
                                                        shroud_layer_container.shape_id.clone(),
                                                    )
                                                    .show_ui(ui, |ui| {
                                                        for selectable_shape in &self.loaded_shapes.0 {
                                                            let response = ui.selectable_value(
                                                                &mut shroud_layer_container
                                                                    .shape_id,
                                                                selectable_shape
                                                                    .get_id()
                                                                    .unwrap()
                                                                    .to_string()
                                                                    .to_owned(),
                                                                selectable_shape.get_id().unwrap().to_string(),
                                                            );
                                                            if response.clicked() {
                                                                // shroud_layer_container.vertices = self.loaded_shapes.0.iter().find(|loaded_shape| loaded_shape.get_id() ==).loaded_shape.get_first_scale_vertices();
                                                                shroud_layer_container.vertices = restructure_vertices(selectable_shape.get_first_scale_vertices());
                                                                shroud_layer_container.shroud_layer.shape = selectable_shape.get_id();
                                                            }
                                                        }
                                                    });
                                            });
                                            let offset = shroud_layer_container
                                                .shroud_layer
                                                .offset
                                                .clone()
                                                .unwrap();
                                            ui.label(format!(
                                                "offset={{{:.3},{:.3},{:.3}}}",
                                                offset.x.to_f32(),
                                                offset.y.to_f32(),
                                                offset.z.to_f32(),
                                            ));
                                            // ui.label(
                                            //     format!("shape={}",
                                            //     shroud_layer_container.shroud_layer.shape.clone().unwrap()
                                            // ));
                                        });
                                });
                            },
                        );

                        ui.add_space(4.0);
                    });
                // ui.painter();

                ui.painter().circle(
                    Pos2::new(50.0, 50.0),
                    16.0,
                    Color32::from_rgb(50, 0, 0),
                    Stroke::new(5.0, Color32::from_rgb(255, 0, 0)),
                );
            });
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
                self.shroud.iter().for_each(|shroud_layer_container| {
                    let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
                    let stroke = if let Some(mouse_pos) = mouse_pos {
                        if is_pos_in_polygon(
                            // self.position_to_screen_position(mouse_pos, rect),
                            // dbg!(mouse_pos),
                            mouse_pos,
                            self.positions_to_screen_positions(
                                &shroud_layer_container
                                    .vertices
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
                            Stroke::new(1.0, Color32::GREEN)
                        } else {
                            Stroke::new(1.0, Color32::RED)
                        }
                    } else {
                        Stroke::new(1.0, Color32::RED)
                    };
                    // let stroke = Stroke::new(1.0, Color32::RED);
                    render_shroud_layer(
                        ui.painter(),
                        ctx,
                        self,
                        shroud_layer_container,
                        rect,
                        stroke,
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
                                    .vertices
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
                            dbg!(vertices[0]);
                            dbg!(mouse_pos);
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
                    // if let Some(mouse_pos) = mouse_pos {
                    // if let Some(last_post) = ui.input(|i| i.pointer.delta()) {
                    // let delta = mouse_pos - *drag_start_pos;
                    let delta = ui.input(|i| i.pointer.delta()) / self.zoom;
                    selection.iter().for_each(|selected_index| {
                        // self.shroud.0[*selected_index].offset.unwrap().x = don_float_from(5.0);
                        let old_offset = self
                            .shroud
                            .get(*selected_index)
                            .unwrap()
                            .shroud_layer
                            .offset
                            .clone()
                            .unwrap();
                        self.shroud[*selected_index].shroud_layer.offset =
                            Some(DisplayOriented3D {
                                x: don_float_from(delta.x + old_offset.x.to_f32()),
                                y: don_float_from(delta.y + old_offset.y.to_f32()),
                                // x: don_float_from(delta.x + old_offset.x.to_f32()),
                                // y: don_float_from(delta.y + old_offset.y.to_f32()),
                                // x: don_float_from(5.0),
                                // y: don_float_from(5.0),
                                z: old_offset.z,
                            });
                    });
                    // }
                    if response.drag_stopped() {
                        self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                            selection: self.shroud_layer_interaction.selection(),
                        }
                    }
                }
            });
    }
}
