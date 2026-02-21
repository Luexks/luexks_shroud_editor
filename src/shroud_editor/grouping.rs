use egui::Context;
use itertools::Itertools;

use crate::{keybinds::is_shortcut_pressed, shroud_editor::ShroudEditor};

impl ShroudEditor {
    pub fn hotkey_grouping(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.group) {
            self.group_selection();
        }
    }

    fn group_selection(&mut self) {
        let selection = self.shroud_interaction.selection();
        if selection.len() < 2 {
            return;
        }
        if let Some(first_group_idx) = self.shroud[selection[0]].group_idx_option
            && selection
                .iter()
                .skip(1)
                .all(|i| self.shroud[*i].group_idx_option == Some(first_group_idx))
        {
            self.remove_group(first_group_idx);
            return;
        }
        selection
            .iter()
            .filter_map(|i| self.shroud[*i].group_idx_option)
            .sorted_unstable()
            .unique()
            .rev()
            .for_each(|group_idx| {
                self.remove_group(group_idx);
            });
        selection.iter().for_each(|i| {
            self.shroud[*i].group_idx_option = Some(self.groups.len());
        });
        self.groups.push(selection);
    }

    fn remove_group(&mut self, group_idx: usize) {
        if group_idx >= self.groups.len() {
            return;
        }
        let group = self.groups.remove(group_idx);
        group.into_iter().for_each(|i| {
            self.shroud[i].group_idx_option = None;
        });
    }

    pub fn cull_empty_groups(&mut self) {
        self.groups.retain(|group| !group.is_empty());
    }
}
