use std::f32::consts::SQRT_2;

use egui::{Key, Pos2, Rect, Ui, Vec2, pos2};

use crate::{
    shroud_editor::ShroudEditor,
    shroud_interaction::{MovingShroudLayerInteraction, ShroudInteraction},
};

impl ShroudEditor {
    pub fn pan_controls(&mut self) {
        let speed: f32 = 1000.0 * self.dt as f32 / self.zoom;
        let mut delta = Vec2::default();
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

        if let ShroudInteraction::Dragging {
            main_idx,
            selection,
            drag_pos,
            position_change,
        } = &mut self.shroud_interaction
        {
            // selection.0.iter_mut().for_each(
            //     |MovingShroudLayerInteraction {
            //          idx: index,
            //          drag_pos,
            //      }| {
            //         *drag_pos = pos2(drag_pos.x - delta.x, drag_pos.y + delta.y);
            //     },
            // );
            *drag_pos += delta;
        }
        if let ShroudInteraction::Placing {
            main_idx,
            selection,
            drag_pos,
            position_change,
        } = &mut self.shroud_interaction
        {
            // selection.0.iter_mut().for_each(
            //     |MovingShroudLayerInteraction {
            //          idx: index,
            //          drag_pos,
            //      }| {
            //         *drag_pos = pos2(drag_pos.x - delta.x, drag_pos.y + delta.y);
            //     },
            // );
            *drag_pos += delta;
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
