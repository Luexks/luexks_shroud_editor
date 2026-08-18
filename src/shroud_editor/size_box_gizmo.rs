use std::f32::consts::FRAC_PI_2;

use egui::{
    Color32, CursorIcon, Pos2, Rect, Response, Sense, Stroke, Ui, UiBuilder, Vec2, Widget, pos2,
    vec2,
};

use crate::{
    pos_and_display_oriented_number_conversion::{do2d_to_pos2, do3d_to_pos2},
    position_conversion::screen_pos_to_world_pos,
    shroud_editor::{
        ShroudEditor,
        shroud_settings::{ShroudLayerSettingsTarget, SingleSettingsTarget},
    },
    snap_to_grid::snap_to_grid_linear,
};

impl ShroudEditor {
    pub fn size_box_gizmo(&mut self, ui: &mut Ui, gizmo_centre: Pos2, idx: usize, rect: Rect) {
        let shroud_layer_settings_target = &mut SingleSettingsTarget {
            shroud: &mut self.shroud,
            idx,
        };
        let is_square = shroud_layer_settings_target.get_shape_id_str() == "SQUARE";
        let layer = shroud_layer_settings_target.get_main_layer_mut();
        let angle = -layer.angle.as_ref().unwrap().as_radians().get_value();
        let add_undo_history = &mut false;
        let mut changed = false;
        let mut size = do2d_to_pos2(layer.size.as_ref().unwrap());
        let mut offset = do3d_to_pos2(layer.offset.as_ref().unwrap());
        let zoom = self.zoom;
        let pan = self.pan;
        let (x, y) = (size.x / 2. * self.zoom, -size.y / 2. * self.zoom);
        let (n, ne, e, se, s, sw, w, nw) = if is_square {
            let y = y * 2.;
            (
                gizmo_centre + apply_angle(vec2(x, y), angle),
                gizmo_centre + apply_angle(vec2(x * 2., y), angle),
                gizmo_centre + apply_angle(vec2(x * 2., 0.), angle),
                gizmo_centre + apply_angle(vec2(x * 2., -y), angle),
                gizmo_centre + apply_angle(vec2(x, -y), angle),
                gizmo_centre + apply_angle(vec2(0., -y), angle),
                gizmo_centre + apply_angle(vec2(0., 0.), angle),
                gizmo_centre + apply_angle(vec2(0., y), angle),
            )
        } else {
            (
                gizmo_centre + vec2(0., y),
                gizmo_centre + vec2(x, y),
                gizmo_centre + vec2(x, 0.),
                gizmo_centre + vec2(x, -y),
                gizmo_centre + vec2(0., -y),
                gizmo_centre + vec2(-x, -y),
                gizmo_centre + vec2(-x, 0.),
                gizmo_centre + vec2(-x, y),
            )
        };
        let option_pos_n = size_box_gizmo_individual(ui, n, add_undo_history);
        let option_pos_ne = size_box_gizmo_individual(ui, ne, add_undo_history);
        let option_pos_e = size_box_gizmo_individual(ui, e, add_undo_history);
        let option_pos_se = size_box_gizmo_individual(ui, se, add_undo_history);
        let option_pos_s = size_box_gizmo_individual(ui, s, add_undo_history);
        let option_pos_sw = size_box_gizmo_individual(ui, sw, add_undo_history);
        let option_pos_w = size_box_gizmo_individual(ui, w, add_undo_history);
        let option_pos_nw = size_box_gizmo_individual(ui, nw, add_undo_history);
        let old_size = size;
        let shift = ui.ctx().input(|i| i.modifiers.shift);
        let grid_size = self.grid_size;
        let grid_snap_enabled = self.grid_snap_enabled;

        if is_square {
            let cardinal = |is_horizontal: bool,
                            direction: bool,
                            gizmo_pos: Pos2,
                            mouse_pos: Pos2,
                            size_component: &mut f32,
                            offset: &mut Pos2| {
                let is_vertical_factor = if !is_horizontal { 0.5 } else { -1. };
                let delta = screen_pos_to_world_pos(mouse_pos, rect, pan, zoom)
                    - screen_pos_to_world_pos(gizmo_pos, rect, pan, zoom);
                let (sin, cos) = (-angle).sin_cos();
                let delta_rotated =
                    pos2(delta.x * cos - delta.y * sin, delta.x * sin + delta.y * cos);
                let dist = if is_horizontal {
                    -delta_rotated.x
                } else {
                    delta_rotated.y * is_vertical_factor
                };
                let dist = if grid_snap_enabled {
                    if is_horizontal {
                        snap_to_grid_linear(grid_size, dist)
                    } else {
                        snap_to_grid_linear(grid_size, dist * 2.) / 2.
                    }
                } else {
                    dist
                };
                *size_component -= dist.b(direction);
                if shift {
                    if is_horizontal {
                        *offset += vec2(dist * cos, dist * sin).b(direction) / 2.;
                    }
                } else {
                    let offset_vector = if is_horizontal {
                        if direction {
                            Vec2::ZERO
                        } else {
                            vec2(dist * cos, dist * sin)
                        }
                    } else {
                        let (perpendicular_sin, perpendicular_cos) = (-angle + FRAC_PI_2).sin_cos();
                        vec2(dist * perpendicular_cos, dist * perpendicular_sin)
                    };
                    *offset -= offset_vector;
                }
            };

            if let Some(mouse_pos) = option_pos_n {
                cardinal(false, true, n, mouse_pos, &mut size.y, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_e {
                cardinal(true, true, e, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_s {
                cardinal(false, false, s, mouse_pos, &mut size.y, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_w {
                cardinal(true, false, w, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }

            if let Some(mouse_pos) = option_pos_ne {
                cardinal(false, true, ne, mouse_pos, &mut size.y, &mut offset);
                cardinal(true, true, ne, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_se {
                cardinal(false, false, se, mouse_pos, &mut size.y, &mut offset);
                cardinal(true, true, se, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_sw {
                cardinal(false, false, sw, mouse_pos, &mut size.y, &mut offset);
                cardinal(true, false, sw, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_nw {
                cardinal(false, true, nw, mouse_pos, &mut size.y, &mut offset);
                cardinal(true, false, nw, mouse_pos, &mut size.x, &mut offset);
                changed = true;
            }
        } else {
            let cardinal = |is_horizontal: bool,
                            direction: bool,
                            gizmo_pos: Pos2,
                            mouse_pos: Pos2,
                            size_component: &mut f32,
                            offset_component: &mut f32| {
                let old_size_component = if is_horizontal {
                    old_size.x
                } else {
                    old_size.y
                };
                let dist = if is_horizontal {
                    screen_pos_to_world_pos(mouse_pos, rect, pan, zoom).x
                        - screen_pos_to_world_pos(gizmo_pos, rect, pan, zoom).x
                } else {
                    screen_pos_to_world_pos(mouse_pos, rect, pan, zoom).y
                        - screen_pos_to_world_pos(gizmo_pos, rect, pan, zoom).y
                };
                *size_component -= dist.b(direction).b(!is_horizontal);

                if grid_snap_enabled {
                    *size_component = snap_to_grid_linear(grid_size, *size_component);
                }
                if !shift {
                    if grid_snap_enabled {
                        let snapped_dist = old_size_component - *size_component;
                        *offset_component -= snapped_dist.b(direction) / 2.;
                    } else {
                        *offset_component -= dist.b(!is_horizontal) / 2.;
                    }
                }
            };

            if let Some(mouse_pos) = option_pos_n {
                cardinal(false, true, n, mouse_pos, &mut size.y, &mut offset.y);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_e {
                cardinal(true, true, e, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_s {
                cardinal(false, false, s, mouse_pos, &mut size.y, &mut offset.y);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_w {
                cardinal(true, false, w, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }

            if let Some(mouse_pos) = option_pos_ne {
                cardinal(false, true, ne, mouse_pos, &mut size.y, &mut offset.y);
                cardinal(true, true, ne, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_se {
                cardinal(false, false, se, mouse_pos, &mut size.y, &mut offset.y);
                cardinal(true, true, se, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_sw {
                cardinal(false, false, sw, mouse_pos, &mut size.y, &mut offset.y);
                cardinal(true, false, sw, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }
            if let Some(mouse_pos) = option_pos_nw {
                cardinal(false, true, nw, mouse_pos, &mut size.y, &mut offset.y);
                cardinal(true, false, nw, mouse_pos, &mut size.x, &mut offset.x);
                changed = true;
            }
        }
        if *add_undo_history {
            self.add_undo_history = true;
        }
        if changed {
            *layer.offset.as_mut().unwrap().x.to_f32_mut() = offset.x;
            *layer.offset.as_mut().unwrap().y.to_f32_mut() = offset.y;
            *layer.size.as_mut().unwrap().x.to_f32_mut() = size.x;
            *layer.size.as_mut().unwrap().y.to_f32_mut() = size.y;
            shroud_layer_settings_target.on_x_changed(offset.x);
            shroud_layer_settings_target.on_y_changed(offset.y);
            shroud_layer_settings_target.on_width_changed(size.x);
            shroud_layer_settings_target.on_height_changed(size.y);
        }
    }
}

fn apply_angle(pos: Vec2, radians: f32) -> Vec2 {
    let (sin, cos) = radians.sin_cos();
    vec2(pos.x * cos - pos.y * sin, pos.x * sin + pos.y * cos)
}

fn size_box_gizmo_individual(
    ui: &mut Ui,
    gizmo_pos: Pos2,
    add_undo_history: &mut bool,
) -> Option<Pos2> {
    let mut mouse_pos_option = None;
    let gizmo_rect = Rect::from_two_pos(gizmo_pos, gizmo_pos + vec2(1.0, 1.0));
    ui.scope_builder(UiBuilder::new().max_rect(gizmo_rect), |ui| {
        egui::Frame::new()
            .fill(Color32::TRANSPARENT)
            .show(ui, |ui| {
                ui.add(SizeBoxGizmo::new(&mut mouse_pos_option, add_undo_history));
            });
    });
    mouse_pos_option
}

pub struct SizeBoxGizmo<'a> {
    mouse_pos_option: &'a mut Option<Pos2>,
    add_undo_history: &'a mut bool,
}
impl<'a> SizeBoxGizmo<'a> {
    pub fn new(mouse_pos_option: &'a mut Option<Pos2>, add_undo_history: &'a mut bool) -> Self {
        Self {
            mouse_pos_option,
            add_undo_history,
        }
    }
}

const SIZE_BOX_GIZMO_SIZE: Vec2 = vec2(20., 20.);

impl Widget for SizeBoxGizmo<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut interaction = Interaction::None;
        let (rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::empty());
        let gizmo_pos = rect.min;
        let interaction_rect = Rect::from_two_pos(
            gizmo_pos - SIZE_BOX_GIZMO_SIZE / 2.0,
            gizmo_pos + SIZE_BOX_GIZMO_SIZE / 2.0,
        );

        ui.scope_builder(UiBuilder::new().max_rect(interaction_rect), |ui| {
            let response = ui
                .allocate_exact_size(SIZE_BOX_GIZMO_SIZE, Sense::click_and_drag())
                .1;
            if response.dragged()
                && let Some(mouse_pos) = response.ctx.pointer_interact_pos()
            {
                *self.mouse_pos_option = Some(mouse_pos);
                interaction = Interaction::Dragged;
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            } else if response.is_pointer_button_down_on() {
                interaction = Interaction::Dragged;
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            } else if response.hovered() {
                interaction = Interaction::Hovered;
                ui.ctx().set_cursor_icon(CursorIcon::Grab);
            }
            if response.drag_stopped() {
                *self.add_undo_history = true;
            }
        });
        let painter = ui.painter();
        painter.circle(
            gizmo_pos,
            match interaction {
                Interaction::None => 3.,
                Interaction::Hovered => 6.,
                Interaction::Dragged => 6.,
            },
            Color32::GRAY,
            Stroke::new(1.0, Color32::WHITE),
        );
        response
    }
}

enum Interaction {
    None,
    Hovered,
    Dragged,
}

trait BoolFlip {
    fn b(self, b: bool) -> Self;
}

impl BoolFlip for f32 {
    fn b(self, b: bool) -> Self {
        if b { self } else { -self }
    }
}
impl BoolFlip for Vec2 {
    fn b(self, b: bool) -> Self {
        if b { self } else { -self }
    }
}
