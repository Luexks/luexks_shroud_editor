use eframe::egui_glow::Painter;
use egui::{Context, Pos2, Rect, Rgba, Ui, pos2};
use itertools::Itertools;

use crate::{
    keybinds::is_shortcut_pressed,
    position_conversion::world_pos_to_screen_pos,
    shroud_editor::{
        ShroudEditor,
        render_shroud::{RenderVertex, render_lines},
    },
    shroud_interaction::ShroudInteraction,
    shroud_layer_container::ShroudLayerContainer,
    verts_to_convex_hull::verts_to_convex_hull,
};

impl ShroudEditor {
    pub fn hotkey_grouping(&mut self, ctx: &Context) {
        if is_shortcut_pressed(ctx, &self.keybinds.group) {
            self.group_selection();
            dbg!(&self.groups);
        }
    }

    fn group_selection(&mut self) {
        let selection = self.shroud_interaction.selection();
        if selection.len() < 2 {
            return;
        }
        self.add_undo_history = true;
        if let Some(first_layer_group_idx) =
            self.get_one_entire_selected_group_idx_option(&selection)
        {
            self.remove_group(first_layer_group_idx);
            return;
        }
        selection
            .iter()
            .filter_map(|i| self.shroud[*i].group_idx_option)
            .sorted_unstable()
            .unique()
            .rev()
            .for_each(|layer_group_idx| {
                self.remove_group(layer_group_idx);
            });
        selection.iter().for_each(|i| {
            self.shroud[*i].group_idx_option = Some(self.groups.len());
        });
        self.groups.push(selection);
    }

    fn remove_group(&mut self, groups_group_idx: usize) {
        assert!(groups_group_idx < self.groups.len());
        let group = self.groups.remove(groups_group_idx);
        group.into_iter().for_each(|group_layer_idx| {
            self.shroud[group_layer_idx].group_idx_option = None;
        });
        self.shroud.iter_mut().for_each(|layer| {
            if let Some(layer_group_idx) = &mut layer.group_idx_option
                && *layer_group_idx > groups_group_idx
            {
                *layer_group_idx -= 1;
            }
        });
    }

    pub fn cull_groups(&mut self) {
        self.groups
            .iter_mut()
            .enumerate()
            .rev()
            .for_each(|(groups_group_idx, group)| {
                if group.is_empty() {
                    self.shroud.iter_mut().for_each(|layer| {
                        if let Some(layer_group_idx) = &mut layer.group_idx_option
                            && *layer_group_idx > groups_group_idx
                        {
                            *layer_group_idx -= 1;
                        }
                    });
                } else if group.len() == 1 {
                    self.shroud[group[0]].group_idx_option = None;
                    group.remove(0);
                    self.shroud.iter_mut().for_each(|layer| {
                        if let Some(layer_group_idx) = &mut layer.group_idx_option
                            && *layer_group_idx > groups_group_idx
                        {
                            *layer_group_idx -= 1;
                        }
                    });
                }
            });
        self.groups.retain(|group| !group.is_empty());
    }

    pub fn get_one_entire_selected_group_idx_option(&self, selection: &[usize]) -> Option<usize> {
        let first = selection.first()?;
        if let Some(first_layer_group_idx) = self.shroud[*first].group_idx_option
            && selection.len() == self.groups[first_layer_group_idx].len()
            && selection
                .iter()
                .skip(1)
                .all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx))
        {
            Some(first_layer_group_idx)
        } else {
            None
        }
    }

    // pub fn get_one_entire_group_idx_option_of_layers(
    //     &self,
    //     layers: &[ShroudLayerContainer],
    // ) -> Option<usize> {
    //     let first_layer_group_idx = layers.first()?.group_idx_option?;
    //     (layers.len() == self.groups[first_layer_group_idx].len()
    //         && layers
    //             .iter()
    //             .skip(1)
    //             .all(|layer| layer.group_idx_option == Some(first_layer_group_idx)))
    //     .then_some(first_layer_group_idx)
    // }

    // pub fn are_group_idxs_of_one_entire_group(&self, group_idxs: &[usize]) -> bool {
    //     let first = selection.first()?;
    //         if let Some(first_layer_group_idx) = self.shroud[*first].group_idx_option
    //         && selection.len() == self.groups[*first].len()
    //         && selection.iter().skip(1).all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx)) {
    //             Some(first_layer_group_idx)
    //         } else {
    //             None
    //         }
    // }

    // pub fn is_one_entire_group_selected(&self, selection: &[usize]) -> bool {
    //     selection
    //         .first()
    //         .and_then(|first| self.shroud[*first].group_idx_option)
    //         .map(|first_layer_group_idx| selection.len() == self.groups[first_layer_group_idx].len()
    //             && selection.iter().skip(1).all(|&i| self.shroud[i].group_idx_option == Some(first_layer_group_idx))
    // ).unwrap_or(false)

    // //     selection
    // //         .first()
    // //         .and_then(|first| self.shroud[*first].group_idx_option)
    // //         .map(|first_layer_group_idx| (selection.len() == self.groups[first_layer_group_idx].len()
    // //             && selection.iter().skip(1).all(|&i| self.shroud[i].group_idx_option == Some(first_layer_group_idx))).then_some(first_layer_group_idx)
    // // ).unwrap_or_default()

    //     // if let Some(first_layer_group_idx) = self.shroud[selection[0]].group_idx_option
    //     //     && selection
    //     //         .iter()
    //     //         .skip(1)
    //     //         .all(|i| self.shroud[*i].group_idx_option == Some(first_layer_group_idx))
    //     //     && selection.len() == self.groups[first_layer_group_idx].len() {
    //     //         true
    //     //     } else {
    //     //         false
    //     //     }
    // }
    pub fn individual_shroud_layer_group_settings(&mut self, ui: &mut Ui, layer_idx: usize) {
        let Some(layer_group_idx) = self.shroud[layer_idx].group_idx_option else {
            return;
        };
        ui.horizontal(|ui| {
            if ui.button("Ungroup").clicked() {
                self.remove_group(layer_group_idx);
            }
            if ui.button("Unlink From Group").clicked() {
                if self.groups[layer_group_idx].len() == 2 {
                    self.remove_group(layer_group_idx);
                } else {
                    let Some(group_layer_idx_idx) = self.groups[layer_group_idx]
                        .iter()
                        .position(|group_layer_idx| *group_layer_idx == layer_idx)
                    else {
                        panic!("Layer idx not in group which it points to.");
                    };
                    self.groups[layer_group_idx].remove(group_layer_idx_idx);
                }
                self.shroud[layer_idx].group_idx_option = None;
            }
            if ui.button("Select Group").clicked() {
                self.shroud_interaction = ShroudInteraction::Inaction {
                    selection: self.groups[layer_group_idx].clone(),
                };
            }
        });
    }

    pub fn collective_shroud_layer_group_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Group/Ungroup Selection").clicked() {
                self.group_selection();
            }
        });
    }

    pub fn editor_shroud_layer_group_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Group Outlines:");
            ui.checkbox(&mut self.outline_groups, "");
        });
    }
}

