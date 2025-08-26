use egui::{Context, Event, Key};
use std::collections::HashSet;

#[derive(Default)]
pub struct KeyTracker {
    held_keys: HashSet<Key>,
}

impl KeyTracker {
    pub fn update(&mut self, ctx: &Context) {
        ctx.input(|input| {
            let _ = &input.events.iter().for_each(|event| {
                match event {
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
        })
    }

    pub fn is_held(&self, key: Key) -> bool {
        self.held_keys.contains(&key)
    }
}
