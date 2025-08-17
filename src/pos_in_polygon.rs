use egui::Pos2;

pub fn is_pos_in_polygon(pos: Pos2, polygon: Vec<Pos2>) -> bool {
    polygon.iter().zip(polygon.iter().cycle().skip(1)).fold(
        false,
        |is_pos_in_polygon, (vertex_a, vertex_b)| {
            let intersects_band = (pos.y < vertex_a.y) != (pos.y < vertex_b.y);
            let on_left_side_of_band = pos.x
                < (vertex_a.x
                    + ((pos.y - vertex_a.y) / (vertex_b.y - vertex_a.y))
                        * (vertex_b.x - vertex_a.x));
            if intersects_band && on_left_side_of_band {
                !is_pos_in_polygon
                // dbg!(!is_pos_in_polygon)
            } else {
                is_pos_in_polygon
                // dbg!(is_pos_in_polygon)
            }
        },
    )
}