const GROUP_OUTLINE_COLOUR: Rgba = Rgba::from_gray(200.0);

pub fn render_group_outlines(
    shroud: &[ShroudLayerContainer],
    painter: &Painter,
    rect: Rect,
    pan: Pos2,
    zoom: f32,
    groups: &[Vec<usize>],
) {
    let gl = painter.gl();
    groups.iter().for_each(|group| {
        let verts = group
            .iter()
            .flat_map(|group_layer_idx| {
                shroud[*group_layer_idx]
                    .apply_offset_to_verts(shroud[*group_layer_idx].get_shroud_layer_vertices())
                // verts.iter_mut().for_each(|vert| {
                //     *vert = world_pos_to_screen_pos(*vert, rect, pan, zoom);
                // });
                // let mut verts = shroud[*group_layer_idx]
                //     .apply_offset_to_verts(shroud[*group_layer_idx].get_shroud_layer_vertices());
                // verts.iter_mut().for_each(|vert| {
                //     *vert = world_pos_to_screen_pos(*vert, rect, pan, zoom);
                // });

                // let avg_vert_pos = verts.iter().fold(Pos2::default(), |pos, vert| {
                //     pos2(pos.x + vert.x, pos.y + vert.y)
                // }) / verts.len() as f32;
                // verts.iter_mut().for_each(|vert| {
                //     let dx = vert.x - avg_vert_pos.x;
                //     let dy = vert.y - avg_vert_pos.y;
                //     let angle = dy.atan2(dx);
                //     let distance = (dx.powi(2) + dy.powi(2)).powf(0.5);
                //     let selection_distance = distance + 5.0;
                //     let selection_x = avg_vert_pos.x + selection_distance * angle.cos();
                //     let selection_y = avg_vert_pos.y + selection_distance * angle.sin();
                //     vert.x = selection_x;
                //     vert.y = selection_y;
                // });
                // verts
            })
            .collect::<Vec<_>>();
        let mut group_verts = verts_to_convex_hull(verts, false);
        group_verts.iter_mut().for_each(|vert| {
            *vert = world_pos_to_screen_pos(*vert, rect, pan, zoom);
        });
        let avg_vert_pos = group_verts.iter().fold(Pos2::default(), |pos, vert| {
            pos2(pos.x + vert.x, pos.y + vert.y)
        }) / group_verts.len() as f32;
        group_verts.iter_mut().for_each(|vert| {
            let dx = vert.x - avg_vert_pos.x;
            let dy = vert.y - avg_vert_pos.y;
            let angle = dy.atan2(dx);
            let distance = (dx.powi(2) + dy.powi(2)).powf(0.5);
            let selection_distance = distance + 5.0;
            let selection_x = avg_vert_pos.x + selection_distance * angle.cos();
            let selection_y = avg_vert_pos.y + selection_distance * angle.sin();
            vert.x = selection_x;
            vert.y = selection_y;
        });
        let mut render_outline_vertices = Vec::new();
        group_verts
            .iter()
            .zip(group_verts.iter().cycle().skip(1))
            .for_each(|(a, b)| {
                render_outline_vertices.push(RenderVertex::from_screen_data(
                    *a,
                    GROUP_OUTLINE_COLOUR,
                    rect,
                ));
                render_outline_vertices.push(RenderVertex::from_screen_data(
                    *b,
                    GROUP_OUTLINE_COLOUR,
                    rect,
                ));
            });
        render_lines(&render_outline_vertices, gl);
    });
}
