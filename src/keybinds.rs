use egui::{Context, Key, KeyboardShortcut, Modifiers, Ui, collapsing_header::CollapsingState};
use egui_keybind::{Bind, Keybind, Shortcut};

use crate::shroud_editor::ShroudEditor;

pub struct Keybinds {
    pub pan_up: Shortcut,
    pub pan_down: Shortcut,
    pub pan_right: Shortcut,
    pub pan_left: Shortcut,
    // zoom_in: Shortcut,
    // zoom_out: Shortcut,
    // select: Shortcut,
    pub copy: Shortcut,
    pub paste: Shortcut,
    pub mirror: Shortcut,
    pub delete: Shortcut,
}

#[rustfmt::skip]
impl Default for Keybinds {
    fn default() -> Self {
        Keybinds {
            pan_up:     Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::W)), None),
            pan_down:   Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::S)), None),
            pan_right:  Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::D)), None),
            pan_left:   Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::A)), None),
            // zoom_in:    Shortcut::new(None, Some(Poi)),
            // zoom_out:   Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::W)), None),
            // select:     Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::W)), None),
            copy:       Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::C)), None),
            paste:      Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::V)), None),
            mirror:     Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::F)), None),
            delete:     Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::R)), None),
        }
    }
}

impl ShroudEditor {
    pub fn binding_config(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "bindings".into(), true)
            .show_header(ui, |ui| ui.label("Bindings"))
            .body_unindented(|ui| {
                ui.add(Keybind::new(&mut self.keybinds.pan_up, "up").with_text("Pan Up"));
                ui.add(Keybind::new(&mut self.keybinds.pan_down, "down").with_text("Pan Down"));
                ui.add(Keybind::new(&mut self.keybinds.pan_right, "right").with_text("Pan Right"));
                ui.add(Keybind::new(&mut self.keybinds.pan_left, "left").with_text("Pan Left"));
            });
    }
}
