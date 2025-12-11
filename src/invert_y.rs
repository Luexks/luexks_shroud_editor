use egui::{Pos2, pos2};

pub const fn invert_y_of_pos2(position: Pos2) -> Pos2 {
    pos2(position.x, -position.y)
}
