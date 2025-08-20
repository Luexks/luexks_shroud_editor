use std::f32;
use std::time::Duration;

use crate::block_container::BlockContainer;
use crate::key_tracker::KeyTracker;
use crate::shroud_layer_container::ShroudLayerContainer;
use crate::shroud_layer_interaction::ShroudLayerInteraction;
use egui::Pos2;
use luexks_reassembly::shapes::shapes::Shapes;
use parse_vanilla_shapes::get_vanilla_shapes;

const FILL_COLOR_GRADIENT_TIME: f32 = 4.0;

pub struct ShroudEditor {
    pub block_container: BlockContainer,
    pub shroud: Vec<ShroudLayerContainer>,
    pub shroud_layer_interaction: ShroudLayerInteraction,
    pub zoom: f32,
    pub grid_size: f32,
    pub grid_enabled: bool,
    pub snap_to_grid: bool,
    pub angle_snap: f32,
    pub angle_snap_enabled: bool,
    pub pan: Pos2,
    pub key_tracker: KeyTracker,
    pub loaded_shapes: Shapes,
    pub just_exported_to_clipboard_success_option: Option<bool>,
    pub fill_color_gradient: f32,
    fill_color_gradient_increasing: bool,
    fill_color_gradient_delta_enabled: bool,
    last_frame_time: f64,
    dt: f64,
    only_show_selected_shroud_layers: bool,
    world_mouse_pos: Pos2,
    shroud_clipboard: Vec<ShroudLayerContainer>,
}

impl Default for ShroudEditor {
    fn default() -> Self {
        Self {
            block_container: Default::default(),
            shroud: Vec::default(),
            shroud_layer_interaction: ShroudLayerInteraction::Inaction {
                selection: Vec::new(),
            },
            // zoom: 1,
            zoom: 1.0,
            grid_size: 2.5,
            grid_enabled: true,
            snap_to_grid: true,
            angle_snap: 10.0,
            angle_snap_enabled: true,
            pan: Pos2::new(0.0, 0.0),
            key_tracker: KeyTracker::default(),
            loaded_shapes: get_vanilla_shapes(),
            just_exported_to_clipboard_success_option: None,
            fill_color_gradient: 0.0,
            fill_color_gradient_increasing: true,
            fill_color_gradient_delta_enabled: true,
            last_frame_time: 0.0,
            dt: 0.0,
            only_show_selected_shroud_layers: false,
            world_mouse_pos: Pos2::default(),
            shroud_clipboard: Vec::new(),
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

        self.delete_shroud_layers();
        self.hotkey_shroud_layer_deletion(ctx);

        self.hotkey_copy(ctx);
        self.hotkey_paste(ctx);

        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / 60.0));
    }
}

mod delete_shroud_layers;
mod delta_time;
mod draw_grid;
mod hotkey_copy_and_paste;
mod hotkey_shroud_layer_deletion;
mod left_panel;
mod position_conversion;
mod render_shroud;
mod shape_combo_box;
mod shroud_interaction_checks;
mod shroud_layer_dragging;
mod shroud_layer_gizmos;
mod shroud_settings;
mod viewport_controls;
mod visual_panel;
