use std::f32::consts::SQRT_2;

use egui::{Key, Pos2, Rect, Ui, pos2};

use crate::{shroud_editor::ShroudEditor, shroud_layer_interaction::ShroudLayerInteraction};

impl ShroudEditor {
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
                // let old_offset = self.shroud[*index].shroud_layer.offset.clone().unwrap();
                let old_drag_pos = self.shroud[*index].drag_pos_option.unwrap();
                self.shroud[*index].drag_pos_option =
                    Some(pos2(old_drag_pos.x - delta.x, old_drag_pos.y + delta.y));
                // self.shroud[*index].shroud_layer.offset = Some(do3d_float_from(
                //     old_offset.x.to_f32() - delta.x,
                //     old_offset.y.to_f32() - delta.y,
                //     old_offset.z.to_f32(),
                // ));
            });
        }
        if let ShroudLayerInteraction::Placing { selection } = &self.shroud_layer_interaction {
            selection.iter().for_each(|index| {
                // dbg!(&self.shroud);
                // dbg!(index);
                // dbg!(selection);
                let old_drag_pos = self.shroud[*index].drag_pos_option.unwrap();
                self.shroud[*index].drag_pos_option =
                    Some(pos2(old_drag_pos.x - delta.x, old_drag_pos.y + delta.y));
            });
        }
    }

    pub fn zoom(&mut self, ui: &mut Ui, rect: Rect) {
        if let Some(pos) = ui.ctx().pointer_interact_pos() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 && rect.contains(pos) {
                self.zoom_at_position(pos, rect, scroll_delta * 0.01);
            }
        }
    }

    fn zoom_at_position(&mut self, screen_pos: Pos2, rect: Rect, delta: f32) {
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
}
