use egui::{Response, Ui, pos2};
use luexks_reassembly::utility::display_oriented_math::{
    DisplayOriented3D, do3d_float_from, don_float_from,
};

use crate::{
    shroud_editor::snap_to_grid::snap_to_grid, shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

pub fn shroud_layer_dragging(
    ui: &mut Ui,
    response: &Response,
    selection: &Vec<usize>,
    shroud: &mut Vec<ShroudLayerContainer>,
    zoom: f32,
    grid_size: f32,
    snap_to_grid_enabled: bool,
    shroud_layer_interaction: &mut ShroudLayerInteraction,
) {
    let delta = ui.input(|i| i.pointer.delta()) / zoom;
    selection.iter().for_each(|selected_index| {
        let old_offset = shroud
            .get(*selected_index)
            .unwrap()
            .shroud_layer
            .offset
            .clone()
            .unwrap();
        if let None = shroud[*selected_index].drag_pos {
            shroud[*selected_index].drag_pos =
                Some(pos2(old_offset.x.to_f32(), old_offset.y.to_f32()));
        }
        let (x, y) = (
            delta.x + shroud[*selected_index].drag_pos.unwrap().x,
            delta.y + shroud[*selected_index].drag_pos.unwrap().y,
        );
        shroud[*selected_index].drag_pos = Some(pos2(x, y));
        shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
            x: don_float_from(x),
            y: don_float_from(y),
            z: old_offset.z.clone(),
        });
        if snap_to_grid_enabled {
            let snapped_offset = snap_to_grid(grid_size, pos2(x, y));
            shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
                x: don_float_from(snapped_offset.x),
                y: don_float_from(snapped_offset.y),
                z: old_offset.z,
            });
        }
        if let Some(mirrored_index) = shroud[*selected_index].mirror_index_option {
            let offset = shroud[*selected_index].shroud_layer.offset.clone().unwrap();
            let mirrored_offset =
                do3d_float_from(offset.x.to_f32(), -offset.y.to_f32(), offset.z.to_f32());
            shroud[mirrored_index].shroud_layer.offset = Some(mirrored_offset);
        }
    });
    if response.drag_stopped() {
        selection.iter().for_each(|selected_index| {
            shroud[*selected_index].drag_pos = None;
        });
        *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
            selection: shroud_layer_interaction.selection(),
        };
    }
}
