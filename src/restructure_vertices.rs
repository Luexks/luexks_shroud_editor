use egui::Pos2;
use luexks_reassembly::shapes::vertices::Vertices;

pub fn restructure_vertices(vertices: Vertices) -> Vec<Pos2> {
    vertices
        .0
        .iter()
        .map(|vertex| Pos2::new(vertex.0.x.to_f32(), vertex.0.y.to_f32()))
        .collect()
}
