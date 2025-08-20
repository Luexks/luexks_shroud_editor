use egui::{Context, Key, pos2};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer, utility::display_oriented_math::do3d_float_from,
};

use crate::{
    shroud_editor::ShroudEditor,
    shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::{self, ShroudLayerInteraction},
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
                        let acc_pos =
                            pos2(acc_pos.x + offset.x.to_f32(), acc_pos.y + offset.y.to_f32());
                        acc_pos
                    },
                ) / clipboard_count as f32;
                self.shroud_clipboard
                    .iter()
                    .for_each(|shroud_layer_container| {
                        let old_offset =
                            shroud_layer_container.shroud_layer.offset.clone().unwrap();
                        let new_offset = do3d_float_from(
                            -avg_pos.x + old_offset.x.to_f32() + self.world_mouse_pos.x,
                            -avg_pos.y + old_offset.y.to_f32() + self.world_mouse_pos.y,
                            old_offset.z.to_f32(),
                        );
                        let new_shroud_layer_container = ShroudLayerContainer {
                            shroud_layer: ShroudLayer {
                                offset: Some(new_offset),
                                ..shroud_layer_container.shroud_layer.clone()
                            },
                            ..shroud_layer_container.clone()
                        };
                        self.shroud.push(new_shroud_layer_container);
                    });
                self.shroud_layer_interaction = ShroudLayerInteraction::Placing {
                    selection: (self.shroud.len() - clipboard_count..self.shroud.len()).collect(),
                }
            }
        }
    }
}
