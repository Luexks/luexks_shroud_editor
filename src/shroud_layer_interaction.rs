use egui::Pos2;

pub enum ShroudLayerInteraction {
    Inaction {
        selection: Vec<usize>,
    },
    Dragging {
        drag_start_pos: Pos2,
        selection: Vec<usize>,
    },
}

impl ShroudLayerInteraction {
    pub fn selection(&self) -> Vec<usize> {
        match self {
            ShroudLayerInteraction::Inaction { selection } => selection.clone(),
            ShroudLayerInteraction::Dragging { selection, .. } => selection.clone(),
        }
    }
    pub fn is_shroud_layer_index_selected(&self, index: usize) -> bool {
        let is_shroud_index_selected = match self {
            ShroudLayerInteraction::Inaction { selection } => {
                if let Some(_index) = selection
                    .iter()
                    .find(|selected_index| index == **selected_index)
                {
                    true
                } else {
                    false
                }
            }
            ShroudLayerInteraction::Dragging { selection, .. } => {
                if let Some(_index) = selection
                    .iter()
                    .find(|selected_index| index == **selected_index)
                {
                    true
                } else {
                    false
                }
            }
        };
        is_shroud_index_selected
    }
}
