use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

use egui::{Key, KeyboardShortcut, ModifierNames};

use crate::{keybinds::Keybinds, shroud_editor::ShroudEditor};

impl Display for Keybinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            format_keyboard_binding("up", &self.pan_up),
            format_keyboard_binding("down", &self.pan_down),
            format_keyboard_binding("right", &self.pan_right),
            format_keyboard_binding("left", &self.pan_left),
            format_keyboard_shortcut_binding("yank", &self.copy),
            format_keyboard_shortcut_binding("paste", &self.paste),
            format_keyboard_shortcut_binding("mirror", &self.mirror),
            format_keyboard_shortcut_binding("delete", &self.delete),
        )
    }
}

fn format_keyboard_binding(name: &str, binding_option: &Option<Key>) -> String {
    match binding_option {
        Some(binding) => format!("{} {}\n", name, binding.name()),
        None => format!("{} in the Gamma Void\n", name),
    }
}

fn format_keyboard_shortcut_binding(
    name: &str,
    binding_option: &Option<KeyboardShortcut>,
) -> String {
    match binding_option {
        Some(binding) => format!(
            "{} {}\n",
            name,
            binding.format(&ModifierNames::NAMES, false)
        ),
        None => format!("{} is in the Gamma Void\n", name),
    }
}

impl ShroudEditor {
    pub fn save_keybinds(&self) -> io::Result<()> {
        let mut file = File::create("arthur.danskin")?;
        write!(file, "{}", self.keybinds.to_string())?;
        Ok(())
    }
}
