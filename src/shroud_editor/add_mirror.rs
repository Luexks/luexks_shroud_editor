use egui::Pos2;
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer,
    shapes::{shape_id::ShapeId, shapes::Shapes},
    utility::{angle::Angle, display_oriented_math::do3d_float_from},
};

use crate::{
    restructure_vertices::restructure_vertices, shroud_layer_container::ShroudLayerContainer,
};

pub fn add_mirror(
    shroud: &mut Vec<ShroudLayerContainer>,
    index: usize,
    should_mirror_be_selected: bool,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &Vec<(usize, usize)>,
) {
    shroud[index].mirror_index_option = Some(shroud.len());

    let offset = shroud[index].shroud_layer.offset.clone().unwrap();
    // let size = shroud[index].shroud_layer.size.clone().unwrap();
    let (shape, shape_id, vertices) =
        get_mirrored_shape_data(shroud, index, loaded_shapes, loaded_shapes_mirror_pairs);

    let shroud_layer_mirror = ShroudLayerContainer {
        shroud_layer: ShroudLayer {
            offset: Some(do3d_float_from(
                offset.x.to_f32(),
                -offset.y.to_f32(),
                offset.z.to_f32(),
            )),
            // size: Some(do2d_float_from(size.x.to_f32(), -size.y.to_f32())),
            angle: Some(Angle::Radian(
                -shroud[index]
                    .shroud_layer
                    .angle
                    .clone()
                    .unwrap()
                    .as_radians()
                    .get_value(),
            )),
            shape: Some(shape),
            ..shroud[index].shroud_layer.clone()
        },
        mirror_index_option: Some(index),
        drag_pos: if should_mirror_be_selected {
            shroud[index].drag_pos
        } else {
            None
        },
        shape_id,
        vertices,
        ..shroud[index].clone()
    };

    shroud.push(shroud_layer_mirror);
}

pub fn get_mirrored_shape_data(
    shroud: &Vec<ShroudLayerContainer>,
    index: usize,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &Vec<(usize, usize)>,
) -> (ShapeId, String, Vec<Pos2>) {
    let shape = shroud[index].shroud_layer.shape.clone().unwrap();
    let shape_id = shroud[index].shape_id.clone();
    let vertices = shroud[index].vertices.clone();
    let loaded_shape_index = loaded_shapes
        .0
        .iter()
        .position(|loaded_shape| loaded_shape.get_id().unwrap() == shape)
        .unwrap();

    let loaded_shape_mirror_index_l_option = loaded_shapes_mirror_pairs
        .iter()
        .position(|(l, _)| *l == loaded_shape_index);
    let (shape, shape_id, vertices) = if let Some(index) = loaded_shape_mirror_index_l_option {
        let r = loaded_shapes_mirror_pairs[index].1;
        let shape = loaded_shapes.0[r].get_id().unwrap();
        let vertices = restructure_vertices(loaded_shapes.0[r].get_first_scale_vertices());
        (shape.clone(), shape.to_string(), vertices)
    } else {
        let loaded_shape_mirror_index_r_option = loaded_shapes_mirror_pairs
            .iter()
            .position(|(_, r)| *r == loaded_shape_index);
        if let Some(index) = loaded_shape_mirror_index_r_option {
            let l = loaded_shapes_mirror_pairs[index].0;
            let shape = loaded_shapes.0[l].get_id().unwrap();
            let vertices = restructure_vertices(loaded_shapes.0[l].get_first_scale_vertices());
            (shape.clone(), shape.to_string(), vertices)
        } else {
            (shape, shape_id, vertices)
        }
    };
    (shape, shape_id, vertices)
}
