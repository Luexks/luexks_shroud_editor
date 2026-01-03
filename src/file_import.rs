use egui::Context;

use crate::shroud_editor::ShroudEditor;

pub enum WhichFileImport {
    ReferenceImage,
    Shroud,
    Shape,
}

impl ShroudEditor {
    pub fn file_import_logic(&mut self, ctx: &Context) {
        self.file_dialog.update(ctx);
        if let Some(path) = self.file_dialog.take_picked() {
            match self.which_file_import {
                WhichFileImport::ReferenceImage => {
                    self.import_reference_image_from_file(ctx, path);
                }
                _ => {}
            }
        }
    }
}
