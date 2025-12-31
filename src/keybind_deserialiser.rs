use std::{fs::File, io::Read};

use crate::{keybinds::Keybinds, shroud_editor::ShroudEditor};
use egui::{Key, KeyboardShortcut, Modifiers};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take_until},
    character::complete::newline,
    combinator::{complete, map, map_parser, opt, rest, value},
    sequence::delimited,
};

impl ShroudEditor {
    pub fn _try_load_keybinds(&mut self) -> Result<(), ()> {
        let mut file = File::open("arthur.danskin").map_err(|_| ())?;
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(|_| ())?;
        let keybinds = deserialise_keybinds(&s).map_err(|_| ())?.1;
        self.keybinds = keybinds;
        Ok(())
    }
}

pub fn try_load_keybinds() -> Result<Keybinds, ()> {
    let mut file = File::open("arthur.danskin").map_err(|_| ())?;
    let mut s = String::new();
    file.read_to_string(&mut s).map_err(|_| ())?;
    Ok(deserialise_keybinds(&s).map_err(|_| ())?.1)
}

fn deserialise_keybinds(s: &str) -> IResult<&str, Keybinds> {
    let (s, pan_up) = deserialise_keyboard_binding(s, "up")?;
    let (s, pan_down) = deserialise_keyboard_binding(s, "down")?;
    let (s, pan_right) = deserialise_keyboard_binding(s, "right")?;
    let (s, pan_left) = deserialise_keyboard_binding(s, "left")?;
    let (s, copy) = deserialise_shortcut_binding(s, "yank")?;
    let (s, paste) = deserialise_shortcut_binding(s, "paste")?;
    let (s, mirror) = deserialise_shortcut_binding(s, "mirror")?;
    let (s, delete) = deserialise_shortcut_binding(s, "delete")?;
    let (s, undo) = deserialise_shortcut_binding(s, "undo")?;
    let (s, redo) = deserialise_shortcut_binding(s, "redo")?;

    Ok((
        s,
        Keybinds {
            pan_up,
            pan_down,
            pan_right,
            pan_left,
            copy,
            paste,
            mirror,
            delete,
            undo,
            redo,
            ..Default::default()
        },
    ))
}

fn deserialise_keyboard_binding<'a>(s: &'a str, name: &'a str) -> IResult<&'a str, Option<Key>> {
    delimited(
        (tag(name), tag(" ")),
        alt((
            value(None, tag("is in the Gamma Void")),
            map(take_until("\n"), |s| Key::from_name(s)),
        )),
        newline,
    )
    .parse(s)
}

fn deserialise_shortcut_binding<'a>(
    s: &'a str,
    name: &'a str,
) -> IResult<&'a str, Option<KeyboardShortcut>> {
    let (s, binding_option) = delimited(
        (tag(name), tag(" ")),
        alt((
            value(None, tag("is in the Gamma Void")),
            map_parser(
                take_until("\n"),
                map(
                    (
                        complete(opt(value((), tag("Ctrl+")))),
                        complete(opt(value((), tag("Alt+")))),
                        complete(opt(value((), tag("Shift+")))),
                        complete(map(rest, |s| Key::from_name(s))),
                    ),
                    |x| Some(x),
                ),
            ),
        )),
        newline,
    )
    .parse(s)?;
    Ok((
        s,
        match binding_option {
            None => None,
            Some((ctrl, alt, shift, key)) => {
                if let Some(key) = key {
                    Some(KeyboardShortcut::new(
                        Modifiers {
                            alt: alt.is_some(),
                            ctrl: ctrl.is_some(),
                            shift: shift.is_some(),
                            mac_cmd: false,
                            command: ctrl.is_some(),
                        },
                        key,
                    ))
                } else {
                    None
                }
            }
        },
    ))
}
