use egui::Context;

use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn half_grid_size_key_logic(&mut self, ctx: &Context) {
        if self.visual_panel_key_bindings_enabled && ctx.input(|i| i.modifiers.command) {
            self.grid_size = self.settings_grid_size / 2.;
        } else {
            self.grid_size = self.settings_grid_size;
        }
    }
}
