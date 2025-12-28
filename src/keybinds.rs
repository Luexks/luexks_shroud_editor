use egui::{
    Context, Event, Grid, Key, KeyboardShortcut, ModifierNames, Modifiers, Ui,
    collapsing_header::CollapsingState,
};

use crate::shroud_editor::ShroudEditor;

#[derive(Debug)]
pub struct Keybinds {
    pub pan_up: Option<Key>,
    pub pan_up_expecting: bool,
    pub pan_down: Option<Key>,
    pub pan_down_expecting: bool,
    pub pan_right: Option<Key>,
    pub pan_right_expecting: bool,
    pub pan_left: Option<Key>,
    pub pan_left_expecting: bool,
    pub copy: Option<KeyboardShortcut>,
    pub paste: Option<KeyboardShortcut>,
    pub mirror: Option<KeyboardShortcut>,
    pub delete: Option<KeyboardShortcut>,
    pub copy_expecting: bool,
    pub paste_expecting: bool,
    pub mirror_expecting: bool,
    pub delete_expecting: bool,
}

#[rustfmt::skip]
impl Default for Keybinds {
    fn default() -> Self {
        Keybinds {
            pan_up:     Some(Key::W),
            pan_down:   Some(Key::S),
            pan_right:  Some(Key::D),
            pan_left:   Some(Key::A),
            copy:       Some(KeyboardShortcut::new(Modifiers::NONE, Key::C)),
            paste:      Some(KeyboardShortcut::new(Modifiers::NONE, Key::V)),
            mirror:     Some(KeyboardShortcut::new(Modifiers::NONE, Key::F)),
            delete:     Some(KeyboardShortcut::new(Modifiers::NONE, Key::R)),

            pan_up_expecting: false,
            pan_down_expecting: false,
            pan_right_expecting: false,
            pan_left_expecting: false,
            copy_expecting: false,
            paste_expecting: false,
            mirror_expecting: false,
            delete_expecting: false,
        }
    }
}

#[rustfmt::skip]
impl ShroudEditor {
    pub fn binding_config(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "bindings".into(), true)
            .show_header(ui, |ui| ui.label("Bindings"))
            .body_unindented(|ui| {
                ui.label("Bindings are automatically loaded on startup from bindings file if one exists.");
                ui.small("Apology: if your clipboard does not contain text, this program cannot detect Ctrl+V because my GUI framework is eating the paste event, sorry :(");
                if ui.button("Save Bindings").clicked() {
                    let _ = self.save_keybinds();
                }
                let keybinds = &mut self.keybinds;
                ui.label("Click away to cancel, press escape to unbind.");
                Grid::new("bindingsgrid").show(ui, |ui| {
                    keyboard_binding_button(ctx, ui, &mut keybinds.pan_up, &mut keybinds.pan_up_expecting, "Pan Up");
                    keyboard_binding_button(ctx, ui, &mut keybinds.pan_down, &mut keybinds.pan_down_expecting, "Pan Down");
                    keyboard_binding_button(ctx, ui, &mut keybinds.pan_right, &mut keybinds.pan_right_expecting, "Pan Right");
                    keyboard_binding_button(ctx, ui, &mut keybinds.pan_left, &mut keybinds.pan_left_expecting, "Pan Left");
                    keyboard_and_modifiers_binding_button(ctx, ui, &mut keybinds.copy, &mut keybinds.copy_expecting, "Copy");
                    keyboard_and_modifiers_binding_button(ctx, ui, &mut keybinds.paste, &mut keybinds.paste_expecting, "Paste");
                    keyboard_and_modifiers_binding_button(ctx, ui, &mut keybinds.mirror, &mut keybinds.mirror_expecting, "Mirror");
                    keyboard_and_modifiers_binding_button(ctx, ui, &mut keybinds.delete, &mut keybinds.delete_expecting, "Delete");
                });
            });
    }
}

