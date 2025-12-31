use crate::keybinds::is_shortcut_pressed;
use crate::{shroud_editor::ShroudEditor, shroud_interaction::ShroudInteraction};
use egui::{Area, Color32, Context, Frame, Id, Ui, pos2};

const UNDO_HISTORY_MAX_SNAPSHOTS: usize = 32;
// const UNDO_HISTORY_MAX_SNAPSHOTS: usize = 3;

impl ShroudEditor {
    pub fn add_undo_history_logic(&mut self) {
        if !self.add_undo_history {
            return;
        }
        // println!("{}\t{}", self.undo_history.len(), self.undo_history_index);
        self.add_undo_history = false;
        if self.undo_history_index + 1 < self.undo_history.len() {
            self.undo_history.truncate(self.undo_history_index + 1);
        }
        if self.undo_history_index == UNDO_HISTORY_MAX_SNAPSHOTS {
            self.undo_history.remove(0);
        } else {
            self.undo_history_index += 1;
        }
        self.undo_history.push(self.shroud.clone());
        // println!("\t\t{}\t{}", self.undo_history.len(), self.undo_history_index);
        // println!("{}\t{}", self.undo_history.len(), self.undo_history_index);
    }

    pub fn try_undo(&mut self) {
        if self.undo_history_index == 0 {
            return;
        }
        self.undo_history_index -= 1;
        self.shroud = self.undo_history[self.undo_history_index].clone();
        self.shroud_interaction = ShroudInteraction::none();
    }

    pub fn try_redo(&mut self) {
        if self.undo_history_index + 1 == self.undo_history.len() {
            return;
        }
        self.undo_history_index += 1;
        self.shroud = self.undo_history[self.undo_history_index].clone();
        self.shroud_interaction = ShroudInteraction::none();
    }

    pub fn hotkey_undo_redo(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.undo) {
            self.try_undo();
            // println!("{}\t{}", self.undo_history.len(), self.undo_history_index);
        }
        if is_shortcut_pressed(ctx, &self.keybinds.redo) {
            self.try_redo();
            // println!("{}\t{}", self.undo_history.len(), self.undo_history_index);
        }
    }

    pub fn undo_redo_buttons(&mut self, ctx: &Context, ui: &mut Ui) {
        Area::new(Id::new("undoredo"))
            .fixed_pos(pos2(8.0, 23.0))
            .default_width(290.0)
            .fade_in(false)
            .show(ctx, |ui| {
                Frame::new().fill(Color32::WHITE).show(ui, |ui| {
                    ui.take_available_width();
                    ui.horizontal(|ui| {
                        if ui.button("Undo").clicked() {
                            self.try_undo();
                        }
                        if ui.button("Redo").clicked() {
                            self.try_redo();
                        }
                    });
                    ui.add_space(4.0);
                });
            });
        ui.add_space(23.0);
    }
}
