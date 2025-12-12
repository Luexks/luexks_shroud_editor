use std::f32;
use std::time::Duration;

use crate::block_container::BlockContainer;
use crate::key_tracker::KeyTracker;
use crate::mirror_pairs::get_loaded_shapes_mirror_pairs;
use crate::shapes_import_text_default::SHAPES_IMPORT_TEXT_DEFAULT;
use crate::shroud_editor::parse_shapes_text::ShapesParseResult;
use crate::shroud_editor::parse_shroud_text::ShroudParseResult;
use crate::shroud_editor::tools::ToolSettings;
use crate::shroud_import_text_default::SHROUD_IMPORT_TEXT_DEFAULT;
use crate::shroud_interaction::ShroudInteraction;
use crate::shroud_layer_container::ShroudLayerContainer;
use egui::Pos2;
use luexks_reassembly::shapes::shapes::Shapes;
use parse_vanilla_shapes::get_vanilla_shapes;

const FILL_COLOR_GRADIENT_TIME: f32 = 4.0;

pub struct ShroudEditor {
    pub block_container: BlockContainer,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_interaction: ShroudInteraction,
    zoom: f32,
    pub(crate) grid_size: f32,
    grid_visible: bool,
    grid_snap_enabled: bool,
    angle_snap: f32,
    angle_snap_enabled: bool,
    pan: Pos2,
    key_tracker: KeyTracker,
    loaded_shapes: Shapes,
    just_exported_to_clipboard_success_option: Option<bool>,
    pub fill_color_gradient: f32,
    fill_color_gradient_increasing: bool,
    fill_color_gradient_delta_enabled: bool,
    last_frame_time: f64,
    dt: f64,
    only_show_selected_shroud_layers: bool,
    world_mouse_pos: Pos2,
    shroud_clipboard: Vec<ShroudLayerContainer>,
    loaded_shapes_mirror_pairs: Vec<(usize, usize)>,
    shroud_import_text: String,
    just_imported_shroud_from_paste_box_message_option: Option<ShroudParseResult>,
    shape_search_buf: String,
    shape_search_show_vanilla: bool,
    shapes_import_text: String,
    just_imported_shapes_from_paste_box_message_option: Option<ShapesParseResult>,
    tool_settings: ToolSettings,
}

impl Default for ShroudEditor {
    fn default() -> Self {
        let loaded_shapes = get_vanilla_shapes();
        let loaded_shapes_mirror_pairs = get_loaded_shapes_mirror_pairs(&loaded_shapes);
        Self {
            block_container: Default::default(),
            shroud: Vec::default(),
            shroud_interaction: ShroudInteraction::Inaction {
                selection: Vec::new(),
            },
            zoom: 1.0,
            grid_size: 2.5,
            grid_visible: true,
            grid_snap_enabled: true,
            angle_snap: 10.0,
            angle_snap_enabled: true,
            pan: Pos2::new(0.0, 0.0),
            key_tracker: KeyTracker::default(),
            loaded_shapes,
            just_exported_to_clipboard_success_option: None,
            fill_color_gradient: 0.0,
            fill_color_gradient_increasing: true,
            fill_color_gradient_delta_enabled: true,
            last_frame_time: 0.0,
            dt: 0.0,
            only_show_selected_shroud_layers: false,
            world_mouse_pos: Pos2::default(),
            shroud_clipboard: Vec::new(),
            loaded_shapes_mirror_pairs,
            shroud_import_text: SHROUD_IMPORT_TEXT_DEFAULT.to_string(),
            just_imported_shroud_from_paste_box_message_option: None,
            shape_search_buf: String::new(),
            shape_search_show_vanilla: true,
            shapes_import_text: SHAPES_IMPORT_TEXT_DEFAULT.to_string(),
            just_imported_shapes_from_paste_box_message_option: None,
            tool_settings: ToolSettings::default(),
        }
    }
}

impl eframe::App for ShroudEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_dt(ctx);
        self.key_tracker.update(ctx);
        self.pan_controls();

        self.left_panel(ctx);
        self.visual_panel(ctx);

        self.hotkey_shroud_layer_deletion(ctx);
        self.delete_shroud_layers(ctx);

        self.hotkey_copy(ctx);
        self.hotkey_paste(ctx);
        self.hotkey_mirroring(ctx);

        // println!("{:.1}", 1.0 / self.dt);

        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / 60.0));
    }
}

mod add_mirror;
mod delete_shroud_layers;
mod delta_time;
mod draw_grid;
mod hotkey_copy_and_paste;
mod hotkey_mirroring;
mod hotkey_shroud_layer_deletion;
mod left_panel;
mod parse_shapes_text;
mod parse_shroud_text;
mod parsing;
mod position_conversion;
mod render_shroud;
mod shape_combo_box;
mod shroud_interaction_checks;
mod shroud_layer_gizmos;
mod shroud_layer_moving;
mod shroud_settings;
mod tools;
mod viewport_controls;
mod visual_panel;
