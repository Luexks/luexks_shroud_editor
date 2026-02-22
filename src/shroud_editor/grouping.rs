use egui::Context;
use itertools::Itertools;

use crate::{keybinds::is_shortcut_pressed, shroud_editor::ShroudEditor};

impl ShroudEditor {
    pub fn hotkey_grouping(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.group) {
            self.group_selection();
            // dbg!(&self.groups);
        }
    }

    fn group_selection(&mut self) {
        let selection = self.shroud_interaction.selection();
        if selection.len() < 2 {
            return;
        }
        if let Some(first_layer_group_idx) =
            self.get_one_entire_selected_group_idx_option(&selection)
        {
            self.remove_group(first_layer_group_idx);
            return;
        }
        selection
            .iter()
            .filter_map(|i| self.shroud[*i].group_idx_option)
            .sorted_unstable()
            .unique()
            .rev()
            .for_each(|layer_group_idx| {
                self.remove_group(layer_group_idx);
            });
        selection.iter().for_each(|i| {
            self.shroud[*i].group_idx_option = Some(self.groups.len());
        });
        self.groups.push(selection);
    }

    fn remove_group(&mut self, groups_group_idx: usize) {
        assert!(groups_group_idx < self.groups.len());
        let group = self.groups.remove(groups_group_idx);
        group.into_iter().for_each(|group_layer_idx| {
            self.shroud[group_layer_idx].group_idx_option = None;
        });
        self.shroud.iter_mut().for_each(|layer| {
            if let Some(layer_group_idx) = &mut layer.group_idx_option
                && *layer_group_idx > groups_group_idx
            {
                *layer_group_idx -= 1;
            }
        });
    }

    pub fn cull_empty_groups(&mut self) {
        self.groups
            .iter()
            .enumerate()
            .rev()
            .for_each(|(groups_group_idx, group)| {
                if group.is_empty() {
                    self.shroud.iter_mut().for_each(|layer| {
                        if let Some(layer_group_idx) = &mut layer.group_idx_option
                            && *layer_group_idx > groups_group_idx
                        {
                            *layer_group_idx -= 1;
                        }
                    });
                }
            });
        self.groups.retain(|group| !group.is_empty());
    }

    pub fn get_one_entire_selected_group_idx_option(&self, selection: &[usize]) -> Option<usize> {
        let first = selection.first()?;
        if let Some(first_layer_group_idx) = self.shroud[*first].group_idx_option
            && selection.len() == self.groups[first_layer_group_idx].len()
            && selection
                .iter()
                .skip(1)
                .all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx))
        {
            Some(first_layer_group_idx)
        } else {
            None
        }
    }

    // pub fn get_one_entire_group_idx_option_of_layers(
    //     &self,
    //     layers: &[ShroudLayerContainer],
    // ) -> Option<usize> {
    //     let first_layer_group_idx = layers.first()?.group_idx_option?;
    //     (layers.len() == self.groups[first_layer_group_idx].len()
    //         && layers
    //             .iter()
    //             .skip(1)
    //             .all(|layer| layer.group_idx_option == Some(first_layer_group_idx)))
    //     .then_some(first_layer_group_idx)
    // }

    // pub fn are_group_idxs_of_one_entire_group(&self, group_idxs: &[usize]) -> bool {
    //     let first = selection.first()?;
    //         if let Some(first_layer_group_idx) = self.shroud[*first].group_idx_option
    //         && selection.len() == self.groups[*first].len()
    //         && selection.iter().skip(1).all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx)) {
    //             Some(first_layer_group_idx)
    //         } else {
    //             None
    //         }
    // }

    // pub fn is_one_entire_group_selected(&self, selection: &[usize]) -> bool {
    //     selection
    //         .first()
    //         .and_then(|first| self.shroud[*first].group_idx_option)
    //         .map(|first_layer_group_idx| selection.len() == self.groups[first_layer_group_idx].len()
    //             && selection.iter().skip(1).all(|&i| self.shroud[i].group_idx_option == Some(first_layer_group_idx))
    // ).unwrap_or(false)

    // //     selection
    // //         .first()
    // //         .and_then(|first| self.shroud[*first].group_idx_option)
    // //         .map(|first_layer_group_idx| (selection.len() == self.groups[first_layer_group_idx].len()
    // //             && selection.iter().skip(1).all(|&i| self.shroud[i].group_idx_option == Some(first_layer_group_idx))).then_some(first_layer_group_idx)
    // // ).unwrap_or_default()

    //     // if let Some(first_layer_group_idx) = self.shroud[selection[0]].group_idx_option
    //     //     && selection
    //     //         .iter()
    //     //         .skip(1)
    //     //         .all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx))
    //     //     && selection.len() == self.groups[first_layer_group_idx].len() {
    //     //         true
    //     //     } else {
    //     //         false
    //     //     }
    // }
}
