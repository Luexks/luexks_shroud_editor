use egui;

pub fn apply_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    let grey = egui::Color32::from_rgb(40, 40, 40); // dark grey background
    let white = egui::Color32::from_rgb(255, 255, 255); // white outlines

    // Global visuals (colors, corners)
    style.visuals = egui::Visuals {
        dark_mode: true,
        window_fill: grey,
        window_stroke: egui::Stroke::new(1.0, white), // white border on windows
        // window_rounding: egui::Rounding::none(),      // no rounding
        window_corner_radius: egui::CornerRadius::ZERO,
        ..egui::Visuals::dark()
    };

    // // Widget visuals
    // style.visuals.widgets = egui::style::Widgets {
    //     noninteractive: egui::style::WidgetVisuals {
    //         bg_fill: grey,
    //         bg_stroke: egui::Stroke::new(1.0, white),
    //         // rounding: egui::Rounding::none(),
    //         fg_stroke: egui::Stroke::new(1.0, white),
    //         expansion: 0.0,
    //         weak_bg_fill: grey,
    //         corner_radius: egui::CornerRadius::ZERO,
    //     },
    //     inactive: egui::style::WidgetVisuals {
    //         bg_fill: grey,
    //         bg_stroke: egui::Stroke::new(1.0, white),
    //         // rounding: egui::Rounding::none(),
    //         fg_stroke: egui::Stroke::new(1.0, white),
    //         expansion: 0.0,
    //         weak_bg_fill: grey,
    //         corner_radius: egui::CornerRadius::ZERO,
    //     },
    //     hovered: egui::style::WidgetVisuals {
    //         bg_fill: grey,
    //         bg_stroke: egui::Stroke::new(1.5, white),
    //         // rounding: egui::Rounding::none(),
    //         fg_stroke: egui::Stroke::new(1.5, white),
    //         expansion: 1.0,
    //         weak_bg_fill: grey,
    //         corner_radius: egui::CornerRadius::ZERO,
    //     },
    //     active: egui::style::WidgetVisuals {
    //         bg_fill: grey,
    //         bg_stroke: egui::Stroke::new(2.0, white),
    //         // rounding: egui::Rounding::none(),
    //         fg_stroke: egui::Stroke::new(2.0, white),
    //         expansion: 1.0,
    //         weak_bg_fill: grey,
    //         corner_radius: egui::CornerRadius::ZERO,
    //     },
    //     open: egui::style::WidgetVisuals {
    //         bg_fill: grey,
    //         bg_stroke: egui::Stroke::new(1.0, white),
    //         // rounding: egui::Rounding::none(),
    //         fg_stroke: egui::Stroke::new(1.0, white),
    //         expansion: 0.0,
    //         weak_bg_fill: grey,
    //         corner_radius: egui::CornerRadius::ZERO,
    //     },
    // };

    ctx.set_style(style);
}
