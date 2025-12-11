use egui::{Context, Key, Pos2, Vec2, pos2};

use crate::{
    invert_y::invert_y_of_pos2, pos_and_display_oriented_number_conversion::do3d_to_pos2, shroud_editor::{ShroudEditor, add_mirror::add_mirror}, shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection, ShroudInteraction}, shroud_layer_container::ShroudLayerContainer
};

impl ShroudEditor {
    pub fn hotkey_copy(&mut self, ctx: &Context) {
        let hotkey_pressed = ctx.input(|i| i.key_pressed(Key::C));
        if hotkey_pressed {
            self.shroud_clipboard = self
                .shroud_interaction
                .selection()
                .iter()
                .map(|index| self.shroud[*index].clone())
                .collect();
        }
    }
    pub fn hotkey_paste(&mut self, ctx: &Context) {
        if let ShroudInteraction::Placing { .. } = self.shroud_interaction {
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
                        let new_shroud_layer_container = ShroudLayerContainer {
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
                            if shroud_layer_container.mirror_index_option.is_some() {
                                count + 2
                            } else {
                                count + 1
                            }
                        });
                // let drag_pos = pos2(world_mouse_pos.x, -world_mouse_pos.y);
                let world_mouse_pos_inverted_y = invert_y_of_pos2(self.world_mouse_pos);
                let drag_pos = Pos2::default() - do3d_to_pos2(self.shroud_clipboard[0].shroud_layer.offset.as_ref().unwrap());
                self.shroud_interaction = ShroudInteraction::Placing {
                    selection: MovingShroudSelection(
                        (self.shroud.len() - clipboard_count_plus_mirrors..self.shroud.len())
                            .map(|idx| {
                                MovingShroudLayerInteraction {
                                    idx,
                                    // relative_pos: (do3d_to_pos2(self.shroud[idx].shroud_layer.offset.as_ref().unwrap()) + drag_pos).to_vec2(),
                                    // relative_pos: Vec2::default(),
                                    relative_pos: (do3d_to_pos2(self.shroud[idx].shroud_layer.offset.as_ref().unwrap()) + drag_pos).to_vec2(),
                                }
                            })
                            .collect(),
                    ),
                    drag_pos: world_mouse_pos_inverted_y,
                    potentially_snapped_drag_pos: world_mouse_pos_inverted_y,
                }
            }
        }
    }
}
