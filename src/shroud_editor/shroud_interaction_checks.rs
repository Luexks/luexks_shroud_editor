use egui::{Pos2, Rect};

use crate::{
    pos_in_polygon::is_pos_in_polygon, shroud_editor::ShroudEditor,
    shroud_layer_container::ShroudLayerContainer,
};

impl ShroudEditor {
    pub fn is_shroud_layer_index_selected(&self, index: usize) -> bool {
        self.shroud_layer_interaction
            .is_shroud_layer_index_selected(index)
    }

    pub fn is_shroud_hovered(
        &self,
        mouse_pos: Option<Pos2>,
        shroud_layer_container: &ShroudLayerContainer,
        rect: Rect,
    ) -> bool {
        if let Some(mouse_pos) = mouse_pos {
            let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
            is_pos_in_polygon(
                mouse_pos,
                self.positions_to_screen_positions(
                    &shroud_layer_container
                        .get_shroud_layer_vertices()
                        .iter()
                        .map(|vertex| {
                            Pos2::new(vertex.x + offset.x.to_f32(), vertex.y - offset.y.to_f32())
                        })
                        .collect::<Vec<_>>(),
                    rect,
                ),
            )
        } else {
            false
        }
    }

    pub fn get_shroud_that_would_be_selected_index_option(
        &self,
        mouse_pos: Pos2,
        rect: Rect,
    ) -> Option<usize> {
        let mut dragged_on_shroud_layer_data: Vec<(usize, f32)> = Vec::default();
        self.shroud
            .iter()
            .enumerate()
            .for_each(|(index, shroud_layer_container)| {
                if self.is_shroud_hovered(Some(mouse_pos), shroud_layer_container, rect) {
                    dragged_on_shroud_layer_data.push((
                        index,
                        shroud_layer_container
                            .shroud_layer
                            .offset
                            .clone()
                            .unwrap()
                            .z
                            .to_f32(),
                    ));
                }
            });
        dragged_on_shroud_layer_data.sort_by(|(_, z1), (_, z2)| z2.partial_cmp(z1).unwrap());
        if let Some((index, _)) = dragged_on_shroud_layer_data.first() {
            Some(*index)
        } else {
            None
        }
    }
}
