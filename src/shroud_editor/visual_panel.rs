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

                if let ShroudInteraction::Inaction { .. } = &self.shroud_interaction {
                    self.shroud_layer_gizmos(ui, rect);
                }

                self.zoom(ui, rect);

                self.shroud_interaction_update(ui, ctx, &response, &rect);

                self.selection_release_logic(ctx, ui);

                self.dragging_logic(ui);
                self.placing_logic(ui);
            });
    }

    fn selection_release_logic(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if ui.ui_contains_pointer() && ctx.input(|i| i.pointer.primary_released()) {
            self.shroud_interaction = ShroudInteraction::Inaction {
                selection: self.shroud_interaction.selection(),
            };
        }
    }

    fn dragging_logic(&mut self, ui: &mut egui::Ui) {
        if let ShroudInteraction::Dragging {
            selection,
            drag_pos,
            potentially_snapped_drag_pos: potentially_snapped_drag_pos,
        } = &mut self.shroud_interaction
        {
            shroud_layer_moving(
                ui,
                drag_pos,
                potentially_snapped_drag_pos,
                selection,
                &mut self.shroud,
                self.zoom,
                self.grid_size,
                self.grid_snap_enabled,
            );
        }
    }

    fn placing_logic(&mut self, ui: &mut egui::Ui) {
        if let ShroudInteraction::Placing {
            selection,
            drag_pos,
            potentially_snapped_drag_pos: potentially_snapped_drag_pos,
        } = &mut self.shroud_interaction
        {
            shroud_layer_moving(
                ui,
                drag_pos,
                potentially_snapped_drag_pos,
                selection,
                &mut self.shroud,
                self.zoom,
                self.grid_size,
                self.grid_snap_enabled,
            );
        }
    }
}
