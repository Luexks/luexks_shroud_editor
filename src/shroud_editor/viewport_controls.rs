use std::f32::consts::SQRT_2;

use egui::{Context, Key, Pos2, Rect, Ui, pos2, vec2};
use egui_keybind::Bind;

use crate::{shroud_editor::ShroudEditor, shroud_interaction::ShroudInteraction};

impl ShroudEditor {
    pub fn pan_controls(&mut self, ctx: &Context) {
        let speed: f32 = 1000.0 * self.dt as f32 / self.zoom;
        let mut delta = Pos2::default();
        ctx.input(|i| {
            if let Some(keyboard_shortcut) = self.keybinds.pan_up.keyboard()
                && i.key_down(keyboard_shortcut.logical_key)
            {
                delta.y += speed;
            }
        });
        ctx.input(|i| {
            if let Some(keyboard_shortcut) = self.keybinds.pan_down.keyboard()
                && i.key_down(keyboard_shortcut.logical_key)
            {
                delta.y -= speed;
            }
        });
        ctx.input(|i| {
            if let Some(keyboard_shortcut) = self.keybinds.pan_right.keyboard()
                && i.key_down(keyboard_shortcut.logical_key)
            {
                delta.x -= speed;
            }
        });
        ctx.input(|i| {
            if let Some(keyboard_shortcut) = self.keybinds.pan_left.keyboard()
                && i.key_down(keyboard_shortcut.logical_key)
            {
                delta.x += speed;
            }
        });
        if delta.x != 0.0 && delta.y != 0.0 {
            delta *= SQRT_2 * 0.5;
        }
        self.pan = pos2(self.pan.x + delta.x, self.pan.y + delta.y);

        if let ShroudInteraction::Dragging { drag_pos, .. } = &mut self.shroud_interaction {
            *drag_pos += vec2(-delta.x, delta.y);
        }
        if let ShroudInteraction::Placing { drag_pos, .. } = &mut self.shroud_interaction {
            *drag_pos += vec2(-delta.x, delta.y);
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
