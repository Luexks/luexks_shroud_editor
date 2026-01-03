use egui::{Color32, Rect, Stroke, Ui, pos2};

use crate::shroud_editor::{MIN_GRID_LINE_DIST, ShroudEditor};

impl ShroudEditor {
    pub fn draw_grid(&self, ui: &mut Ui, rect: Rect) {
        let stroke = Stroke::new(1.0, Color32::from_rgb(0, 0, 150));
        let axis_stroke = Stroke::new(1.0, Color32::from_rgb(255, 0, 255));

        let mut grid_size = self.grid_size;
        while grid_size * self.zoom <= MIN_GRID_LINE_DIST {
            grid_size *= 2.0;
        }

        let world_min = self.screen_pos_to_world_pos(rect.min, rect);
        let world_max = self.screen_pos_to_world_pos(rect.max, rect);
        let first_vertical_line_x = (world_min.x / grid_size).ceil() * grid_size;
        let first_horizontal_line_y = (world_min.y / grid_size).ceil() * grid_size;
        let vertical_grid_line_count = ((-world_min.x + world_max.x) / grid_size).ceil() as usize;
        let horizontal_grid_line_count = ((-world_min.y + world_max.y) / grid_size).ceil() as usize;

        let mut y_axis_x_option = None;
        let mut x_axis_y_option = None;

        let y_top = world_min.y;
        let y_bottom = world_max.y;
        let x_left = world_min.x;
        let x_right = world_max.x;

        (0..vertical_grid_line_count).for_each(|index| {
            let x = first_vertical_line_x + grid_size * index as f32;
            let pos_top = self.world_pos_to_screen_pos(pos2(x, y_top), rect);
            let pos_bottom = self.world_pos_to_screen_pos(pos2(x, y_bottom), rect);
            if x.abs() < f32::EPSILON {
                y_axis_x_option = Some(x);
            } else {
                ui.painter().line_segment([pos_top, pos_bottom], stroke);
            }
        });

        (0..horizontal_grid_line_count).for_each(|index| {
            let y = first_horizontal_line_y + grid_size * index as f32;
            let pos_left = self.world_pos_to_screen_pos(pos2(x_left, y), rect);
            let pos_right = self.world_pos_to_screen_pos(pos2(x_right, y), rect);
            if y.abs() < f32::EPSILON {
                x_axis_y_option = Some(y)
            } else {
                ui.painter().line_segment([pos_left, pos_right], stroke);
            }
        });

        if let Some(y_axis_x) = y_axis_x_option {
            let pos_top = self.world_pos_to_screen_pos(pos2(y_axis_x, y_top), rect);
            let pos_bottom = self.world_pos_to_screen_pos(pos2(y_axis_x, y_bottom), rect);
            ui.painter()
                .line_segment([pos_top, pos_bottom], axis_stroke);
        }
        if let Some(x_axis_y) = x_axis_y_option {
            let pos_left = self.world_pos_to_screen_pos(pos2(x_left, x_axis_y), rect);
            let pos_right = self.world_pos_to_screen_pos(pos2(x_right, x_axis_y), rect);
            ui.painter()
                .line_segment([pos_left, pos_right], axis_stroke);
        }
        ui.painter().line_segment(
            [
                self.world_pos_to_screen_pos(pos2(50.0, 0.0), rect),
                self.world_pos_to_screen_pos(pos2(40.0, 10.0), rect),
            ],
            axis_stroke,
        );
        ui.painter().line_segment(
            [
                self.world_pos_to_screen_pos(pos2(50.0, 0.0), rect),
                self.world_pos_to_screen_pos(pos2(40.0, -10.0), rect),
            ],
            axis_stroke,
        );
    }
}
