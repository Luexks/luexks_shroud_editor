use egui::{Context, Key, pos2};

use crate::{
    shroud_editor::{ShroudEditor, add_mirror::add_mirror},
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

impl ShroudEditor {
    pub fn hotkey_copy(&mut self, ctx: &Context) {
        let hotkey_pressed = ctx.input(|i| i.key_pressed(Key::C));
        if hotkey_pressed {
            self.shroud_clipboard = self
                .shroud_layer_interaction
                .selection()
                .iter()
                .map(|index| self.shroud[*index].clone())
                .collect();
        }
    }
    pub fn hotkey_paste(&mut self, ctx: &Context) {
        if let ShroudLayerInteraction::Placing { .. } = self.shroud_layer_interaction {
        } else {
            let hotkey_pressed = ctx.input(|i| i.key_pressed(Key::V));
            if hotkey_pressed {
                let clipboard_count = self.shroud_clipboard.len();
                let avg_pos = self.shroud_clipboard.iter().fold(
                    pos2(0.0, 0.0),
                    |acc_pos, shroud_layer_container| {
                        let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
                        pos2(acc_pos.x + offset.x.to_f32(), acc_pos.y + offset.y.to_f32())
                    },
                ) / clipboard_count as f32;
                self.shroud_clipboard
                    .iter()
                    .for_each(|shroud_layer_container| {
                        let old_offset =
                            shroud_layer_container.shroud_layer.offset.clone().unwrap();
                        let drag_pos = pos2(
                            old_offset.x.to_f32() - avg_pos.x + self.world_mouse_pos.x,
                            old_offset.y.to_f32() - avg_pos.y + self.world_mouse_pos.y,
                        );
                        let new_shroud_layer_container = ShroudLayerContainer {
                            drag_pos: Some(drag_pos),
                            ..shroud_layer_container.clone()
                        };
                        self.shroud.push(new_shroud_layer_container);
                        let last = self.shroud.len() - 1;
                        if let Some(_mirror_index) = self.shroud[last].mirror_index_option {
                            add_mirror(
                                &mut self.shroud,
                                last,
                                true,
                                &self.loaded_shapes,
                                &self.loaded_shapes_mirror_pairs,
                            );
                        }
                    });
                let clipboard_count_plus_mirrors =
                    self.shroud_clipboard
                        .iter()
                        .fold(0, |count, shroud_layer_container| {
                            if let Some(_mirror_index) = shroud_layer_container.mirror_index_option
                            {
                                count + 2
                            } else {
                                count + 1
                            }
                        });
                self.shroud_layer_interaction = ShroudLayerInteraction::Placing {
                    selection: (self.shroud.len() - clipboard_count_plus_mirrors
                        ..self.shroud.len())
                        .collect(),
                }
            }
        }
    }
}
