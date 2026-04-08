use egui::{Area, Color32, Frame, Id, Rect, RichText, Ui, pos2};

use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn world_mouse_pos_text(&self, ui: &mut Ui, rect: Rect) {
        Area::new(Id::new("float_shroud_settings"))
            .fixed_pos(pos2(rect.min.x, 0.))
            .fade_in(false)
            .default_width(500.)
            .show(ui.ctx(), |ui| {
                Frame::new().fill(Color32::BLACK).inner_margin(2).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let s = format!("X: {:.3}\nY: {:.3}", self.world_mouse_pos.x, self.world_mouse_pos.y);
                        ui.label(
                            RichText::new(s).color(Color32::MAGENTA),
                        );
                    });
                });
            });
    }
}
