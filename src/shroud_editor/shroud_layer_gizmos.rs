use egui::{Color32, DragValue, Pos2, Rect, Ui, UiBuilder, pos2};
use luexks_reassembly::utility::{angle::Angle, display_oriented_math::do2d_float_from};

use crate::shroud_editor::{ShroudEditor, shroud_settings::angle_knob_settings};

const GIZMO_SET_LIMIT: usize = 16;

impl ShroudEditor {
    pub fn shroud_layer_gizmos(&mut self, ui: &mut Ui, rect: Rect) {
        self.shroud_interaction
            .selection()
            .into_iter()
            .rev()
            .take(GIZMO_SET_LIMIT)
            .for_each(|index| {
                if index < self.shroud.len() {
                    let offset = self.shroud[index].shroud_layer.offset.clone().unwrap();
                    let gizmo_center = pos2(offset.x.to_f32(), -offset.y.to_f32());
                    let (gizmo_pos_top_left, gizmo_pos_bottom_right) = (
                        self.world_pos_to_screen_pos(gizmo_center, rect),
                        self.world_pos_to_screen_pos(gizmo_center, rect),
                    );
                    let gizmo_size = 20.0;
                    self.angle_gizmo(
                        ui,
                        gizmo_pos_top_left,
                        gizmo_pos_bottom_right,
                        gizmo_size,
                        index,
                    );
                    self.size_gizmo(
                        ui,
                        gizmo_pos_top_left,
                        gizmo_pos_bottom_right,
                        gizmo_size,
                        index,
                    );
                }
            });
    }

    pub fn angle_gizmo(
        &mut self,
        ui: &mut Ui,
        gizmo_pos_top_left: Pos2,
        gizmo_pos_bottom_right: Pos2,
        gizmo_size: f32,
        index: usize,
    ) {
        let gizmo_pos_top_left = pos2(
            gizmo_pos_top_left.x - gizmo_size,
            gizmo_pos_top_left.y - gizmo_size,
        );
        let gizmo_pos_bottom_right = pos2(
            gizmo_pos_bottom_right.x + gizmo_size,
            gizmo_pos_bottom_right.y + gizmo_size,
        );
        let gizmo_rect = Rect::from_two_pos(gizmo_pos_top_left, gizmo_pos_bottom_right);
        ui.scope_builder(UiBuilder::new().max_rect(gizmo_rect), |ui| {
            egui::Frame::new().fill(Color32::BLACK).show(ui, |ui| {
                let angle = self.shroud[index]
                    // let angle = 360.0 - self.shroud[index]
                    .shroud_layer
                    .angle
                    .clone()
                    .unwrap()
                    .as_degrees()
                    .get_value();
                let original_angle = angle;
                let (angle, knob_drag_stopped) =
                    angle_knob_settings(ui, angle, self.angle_snap, self.angle_snap_enabled);
                self.shroud[index].shroud_layer.angle = Some(Angle::Degree(angle));
                if original_angle != angle
                    && let Some(mirror_index) = self.shroud[index].mirror_index_option
                {
                    self.shroud[mirror_index].shroud_layer.angle =
                        Some(Angle::Degree(360.0 - angle));
                    // Some(Angle::Degree(-(angle - 360.0)));
                }
                if knob_drag_stopped {
                    self.add_undo_history = true;
                }
            });
        });
    }

    pub fn size_gizmo(
        &mut self,
        ui: &mut Ui,
        gizmo_pos_top_left: Pos2,
        gizmo_pos_bottom_right: Pos2,
        gizmo_size: f32,
        index: usize,
    ) {
        let size = self.shroud[index].shroud_layer.size.clone().unwrap();
        let mut width = size.x.to_f32();
        let mut height = size.y.to_f32();
        let original_size = (width, height);

        let is_square = self.shroud[index].shape_id == "SQUARE";
        const GIZMO_DISTANCE: f32 = 50.0;
        let height_gizmo_pos_top_left = if is_square {
            let angle = -self.shroud[index]
                .shroud_layer
                .angle
                .clone()
                .unwrap()
                .as_radians()
                .get_value()
                + std::f32::consts::PI * 0.5;
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
            let angle = -self.shroud[index]
                .shroud_layer
                .angle
                .clone()
                .unwrap()
                .as_radians()
                .get_value()
                + std::f32::consts::PI * 0.5;
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
        let gizmo_rect =
            Rect::from_two_pos(height_gizmo_pos_top_left, height_gizmo_pos_bottom_right);
        ui.scope_builder(UiBuilder::new().max_rect(gizmo_rect), |ui| {
            egui::Frame::new().fill(Color32::BLACK).show(ui, |ui| {
                let xy_speed = self.get_xy_speed();
                let response = ui.add(DragValue::new(&mut height).speed(xy_speed));
                if response.drag_stopped() || response.lost_focus() {
                    self.add_undo_history = true;
                }
            });
        });
        // let angle = angle;
        // let gizmo_distance = 20.0;
        let width_gizmo_pos_top_left = if is_square {
            let angle = -self.shroud[index]
                .shroud_layer
                .angle
                .clone()
                .unwrap()
                .as_radians()
                .get_value();
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
            let angle = -self.shroud[index]
                .shroud_layer
                .angle
                .clone()
                .unwrap()
                .as_radians()
                .get_value();
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
        ui.scope_builder(UiBuilder::new().max_rect(gizmo_rect), |ui| {
            egui::Frame::new().fill(Color32::BLACK).show(ui, |ui| {
                let xy_speed = self.get_xy_speed();
                let response = ui.add(DragValue::new(&mut width).speed(xy_speed));
                if response.drag_stopped() || response.lost_focus() {
                    self.add_undo_history = true;
                }
            });
        });
        self.shroud[index].shroud_layer.size = Some(do2d_float_from(width, height));

        if original_size != (width, height)
            && let Some(mirror_index) = self.shroud[index].mirror_index_option
        {
            self.shroud[mirror_index].shroud_layer.size = Some(do2d_float_from(width, height));
        }
    }
}
