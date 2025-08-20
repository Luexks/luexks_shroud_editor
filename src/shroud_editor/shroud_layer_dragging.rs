use egui::{Response, Ui};
use luexks_reassembly::utility::display_oriented_math::{DisplayOriented3D, don_float_from};

use crate::{
    shroud_editor::ShroudEditor, shroud_layer_container::ShroudLayerContainer,
    shroud_layer_interaction::ShroudLayerInteraction,
};

pub fn shroud_layer_dragging(
    ui: &mut Ui,
    response: &Response,
    selection: &Vec<usize>,
    shroud: &mut Vec<ShroudLayerContainer>,
    zoom: f32,
    grid_size: f32,
    snap_to_grid: bool,
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
        let (x, y) = (
            don_float_from(delta.x + old_offset.x.to_f32()),
            don_float_from(delta.y + old_offset.y.to_f32()),
        );
        shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
            x: x,
            y: y,
            z: old_offset.z,
        });
    });
    if response.drag_stopped() {
        selection.iter().for_each(|selected_index| {
            let old_offset = shroud
                .get(*selected_index)
                .unwrap()
                .shroud_layer
                .offset
                .clone()
                .unwrap();
            if snap_to_grid {
                let (x, y) = (
                    don_float_from(
                        (delta.x + old_offset.x.to_f32() / grid_size).round() * grid_size,
                    ),
                    don_float_from(
                        (delta.y + old_offset.y.to_f32() / grid_size).round() * grid_size,
                    ),
                );
                shroud[*selected_index].shroud_layer.offset = Some(DisplayOriented3D {
                    x: x,
                    y: y,
                    z: old_offset.z,
                });
            }
        });
        *shroud_layer_interaction = ShroudLayerInteraction::Inaction {
            selection: shroud_layer_interaction.selection(),
        };
    }
}
