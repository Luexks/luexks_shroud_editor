use egui::Context;

use crate::{
    keybinds::is_shortcut_pressed, shroud_editor::ShroudEditor,
    shroud_interaction::ShroudInteraction,
};

impl ShroudEditor {
    pub fn hotkey_shroud_layer_deletion(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.delete) {
            self.shroud_interaction
                .selection()
                .into_iter()
                .for_each(|idx| {
                    let shroud_layer = &mut self.shroud[idx];
                    shroud_layer.delete_next_frame = true;
                    if let Some(mirror_idx) = shroud_layer.mirror_index_option {
                        self.shroud[mirror_idx].delete_next_frame = true;
                    }
                });
            self.shroud_interaction = ShroudInteraction::Inaction {
                selection: Vec::new(),
            };
            self.add_undo_history = true;
            // let selection = self.shroud_interaction.selection();
            // if !selection.is_empty() {
            // let mut descending_selection = selection;
            // descending_selection.sort_by(|index_a, index_b| index_b.cmp(index_a));
            // descending_selection.iter().for_each(|index| {
            //     if let Some(widowed_mirror_index) = self.shroud[*index].mirror_index_option {
            //         self.shroud.remove(*index);
            //         self.shroud.remove(if widowed_mirror_index < *index {
            //                 widowed_mirror_index }
            //             else {
            //                 widowed_mirror_index - 1
            //             });
            //         self.shroud.iter_mut().for_each(|shroud_layer_container| {
            //             if let Some(mirror_index) = &mut shroud_layer_container.mirror_index_option
            //             {
            //                 if *mirror_index > *index {
            //                     *mirror_index -= 1;
            //                 }
            //                 if *mirror_index >= widowed_mirror_index {
            //                     *mirror_index -= 1;
            //                 }
            //             }
            //         });
            //     } else {
            //         self.shroud.remove(*index);
            //         self.shroud.iter_mut().for_each(|shroud_layer_container| {
            //             if let Some(mirror_index) = &mut shroud_layer_container.mirror_index_option
            //                 && *mirror_index > *index
            //             {
            //                     *mirror_index -= 1;
            //             }
            //         });
            //     }
            // });
            // self.shroud_interaction = ShroudInteraction::Inaction {
            //     selection: Vec::new(),
            // };
            // }
        }
    }
}
