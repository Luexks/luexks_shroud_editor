use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn delete_shroud_layers(&mut self) {
        let widowed_mirror_indexes = self
            .shroud
            .iter()
            .filter_map(|shroud_layer_container| {
                if shroud_layer_container.delete_next_frame
                    && let Some(mirror_index) = shroud_layer_container.mirror_index_option
                {
                    Some(mirror_index)
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
            .collect::<Vec<_>>();
        to_be_deleted_indexes.iter().for_each(|index| {
            if let Some(widowed_mirror_index) = self.shroud[*index].mirror_index_option {
                self.shroud[widowed_mirror_index].mirror_index_option = None;
                self.shroud[widowed_mirror_index].drag_pos = None;
            }
            self.shroud.remove(*index);
            self.shroud.iter_mut().for_each(|shroud_layer_container| {
                if let Some(mirror_index) = shroud_layer_container.mirror_index_option && mirror_index > *index {
                    shroud_layer_container.mirror_index_option = Some(mirror_index - 1);
                }
            });
        });
    }
}
