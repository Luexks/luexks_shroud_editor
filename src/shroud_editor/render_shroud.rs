use egui::{Pos2, Rect, Ui};
use itertools::Itertools;

use crate::{
    render_polygon::{render_polygon_fill, render_polygon_outline},
    selection_type::SelectionType,
    shroud_editor::ShroudEditor,
};

impl ShroudEditor {
    #[rustfmt::skip]
    pub fn render_shroud(&self, mouse_pos: Option<Pos2>, ui: &mut Ui, rect: Rect) {
        if !self.shroud.is_empty() {
            let render_pipeline = self.shroud.iter()
                .enumerate()
                .sorted_by(|(_, shroud_layer_container_1), (_, shroud_layer_container_2)| {
                    let z1 = shroud_layer_container_1.shroud_layer.offset.clone().unwrap().z.to_f32();
                    let z2 = shroud_layer_container_2.shroud_layer.offset.clone().unwrap().z.to_f32();
                    z1.partial_cmp(&z2).unwrap()
                })
                .collect::<Vec<_>>();
            let mut current_z = render_pipeline.first().unwrap().1.shroud_layer.offset.clone().unwrap().z.to_f32();
            let mut next_outline_render_start_index = usize::default();
            render_pipeline.iter()
                .enumerate()
                .for_each(|(pipeline_index, (index, shroud_layer_container))| {
                    let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();

                    let is_clipping_and_on_top = offset.z.to_f32() == current_z && pipeline_index == render_pipeline.len() - 1;
                    if  is_clipping_and_on_top {
                        render_polygon_fill(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            offset.clone(),
                            shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
                            shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
                        );
                    }

                    let is_above_last = offset.z.to_f32() > current_z;
                    let is_on_top = pipeline_index == render_pipeline.len() - 1;
                    if is_above_last || is_on_top {
                        render_pipeline[next_outline_render_start_index..pipeline_index].iter()
                            .for_each(|(index, shroud_layer_container)| {
                                let is_hovered = if let Some(mouse_pos) = mouse_pos {
                                    if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                                        *index == shroud_that_would_be_selected_index
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };
                                let is_selected = self.is_shroud_layer_index_selected(*index);
                                let selection_type_option = match (is_hovered, is_selected) {
                                    (true, _) => Some(SelectionType::Hovered),
                                    (false, true) => Some(SelectionType::Selected),
                                    _ => None,
                                };
                                render_polygon_outline(
                                    ui.painter(),
                                    self,
                                    rect,
                                    shroud_layer_container.get_shroud_layer_vertices(),
                                    shroud_layer_container.shroud_layer.offset.clone().unwrap(),
                                    shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
                                    selection_type_option.clone(),
                                );
                            });
                        next_outline_render_start_index = pipeline_index;
                    }

                    let is_not_clipping_and_on_top = offset.z.to_f32() > current_z && pipeline_index == render_pipeline.len() - 1;
                    let is_below_top = pipeline_index != render_pipeline.len() - 1;
                    if is_not_clipping_and_on_top || is_below_top {
                        render_polygon_fill(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            offset.clone(),
                            shroud_layer_container.shroud_layer.color_1.clone().unwrap(),
                            shroud_layer_container.shroud_layer.color_2.clone().unwrap(),
                        );
                    }
                    current_z = offset.z.to_f32();

                    if is_on_top {
                        let is_hovered = if let Some(mouse_pos) = mouse_pos {
                            if let Some(shroud_that_would_be_selected_index) = self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect) {
                                *index == shroud_that_would_be_selected_index
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        let is_selected = self.is_shroud_layer_index_selected(*index);
                        let selection_type_option = match (is_hovered, is_selected) {
                            (true, _) => Some(SelectionType::Hovered),
                            (false, true) => Some(SelectionType::Selected),
                            _ => None,
                        };
                        render_polygon_outline(
                            ui.painter(),
                            self,
                            rect,
                            shroud_layer_container.get_shroud_layer_vertices(),
                            shroud_layer_container.shroud_layer.offset.clone().unwrap(),
                            shroud_layer_container.shroud_layer.line_color.clone().unwrap(),
                            selection_type_option.clone(),
                        );
                    }
                });
        }
    }
}
