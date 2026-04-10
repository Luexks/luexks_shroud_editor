use egui::Pos2;

pub fn is_pos_in_polygon(pos: Pos2, polygon: &[Pos2]) -> bool {
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
            } else {
                is_pos_in_polygon
            }
        },
    )
}

pub fn is_pos_on_perimeter(pos: Pos2, polygon: &[Pos2]) -> bool {
    const TOLERANCE: f32 = 1e-1;
    for (a, b) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        let a_b = a.distance(*b);
        let a_p = a.distance(pos);
        let p_b = pos.distance(*b);
        if (a_p + p_b - a_b).abs() < TOLERANCE {
            return true;
        }
    }
    false
}
