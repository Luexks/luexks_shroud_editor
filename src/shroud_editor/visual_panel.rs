use egui::{Context, Frame};

use crate::{
    shroud_editor::{ShroudEditor, shroud_layer_moving::shroud_layer_moving},
    shroud_interaction::ShroudInteraction,
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

                if let ShroudInteraction::Placing { .. } = &self.shroud_interaction {
                } else {
                    self.shroud_layer_gizmos(ui, rect);
                }

                self.zoom(ui, rect);

                self.shroud_interaction_update(ui, ctx, &response, &rect);
                // Todo: fix drag stopped not working when mouse is on a gizmo.
                if response.drag_stopped() {
                    self.shroud_interaction = ShroudInteraction::Inaction {
                        selection: self.shroud_interaction.selection(),
                    };
                }

                if let ShroudInteraction::Dragging {
                    main_idx,
                    mut selection,
                } = self.shroud_interaction.clone()
                {
                    shroud_layer_moving(
                        ui,
                        &mut selection,
                        &mut self.shroud,
                        self.zoom,
                        self.grid_size,
                        self.grid_snap_enabled,
                    );
                    self.shroud_interaction = ShroudInteraction::Dragging {
                        main_idx,
                        selection,
                    }
                }
                if let ShroudInteraction::Placing {
                    main_idx,
                    mut selection,
                } = self.shroud_interaction.clone()
                {
                    shroud_layer_moving(
                        ui,
                        &mut selection,
                        &mut self.shroud,
                        self.zoom,
                        self.grid_size,
                        self.grid_snap_enabled,
                    );
                    self.shroud_interaction = ShroudInteraction::Dragging {
                        main_idx,
                        selection,
                    }
                }
            });
    }
}
