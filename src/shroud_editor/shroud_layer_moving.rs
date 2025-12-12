use egui::{Pos2, Ui, vec2};
use luexks_reassembly::utility::display_oriented_math::do3d_float_from;

use crate::{
    pos_and_display_oriented_number_conversion::pos2_to_do3d,
    shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection},
    shroud_layer_container::ShroudLayerContainer,
    snap_to_grid::snap_to_grid,
};

pub fn shroud_layer_moving(
    ui: &mut Ui,
    drag_pos: &mut Pos2,
    potentially_snapped_drag_pos: &mut Pos2,
    selection: &mut MovingShroudSelection,
    shroud: &mut [ShroudLayerContainer],
    zoom: f32,
    grid_size: f32,
    grid_snap_enabled: bool,
) {
    let delta = ui.input(|i| i.pointer.delta()) / zoom;
    *drag_pos += vec2(delta.x, -delta.y);
    *potentially_snapped_drag_pos = if grid_snap_enabled {
        snap_to_grid(grid_size, *drag_pos)
    } else {
        *drag_pos
    };
    selection.0.iter_mut().for_each(
        |MovingShroudLayerInteraction {
             idx: selected_index,
             relative_pos,
         }| {
            let old_offset = shroud
                .get(*selected_index)
                .unwrap()
                .shroud_layer
                .offset
                .clone()
                .unwrap();
            shroud[*selected_index].shroud_layer.offset = Some(pos2_to_do3d(
                &(*potentially_snapped_drag_pos - *relative_pos),
                old_offset.z.to_f32(),
            ));
            // if grid_snap_enabled {
            //     let snapped_offset = snap_to_grid(grid_size, pos2(x, y));
            //     shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
            //         x: don_float_from(snapped_offset.x),
            //         y: don_float_from(snapped_offset.y),
            //         z: old_offset.z,
            //     });
            // }
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
