use egui::{Context, Key};

use crate::{shroud_editor::ShroudEditor, shroud_layer_interaction::ShroudLayerInteraction};

impl ShroudEditor {
    pub fn hotkey_shroud_layer_deletion(&mut self, ctx: &Context) {
        let shroud_delete_hotkey_pressed = ctx.input(|i| i.key_pressed(Key::R));
        // let shroud_delete_hotkey_pressed = ctx.input(|i| i.key_pressed(Key::Backspace))
        //     || ctx.input(|i| i.key_pressed(Key::Delete))
        //     || ctx.input(|i| i.key_pressed(Key::R));
        if shroud_delete_hotkey_pressed {
            let selection = self.shroud_layer_interaction.selection();
            if !selection.is_empty() {
                let mut descending_selection = selection;
                descending_selection.sort_by(|index_a, index_b| index_b.cmp(index_a));
                descending_selection.iter().for_each(|index| {
                    self.shroud.remove(*index);
                });
                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                    selection: Vec::new(),
                };
            }
        }
    }
}
