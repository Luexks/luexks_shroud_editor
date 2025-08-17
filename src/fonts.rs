use std::collections::BTreeMap;

use egui::{FontId, TextStyle};

// Demonstrates how to replace all fonts.
pub fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Dekar2".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "./fonts/Dekar2.ttf"
        ))),
    );
    fonts.font_data.insert(
        "DroidSans".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "./fonts/DroidSans.ttf"
        ))),
    );
    fonts.font_data.insert(
        "Cousine-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "./fonts/Cousine-Regular.ttf"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    // fonts
    //     .families
    //     // .entry(egui::FontFamily::Proportional)
    //     .entry(egui::FontFamily::Name("Dekar2".into()))
    //     .or_default()
    //     // .insert(0, "Dekar2".to_owned());
    //     .push("Dekar2".to_owned());

    // Put my font as last fallback for monospace:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     // .entry(egui::FontFamily::Name("DroidSans".into()))
    //     .or_default()
    //     // .push("DroidSans".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Name("Dekar2".into()))
        .or_default()
        .push("Dekar2".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        // .entry(egui::FontFamily::Name("Cousine-Regular".into()))
        .or_default()
        // .push("Cousine-Regular".to_owned());
        .insert(0, "Cousine-Regular".to_owned());
    fonts
        .families
        // .entry(egui::FontFamily::Proportional)
        .entry(egui::FontFamily::Name("Dekar2".into()))
        .or_default()
        // .insert(0, "Dekar2".to_owned());
        .push("Dekar2".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);

    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (TextStyle::Heading, FontId::new(24.0, egui::FontFamily::Name("Dekar2".into()))),
    //     (TextStyle::Body, FontId::new(24.0, egui::FontFamily::Name("DroidSans".into()))),
    //     (TextStyle::Button, FontId::new(24.0, egui::FontFamily::Name("DroidSans".into()))),
    //     (TextStyle::Monospace, FontId::new(24.0, egui::FontFamily::Name("Cousine-Regular".into()))),
    //     (TextStyle::Small, FontId::new(24.0, egui::FontFamily::Name("DroidSans".into()))),
    // ].into();

    // ctx.set_style(style);
    use egui::FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (
            TextStyle::Heading,
            FontId::new(24.0, egui::FontFamily::Name("Dekar2".into())),
        ),
        // (TextStyle::Heading, FontId::new(25.0, Proportional)),
        // (heading2(), FontId::new(22.0, Proportional)),
        // (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

// #[inline]
// fn dekar() -> TextStyle {
//     TextStyle::Name("Dekar2".into())
// }
// #[inline]
// fn droid_sans() -> TextStyle {
//     TextStyle::Name("DroidSans".into())
// }
// #[inline]
// fn cousine_regular() -> TextStyle {
//     TextStyle::Name("Cousine-Regular".into())
// }
