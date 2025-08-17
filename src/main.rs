// https://den5-tech.github.io/resource/RSE.html

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
mod shroud_editor;
use shroud_editor::*;
mod background_grid;
mod fonts;
mod gui;
mod key_tracker;
mod pos_in_polygon;
mod restructure_vertices;
mod shroud_layer_container;
mod shroud_layer_rendering;
mod styles;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Luexks Shroud Editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Ok(Box::<ShroudEditor>::default())
            Ok(Box::new(ShroudEditor::new(cc)))
        }),
    )
}
