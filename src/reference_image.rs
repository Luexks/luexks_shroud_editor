use std::{f32::INFINITY, path::PathBuf};

use arboard::Clipboard;
use egui::{
    ColorImage, Context, DragValue, Image, Pos2, Rect, TextureHandle, Ui,
    collapsing_header::CollapsingState, load::SizedTexture, vec2,
};
use image::{ImageError, ImageReader};

use crate::{
    file_import::WhichFileImport, invert_y::invert_y_of_pos2, shroud_editor::ShroudEditor,
};

pub struct ReferenceImage {
    image_option: Option<SizedTexture>,
    handle_option: Option<TextureHandle>,
    pos: Pos2,
    scale: f32,
    opacity: f32,
    pub image_layer: ImageLayer,
    pub enabled: bool,
}

#[derive(PartialEq)]
pub enum ImageLayer {
    ImageAbove,
    ImageBelow,
}

impl Default for ReferenceImage {
    fn default() -> Self {
        ReferenceImage {
            image_option: None,
            handle_option: None,
            pos: Pos2::ZERO,
            scale: 1.0,
            opacity: 1.0,
            image_layer: ImageLayer::ImageBelow,
            enabled: true,
        }
    }
}

impl ShroudEditor {
    pub fn reference_image_settings(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "image".into(), false)
            .show_header(ui, |ui| ui.label("Reference Image"))
            .body_unindented(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Enabled");
                    ui.checkbox(&mut self.reference_image.enabled, "");
                });
                if ui.button("Import from Clipboard").clicked() {
                    self.import_reference_image_from_clipboard(ctx);
                }
                if ui.button("Import from File").clicked() {
                    self.file_dialog_import_reference_image_from_file();
                }
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.reference_image.image_layer,
                        ImageLayer::ImageBelow,
                        "Image Below",
                    );
                    ui.selectable_value(
                        &mut self.reference_image.image_layer,
                        ImageLayer::ImageAbove,
                        "Image Above",
                    );
                });
                ui.horizontal(|ui| {
                    let xy_speed = self.get_xy_speed();
                    ui.label("X:");
                    ui.add(DragValue::new(&mut self.reference_image.pos.x).speed(xy_speed));
                    ui.label("Y:");
                    ui.add(DragValue::new(&mut self.reference_image.pos.y).speed(xy_speed));
                    ui.label("Scale:");
                    ui.add(
                        DragValue::new(&mut self.reference_image.scale)
                            .speed(0.01)
                            .range(0.0..=INFINITY),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Opacity:");
                    ui.add(
                        DragValue::new(&mut self.reference_image.opacity)
                            .speed(0.01)
                            .range(0.0..=1.0),
                    );
                });
            });
    }

    fn import_reference_image_from_clipboard(&mut self, ctx: &Context) {
        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(image) = clipboard.get_image() {
            let color_image =
                ColorImage::from_rgba_premultiplied([image.width, image.height], &image.bytes);
            self.load_image(ctx, color_image);
        }
    }

    fn load_image(&mut self, ctx: &Context, color_image: ColorImage) {
        let width = color_image.size[0] as f32;
        let height = color_image.size[1] as f32;
        let handle = ctx.load_texture("ref", color_image, Default::default());
        let sized_image = SizedTexture::new(handle.id(), vec2(width, height));
        self.reference_image.image_option = Some(sized_image);
        self.reference_image.handle_option = Some(handle);
    }

    fn file_dialog_import_reference_image_from_file(&mut self) {
        self.which_file_import = WhichFileImport::ReferenceImage;
        self.file_dialog.pick_file();
    }

    pub fn import_reference_image_from_file(&mut self, ctx: &Context, path: PathBuf) {
        if let Ok(color_image) = self.try_import_reference_image_from_path(path) {
            self.load_image(ctx, color_image);
        }
    }

    pub fn try_import_reference_image_from_path(
        &mut self,
        path: PathBuf,
    ) -> Result<ColorImage, ImageError> {
        let image = ImageReader::open(path)?.decode()?;
        let width = image.width() as _;
        let height = image.height() as _;
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(ColorImage::from_rgba_unmultiplied(
            [width, height],
            pixels.as_slice(),
        ))
    }

    pub fn render_reference_image(&self, ui: &mut Ui, rect: Rect) {
        if let Some(image) = &self.reference_image.image_option {
            let pos = invert_y_of_pos2(self.reference_image.pos);
            let scale = self.reference_image.scale;
            let top_left = self.world_pos_to_screen_pos(pos - image.size * 0.5 * scale, rect);
            let bottom_right = self.world_pos_to_screen_pos(pos + image.size * 0.5 * scale, rect);
            let image_rect = Rect::from_min_max(top_left, bottom_right);
            ui.set_opacity(self.reference_image.opacity);
            Image::new(*image).paint_at(ui, image_rect);
            ui.set_opacity(1.0);
        }
    }
}
