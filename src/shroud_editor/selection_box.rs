use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Ui, pos2};

use crate::{shroud_editor::ShroudEditor, shroud_interaction::ShroudInteraction};

impl ShroudEditor {
    pub fn selection_box_logic(&mut self, ui: &mut Ui, rect: Rect) {
        if let Some(selection_box_start_pos) = self.selection_box_start_pos_option {
            self.draw_selection_box(ui, rect, selection_box_start_pos);
            self.selection_box_update_selection(
                selection_box_start_pos,
                ui.ctx().input(|i| i.modifiers.shift),
            );
        }
    }

    pub fn draw_selection_box(&self, ui: &mut Ui, rect: Rect, selection_box_start_pos: Pos2) {
        let painter = ui.painter();
        let fill_color = Color32::from_rgba_unmultiplied(255, 0, 255, 50);
        let stroke = Stroke::new(1.0, Color32::from_rgb(255, 0, 255));
        painter.rect(
            Rect::from_points(&[
                self.world_pos_to_screen_pos(self.world_mouse_pos, rect),
                self.world_pos_to_screen_pos(selection_box_start_pos, rect),
            ]),
            1,
            fill_color,
            stroke,
            StrokeKind::Inside,
        );
    }

    pub fn selection_box_update_selection(&mut self, selection_box_start_pos: Pos2, shift: bool) {
        let selection_box = Rect::from_two_pos(selection_box_start_pos, self.world_mouse_pos);
        let original_selection = self.shroud_interaction.selection();
        let mut to_be_selected = Vec::new();
        (0..self.shroud.len()).for_each(|i| {
            if !shift || !original_selection.contains(&i) {
                let shroud_layer_container = &self.shroud[i];
                let verts = shroud_layer_container
                    .get_shroud_layer_vertices()
                    .into_iter()
                    .map(|vert| {
                        pos2(
                            vert.x
                                + shroud_layer_container
                                    .shroud_layer
                                    .offset
                                    .as_ref()
                                    .unwrap()
                                    .x
                                    .to_f32(),
                            vert.y
                                - shroud_layer_container
                                    .shroud_layer
                                    .offset
                                    .as_ref()
                                    .unwrap()
                                    .y
                                    .to_f32(),
                        )
                    })
                    .collect::<Vec<_>>();
                let aabb = Rect::from_points(&verts);
                if aabb.intersects(selection_box) {
                    for tri in triangulate(&verts) {
                        if sat_aabb_and_triangle(selection_box, tri) {
                            to_be_selected.push(i);
                            break;
                        }
                    }
                }
            }
        });
        if shift {
            self.shroud_interaction = ShroudInteraction::Inaction {
                selection: original_selection
                    .into_iter()
                    .chain(to_be_selected.into_iter())
                    .collect(),
            };
        } else {
            self.shroud_interaction = ShroudInteraction::Inaction {
                selection: to_be_selected,
            }
        }
    }
}

fn sat_aabb_and_triangle(aabb: Rect, tri: [Pos2; 3]) -> bool {
    let aabb = [
        pos2(aabb.min.x, aabb.min.y),
        pos2(aabb.max.x, aabb.min.y),
        pos2(aabb.max.x, aabb.max.y),
        pos2(aabb.min.x, aabb.max.y),
    ];
    let axes = [
        pos2(1.0, 0.0),
        pos2(0.0, 1.0),
        perp([tri[0], tri[1]]),
        perp([tri[1], tri[2]]),
        perp([tri[2], tri[0]]),
    ];
    for axis in axes {
        let p1 = project_tri(axis, tri);
        let p2 = project_aabb(axis, aabb);
        if !overlap(p1, p2) {
            return false;
        }
    }
    true
}

fn perp(line: [Pos2; 2]) -> Pos2 {
    let line = line[0] - line[1];
    pos2(line.y, -line.x)
}

fn triangulate(verts: &[Pos2]) -> Vec<[Pos2; 3]> {
    if verts.len() < 3 {
        return Vec::new();
    };
    let a = verts[0];
    verts[1..].windows(2).map(|w| [a, w[0], w[1]]).collect()
}

fn dot(a: Pos2, b: Pos2) -> f32 {
    a.x * b.x + a.y * b.y
}

fn project_tri(axis: Pos2, tri: [Pos2; 3]) -> (f32, f32) {
    let mut min = dot(axis, tri[0]);
    let mut max = min;
    tri[1..].iter().for_each(|point| {
        let p = dot(axis, *point);
        min = min.min(p);
        max = max.max(p);
    });
    (min, max)
}

fn project_aabb(axis: Pos2, tri: [Pos2; 4]) -> (f32, f32) {
    let mut min = dot(axis, tri[0]);
    let mut max = min;
    tri[1..].iter().for_each(|point| {
        let p = dot(axis, *point);
        min = min.min(p);
        max = max.max(p);
    });
    (min, max)
}

fn overlap(a: (f32, f32), b: (f32, f32)) -> bool {
    a.1 >= b.0 && a.0 <= b.1
}
