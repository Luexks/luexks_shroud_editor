use egui::{Color32, Painter, Pos2, Rect, Stroke};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer, shapes::shape::Shape, shapes::vertices::Vertices,
};

use crate::{
    shroud_editor::{ShroudEditor, TEST_SQUARE},
    shroud_layer_container::{self, ShroudLayerContainer},
};

// pub fn render_shroud_layer(painter: &Painter, ctx: &egui::Context, shroud_editor: &ShroudEditor, shroud_layer: &ShroudLayer, rect: Rect, stroke: Stroke, vertices: Vertices) {
pub fn render_shroud_layer(
    painter: &Painter,
    ctx: &egui::Context,
    shroud_editor: &ShroudEditor,
    shroud_layer_container: &ShroudLayerContainer,
    rect: Rect,
    stroke: Stroke,
) {
    // painter.circle(Pos2::new(50.0, 50.0), 16.0, Color32::from_rgb(50, 0, 0), Stroke::new(5.0, Color32::from_rgb(255, 0, 0)));
    let fill_color = Color32::from_rgba_premultiplied(30, 40, 80, 160);
    // let stroke = Stroke::new(1.0, Color32::RED);

    // let vertices = vec![Pos2::new(-5.0, -5.0), Pos2::new(-5.0, 5.0), Pos2::new(5.0, 5.0), Pos2::new(5.0, -5.0), Pos2::new(1000.0, 1000.0)];
    let offset = shroud_layer_container.shroud_layer.offset.clone().unwrap();
    // let vertices = vec![Pos2::new(-5.0, -5.0), Pos2::new(-5.0, 5.0), Pos2::new(5.0, 5.0), Pos2::new(5.0, -5.0)];
    // let vertices: Vec<Pos2> = TEST_SQUARE.into();
    let vertices = shroud_layer_container.vertices.clone();
    // let vertices = restructure_vertices(shroud_layer_container.vertices.clone());
    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y + offset.y.to_f32()))
        .collect();
    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| shroud_editor.position_to_screen_position(*vertex, rect))
        .collect();

    // painter.line(vertices, stroke);
    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .for_each(|(vertex_a, vertex_b)| {
            painter.line_segment([*vertex_a, *vertex_b], stroke);
        });
}
