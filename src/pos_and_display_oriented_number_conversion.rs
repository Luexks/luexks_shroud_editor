use egui::{Pos2, pos2};
use luexks_reassembly::utility::display_oriented_math::{
    DisplayOriented2D, DisplayOriented3D, do2d_float_from, do3d_float_from,
};

pub fn do2d_to_pos2(position: &DisplayOriented2D) -> Pos2 {
    pos2(position.x.to_f32(), position.y.to_f32())
}

pub fn do2d_to_pos2_invert_y(position: &DisplayOriented2D) -> Pos2 {
    pos2(position.x.to_f32(), -position.y.to_f32())
}

pub fn do3d_to_pos2(position: &DisplayOriented3D) -> Pos2 {
    pos2(position.x.to_f32(), position.y.to_f32())
}

pub fn do3d_to_pos2_invert_y(position: &DisplayOriented3D) -> Pos2 {
    pos2(position.x.to_f32(), -position.y.to_f32())
}

pub fn pos2_to_do2d(position: &Pos2) -> DisplayOriented2D {
    do2d_float_from(position.x, position.y)
}

pub fn pos2_to_do2d_invert_y(position: &Pos2) -> DisplayOriented2D {
    do2d_float_from(position.x, -position.y)
}

pub fn pos2_to_do3d(position: &Pos2, z: f32) -> DisplayOriented3D {
    do3d_float_from(position.x, position.y, z)
}

pub fn pos2_to_do3d_invert_y(position: &Pos2, z: f32) -> DisplayOriented3D {
    do3d_float_from(position.x, -position.y, z)
}
