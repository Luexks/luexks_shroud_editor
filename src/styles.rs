use egui;
use egui::{Color32, Context, Stroke, Theme, Visuals};

// use crate::fonts::replace_fonts;
use crate::shroud_editor::ShroudEditor;

pub const BACKGROUND_COLOUR: Color32 = Color32::from_gray(240);

impl ShroudEditor {
    pub fn first_frame_styling_logic(&mut self, ctx: &Context) {
        if self.is_first_frame {
            self.is_first_frame = false;
            apply_styles(ctx);
            // replace_fonts(ctx);
        }
    }
}

pub fn apply_styles(ctx: &egui::Context) {
    ctx.set_theme(Theme::Light);
    let mut style = (*ctx.style()).clone();

    // Global visuals (colors, corners)
    style.visuals = egui::Visuals {
        // dark_mode: true,
        extreme_bg_color: BACKGROUND_COLOUR,
        window_fill: BACKGROUND_COLOUR,
        panel_fill: BACKGROUND_COLOUR,
        // window_stroke: egui::Stroke::new(1.0, BACKGROUND_COLOUR), // white border on windows
        window_stroke: Stroke::NONE,
        // window_rounding: egui::Rounding::none(),      // no rounding
        window_corner_radius: egui::CornerRadius::ZERO,
        override_text_color: Some(Color32::BLACK),
        ..Visuals::light()
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
