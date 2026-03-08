use crate::shroud_editor::ShroudEditor;

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
            self.mirror_idx_logic_for_deleted_layer_idx(*layer_idx);
            self.groups_logic_for_deleted_layer_idx(*layer_idx);
            self.selection_logic_for_deleted_layer_idx(*layer_idx);
        });
        self.cull_groups();
    }
}
