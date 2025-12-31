use egui::Context;

use crate::{
    keybinds::is_shortcut_pressed,
    shroud_editor::{ShroudEditor, add_mirror::add_mirror},
};

impl ShroudEditor {
    pub fn hotkey_mirroring(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.mirror) {
            self.shroud_interaction
                .selection()
                .iter()
                .for_each(|shroud_layer_index| {
                    if self.shroud[*shroud_layer_index]
                        .mirror_index_option
                        .is_none()
                    {
                        add_mirror(
                            &mut self.shroud,
                            *shroud_layer_index,
                            false,
                            &self.loaded_shapes,
                            &self.loaded_shapes_mirror_pairs,
                        );
                    }
                });
            self.add_undo_history = true;
        }
    }
}
