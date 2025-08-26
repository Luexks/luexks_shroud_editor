use egui::{Context, Key};

use crate::{shroud_editor::ShroudEditor, shroud_layer_interaction::ShroudLayerInteraction};

impl ShroudEditor {
    pub fn hotkey_shroud_layer_deletion(&mut self, ctx: &Context) {
        let shroud_delete_hotkey_pressed = ctx.input(|i| i.key_pressed(Key::R));
        if shroud_delete_hotkey_pressed {
            let selection = self.shroud_layer_interaction.selection();
            if !selection.is_empty() {
                let mut descending_selection = selection;
                descending_selection.sort_by(|index_a, index_b| index_b.cmp(index_a));
                descending_selection.iter().for_each(|index| {
                    if let Some(widowed_mirror_index) = self.shroud[*index].mirror_index_option {
                        self.shroud[widowed_mirror_index].mirror_index_option = None;
                        self.shroud[widowed_mirror_index].drag_pos = None;
                    }
                    self.shroud.remove(*index);
                    self.shroud.iter_mut().for_each(|shroud_layer_container| {
                        if let Some(mirror_index) = shroud_layer_container.mirror_index_option
                            && mirror_index > *index
                        {
                            shroud_layer_container.mirror_index_option = Some(mirror_index - 1);
                        }
                    });
                });
                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                    selection: Vec::new(),
                };
            }
        }
    }
}
