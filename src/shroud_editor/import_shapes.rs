use std::{fs::read_to_string, path::PathBuf};

use egui::{ScrollArea, TextBuffer, TextEdit, Ui, collapsing_header::CollapsingState};
use egui_extras::syntax_highlighting::{CodeTheme, highlight};
use parse_vanilla_shapes::VANILLA_SHAPE_COUNT;

use crate::{
    file_import_export::WhichFileDialog,
    mirror_pairs::get_loaded_shapes_mirror_pairs,
    restructure_vertices::restructure_vertices,
    shape_container::ShapeContainer,
    shroud_editor::{
        ShroudEditor,
        parse_shapes_text::{ShapesMessage, parse_shapes_text},
    },
};

impl ShroudEditor {
    fn load_shapes(
        &mut self,
        imported_shapes: Vec<ShapeContainer>,
        mirror_pairs: Vec<(usize, usize)>,
        non_mirrors: Vec<usize>,
    ) {
        self.loaded_shapes = self.loaded_shapes[0..VANILLA_SHAPE_COUNT]
            .iter()
            .cloned()
            .chain(imported_shapes)
            .collect();
        non_mirrors.into_iter().for_each(|non_mirror| {
            self.loaded_shapes[non_mirror].set_invert_height_of_mirror();
        });
        self.loaded_shapes_mirror_pairs = get_loaded_shapes_mirror_pairs(&self.loaded_shapes);
        self.loaded_shapes_mirror_pairs.extend(mirror_pairs);
        (0..self.shroud.len()).for_each(|shroud_layer_index| {
            let shroud_layer = &mut self.shroud[shroud_layer_index];
            if let Some(shape_idx) = self.loaded_shapes.iter().position(|shape| {
                shape.s.get_id().unwrap() == *shroud_layer.shroud_layer.shape.as_ref().unwrap()
            }) {
                shroud_layer.vertices = restructure_vertices(
                    self.loaded_shapes[shape_idx].s.get_first_scale_vertices(),
                );
                shroud_layer.invert_height_of_mirror =
                    self.loaded_shapes[shape_idx].invert_height_of_mirror
            }
        });
        if let Some(shape_idx) = self.loaded_shapes.iter().position(|shape| {
            shape.s.get_id().unwrap() == *self.block_container.block.shape.as_ref().unwrap()
        }) {
            self.block_container.vertices =
                restructure_vertices(self.loaded_shapes[shape_idx].s.get_first_scale_vertices());
        }
    }

    pub fn import_shapes_from_file_button(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let response = ui.button("Import Shapes from File");
            if response.clicked() {
                self.which_file_import = WhichFileDialog::ShapeImport;
                self.file_dialog.pick_file();
            }
            if let Some(message) = &self.just_imported_shapes_from_file_message_option {
                ui.label(message.to_string());
            }
            if !response.contains_pointer() {
                self.just_imported_shapes_from_file_message_option = None;
            }
        });
    }

    pub fn import_shapes_from_file(&mut self, path: PathBuf) {
        if let Ok(s) = read_to_string(path) {
            match parse_shapes_text(&s) {
                Ok((imported_shapes, mirror_pairs, non_mirrors)) => {
                    self.load_shapes(imported_shapes, mirror_pairs, non_mirrors);
                    self.just_imported_shapes_from_file_message_option =
                        Some(ShapesMessage::Success);
                }
                Err(err) => {
                    self.just_imported_shapes_from_file_message_option = Some(err);
                }
            }
        } else {
            self.just_imported_shapes_from_file_message_option =
                Some(ShapesMessage::CouldNotOpenFile);
        }
    }

    pub fn import_shapes_from_paste_box(&mut self, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ui.ctx(), "import_shape".into(), false)
            .show_header(ui, |ui| {
                ui.strong("Import Shapes from Paste Box");
            })
            .body(|ui| {
                ui.label("READ: Keep all custom shapes in paste box.");
                ui.label("READ: If a custom shape used by a shroud layer is not reimported, nothing significant will happen, but it will just be weird.");
                ui.horizontal(|ui| {
                    let response = ui.button("Import");
                    if response.clicked() {
                        match parse_shapes_text(&self.shapes_import_text) {
                            Ok((imported_shapes, mirror_pairs, non_mirrors)) => {
                                self.load_shapes(imported_shapes, mirror_pairs, non_mirrors);
                                self.just_imported_shapes_from_paste_box_message_option =
                                    Some(ShapesMessage::Success);
                            }
                            Err(err) => {
                                self.just_imported_shapes_from_paste_box_message_option = Some(err);
                            }
                        }
                    }
                    if let Some(message) = &self.just_imported_shapes_from_paste_box_message_option
                    {
                        ui.label(message.to_string());
                    }
                    if !response.contains_pointer() {
                        self.just_imported_shapes_from_paste_box_message_option = None;
                    }
                });
                ScrollArea::horizontal().show(ui, |ui| {
                    let theme = CodeTheme::light(12.0);
                    let mut layouter = |ui: &Ui, buf: &dyn TextBuffer, wrap_width: f32| {
                        let mut layout_job =
                            highlight(ui.ctx(), ui.style(), &theme, buf.as_str(), "toml");
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts_mut(|f| f.layout_job(layout_job))
                    };
                    let text_edit = ui.add(
                        TextEdit::multiline(&mut self.shapes_import_text)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                    if text_edit.has_focus() {
                        self.visual_panel_key_bindings_enabled = false;
                    }
                });
            });
    }
}
