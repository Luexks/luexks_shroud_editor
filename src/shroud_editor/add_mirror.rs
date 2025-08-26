use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer,
    utility::{angle::Angle, display_oriented_math::do3d_float_from},
};

use crate::shroud_layer_container::ShroudLayerContainer;

pub fn add_mirror(shroud: &mut Vec<ShroudLayerContainer>, index: usize, should_mirror_be_selected: bool) {
    shroud[index].mirror_index_option = Some(shroud.len());

    let offset = shroud[index].shroud_layer.offset.clone().unwrap();
    let shroud_layer_mirror = ShroudLayerContainer {
        shroud_layer: ShroudLayer {
            offset: Some(do3d_float_from(
                offset.x.to_f32(),
                -offset.y.to_f32(),
                offset.z.to_f32(),
            )),
            angle: Some(Angle::Radian(
                -shroud[index]
                    .shroud_layer
                    .angle
                    .clone()
                    .unwrap()
                    .as_radians()
                    .get_value(),
            )),
            ..shroud[index].shroud_layer.clone()
        },
        mirror_index_option: Some(index),
        drag_pos: if should_mirror_be_selected {
            shroud[index].drag_pos
        } else {
            None
        },
        ..shroud[index].clone()
    };

    shroud.push(shroud_layer_mirror);
}
