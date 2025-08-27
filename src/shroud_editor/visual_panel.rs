use egui::{Context, Frame};

use crate::{
    shroud_editor::{ShroudEditor, shroud_layer_dragging::shroud_layer_dragging},
    shroud_layer_interaction::ShroudLayerInteraction,
};

impl ShroudEditor {
    pub fn visual_panel(&mut self, ctx: &Context) {
        let central_panel_frame = Frame::new().inner_margin(0.0);

        egui::CentralPanel::default()
            .frame(central_panel_frame)
            .show(ctx, |ui| {
                let mouse_pos = ui.input(|i| i.pointer.hover_pos());
                let response =
                    ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect = response.rect;
                if let Some(mouse_pos) = mouse_pos {
                    self.world_mouse_pos = self.screen_pos_to_world_pos(mouse_pos, rect);
                }

                if self.grid_visible {
                    self.draw_grid(ui, rect);
                }

                self.render_shroud(mouse_pos, ui, rect);

                if let ShroudLayerInteraction::Placing { .. } = &self.shroud_layer_interaction {
                } else {
                    self.shroud_layer_gizmos(ui, rect);
                }

                self.zoom(ui, rect);

                self.shroud_layer_interaction_update(ui, ctx, &response, &rect);

                if let ShroudLayerInteraction::Dragging {
                    drag_start_pos: _,
                    selection,
                } = self.shroud_layer_interaction.clone()
                {
                    shroud_layer_dragging(
                        ui,
                        &response,
                        &selection,
                        &mut self.shroud,
                        self.zoom,
                        self.grid_size,
                        self.grid_snap_enabled,
                        &mut self.shroud_layer_interaction,
                    );
                }
                if let ShroudLayerInteraction::Placing { selection } =
                    self.shroud_layer_interaction.clone()
                {
                    shroud_layer_dragging(
                        ui,
                        &response,
                        &selection,
                        &mut self.shroud,
                        self.zoom,
                        self.grid_size,
                        self.grid_snap_enabled,
                        &mut self.shroud_layer_interaction,
                    );
                }
            });
    }
}
