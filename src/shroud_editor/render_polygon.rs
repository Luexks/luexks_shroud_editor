use egui::{Pos2, Rect, Rgba, lerp, pos2};
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayerColor, utility::display_oriented_math::DisplayOriented3D,
};

use crate::{
    block_container::BlockContainer, position_conversion::world_pos_to_screen_pos,
    selection_type::SelectionType, shroud_editor::render_shroud::RenderVertex,
};

pub fn polygon_line_logic(
    render_outline_vertices_buffer: &mut Vec<RenderVertex>,
    block_container: &BlockContainer,
    rect: Rect,
    vertices: Vec<Pos2>,
    offset: &DisplayOriented3D,
    line_color: ShroudLayerColor,
    selection_type_option: Option<SelectionType>,
    pan: Pos2,
    zoom: f32,
) {
    let line_color = block_container.get_shroud_color(line_color);

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y - offset.y.to_f32()))
        .collect();

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| world_pos_to_screen_pos(*vertex, rect, pan, zoom))
        .collect();

    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .for_each(|(vertex_a, vertex_b)| {
            render_outline_vertices_buffer
                .push(RenderVertex::from_screen_data(*vertex_a, line_color, rect));
            render_outline_vertices_buffer
                .push(RenderVertex::from_screen_data(*vertex_b, line_color, rect));
        });

    if let Some(selection_type) = selection_type_option {
        let selection_line_color = match selection_type {
            SelectionType::Hovered => Rgba::from_rgb(0.0, 1.0, 0.0),
            SelectionType::Selected => Rgba::from_rgb(0.0, 1.0, 1.0),
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
                render_outline_vertices_buffer.push(RenderVertex::from_screen_data(
                    *vertex_a,
                    selection_line_color,
                    rect,
                ));
                render_outline_vertices_buffer.push(RenderVertex::from_screen_data(
                    *vertex_b,
                    selection_line_color,
                    rect,
                ));
            });
    }
}

pub fn polygon_fill_logic(
    render_fill_vertices_buffer: &mut Vec<RenderVertex>,
    block_container: &BlockContainer,
    rect: Rect,
    vertices: Vec<Pos2>,
    offset: &DisplayOriented3D,
    color_1: ShroudLayerColor,
    color_2: ShroudLayerColor,
    shape_id: &str,
    fill_color_gradient: f32,
    pan: Pos2,
    zoom: f32,
) {
    let color_1 = block_container.get_shroud_color(color_1);
    let color_2 = block_container.get_shroud_color(color_2);
    let fill_color = lerp(color_1..=color_2, fill_color_gradient);

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| Pos2::new(vertex.x + offset.x.to_f32(), vertex.y - offset.y.to_f32()))
        .collect();

    let vertices: Vec<Pos2> = vertices
        .iter()
        .map(|vertex| world_pos_to_screen_pos(*vertex, rect, pan, zoom))
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
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(vertex_a, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_b, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_c, fill_color, rect));
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
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(vertex_a, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_b, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_c, fill_color, rect));
                    }
                });
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
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(vertex_a, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_b, fill_color, rect));
                        render_fill_vertices_buffer
                            .push(RenderVertex::from_screen_data(*vertex_c, fill_color, rect));
                    }
                });
        }
    }
}
