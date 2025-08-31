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

// pub fn rgba_to_color_string(rgba: Rgba) -> String {
//     // vec![
//     //     format!("{:X?}", rgba.a() * 255.0 as u8)
//     // ]
//     format!("0x{}{}{}{}", rgba.a(), rgba.r(), rgba.g(), rgba.b())
// }

// pub fn rgba_to_input_color(rgba: Rgba) -> Vec<u8> {
//     // vec![
//     //     format!("{:X?}", rgba.a() * 255.0 as u8)
//     // ]
//     vec![
//         (rgba.a() * 255.0) as u8,
//         (rgba.r() * 255.0) as u8,
//         (rgba.g() * 255.0) as u8,
//         (rgba.b() * 255.0) as u8,
//     ]
// }

pub fn str_to_rgba_option(str: &str) -> Option<Rgba> {
    let str = str.to_uppercase();
    let bytes = str.bytes().collect::<Vec<u8>>();
    // println!("{:?}", bytes);
    if bytes.len() != 8 && bytes.len() != 10 {
        return None;
    }
    // println!("Checkpoint Alpha");
    let bytes = &bytes[2..];
    if bytes.iter().any(|byte| {
        ![
            '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8,
            '8' as u8, '9' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8,
        ]
        .contains(byte)
    }) {
        return None;
    }
    if bytes.len() == 8 {
        Some(Rgba::from_rgba_premultiplied(
            u8::from_ascii_radix(&bytes[2..=3], 16).unwrap() as f32 / 255.0,
            u8::from_ascii_radix(&bytes[4..=5], 16).unwrap() as f32 / 255.0,
            u8::from_ascii_radix(&bytes[6..=7], 16).unwrap() as f32 / 255.0,
            u8::from_ascii_radix(&bytes[0..=1], 16).unwrap() as f32 / 255.0,
        ))
    } else {
        Some(Rgba::from_rgba_premultiplied(
            u8::from_ascii_radix(&bytes[0..=1], 16).unwrap() as f32 / 255.0,
            u8::from_ascii_radix(&bytes[2..=3], 16).unwrap() as f32 / 255.0,
            u8::from_ascii_radix(&bytes[4..=5], 16).unwrap() as f32 / 255.0,
            1.0,
        ))
    }
    // todo!()
}

pub fn rgba_to_color_string(rgba: Rgba) -> String {
    format!("0x{:02X?}{:02X?}{:02X?}{:02X?}",
        (rgba.a() * 255.0) as u8,
        (rgba.r() * 255.0) as u8,
        (rgba.g() * 255.0) as u8,
        (rgba.b() * 255.0) as u8,
)
}
