use egui::{Context, Ui};

use crate::{
    shroud_editor::ShroudEditor,
    shroud_interaction::{ShroudInteraction, is_sorted_selection_contiguous},
};

pub struct ShroudLayerReorderingMessageData {
    message: Message,
    direction: Direction,
    is_floating_panel: bool,
}

enum Message {
    NotContiguous,
    EmptySelection,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

impl ShroudEditor {
    pub fn shroud_layer_reordering_buttons(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        is_floating_panel: bool,
    ) {
        ui.horizontal(|ui| {
            let up_button = ui.button("Reorder Selection Up");
            if up_button.clicked() {
                self.move_selection(Direction::Up, is_floating_panel, ctx);
            }
            let down_button = ui.button("Reorder Selection Down");
            if down_button.clicked() {
                self.move_selection(Direction::Down, is_floating_panel, ctx);
            }
            self.stop_displaying_message_logic(up_button, down_button, is_floating_panel);
        });
        self.message(ui, is_floating_panel);
        if is_floating_panel {
            ui.separator();
        }
    }

    fn message(&mut self, ui: &mut Ui, is_floating_panel: bool) {
        if let Some(message_data) = &self.shroud_layer_reordering_message_data_option
            && message_data.is_floating_panel == is_floating_panel
        {
            ui.label(match message_data.message {
                Message::NotContiguous => {
                    "Please make sure selection is contiguous (all next to each other in the list)"
                }
                Message::EmptySelection => "Selection is empty :p",
            });
        }
    }

    fn stop_displaying_message_logic(
        &mut self,
        up_button: egui::Response,
        down_button: egui::Response,
        is_floating_panel: bool,
    ) {
        if let Some(message_data) = &self.shroud_layer_reordering_message_data_option {
            let is_on_correct_panel = is_floating_panel == message_data.is_floating_panel;
            let because_up_button_is_not_interacted_with = (!up_button.hovered()
                || up_button.is_pointer_button_down_on())
                && message_data.direction == Direction::Up;
            let because_down_button_is_not_interacted_with = (!down_button.hovered()
                || down_button.is_pointer_button_down_on())
                && message_data.direction == Direction::Down;

            let stop_showing_message = (because_up_button_is_not_interacted_with
                || because_down_button_is_not_interacted_with)
                && is_on_correct_panel;
            if stop_showing_message {
                self.shroud_layer_reordering_message_data_option = None;
            }
        }
    }

    fn move_selection(&mut self, direction: Direction, is_floating_panel: bool, ctx: &Context) {
        let mut selection = self.shroud_interaction.selection();
        selection.sort();
        if self.shroud_interaction.selection().is_empty() {
            self.shroud_layer_reordering_message_data_option =
                Some(ShroudLayerReorderingMessageData {
                    message: Message::EmptySelection,
                    direction,
                    is_floating_panel,
                });
            return;
        }
        if !is_sorted_selection_contiguous(&selection) {
            self.shroud_layer_reordering_message_data_option =
                Some(ShroudLayerReorderingMessageData {
                    message: Message::NotContiguous,
                    direction,
                    is_floating_panel,
                });
            return;
        }
        if selection.len() == self.shroud.len() {
            return;
        }
        let top_idx = selection[0];
        let bottom_idx = selection[selection.len() - 1];
        if direction == Direction::Up && top_idx == 0 {
            return;
        }
        if direction == Direction::Down && bottom_idx == self.shroud.len() - 1 {
            return;
        }
        self.add_undo_history = true;
        self.shroud.iter_mut().for_each(|shroud_layer_container| {
            if let Some(mirror_idx) = &mut shroud_layer_container.mirror_index_option {
                *mirror_idx = reorder_idx(*mirror_idx, top_idx, bottom_idx, direction);
            }
        });
        let slice_range = match direction {
            Direction::Up => top_idx - 1..=bottom_idx,
            Direction::Down => top_idx..=bottom_idx + 1,
        };
        match direction {
            Direction::Up => {
                self.shroud[slice_range].rotate_left(1);
                let rotated_idx = top_idx - 1;
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: (rotated_idx..bottom_idx).collect(),
                };
            }
            Direction::Down => {
                let rotated_idx = bottom_idx + 1;
                self.shroud[slice_range].rotate_right(1);
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: (top_idx + 1..=rotated_idx).collect(),
                };
            }
        }
    }
}

const fn reorder_idx(idx: usize, top_idx: usize, bottom_idx: usize, direction: Direction) -> usize {
    match direction {
        Direction::Up => {
            let rotated_idx = top_idx - 1;
            if idx < rotated_idx || bottom_idx < idx {
                idx
            } else if idx == rotated_idx {
                bottom_idx
            } else {
                idx - 1
            }
        }
        Direction::Down => {
            let rotated_idx = bottom_idx + 1;
            if idx < top_idx || rotated_idx < idx {
                idx
            } else if idx == rotated_idx {
                top_idx
            } else {
                idx + 1
            }
        }
    }
}
