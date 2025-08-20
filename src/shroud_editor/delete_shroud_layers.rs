use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn delete_shroud_layers(&mut self) {
        self.shroud = self
            .shroud
            .iter()
            .filter(|shroud_layer_container| !shroud_layer_container.delete_next_frame)
            .cloned()
            .collect();
    }
}
