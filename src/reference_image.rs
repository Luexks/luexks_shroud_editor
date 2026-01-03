use std::f32::INFINITY;

use arboard::{Clipboard, ImageData};
use egui::{
    Area, ColorImage, Context, DragValue, Frame, Id, Image, Pos2, Rect, TextureHandle, Ui,
    collapsing_header::CollapsingState, load::SizedTexture, pos2, vec2,
};

use crate::{invert_y::invert_y_of_pos2, shroud_editor::ShroudEditor};

pub struct ReferenceImage {
    // image_option: Option<ImageData<'static>>,
    image_option: Option<SizedTexture>,
    handle_option: Option<TextureHandle>,
    // retained_image_option: Option<Retained>
    pos: Pos2,
    scale: f32,
    opacity: f32,
    pub image_layer: ImageLayer,
    // opacity: f32,
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
        }
    }
}

impl ShroudEditor {
    pub fn reference_image_settings(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingState::load_with_default_open(ctx, "image".into(), true)
            .show_header(ui, |ui| ui.label("Reference Image"))
            .body_unindented(|ui| {
                if ui.button("Import from Clipboard").clicked() {
                    let mut clipboard = Clipboard::new().unwrap();
                    if let Ok(image) = clipboard.get_image() {
                        let color_image = ColorImage::from_rgba_premultiplied(
                            [image.width, image.height],
                            &image.bytes,
                        );
                        let width = color_image.size[0] as f32;
                        let height = color_image.size[1] as f32;
                        let handle = ctx.load_texture("ref", color_image, Default::default());
                        let sized_image = SizedTexture::new(handle.id(), vec2(width, height));
                        // let image = Image::from_texture(sized_image);

                        // self.reference_image.image_option = Some(image);
                        // dbg!(&sized_image);
                        self.reference_image.image_option = Some(sized_image);
                        self.reference_image.handle_option = Some(handle);
                    }
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
                let xy_speed = self.get_xy_speed();
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(DragValue::new(&mut self.reference_image.pos.x).speed(xy_speed));
                    ui.label("Y:");
                    ui.add(DragValue::new(&mut self.reference_image.pos.y).speed(xy_speed));
                    ui.label("Scale:");
                    ui.add(
                        DragValue::new(&mut self.reference_image.scale)
                            .speed(xy_speed)
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

    pub fn render_reference_image(&self, ui: &mut Ui, rect: Rect) {
        if let Some(image) = &self.reference_image.image_option {
            // dbg!(&image);
            // ui.image(*image);

            // let center = rect.center();
            // let top_left = center - image.size / 2.0;

            let pos = invert_y_of_pos2(self.reference_image.pos);
            let scale = self.reference_image.scale;
            let top_left = self.world_pos_to_screen_pos(pos - image.size * 0.5 * scale, rect);
            let bottom_right = self.world_pos_to_screen_pos(pos + image.size * 0.5 * scale, rect);
            let image_rect = Rect::from_min_max(top_left, bottom_right);

            // let center = self.world_pos_to_screen_pos(pos2(0.0, 0.0), rect);
            // let image_rect = Rect::from_min_max(center - image.size / 2.0, center + image.size / 2.0);

            // Area::new(Id::new("ref"))
            //     .fixed_pos(top_left)
            //     .fade_in(false)
            //     .show(ctx, |ui| {
            //         Frame::new().show(ui, |ui| {
            //             ui.image(*image);
            //         });
            //     });
            ui.set_opacity(self.reference_image.opacity);
            Image::new(*image).paint_at(ui, image_rect);
            ui.set_opacity(1.0);
        }
    }
}
