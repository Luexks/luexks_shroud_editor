use egui::Ui;

use crate::{
    shroud_editor::ShroudEditor,
    shroud_interaction::{ShroudInteraction, is_sorted_selection_contiguous},
};

pub struct ShroudLayerReorderingMessageData {
    message: Message,
    direction: Direction,
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
    pub fn shroud_layer_reordering_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let up_button = ui.button("Reorder Selection Up");
            if up_button.clicked() {
                self.move_selection(Direction::Up);
            }
            let down_button = ui.button("Reorder Selection Down");
            if down_button.clicked() {
                self.move_selection(Direction::Down);
            }
            self.stop_displaying_message_logic(up_button, down_button);
        });
        self.message(ui);
    }

    fn message(&mut self, ui: &mut Ui) {
        if let Some(message_data) = &self.shroud_layer_reordering_message_data_option {
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
    ) {
        if let Some(message_data) = &self.shroud_layer_reordering_message_data_option {
            let stop_showing_message = (!up_button.hovered()
                && message_data.direction == Direction::Up)
                || (!down_button.hovered() && message_data.direction == Direction::Down);
            if stop_showing_message {
                self.shroud_layer_reordering_message_data_option = None;
            }
        }
    }

    fn move_selection(&mut self, direction: Direction) {
        let mut selection = self.shroud_interaction.selection();
        selection.sort();
        if self.shroud_interaction.selection().is_empty() {
            self.shroud_layer_reordering_message_data_option =
                Some(ShroudLayerReorderingMessageData {
                    message: Message::EmptySelection,
                    direction,
                });
            return;
        }
        if !is_sorted_selection_contiguous(&selection) {
            self.shroud_layer_reordering_message_data_option =
                Some(ShroudLayerReorderingMessageData {
                    message: Message::NotContiguous,
                    direction,
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
        self.shroud.iter_mut().for_each(|shroud_layer_container| {
            if let Some(mirror_idx) = &mut shroud_layer_container.mirror_index_option {
                *mirror_idx = reorder_idx(*mirror_idx, top_idx, bottom_idx, direction);
            }
        });
        match direction {
            Direction::Up => {
                self.shroud[top_idx - 1..=bottom_idx].rotate_left(1);
                let rotated_idx = top_idx - 1;
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: (rotated_idx..bottom_idx).collect(),
                };
            }
            Direction::Down => {
                let rotated_idx = bottom_idx + 1;
                self.shroud[top_idx..=bottom_idx + 1].rotate_right(1);
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
