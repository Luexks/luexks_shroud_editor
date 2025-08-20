use egui::Context;

use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn update_dt(&mut self, ctx: &Context) {
        let now = ctx.input(|i| i.time);
        self.dt = now - self.last_frame_time;
        self.last_frame_time = now;
    }
}
