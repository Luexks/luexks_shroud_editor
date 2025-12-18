use egui::{Pos2, Rect};

use crate::shroud_editor::ShroudEditor;

impl ShroudEditor {
    pub fn world_pos_to_screen_pos(&self, position: Pos2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2 {
            x: center.x + (position.x + self.pan.x) * self.zoom,
            y: center.y + (position.y + self.pan.y) * self.zoom,
        }
    }

    pub fn screen_pos_to_world_pos(&self, position: Pos2, rect: Rect) -> Pos2 {
        let center = rect.center();
        Pos2 {
            x: (position.x - center.x) / self.zoom - self.pan.x,
            y: (position.y - center.y) / self.zoom - self.pan.y,
        }
    }

    pub fn positions_to_screen_positions(&self, positions: &[Pos2], rect: Rect) -> Vec<Pos2> {
        positions
            .iter()
            .map(|position| self.world_pos_to_screen_pos(*position, rect))
            .collect()
    }
}

pub fn world_pos_to_screen_pos(position: Pos2, rect: Rect, pan: Pos2, zoom: f32) -> Pos2 {
    let center = rect.center();
    Pos2 {
        x: center.x + (position.x + pan.x) * zoom,
        y: center.y + (position.y + pan.y) * zoom,
    }
}

pub fn screen_pos_to_world_pos(position: Pos2, rect: Rect, pan: Pos2, zoom: f32) -> Pos2 {
    let center = rect.center();
    Pos2 {
        x: (position.x - center.x) / zoom - pan.x,
        y: (position.y - center.y) / zoom - pan.y,
    }
}

pub fn positions_to_screen_positions(
    positions: &[Pos2],
    rect: Rect,
    pan: Pos2,
    zoom: f32,
) -> Vec<Pos2> {
    positions
        .iter()
        .map(|position| world_pos_to_screen_pos(*position, rect, pan, zoom))
        .collect()
}