pub fn is_keyboard_binding_down(ctx: &Context, binding_option: &Option<Key>) -> bool {
    if let Some(binding) = binding_option {
        ctx.input(|i| i.key_down(*binding))
    } else {
        false
    }
}

fn keyboard_binding_button(
    ctx: &Context,
    ui: &mut Ui,
    binding_option: &mut Option<Key>,
    expecting: &mut bool,
    name: &str,
) {
    ui.label(name);
    let button = ui.button(format_keyboard_binding(binding_option));
    if *expecting {
        if button.clicked_elsewhere() {
            *expecting = false
        } else {
            button.highlight();
            if let Some(key) = ctx.input(|i| {
                i.events.iter().find_map(|e| match e {
                    Event::Key {
                        key,
                        physical_key: _,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::NONE,
                    } => Some(*key),
                    _ => None,
                })
            }) {
                *expecting = false;
                if key == Key::Escape {
                    *binding_option = None;
                } else {
                    *binding_option = Some(key);
                }
            }
        }
    } else if button.clicked() {
        *expecting = true;
    }
    ui.end_row();
}

fn format_keyboard_binding(binding_option: &Option<Key>) -> &str {
    match binding_option {
        Some(binding) => binding.name(),
        None => "None",
    }
}

pub fn is_shortcut_pressed(ctx: &Context, binding_option: &Option<KeyboardShortcut>) -> bool {
    if let Some(shortcut) = binding_option {
        ctx.input_mut(|i| i.consume_shortcut(shortcut))
            || event_pressed_workaround(ctx, binding_option)
    } else {
        false
    }
}

fn keyboard_and_modifiers_binding_button(
    ctx: &Context,
    ui: &mut Ui,
    binding_option: &mut Option<KeyboardShortcut>,
    expecting: &mut bool,
    name: &str,
) {
    ui.label(name);
    let button = ui.button(format_keyboard_and_modifiers_binding(binding_option));
    if *expecting {
        if button.clicked_elsewhere() {
            *expecting = false
        } else {
            button.highlight();
            if let Some(keyboard_shortcut) = ctx.input(|i| {
                i.events.iter().find_map(|e| match e {
                    Event::Key {
                        key,
                        physical_key: _,
                        pressed: true,
                        repeat: false,
                        modifiers,
                    } => Some(KeyboardShortcut::new(*modifiers, *key)),
                    _ => None,
                })
            }) {
                *expecting = false;
                if keyboard_shortcut.logical_key == Key::Escape {
                    *binding_option = None;
                } else {
                    *binding_option = Some(keyboard_shortcut);
                }
            }
            event_binding_workaround(ctx, binding_option, expecting);
        }
    } else if button.clicked() {
        *expecting = true;
    }
    ui.end_row();
}

fn format_keyboard_and_modifiers_binding(binding_option: &Option<KeyboardShortcut>) -> String {
    match binding_option {
        Some(binding) => binding.format(&ModifierNames::NAMES, false),
        None => "None".to_string(),
    }
}

fn event_binding_workaround(
    ctx: &Context,
    binding_option: &mut Option<KeyboardShortcut>,
    expecting: &mut bool,
) {
    ctx.input(|i| {
        if i.events.contains(&Event::Copy) {
            *binding_option = Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::C));
            *expecting = false;
        } else if i.events.iter().any(|e| matches!(*e, Event::Paste(_))) {
            *binding_option = Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::V));
            *expecting = false;
        } else if i.events.contains(&Event::Cut) {
            *binding_option = Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::X));
            *expecting = false;
        }
    });
}

fn event_pressed_workaround(ctx: &Context, binding_option: &Option<KeyboardShortcut>) -> bool {
    (*binding_option == Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::C))
        && ctx.input(|i| i.events.contains(&Event::Copy)))
        || (*binding_option == Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::V))
            && ctx.input(|i| i.events.iter().any(|e| matches!(*e, Event::Paste(_)))))
        || (*binding_option == Some(KeyboardShortcut::new(Modifiers::COMMAND, Key::X))
            && ctx.input(|i| i.events.contains(&Event::Cut)))
}
