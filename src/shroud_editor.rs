use std::f32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::block_container::BlockContainer;
use crate::file_import::WhichFileImport;
use crate::keybind_deserialiser::try_load_keybinds;
use crate::keybinds::Keybinds;
use crate::mirror_pairs::get_loaded_shapes_mirror_pairs;
use crate::reference_image::ReferenceImage;
use crate::shapes_import_text_default::SHAPES_IMPORT_TEXT_DEFAULT;
use crate::shroud_editor::parse_shapes_text::ShapesParseResult;
use crate::shroud_editor::parse_shroud_text::ShroudParseResult;
use crate::shroud_editor::render_shroud::RenderData;
use crate::shroud_editor::shroud_layer_reordering::ShroudLayerReorderingMessageData;
use crate::shroud_editor::tools::ToolSettings;
use crate::shroud_import_text_default::SHROUD_IMPORT_TEXT_DEFAULT;
use crate::shroud_interaction::ShroudInteraction;
use crate::shroud_layer_container::ShroudLayerContainer;
use egui::{Popup, Pos2};
use egui_file_dialog::FileDialog;
use luexks_reassembly::shapes::shapes::Shapes;
use parse_vanilla_shapes::get_vanilla_shapes;

const FILL_COLOR_GRADIENT_TIME: f32 = 4.0;
const ZOOM_MIN: f32 = 0.001;
const ZOOM_MAX: f32 = 10000.0;
const MIN_GRID_LINE_DIST: f32 = 8.0;

pub struct ShroudEditor {
    pub block_container: BlockContainer,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_interaction: ShroudInteraction,
    pub zoom: f32,
    pub(crate) grid_size: f32,
    grid_visible: bool,
    grid_snap_enabled: bool,
    angle_snap: f32,
    angle_snap_enabled: bool,
    pub pan: Pos2,
    // key_tracker: KeyTracker,
    loaded_shapes: Shapes,
    just_exported_to_clipboard_success_option: Option<bool>,
    pub fill_color_gradient: f32,
    fill_color_gradient_increasing: bool,
    fill_color_gradient_delta_enabled: bool,
    last_frame_time: f64,
    dt: f64,
    only_show_selected_shroud_layers: bool,
    pub(crate) world_mouse_pos: Pos2,
    shroud_clipboard: Vec<ShroudLayerContainer>,
    loaded_shapes_mirror_pairs: Vec<(usize, usize)>,
    shroud_import_text: String,
    just_imported_shroud_from_paste_box_message_option: Option<ShroudParseResult>,
    shape_search_buf: String,
    shape_search_show_vanilla: bool,
    shapes_import_text: String,
    just_imported_shapes_from_paste_box_message_option: Option<ShapesParseResult>,
    tool_settings: ToolSettings,
    shroud_layer_reordering_message_data_option: Option<ShroudLayerReorderingMessageData>,
    float_shroud_settings: bool,
    render_data_option: Arc<Mutex<Option<RenderData>>>,
    visual_panel_key_bindings_enabled: bool,
    pub keybinds: Keybinds,
    pub undo_history: Vec<Vec<ShroudLayerContainer>>,
    pub add_undo_history: bool,
    pub undo_history_index: usize,
    pub reference_image: ReferenceImage,
    pub file_dialog: FileDialog,
    pub which_file_import: WhichFileImport,
    pub show_icon_radius: bool,
    pub icon_radius_option: Option<f32>,
    pub selection_box_start_pos_option: Option<Pos2>,
    pub is_first_frame: bool,
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
            grid_size: 1.0,
            grid_visible: true,
            grid_snap_enabled: true,
            angle_snap: 5.0,
            angle_snap_enabled: true,
            pan: Pos2::new(0.0, 0.0),
            // key_tracker: KeyTracker::default(),
            loaded_shapes,
            just_exported_to_clipboard_success_option: None,
            fill_color_gradient: 0.0,
            fill_color_gradient_increasing: true,
            fill_color_gradient_delta_enabled: true,
            last_frame_time: 0.0,
            dt: 0.0,
            only_show_selected_shroud_layers: true,
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
            shroud_layer_reordering_message_data_option: None,
            float_shroud_settings: false,
            render_data_option: Arc::new(Mutex::new(None)),
            visual_panel_key_bindings_enabled: true,
            keybinds: match try_load_keybinds() {
                Ok(keybinds) => keybinds,
                Err(_) => Keybinds::default(),
            },
            undo_history: [Vec::new()].into(),
            add_undo_history: false,
            undo_history_index: 0,
            reference_image: ReferenceImage::default(),
            file_dialog: FileDialog::new(),
            which_file_import: WhichFileImport::Shroud,
            show_icon_radius: false,
            icon_radius_option: None,
            selection_box_start_pos_option: None,
            is_first_frame: true,
        }
    }
}

impl eframe::App for ShroudEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.first_frame_styling_logic(ctx);
        // let copy = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::C);
        // if ctx.input_mut(|i| i.consume_shortcut(&copy)) {
        //     print!("Debug");
        // }
        // if ctx.input(|i| i.key_down(egui::Key::T)) {
        //     print!("Debug2");
        // }
        // if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.command) {
        //     print!("Debug2");
        // }
        // if ctx.input(|i| i.key_down(egui::Key::C)) {
        //     print!("Command");
        // }
        // if ctx.input(|i| i.key_down(egui::Key::C)) {
        //     print!("C");
        // }
        self.update_dt(ctx);

        self.fill_color_gradient_delta();
        self.left_panel(ctx);

        if self.visual_panel_key_bindings_enabled {
            self.pan_controls(ctx);
            self.hotkey_shroud_layer_deletion(ctx);
            self.delete_shroud_layers(ctx);

            self.hotkey_copy(ctx);
            self.hotkey_paste(ctx);
            self.hotkey_mirroring(ctx);
            self.hotkey_undo_redo(ctx);
        }

        self.visual_panel_key_bindings_enabled = true;
        if Popup::is_any_open(ctx) {
            self.visual_panel_key_bindings_enabled = false;
        }

        self.visual_panel(ctx);

        self.add_undo_history_logic();

        self.file_import_logic(ctx);

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
mod render_polygon;
mod render_shroud;
mod selection_box;
mod shape_combo_box;
mod shroud_interaction_checks;
mod shroud_layer_gizmos;
mod shroud_layer_moving;
mod shroud_layer_reordering;
mod shroud_settings;
mod tools;
mod viewport_controls;
mod visual_panel;
mod x_y_z_drag_value_speed;
