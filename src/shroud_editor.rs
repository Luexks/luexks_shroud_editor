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
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
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
    loaded_shapes_mirror_pairs: Vec<(usize, usize)>,
}

impl Default for ShroudEditor {
    fn default() -> Self {
        let loaded_shapes = get_vanilla_shapes();
        let loaded_shapes_mirror_pairs = get_loaded_shapes_mirror_pairs(&loaded_shapes);
        Self {
            block_container: Default::default(),
            shroud: Vec::default(),
            shroud_layer_interaction: ShroudLayerInteraction::Inaction {
                selection: Vec::new(),
            },
            // zoom: 1,
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

        self.delete_shroud_layers(ctx);
        self.hotkey_shroud_layer_deletion(ctx);

        self.hotkey_copy(ctx);
        self.hotkey_paste(ctx);

        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / 60.0));
    }
}

mod add_mirror;
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
mod snap_to_grid;
mod viewport_controls;
mod visual_panel;

#[rustfmt::skip]
fn get_loaded_shapes_mirror_pairs(loaded_shapes: &Shapes) -> Vec<(usize, usize)> {
    let l1 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI2L").unwrap();
    let r1 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI2R").unwrap();
    let l2 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_22_5L").unwrap();
    let r2 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_22_5R").unwrap();
    let l3 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_30L").unwrap();
    let r3 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_30R").unwrap();
    let loaded_shapes_mirror_pairs = vec![
        (l1, r1),
        (l2, r2),
        (l3, r3),
    ];
    loaded_shapes_mirror_pairs
}
