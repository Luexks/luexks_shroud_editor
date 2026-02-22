use crate::{shroud_editor::ShroudEditor, shroud_interaction::ShroudInteraction};

impl ShroudEditor {
    pub fn delete_shroud_layers(&mut self) {
        let widowed_mirror_indexes = self
            .shroud
            .iter()
            .filter_map(|shroud_layer_container| {
                if shroud_layer_container.delete_next_frame {
                    shroud_layer_container.mirror_index_option
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        widowed_mirror_indexes
            .iter()
            .for_each(|widowed_mirror_index| {
                self.shroud[*widowed_mirror_index].mirror_index_option = None;
            });

        let to_be_deleted_indexes = self
            .shroud
            .iter()
            .enumerate()
            .filter_map(|(index, shroud_layer_container)| {
                if shroud_layer_container.delete_next_frame {
                    Some(index)
                } else {
                    None
                }
            })
            .rev()
            .collect::<Vec<_>>();
        to_be_deleted_indexes.iter().for_each(|layer_idx| {
            self.shroud.remove(*layer_idx);
            self.shroud.iter_mut().for_each(|shroud_layer_container| {
                if let Some(mirror_index) = &mut shroud_layer_container.mirror_index_option
                    && *mirror_index > *layer_idx
                {
                    *mirror_index -= 1;
                }
            });
            self.groups.iter_mut().for_each(|group| {
                if let Some(group_layer_idx_idx) = group
                    .iter()
                    .position(|group_layer_idx| *group_layer_idx == *layer_idx)
                {
                    group.remove(group_layer_idx_idx);
                }
                group.iter_mut().for_each(|group_idx| {
                    if *group_idx > *layer_idx {
                        *group_idx -= 1;
                    }
                });
            });
            let mut selection = self.shroud_interaction.selection();
            for selected_index in &mut selection {
                if *selected_index > *layer_idx {
                    *selected_index -= 1;
                }
            }
            self.shroud_interaction = ShroudInteraction::Inaction { selection };
        });
        self.cull_empty_groups();
    }
}
