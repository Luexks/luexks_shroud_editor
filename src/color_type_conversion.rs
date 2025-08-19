use egui::Rgba;
use luexks_reassembly::utility::color::Color;

pub fn rgba_to_color(rgba: Rgba) -> Color {
    Color::new_aarrggb_u8(
        (rgba.a() * 255.0) as u8,
        (rgba.r() * 255.0) as u8,
        (rgba.g() * 255.0) as u8,
        (rgba.b() * 255.0) as u8,
    )
}

pub fn color_to_rgba(color: Color) -> Rgba {
    Rgba::from_rgba_unmultiplied(
        color.rr() as f32 / 255.0,
        color.gg() as f32 / 255.0,
        color.bb() as f32 / 255.0,
        color.aa().unwrap_or(255) as f32 / 255.0,
    )
}
