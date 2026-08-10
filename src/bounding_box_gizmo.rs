use egui::{
    Color32, CursorIcon, Key::I, Pos2, Rect, Response, Sense, Stroke, Ui, UiBuilder, Vec2, Widget, pos2, vec2,
};

use crate::{
    pos_and_display_oriented_number_conversion::{do2d_to_pos2, do3d_to_pos2}, position_conversion::{screen_pos_to_world_pos, screen_vec_to_world_pos}, rotation_edgecase::{RotationEdgecase, rotation_edgecase_logic_radians}, shroud_editor::ShroudEditor,
};

impl ShroudEditor {
    pub fn bounding_box_gizmo(&mut self, ui: &mut Ui, gizmo_centre: Pos2, idx: usize, rect: Rect) {
        let layer = &mut self.shroud[idx];
        let is_square = layer.shape_id == "SQUARE";
        let angle = -layer
            .shroud_layer
            .angle
            .as_ref()
            .unwrap()
            .as_radians()
            .get_value();
        let add_undo_history = &mut false;
        let mut changed = false;
        let mut size = do2d_to_pos2(layer.shroud_layer.size.as_ref().unwrap()) * self.zoom;
        let mut offset = do3d_to_pos2(layer.shroud_layer.offset.as_ref().unwrap()) * self.zoom;
        let zoom = self.zoom;
        let pan = self.pan;
        let (x, y) = (size.x / 2., -size.y / 2.);
        let o = 0.;
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
                gizmo_centre + vec2(o, y),
                gizmo_centre + vec2(x, y),
                gizmo_centre + vec2(x, o),
                gizmo_centre + vec2(x, -y),
                gizmo_centre + vec2(o, -y),
                gizmo_centre + vec2(-x, -y),
                gizmo_centre + vec2(-x, o),
                gizmo_centre + vec2(-x, y),
            )
        };
        let option_pos_n = bounding_box_gizmo_individual(ui, n, add_undo_history);
        let option_pos_ne = bounding_box_gizmo_individual(ui, ne, add_undo_history);
        let option_pos_e = bounding_box_gizmo_individual(ui, e, add_undo_history);
        let option_pos_se = bounding_box_gizmo_individual(ui, se, add_undo_history);
        let option_pos_s = bounding_box_gizmo_individual(ui, s, add_undo_history);
        let option_pos_sw = bounding_box_gizmo_individual(ui, sw, add_undo_history);
        let option_pos_w = bounding_box_gizmo_individual(ui, w, add_undo_history);
        let option_pos_nw = bounding_box_gizmo_individual(ui, nw, add_undo_history);
        if is_square {

        } else {
            if let Some(mouse_pos) = option_pos_n {
                let dist = screen_vec_to_world_pos(mouse_pos - n, rect, pan, zoom).y;
                dbg!(dist);
                size.y += dist;
                offset.y += dist / 2.;
                changed = true;
            }
        }
        if *add_undo_history {
            self.add_undo_history = true;
        }
        if changed {
            *layer.shroud_layer.offset.as_mut().unwrap().x.to_f32_mut() = offset.x;
            *layer.shroud_layer.offset.as_mut().unwrap().y.to_f32_mut() = offset.y;
            *layer.shroud_layer.size.as_mut().unwrap().x.to_f32_mut() = size.x;
            *layer.shroud_layer.size.as_mut().unwrap().y.to_f32_mut() = size.y;
        }
    }

}

fn apply_angle(pos: Vec2, radians: f32) -> Vec2 {
    let (sin, cos) = radians.sin_cos();
    vec2(pos.x * cos - pos.y * sin, pos.x * sin + pos.y * cos)
}

fn bounding_box_gizmo_individual(
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
                ui.add(BoundingBoxGizmo::new(
                    &mut mouse_pos_option,
                    add_undo_history,
                ));
            });
    });
    mouse_pos_option
}

pub struct BoundingBoxGizmo<'a> {
    mouse_pos_option: &'a mut Option<Pos2>,
    add_undo_history: &'a mut bool,
}
impl<'a> BoundingBoxGizmo<'a> {
    pub fn new(mouse_pos_option: &'a mut Option<Pos2>, add_undo_history: &'a mut bool) -> Self {
        Self {
            mouse_pos_option,
            add_undo_history,
        }
    }
}

const BOUNDING_BOX_GIZMO_SIZE: Vec2 = vec2(20., 20.);

impl Widget for BoundingBoxGizmo<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut interaction = Interaction::None;
        let (rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::empty());
        let gizmo_pos = rect.min;
        let interaction_rect = Rect::from_two_pos(
            gizmo_pos - BOUNDING_BOX_GIZMO_SIZE / 2.0,
            gizmo_pos + BOUNDING_BOX_GIZMO_SIZE / 2.0,
        );

        ui.scope_builder(UiBuilder::new().max_rect(interaction_rect), |ui| {
            let response = ui
                .allocate_exact_size(BOUNDING_BOX_GIZMO_SIZE, Sense::click_and_drag())
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
                Interaction::Hovered => 5.,
                Interaction::Dragged => 5.,
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
