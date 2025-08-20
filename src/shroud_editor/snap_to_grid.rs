use egui::{Pos2, pos2};

pub fn snap_to_grid(grid_size: f32, old_offset: Pos2) -> Pos2 {
    pos2(
        (old_offset.x / grid_size).round() * grid_size,
        (old_offset.y / grid_size).round() * grid_size,
    )
}
