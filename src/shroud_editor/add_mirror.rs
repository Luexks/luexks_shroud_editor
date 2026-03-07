use egui::Pos2;
use luexks_reassembly::{
    shapes::{shape_id::ShapeId, shapes::Shapes},
    utility::{angle::Angle, display_oriented_math::do3d_float_from},
};

use crate::{
    restructure_vertices::restructure_vertices,
    right_tri_angle_edge_case::rotate_right_tri_shroud_layer_mirror,
    shroud_layer_container::ShroudLayerContainer,
};

pub fn add_mirror(
    shroud: &mut Vec<ShroudLayerContainer>,
    index: usize,
    _should_mirror_be_selected: bool,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
) {
    shroud[index].mirror_index_option = Some(shroud.len());

    let offset = shroud[index].shroud_layer.offset.clone().unwrap();
    let (shape, shape_id, vertices) =
        get_mirrored_shape_data(shroud, index, loaded_shapes, loaded_shapes_mirror_pairs);

    let is_right_tri = shape_id == "RIGHT_TRI";

    let mut shroud_layer_mirror = ShroudLayerContainer {
        mirror_index_option: Some(index),
        group_idx_option: None,
        shape_id,
        vertices,
        ..shroud[index].clone()
    };

    shroud_layer_mirror.shroud_layer.offset = Some(do3d_float_from(
        offset.x.to_f32(),
        -offset.y.to_f32(),
        offset.z.to_f32(),
    ));
    shroud_layer_mirror.shroud_layer.angle = Some(Angle::Degree(
        -shroud[index]
            .shroud_layer
            .angle
            .clone()
            .unwrap()
            .as_degrees()
            .get_value(),
    ));
    shroud_layer_mirror.shroud_layer.shape = Some(shape);

    if is_right_tri {
        rotate_right_tri_shroud_layer_mirror(&mut shroud_layer_mirror);
    }

    shroud.push(shroud_layer_mirror);
}

pub fn get_mirrored_shape_data(
    shroud: &[ShroudLayerContainer],
    index: usize,
    loaded_shapes: &Shapes,
    loaded_shapes_mirror_pairs: &[(usize, usize)],
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
