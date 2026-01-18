use egui::{Color32, Pos2, Rect, Stroke, Ui, pos2};

use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn icon_radius_logic(&mut self, ui: &mut Ui, rect: Rect) {
        if self.show_icon_radius {
            let icon_radius = match self.icon_radius_option {
                Some(icon_radius) => icon_radius,
                None => {
                    let icon_radius = self.block_container.offset.x / -0.5;
                    self.icon_radius_option = Some(icon_radius);
                    icon_radius
                }
            };
            let screen_icon_radius = icon_radius * self.zoom;
            ui.painter().circle_stroke(
                self.world_pos_to_screen_pos(Pos2::ZERO, rect),
                screen_icon_radius,
                Stroke::new(2.0, Color32::GOLD),
            );
            ui.painter().line_segment(
                [
                    self.world_pos_to_screen_pos(pos2(icon_radius * -0.5, 10.0), rect),
                    self.world_pos_to_screen_pos(pos2(icon_radius * -0.5, -10.0), rect),
                ],
                Stroke::new(2.0, Color32::GOLD),
            );
        } else {
            self.icon_radius_option = None;
        }
    }

    pub fn icon_radius_setting(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Show Icon Radius:");
            ui.checkbox(&mut self.show_icon_radius, "");
        });
    }
}
