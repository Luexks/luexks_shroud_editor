use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, lerp, pos2};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayerColor, utility::display_oriented_math::DisplayOriented3D,
};

use crate::{selection_type::SelectionType, shroud_editor::ShroudEditor};

// pub fn render_polygon(
//     painter: &Painter,
//     shroud_editor: &ShroudEditor,
//     rect: Rect,
//     vertices: Vec<Pos2>,
//     offset: DisplayOriented3D,
//     color_1: ShroudLayerColor,
//     color_2: ShroudLayerColor,
//     line_color: ShroudLayerColor,
//     selection_type_option: Option<SelectionType>,
// ) {
//     let color_1 = shroud_editor.block_container.get_shroud_color(color_1);
//     let color_2 = shroud_editor.block_container.get_shroud_color(color_2);
//     let fill_color_gradient = shroud_editor.fill_color_gradient;
//     let fill_color = lerp(color_1..=color_2, fill_color_gradient);
//     let fill_stroke = Stroke::new(1.0, fill_color);
//     let line_stroke = Stroke::new(
//         1.0,
//         shroud_editor.block_container.get_shroud_color(line_color),
//     );

//     let vertices: Vec<Pos2> = vertices
//         .iter()
//         .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y + offset.y.to_f32()))
//         .collect();

//     let vertices: Vec<Pos2> = vertices
//         .iter()
//         .map(|vertex| shroud_editor.world_pos_to_screen_pos(*vertex, rect))
//         .collect();

//     (0..vertices.len())
//         .map(|_| vertices[0])
//         .zip(vertices.iter().cycle().skip(1))
//         .zip(vertices.iter().cycle().skip(2))
//         .for_each(|((vertex_a, vertex_b), vertex_c)| {
//             let points = vec![vertex_a, *vertex_b, *vertex_c];
//             painter.add(Shape::convex_polygon(points, fill_color, Stroke::NONE));
//         });
//     (0..vertices.len())
//         .map(|_| vertices[0])
//         .zip(vertices[1..vertices.len() - 1].iter())
//         .for_each(|(vertex_a, vertex_b)| {
//             painter.line_segment([vertex_a, *vertex_b], fill_stroke);
//         });

//     vertices
//         .iter()
//         .zip(vertices.iter().cycle().skip(1))
//         .for_each(|(vertex_a, vertex_b)| {
//             painter.line_segment([*vertex_a, *vertex_b], line_stroke);
//         });

//     if let Some(selection_type) = selection_type_option {
//         let selection_line_stroke = match selection_type {
//             SelectionType::Hovered => Stroke::new(1.0, Color32::from_rgb(0, 255, 0)),
//             SelectionType::Selected => Stroke::new(1.0, Color32::from_rgb(0, 255, 255)),
//         };
//         let avg_vert_pos = vertices.iter().fold(Pos2::default(), |pos, vertices| {
//             pos2(pos.x + vertices.x, pos.y + vertices.y)
//         }) / vertices.len() as f32;
//         let selection_vertices = vertices
//             .iter()
//             .map(|vertex| {
//                 let dx = vertex.x - avg_vert_pos.x;
//                 let dy = vertex.y - avg_vert_pos.y;
//                 let angle = dy.atan2(dx);
//                 let distance = (dx.powi(2) + dy.powi(2)).powf(0.5);
//                 let selection_distance = distance + 10.0;
//                 let selection_x = avg_vert_pos.x + selection_distance * angle.cos();
//                 let selection_y = avg_vert_pos.y + selection_distance * angle.sin();
//                 let seleciton_vertex = pos2(selection_x, selection_y);
//                 seleciton_vertex
//             })
//             .collect::<Vec<_>>();
//         selection_vertices
//             .iter()
//             .zip(selection_vertices.iter().cycle().skip(1))
//             .for_each(|(vertex_a, vertex_b)| {
//                 painter.line_segment([*vertex_a, *vertex_b], selection_line_stroke);
//             });
//     }
// }

