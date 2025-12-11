use egui::{Context, Pos2, Rect, Response, Ui, pos2};
use itertools::Itertools;

use crate::{pos_and_display_oriented_number_conversion::do3d_to_pos2, shroud_editor::ShroudEditor};

#[derive(Clone)]
pub struct MovingShroudSelection(pub Vec<MovingShroudLayerInteraction>);

impl MovingShroudSelection {
    fn get_indexes(&self) -> Vec<usize> {
        self.0
            .clone()
            .into_iter()
            .map(
                |MovingShroudLayerInteraction {
                     idx: idx,
                     drag_pos: _,
                 }| idx,
            )
            .collect()
    }
}

#[derive(Clone)]
pub struct MovingShroudLayerInteraction {
    pub idx: usize,
    pub drag_pos: Pos2,
}

#[derive(Clone)]
pub enum ShroudInteraction {
    Inaction {
        selection: Vec<usize>,
    },
    Dragging {
        main_idx: usize,
        drag_pos: Pos2,
        position_change: Pos2,
        selection: MovingShroudSelection,
    },
    Placing {
        main_idx: usize,
        drag_pos: Pos2,
        position_change: Pos2,
        selection: MovingShroudSelection,
    },
}

impl ShroudInteraction {
    pub fn selection(&self) -> Vec<usize> {
        match self {
            ShroudInteraction::Inaction { selection } => selection.clone(),
            ShroudInteraction::Dragging { selection, .. } => selection.get_indexes(),
            ShroudInteraction::Placing { selection, .. } => selection.get_indexes(),
        }
    }
    pub fn is_shroud_layer_index_selected(&self, index: usize) -> bool {
        match self {
            ShroudInteraction::Inaction { selection } => selection.iter().contains(&index),
            ShroudInteraction::Dragging { selection, .. } => {
                selection.get_indexes().iter().contains(&index)
            }
            ShroudInteraction::Placing { selection, .. } => {
                selection.get_indexes().iter().contains(&index)
            }
        }
    }
}

impl ShroudEditor {
    pub fn shroud_interaction_update(
        &mut self,
        ui: &mut Ui,
        ctx: &Context,
        response: &Response,
        rect: &Rect,
    ) {
        let mouse_pos = response.interact_pointer_pos();
        if let Some(mouse_pos) = mouse_pos {
            // if response.clicked() {
            // if ui.input(|i| i.pointer.primary_released()) {
            if let ShroudInteraction::Placing { .. } = &self.shroud_interaction {
                if ui.input(|i| i.pointer.primary_clicked()) {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: Vec::new(),
                    };
                }
            } else {
                if ui.input(|i| i.pointer.primary_pressed()) {
                    if let Some(shroud_that_would_be_selected_index) =
                        self.get_shroud_that_would_be_selected_index_option(mouse_pos, *rect)
                    {
                        // self.shroud_interaction = ShroudInteraction::Inaction { selection: self.shroud_interaction.selection().iter().chain(std::iter::once(&shroud_that_would_be_selected_index)).map(|index| *index).collect() }
                        if ctx.input(|i| i.modifiers.shift) {
                            if !self
                                .shroud_interaction
                                .selection()
                                .contains(&shroud_that_would_be_selected_index)
                            {
                                self.shroud_interaction = ShroudInteraction::Inaction {
                                    selection: self
                                        .shroud_interaction
                                        .selection()
                                        .iter()
                                        .copied()
                                        .chain(std::iter::once(shroud_that_would_be_selected_index))
                                        .collect(),
                                };
                            }
                        } else {
                            self.shroud_interaction = ShroudInteraction::Inaction {
                                selection: vec![shroud_that_would_be_selected_index],
                            };
                        }
                    } else {
                        self.shroud_interaction = ShroudInteraction::Inaction {
                            selection: Vec::new(),
                        };
                    }
                }

                if response.drag_started() && !self.shroud_interaction.selection().is_empty() && let Some(dragged_shroud_layer_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, *rect) {
                    self.shroud_interaction = ShroudInteraction::Dragging {
                        selection: MovingShroudSelection(
                            self.shroud_interaction
                                .selection()
                                .iter()
                                .map(|idx| {
                                    let drag_pos =
                                        self.shroud[*idx].shroud_layer.offset.as_ref().unwrap();
                                    let drag_pos = pos2(drag_pos.x.to_f32(), drag_pos.y.to_f32());
                                    MovingShroudLayerInteraction {
                                        idx: *idx,
                                        drag_pos: drag_pos,
                                    }
                                })
                                .collect(),
                        ),
                        main_idx: 0,
                        drag_pos: do3d_to_pos2(self.shroud[dragged_shroud_layer_index].shroud_layer.offset.as_ref().unwrap()),
                        position_change: Pos2::default(),
                    }
                }
            }
        }
    }
}
