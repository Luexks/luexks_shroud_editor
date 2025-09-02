// https://den5-tech.github.io/resource/RSE.html
#![feature(int_from_ascii)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
mod shroud_editor;
use egui::Pos2;
use shroud_editor::*;
mod block_container;
mod color_type_conversion;
mod fonts;
mod gui;
mod key_tracker;
mod pos_in_polygon;
mod render_polygon;
mod restructure_vertices;
mod selection_type;
mod shroud_import_text_default;
mod shroud_layer_container;
mod shroud_layer_interaction;
mod size_from_verts;
mod styles;

pub const DEFAULT_SQUARE: [Pos2; 4] = [
    Pos2::new(5.0, -5.0),
    Pos2::new(-5.0, -5.0),
    Pos2::new(-5.0, 5.0),
    Pos2::new(5.0, 5.0),
];

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0 / 2.0, 1080.0 / 2.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Luexks Shroud Editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Ok(Box::<ShroudEditor>::default())
            Ok(Box::new(ShroudEditor::default()))
        }),
    )
}
