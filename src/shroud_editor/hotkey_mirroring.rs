use egui::{Context, Key};

use crate::shroud_editor::{ShroudEditor, add_mirror::add_mirror};

impl ShroudEditor {
    pub fn hotkey_mirroring(&mut self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(Key::F)) {
            self.shroud_layer_interaction
                .selection()
                .iter()
                .for_each(|shroud_layer_index| {
                    if self.shroud[*shroud_layer_index].mirror_index_option.is_none() {
                        add_mirror(
                            &mut self.shroud,
                            *shroud_layer_index,
                            false,
                            &self.loaded_shapes,
                            &self.loaded_shapes_mirror_pairs,
                        );
                    }
                });
        }
    }
}
