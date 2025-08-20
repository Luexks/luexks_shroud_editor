use egui::{Context, Pos2, Rect, Response, Ui};

use crate::shroud_editor::ShroudEditor;

#[derive(Clone)]
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

impl ShroudEditor {
    pub fn shroud_layer_interaction_update(&mut self, ui: &mut Ui, ctx: &Context, response: &Response, rect: &Rect) {
                let mouse_pos = response.interact_pointer_pos();
                if let Some(mouse_pos) = mouse_pos {
                    // if response.clicked() {
                    // if ui.input(|i| i.pointer.primary_released()) {
                    if ui.input(|i| i.pointer.primary_pressed()) {
                        if let Some(shroud_that_would_be_selected_index) =
                            self.get_shroud_that_would_be_selected_index_option(mouse_pos, *rect)
                        {
                            // self.shroud_layer_interaction = ShroudLayerInteraction::Inaction { selection: self.shroud_layer_interaction.selection().iter().chain(std::iter::once(&shroud_that_would_be_selected_index)).map(|index| *index).collect() }
                            if ctx.input(|i| i.modifiers.shift) {
                                if !self
                                    .shroud_layer_interaction
                                    .selection()
                                    .contains(&shroud_that_would_be_selected_index)
                                {
                                    self.shroud_layer_interaction =
                                        ShroudLayerInteraction::Inaction {
                                            selection: self
                                                .shroud_layer_interaction
                                                .selection()
                                                .iter()
                                                .copied()
                                                .chain(std::iter::once(
                                                    shroud_that_would_be_selected_index,
                                                ))
                                                .collect(),
                                        };
                                }
                            } else {
                                self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                    selection: vec![shroud_that_would_be_selected_index],
                                };
                            }
                        } else {
                            self.shroud_layer_interaction = ShroudLayerInteraction::Inaction {
                                selection: Vec::new(),
                            };
                        }
                    }

                    if response.drag_started() {
                        if !self.shroud_layer_interaction.selection().is_empty() {
                            self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                                drag_start_pos: mouse_pos,
                                selection: self.shroud_layer_interaction.selection(),
                            };
                        }
                        // if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                        //     self.shroud_layer_interaction = ShroudLayerInteraction::Dragging {
                        //         drag_start_pos: mouse_pos,
                        //         selection: vec![shroud_that_would_be_selected_index],
                        //     };
                        // }
                    }
                }
            }
}