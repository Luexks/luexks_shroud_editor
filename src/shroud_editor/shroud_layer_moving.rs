use egui::{Response, Ui, pos2};
use luexks_reassembly::utility::display_oriented_math::{
    DisplayOriented3D, do3d_float_from, don_float_from,
};

use crate::{
    shroud_editor::snap_to_grid::snap_to_grid,
    shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection, ShroudInteraction},
    shroud_layer_container::ShroudLayerContainer,
};

pub fn shroud_layer_moving(
    ui: &mut Ui,
    selection: &mut MovingShroudSelection,
    shroud: &mut [ShroudLayerContainer],
    zoom: f32,
    grid_size: f32,
    grid_snap_enabled: bool,
) {
    let delta = ui.input(|i| i.pointer.delta()) / zoom;
    selection.0.iter_mut().for_each(
        |MovingShroudLayerInteraction {
             idx: selected_index,
             drag_pos,
         }| {
            let old_offset = shroud
                .get(*selected_index)
                .unwrap()
                .shroud_layer
                .offset
                .clone()
                .unwrap();
            // if shroud[*selected_index].drag_pos_option.is_none() {
            //     shroud[*selected_index].drag_pos_option =
            //         Some(pos2(old_offset.x.to_f32(), old_offset.y.to_f32()));
            // }
            let (x, y) = (delta.x + drag_pos.x, -delta.y + drag_pos.y);
            *drag_pos = pos2(x, y);
            shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
                x: don_float_from(x),
                y: don_float_from(y),
                z: old_offset.z.clone(),
            });
            if grid_snap_enabled {
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
        },
    );
    // if response.drag_stopped() {
    //     *shroud_interaction = ShroudInteraction::Inaction {
    //         selection: shroud_interaction.selection(),
    //     };
    // }
}
