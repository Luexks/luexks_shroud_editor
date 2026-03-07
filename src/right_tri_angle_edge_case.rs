use crate::shroud_layer_container::ShroudLayerContainer;

pub const RIGHT_TRI: &str = "RIGHT_TRI";

pub fn rotate_right_tri_shroud_layer_mirror(shroud_layer_mirror: &mut ShroudLayerContainer) {
    let angle = shroud_layer_mirror
        .shroud_layer
        .angle
        .as_mut()
        .unwrap()
        .get_value_mut();
    *angle -= 90.0;
    *angle %= 360.0;
}
