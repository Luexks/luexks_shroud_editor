use egui::Context;

use crate::shroud_editor::ShroudEditor;

pub enum WhichFileDialog {
    ReferenceImage,
    ShroudImport,
    ShapeImport,
    ShroudExport,
    ShapeExport,
}

impl ShroudEditor {
    pub fn file_import_logic(&mut self, ctx: &Context) {
        self.file_dialog.update(ctx);
        if let Some(path) = self.file_dialog.take_picked() {
            match self.which_file_import {
                WhichFileDialog::ReferenceImage => {
                    self.import_reference_image_from_file(ctx, path);
                }
                WhichFileDialog::ShroudExport => {
                    self.export_shroud_to_file(path);
                }
                _ => {}
            }
        }
    }
}
