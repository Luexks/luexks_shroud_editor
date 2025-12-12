use luexks_reassembly::shapes::shapes::Shapes;

pub type MirrorPairs = Vec<(usize, usize)>;

#[rustfmt::skip]
pub fn get_loaded_shapes_mirror_pairs(loaded_shapes: &Shapes) -> MirrorPairs {
    let l1 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI2L").unwrap();
    let r1 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI2R").unwrap();
    let l2 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_22_5L").unwrap();
    let r2 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_22_5R").unwrap();
    let l3 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_30L").unwrap();
    let r3 = loaded_shapes.0.iter().position(|shape| shape.get_id().unwrap().get_name() == "RIGHT_TRI_30R").unwrap();
    let loaded_shapes_mirror_pairs = vec![
        (l1, r1),
        (l2, r2),
        (l3, r3),
    ];
    loaded_shapes_mirror_pairs
}
