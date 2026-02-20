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
                        if is_there_aabb_triangle_intersection(selection_box, tri) {
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

fn is_there_aabb_triangle_intersection(aabb: Rect, tri: [Pos2; 3]) -> bool {
    if tri.iter().any(|vert| aabb.contains(*vert)) {
        return true;
    }
    let aabb_verts = [aabb.min, pos2(aabb.max.x, aabb.min.y), aabb.max, pos2(aabb.min.x, aabb.max.y)];
    for vert in aabb_verts.iter() {
        if is_point_in_tri(*vert, tri) {
            return true;
        }
    }
    false
}

fn is_point_in_tri(p: Pos2, tri: [Pos2; 3]) -> bool {
    let [a, b, c] = tri;
    let denominator = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    let area_fraction_a = ((b.y - c.y) * (p.x - c.x) + (c.x - b.x) * (p.y - c.y)) / denominator;
    let area_fraction_b = ((c.y - a.y) * (p.x - c.x) + (a.x - c.x) * (p.y - c.y)) / denominator;
    let area_fraction_c = 1.0 - area_fraction_a - area_fraction_b;
    area_fraction_a >= 0.0 && area_fraction_b >= 0.0 && area_fraction_c >= 0.0
}

fn triangulate(verts: &[Pos2]) -> Vec<[Pos2; 3]> {
    if verts.len() < 3 {
        return Vec::new();
    };
    let a = verts[0];
    verts[1..].windows(2).map(|w| [a, w[0], w[1]]).collect()
}
