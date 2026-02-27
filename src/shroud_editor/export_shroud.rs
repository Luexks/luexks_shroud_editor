use std::{fs::File, io::Write, path::PathBuf};

use arboard::Clipboard;
use egui::Ui;
use luexks_reassembly::{
    blocks::{shroud::Shroud, shroud_layer::ShroudLayer},
    utility::{component_formatting::format_component, display_oriented_math::do3d_float_from},
};

use crate::{file_import_export::WhichFileDialog, shroud_editor::ShroudEditor};

impl ShroudEditor {
    fn export_shroud(&self) -> String {
        let shroud = format_component(
            Shroud(
                self.shroud
                    .iter()
                    .map(|shroud_layer_container| {
                        let shroud_layer = shroud_layer_container.shroud_layer.clone();
                        let pre_block_offset_offset = shroud_layer.offset.as_ref().unwrap();
                        let post_block_offset_offset = do3d_float_from(
                            pre_block_offset_offset.x.to_f32() - self.block_container.offset.x,
                            pre_block_offset_offset.y.to_f32() - self.block_container.offset.y,
                            pre_block_offset_offset.z.to_f32(),
                        );
                        ShroudLayer {
                            angle: if shroud_layer
                                .angle
                                .clone()
                                .unwrap()
                                .as_radians()
                                .get_value()
                                .abs()
                                < f32::EPSILON
                            {
                                None
                            } else {
                                shroud_layer.angle.clone()
                            },
                            taper: if shroud_layer_container.shape_id != "SQUARE"
                                || shroud_layer.taper.unwrap() == 1.0
                            {
                                None
                            } else {
                                shroud_layer.taper
                            },
                            offset: Some(post_block_offset_offset),
                            ..shroud_layer
                        }
                    })
                    .collect(),
            ),
            "shroud",
        );
        shroud.to_string()
    }

    pub fn export_shroud_to_clipboard_button(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let export_to_clipboard_button = ui.button("Export Shroud to Clipboard");
            if export_to_clipboard_button.clicked() {
                let mut clipboard = Clipboard::new().unwrap();
                let shroud_export = self.export_shroud();
                let just_exported_to_clipboard_status = clipboard.set_text(shroud_export).is_ok();
                self.just_exported_to_clipboard_success_option =
                    Some(just_exported_to_clipboard_status)
            }
            if let Some(just_exported_to_clipboard_success) =
                self.just_exported_to_clipboard_success_option
            {
                if export_to_clipboard_button.contains_pointer() {
                    if just_exported_to_clipboard_success {
                        ui.label("Copied to clipboard.");
                    } else {
                        ui.label("Failed :(");
                    }
                } else {
                    self.just_exported_to_clipboard_success_option = None
                }
            }
        });
    }

    pub fn export_shroud_as_file_next_to_exe_button(&mut self, ui: &mut Ui) {
        let response = ui.button("Export Shroud as File Next to .exe");
        if response.clicked() {
            let shroud_export = self.export_shroud();
            let just_exported_to_file_next_to_exe_status = File::create("shroud.lua")
                .and_then(|mut file| file.write_all(shroud_export.as_bytes()))
                .is_ok();
            self.just_exported_to_file_next_to_exe_status =
                Some(just_exported_to_file_next_to_exe_status)
        }
        if let Some(just_exported_to_file_next_to_exe_status) =
            self.just_exported_to_file_next_to_exe_status
        {
            if response.contains_pointer() {
                if just_exported_to_file_next_to_exe_status {
                    ui.label("Exported next to wherever this editor is stored.");
                } else {
                    ui.label("Failed :(");
                }
            } else {
                self.just_exported_to_file_next_to_exe_status = None
            }
        }
    }

    pub fn export_shroud_to_file_button(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let response = ui.button("Export Shroud as File");
            if response.clicked() {
                self.which_file_import = WhichFileDialog::ShroudExport;
                self.file_dialog.save_file();
            }
            if let Some(just_exported_to_file_status) = self.just_exported_to_file_status {
                if response.contains_pointer() {
                    if just_exported_to_file_status {
                        ui.label("Exported to file.");
                    } else {
                        ui.label("Failed :(");
                    }
                } else {
                    self.just_exported_to_file_status = None
                }
            }
        });
    }

    pub fn export_shroud_to_file(&mut self, path: PathBuf) {
        let shroud_export = self.export_shroud();
        let just_exported_to_file_status = File::create(path)
            .and_then(|mut file| file.write_all(shroud_export.as_bytes()))
            .is_ok();
        self.just_exported_to_file_status = Some(just_exported_to_file_status)
    }
}
