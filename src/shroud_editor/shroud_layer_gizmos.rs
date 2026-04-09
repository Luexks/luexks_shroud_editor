use egui::{Color32, DragValue, Pos2, Rect, Ui, UiBuilder, pos2, vec2};

use crate::{
    angle_gizmo::AngleGizmo,
    rotation_edgecase::RotationEdgecase,
    shroud_editor::{
        DRAG_VALUE_MAX, DRAG_VALUE_MIN, ShroudEditor,
        shroud_settings::{ShroudLayerSettingsTarget, SingleSettingsTarget},
    },
};

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
                    let gizmo_center = self
                        .world_pos_to_screen_pos(pos2(offset.x.to_f32(), -offset.y.to_f32()), rect);
                    let (gizmo_pos_top_left, gizmo_pos_bottom_right) = (gizmo_center, gizmo_center);
                    let gizmo_size = 20.0;
                    self.angle_gizmo(ui, gizmo_center, index);
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

    pub fn angle_gizmo(&mut self, ui: &mut Ui, gizmo_centre: Pos2, idx: usize) {
        let gizmo_rect = Rect::from_two_pos(gizmo_centre, gizmo_centre + vec2(1.0, 1.0));
        ui.scope_builder(UiBuilder::new().max_rect(gizmo_rect), |ui| {
            egui::Frame::new()
                .fill(Color32::TRANSPARENT)
                .show(ui, |ui| {
                    self.angle_gizmo_body(idx, ui);
                });
        });
    }

    fn angle_gizmo_body(&mut self, idx: usize, ui: &mut Ui) {
        let rotation_edgecase_option = Into::<Option<RotationEdgecase>>::into(&self.shroud[idx]);
        let shroud_layer_settings_target = &mut SingleSettingsTarget {
            shroud: &mut self.shroud,
            idx,
        };
        let angle = shroud_layer_settings_target
            .get_main_layer_mut()
            .angle
            .as_mut()
            .unwrap()
            .as_degrees_mut()
            .get_value_mut();
        let mut add_undo_history = false;
        let mut changed = false;
        ui.add(AngleGizmo::new(
            angle,
            self.angle_snap,
            self.angle_snap_enabled,
            &mut add_undo_history,
            &mut changed,
            rotation_edgecase_option,
        ));
        if add_undo_history {
            self.add_undo_history = true;
        }
        if changed {
            let angle = *angle;
            shroud_layer_settings_target.on_angle_changed(angle);
        }
    }

    pub fn size_gizmo(
        &mut self,
        ui: &mut Ui,
        gizmo_pos_top_left: Pos2,
        gizmo_pos_bottom_right: Pos2,
        gizmo_size: f32,
        idx: usize,
    ) {
        let xy_speed = self.get_xy_speed();
        let shroud_layer_settings_target = &mut SingleSettingsTarget {
            shroud: &mut self.shroud,
            idx,
        };
        let is_square = shroud_layer_settings_target.get_shape_id_str() == "SQUARE";
        let angle = shroud_layer_settings_target
            .get_main_layer()
            .angle
            .clone()
            .unwrap()
            .as_radians()
            .get_value();
        const GIZMO_DISTANCE: f32 = 50.0;
        let height_gizmo_pos_top_left = if is_square {
            let (sin, cos) = (-angle + std::f32::consts::PI * 0.5).sin_cos();
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE * cos,
                gizmo_pos_top_left.y - gizmo_size + GIZMO_DISTANCE * sin,
            )
        } else {
            pos2(
                gizmo_pos_top_left.x - gizmo_size,
                gizmo_pos_top_left.y - gizmo_size - GIZMO_DISTANCE,
            )
        };
        let height_gizmo_pos_bottom_right = if is_square {
            let (sin, cos) = (-angle + std::f32::consts::PI * 0.5).sin_cos();
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size + GIZMO_DISTANCE * cos,
                gizmo_pos_bottom_right.y - gizmo_size + GIZMO_DISTANCE * sin,
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
                let height = shroud_layer_settings_target
                    .get_main_layer_mut()
                    .size
                    .as_mut()
                    .unwrap()
                    .y
                    .to_f32_mut();
                let response = ui.add(
                    DragValue::new(height)
                        .speed(xy_speed)
                        .range(DRAG_VALUE_MIN..=DRAG_VALUE_MAX),
                );
                if response.changed() {
                    let height = *height;
                    shroud_layer_settings_target.on_height_changed(height);
                }
                if response.drag_stopped() || response.lost_focus() {
                    self.add_undo_history = true;
                }
            });
        });
        let width_gizmo_pos_top_left = if is_square {
            let (sin, cos) = (-angle).sin_cos();
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE * cos,
                gizmo_pos_top_left.y - gizmo_size + GIZMO_DISTANCE * sin,
            )
        } else {
            pos2(
                gizmo_pos_top_left.x - gizmo_size + GIZMO_DISTANCE,
                gizmo_pos_top_left.y - gizmo_size,
            )
        };
        let width_gizmo_pos_bottom_right = if is_square {
            let (sin, cos) = (-angle).sin_cos();
            pos2(
                gizmo_pos_bottom_right.x - gizmo_size + GIZMO_DISTANCE * cos,
                gizmo_pos_bottom_right.y - gizmo_size + GIZMO_DISTANCE * sin,
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
                let width = shroud_layer_settings_target
                    .get_main_layer_mut()
                    .size
                    .as_mut()
                    .unwrap()
                    .x
                    .to_f32_mut();
                let response = ui.add(
                    DragValue::new(width)
                        .speed(xy_speed)
                        .range(DRAG_VALUE_MIN..=DRAG_VALUE_MAX),
                );
                if response.changed() {
                    let width = *width;
                    shroud_layer_settings_target.on_width_changed(width);
                }
                if response.drag_stopped() || response.lost_focus() {
                    self.add_undo_history = true;
                }
            });
        });
    }
}
