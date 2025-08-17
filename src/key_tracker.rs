use egui::{Context, Event, Key};
use std::collections::HashSet;

pub struct KeyTracker {
    held_keys: HashSet<Key>,
}

impl Default for KeyTracker {
    fn default() -> Self {
        Self {
            held_keys: HashSet::new(),
        }
    }
}

impl KeyTracker {
    pub fn update(&mut self, ctx: &Context) {
        ctx.input(|input| {
            let _ = &input.events.iter().for_each(|event| {
                match event {
                    // Event::Copy => todo!(),
                    // Event::Cut => todo!(),
                    // Event::Paste(_) => todo!(),
                    // Event::Text(_) => todo!(),
                    // Event::Key { key, physical_key, pressed, repeat, modifiers } => todo!(),
                    // Event::PointerMoved(pos2) => todo!(),
                    // Event::MouseMoved(vec2) => todo!(),
                    // Event::PointerButton { pos, button, pressed, modifiers } => todo!(),
                    // Event::PointerGone => todo!(),
                    // Event::Zoom(_) => todo!(),
                    // Event::Ime(ime_event) => todo!(),
                    // Event::Touch { device_id, id, phase, pos, force } => todo!(),
                    // Event::MouseWheel { unit, delta, modifiers } => todo!(),
                    // Event::WindowFocused(_) => todo!(),
                    // Event::AccessKitActionRequest(action_request) => todo!(),
                    // Event::Screenshot { viewport_id, user_data, image } => todo!(),
                    Event::Key {
                        key, pressed: true, ..
                    } => {
                        // println!("Insert key");
                        self.held_keys.insert(*key);
                    }
                    Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        // println!("Remove key");
                        self.held_keys.remove(key);
                    }
                    _ => {}
                }
            });
            // dbg!(&self.held_keys);
        })
    }

    pub fn is_held(&self, key: Key) -> bool {
        self.held_keys.contains(&key)
    }
}