pub fn render_polygon_fill(
    painter: &Painter,
    shroud_editor: &ShroudEditor,
    rect: Rect,
    vertices: Vec<Pos2>,
    offset: DisplayOriented3D,
    color_1: ShroudLayerColor,
    color_2: ShroudLayerColor,
    shape_id: &str,
) {
    let color_1 = shroud_editor.block_container.get_shroud_color(color_1);
    let color_2 = shroud_editor.block_container.get_shroud_color(color_2);
    let fill_color_gradient = shroud_editor.fill_color_gradient;
    let fill_color = lerp(color_1..=color_2, fill_color_gradient);
    let fill_stroke = Stroke::new(2.0, fill_color);

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y - offset.y.to_f32()))
        .collect();

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| shroud_editor.world_pos_to_screen_pos(*vertex, rect))
        .collect();

    match shape_id {
        "CANNON2" => {
            (0..=2)
                .map(|_| vertices[0])
                .zip(vertices.iter().cycle().skip(1))
                .zip(vertices.iter().cycle().skip(2))
                .for_each(|((vertex_a, vertex_b), vertex_c)| {
                    let area = ((vertex_a.x * (vertex_b.y - vertex_c.y)
                        + vertex_b.x * (vertex_c.y - vertex_a.y)
                        + vertex_c.x * (vertex_a.y - vertex_b.y))
                        / 2.0)
                        .abs();
                    if area > 0.1 {
                        let points = vec![vertex_a, *vertex_b, *vertex_c];
                        painter.add(Shape::convex_polygon(points, fill_color, Stroke::NONE));
                    }
                });
            (0..=2)
                .map(|_| vertices[4])
                .zip(vertices.iter().cycle().skip(5))
                .zip(vertices.iter().cycle().skip(6))
                .for_each(|((vertex_a, vertex_b), vertex_c)| {
                    let area = ((vertex_a.x * (vertex_b.y - vertex_c.y)
                        + vertex_b.x * (vertex_c.y - vertex_a.y)
                        + vertex_c.x * (vertex_a.y - vertex_b.y))
                        / 2.0)
                        .abs();
                    if area > 0.1 {
                        let points = vec![vertex_a, *vertex_b, *vertex_c];
                        painter.add(Shape::convex_polygon(points, fill_color, Stroke::NONE));
                    }
                });
            (0..4)
                .map(|_| vertices[0])
                .zip(vertices[1..vertices.len() - 1].iter())
                .for_each(|(vertex_a, vertex_b)| {
                    painter.line_segment([vertex_a, *vertex_b], fill_stroke);
                });
            (0..4)
                .map(|_| vertices[4])
                .zip(vertices.iter().cycle().skip(5))
                .for_each(|(vertex_a, vertex_b)| {
                    painter.line_segment([vertex_a, *vertex_b], fill_stroke);
                });
            // painter.circle(vertices[0], 5.0, Color32::RED, Stroke::new(5.0, Color32::BLUE));
            // painter.circle(vertices[4], 5.0, Color32::RED, Stroke::new(5.0, Color32::BLUE));
        }
        _ => {
            (0..vertices.len() - 2)
                .map(|_| vertices[0])
                .zip(vertices.iter().cycle().skip(1))
                .zip(vertices.iter().cycle().skip(2))
                .for_each(|((vertex_a, vertex_b), vertex_c)| {
                    let area = ((vertex_a.x * (vertex_b.y - vertex_c.y)
                        + vertex_b.x * (vertex_c.y - vertex_a.y)
                        + vertex_c.x * (vertex_a.y - vertex_b.y))
                        / 2.0)
                        .abs();
                    if area > 0.1 {
                        let points = vec![vertex_a, *vertex_b, *vertex_c];
                        painter.add(Shape::convex_polygon(points, fill_color, Stroke::NONE));
                    }
                });
            (0..vertices.len())
                .map(|_| vertices[0])
                .zip(vertices[1..vertices.len() - 1].iter())
                .for_each(|(vertex_a, vertex_b)| {
                    painter.line_segment([vertex_a, *vertex_b], fill_stroke);
                });
        }
    }
}

pub fn render_polygon_outline(
    painter: &Painter,
    shroud_editor: &ShroudEditor,
    rect: Rect,
    vertices: Vec<Pos2>,
    offset: DisplayOriented3D,
    line_color: ShroudLayerColor,
    selection_type_option: Option<SelectionType>,
) {
    let line_stroke = Stroke::new(
        1.0,
        shroud_editor.block_container.get_shroud_color(line_color),
    );

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y - offset.y.to_f32()))
        .collect();

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| shroud_editor.world_pos_to_screen_pos(*vertex, rect))
        .collect();

    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .for_each(|(vertex_a, vertex_b)| {
            painter.line_segment([*vertex_a, *vertex_b], line_stroke);
        });

    if let Some(selection_type) = selection_type_option {
        let selection_line_stroke = match selection_type {
            SelectionType::Hovered => Stroke::new(1.0, Color32::from_rgb(0, 255, 0)),
            SelectionType::Selected => Stroke::new(1.0, Color32::from_rgb(0, 255, 255)),
        };
        let avg_vert_pos = vertices.iter().fold(Pos2::default(), |pos, vertices| {
            pos2(pos.x + vertices.x, pos.y + vertices.y)
        }) / vertices.len() as f32;
        let selection_vertices = vertices
            .iter()
            .map(|vertex| {
                let dx = vertex.x - avg_vert_pos.x;
                let dy = vertex.y - avg_vert_pos.y;
                let angle = dy.atan2(dx);
                let distance = (dx.powi(2) + dy.powi(2)).powf(0.5);
                let selection_distance = distance + 10.0;
                let selection_x = avg_vert_pos.x + selection_distance * angle.cos();
                let selection_y = avg_vert_pos.y + selection_distance * angle.sin();
                pos2(selection_x, selection_y)
            })
            .collect::<Vec<_>>();
        selection_vertices
            .iter()
            .zip(selection_vertices.iter().cycle().skip(1))
            .for_each(|(vertex_a, vertex_b)| {
                painter.line_segment([*vertex_a, *vertex_b], selection_line_stroke);
            });
    }
}
