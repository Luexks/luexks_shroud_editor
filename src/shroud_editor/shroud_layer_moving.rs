use egui::{Pos2, Ui, pos2, vec2};
use luexks_reassembly::utility::display_oriented_math::{
    DisplayOriented3D, do3d_float_from, don_float_from,
};

use crate::{
    shroud_editor::snap_to_grid::snap_to_grid,
    shroud_interaction::{MovingShroudLayerInteraction, MovingShroudSelection},
    shroud_layer_container::ShroudLayerContainer,
};

pub fn shroud_layer_moving(
    ui: &mut Ui,
    selection: &mut MovingShroudSelection,
    shroud: &mut [ShroudLayerContainer],
    zoom: f32,
    grid_size: f32,
    grid_snap_enabled: bool,
    drag_pos: &mut Pos2,
    position_change: &mut Pos2,
    main_idx: &mut usize,
) {
    let mouse_delta = ui.input(|i| i.pointer.delta()) / zoom;
    let mut delta = vec2(mouse_delta.x + position_change.x, mouse_delta.y + position_change.y);
    *drag_pos = *drag_pos + delta;

    let old_offset = shroud
        .get(*main_idx)
        .unwrap()
        .shroud_layer
        .offset
        .clone()
        .unwrap();
    let (x, y) = (delta.x + old_offset.x.to_f32(), -delta.y + old_offset.y.to_f32());
    shroud[*main_idx].shroud_layer.offset = Some(DisplayOriented3D {
        x: don_float_from(x),
        y: don_float_from(y),
        z: old_offset.z.clone(),
    });
    if grid_snap_enabled {
        let snapped_offset = snap_to_grid(grid_size, pos2(x, y));
        shroud[*main_idx].shroud_layer.offset = Some(DisplayOriented3D {
            x: don_float_from(snapped_offset.x),
            y: don_float_from(snapped_offset.y),
            z: old_offset.z,
        });
        delta = vec2(snapped_offset.x - x, snapped_offset.y - y);
    }

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
            // let (x, y) = (delta.x + drag_pos.x, -delta.y + drag_pos.y);
            // *drag_pos = pos2(x, y);
            // shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
            //     x: don_float_from(x),
            //     y: don_float_from(y),
            //     z: old_offset.z.clone(),
            // });
            // if grid_snap_enabled {
            //     let snapped_offset = snap_to_grid(grid_size, pos2(x, y));
            //     shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
            //         x: don_float_from(snapped_offset.x),
            //         y: don_float_from(snapped_offset.y),
            //         z: old_offset.z,
            //     });
            // }
            if *selected_index != *main_idx {
                shroud[*selected_index].shroud_layer.offset = Some(do3d_float_from(old_offset.x.to_f32() + delta.x, old_offset.y.to_f32() + delta.y, old_offset.z.to_f32()));
            }
            if let Some(mirrored_index) = shroud[*selected_index].mirror_index_option {
                let offset = shroud[*selected_index].shroud_layer.offset.clone().unwrap();
                let mirrored_offset =
                    do3d_float_from(offset.x.to_f32(), -offset.y.to_f32(), offset.z.to_f32());
                shroud[mirrored_index].shroud_layer.offset = Some(mirrored_offset);
            }
        },
    );
}
