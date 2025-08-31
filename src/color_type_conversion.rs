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

pub fn str_to_rgba_option(s: &str) -> Option<Rgba> {
    let s = s.strip_prefix("0x").unwrap_or(s).to_uppercase();
    match s.len() {
        6 => Some({
            let r = u8::from_str_radix(&s[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&s[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&s[4..6], 16).ok()? as f32 / 255.0;
            Rgba::from_rgba_premultiplied(r, g, b, 1.0)
        }),
        8 => Some({
            let r = u8::from_str_radix(&s[2..4], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&s[4..6], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&s[6..8], 16).ok()? as f32 / 255.0;
            let a = u8::from_str_radix(&s[0..2], 16).ok()? as f32 / 255.0;
            Rgba::from_rgba_premultiplied(r, g, b, a)
        }),
        _ => None,
    }
}

pub fn rgba_to_color_string(rgba: Rgba) -> String {
    let [r, g, b, a] = rgba.to_rgba_unmultiplied();
    format!(
        "0x{:02X}{:02X}{:02X}{:02X}",
        (a * 255.0) as u8,
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}
