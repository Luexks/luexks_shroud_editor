use egui::{Color32, CursorIcon, Rect, Response, Sense, Stroke, Ui, UiBuilder, Vec2, Widget, vec2};

pub struct AngleGizmo<'a> {
    angle: &'a mut f32,
    angle_snap: f32,
    angle_snap_enabled: bool,
    add_undo_history: &'a mut bool,
}

impl<'a> AngleGizmo<'a> {
    pub fn new(
        angle: &'a mut f32,
        angle_snap: f32,
        angle_snap_enabled: bool,
        add_undo_history: &'a mut bool,
    ) -> Self {
        AngleGizmo {
            angle,
            angle_snap,
            angle_snap_enabled,
            add_undo_history,
        }
    }
}

const ANGLE_GIZMO_SIZE: Vec2 = vec2(20.0, 20.0);
const ANGLE_GIZMO_DISTANCE: f32 = 15.0;

impl Widget for AngleGizmo<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::empty());
        let centre = rect.min;
        let painter = ui.painter();
        painter.circle_filled(centre, 2.5, Color32::WHITE);
        let (sin, cos) = self.angle.to_radians().sin_cos();
        let mut gizmo_pos = centre;
        gizmo_pos.x += cos * ANGLE_GIZMO_DISTANCE;
        gizmo_pos.y -= sin * ANGLE_GIZMO_DISTANCE;
        let interaction_rect = Rect::from_two_pos(
            gizmo_pos - ANGLE_GIZMO_SIZE / 2.0,
            gizmo_pos + ANGLE_GIZMO_SIZE / 2.0,
        );
        painter.line_segment([centre, gizmo_pos], Stroke::new(1.0, Color32::WHITE));
        let mut interaction = Interaction::None;

        ui.scope_builder(UiBuilder::new().max_rect(interaction_rect), |ui| {
            let response = ui
                .allocate_exact_size(ANGLE_GIZMO_SIZE, Sense::click_and_drag())
                .1;
            if response.dragged()
                && let Some(mouse_pos) = response.ctx.pointer_interact_pos()
            {
                *self.angle = 360.0 - (mouse_pos - centre).angle().to_degrees();
                *self.angle %= 360.0;
                if self.angle_snap_enabled {
                    *self.angle /= self.angle_snap;
                    *self.angle = self.angle.round();
                    *self.angle *= self.angle_snap;
                }
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
                Interaction::None => 5.0,
                Interaction::Hovered => 7.5,
                Interaction::Dragged => 7.5,
            },
            Color32::MAGENTA,
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
