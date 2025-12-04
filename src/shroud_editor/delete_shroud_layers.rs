use egui::{Context, collapsing_header::CollapsingState};

use crate::{shroud_editor::ShroudEditor, shroud_interaction::ShroudInteraction};

impl ShroudEditor {
    pub fn delete_shroud_layers(&mut self, ctx: &Context) {
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
            let mut drop_down = CollapsingState::load(ctx, index.to_string().into()).unwrap();
            drop_down.set_open(true);
            drop_down.store(ctx);

            self.shroud.remove(*index);
            self.shroud.iter_mut().for_each(|shroud_layer_container| {
                if let Some(mirror_index) = shroud_layer_container.mirror_index_option
                    && mirror_index > *index
                {
                    shroud_layer_container.mirror_index_option = Some(mirror_index - 1);
                }
            });
            self.shroud_interaction
                .selection()
                .iter()
                .copied()
                .enumerate()
                .for_each(|(index_in_selection_list, selected_index)| {
                    if selected_index > *index {
                        let mut new_selection = self.shroud_interaction.selection().clone();
                        new_selection[index_in_selection_list] -= 1;
                        self.shroud_interaction = ShroudInteraction::Inaction {
                            selection: new_selection,
                        };
                    }
                });
        });
    }
}
