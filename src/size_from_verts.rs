use egui::Pos2;
use luexks_reassembly::utility::display_oriented_math::{DisplayOriented2D, do2d_float_from};

pub fn do2d_size_from_verts(verts: &[Pos2]) -> DisplayOriented2D {
    let (min_x, max_x, min_y, max_y) = verts.iter().fold(
        (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
        |(min_x, max_x, min_y, max_y), vert| {
            (
                f32::min(min_x, vert.x),
                f32::max(max_x, vert.x),
                f32::min(min_y, vert.y),
                f32::max(max_y, vert.y),
            )
        },
    );
    do2d_float_from(-min_x + max_x, -min_y + max_y)
}

pub fn do2d_square_size_from_verts(verts: &[Pos2]) -> DisplayOriented2D {
    let (min_x, max_x, min_y, max_y) = verts.iter().fold(
        (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
        |(min_x, max_x, min_y, max_y), vert| {
            (
                f32::min(min_x, vert.x),
                f32::max(max_x, vert.x),
                f32::min(min_y, vert.y),
                f32::max(max_y, vert.y),
            )
        },
    );
    do2d_float_from(-min_x + max_x, 0.5 * (-min_y + max_y))
}
