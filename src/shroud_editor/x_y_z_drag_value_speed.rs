use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn get_xy_speed(&self) -> f32 {
        let xy_speed = if self.grid_snap_enabled {
            self.grid_size / 2.0
        } else {
            0.05
        };
        xy_speed
    }
}
