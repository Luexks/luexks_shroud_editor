use std::{cmp::Ordering, sync::Arc};

use eframe::{
    egui_glow::{self, Painter},
    glow::{
        ARRAY_BUFFER, BLEND, FLOAT, FRAGMENT_SHADER, HasContext, LINES, NativeBuffer,
        NativeProgram, NativeVertexArray, ONE_MINUS_SRC_ALPHA, SRC_ALPHA, STATIC_DRAW, TRIANGLES,
        VERTEX_SHADER,
    },
};
use egui::{PaintCallback, Pos2, Rect, Rgba, Ui};
use itertools::Itertools;
use luexks_reassembly::{
    blocks::shroud_layer::ShroudLayer, utility::display_oriented_math::do3d_float_from,
};

use crate::{
    block_container::BlockContainer,
    selection_type::SelectionType,
    shroud_editor::{
        ShroudEditor,
        render_polygon::{polygon_fill_logic, polygon_line_logic},
    },
    shroud_layer_container::ShroudLayerContainer,
    size_from_verts::{do2d_size_from_verts, do2d_square_size_from_verts},
};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout(location = 0) in vec2 a_position;
layout(location = 1) in vec4 a_color;
out vec4 v_color;
void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_color = a_color;
}
"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
in vec4 v_color;
out vec4 FragColor;

void main() {
    FragColor = v_color;
}
"#;

pub struct RenderData {
    program: NativeProgram,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
}

impl RenderData {
    fn new(painter: &Painter) -> RenderData {
        let gl = painter.gl();
        unsafe {
            let vertex_shader = gl.create_shader(VERTEX_SHADER).unwrap();
            gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!(
                    "Vertex shader error:\n{}",
                    gl.get_shader_info_log(vertex_shader)
                );
            }

            let fragment_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment_shader, FRAGMENT_SHADER_SOURCE);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!(
                    "Fragment shader error:\n{}",
                    gl.get_shader_info_log(fragment_shader)
                );
            }

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("Program link error:\n{}", gl.get_program_info_log(program));
            }

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                2,
                FLOAT,
                false,
                std::mem::size_of::<RenderVertex>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                4,
                FLOAT,
                false,
                std::mem::size_of::<RenderVertex>() as i32,
                std::mem::size_of::<[f32; 2]>() as i32,
            );
            gl.enable(BLEND);
            gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            RenderData { program, vao, vbo }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod, Debug)]
pub struct RenderVertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl RenderVertex {
    pub fn _new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> RenderVertex {
        RenderVertex {
            position: [x, y],
            color: [r, g, b, a],
        }
    }
    pub fn from_screen_data(position: Pos2, color: Rgba, rect: Rect) -> RenderVertex {
        let x = -1.0 + (position.x - rect.left()) / rect.width() * 2.0;
        let y = 1.0 - (position.y - rect.top()) / rect.height() * 2.0;
        RenderVertex {
            position: [x, y],
            color: [color.r(), color.g(), color.b(), color.a()],
        }
    }
}

impl ShroudEditor {
    pub fn render_shroud(&mut self, mouse_pos: Option<Pos2>, ui: &mut Ui, rect: Rect) {
        let block_container = self.block_container.clone();
        let shroud = self.shroud.clone();
        let fill_color_gradient = self.fill_color_gradient;
        let pan = self.pan;
        let zoom = self.zoom;
        let render_data_option = self.render_data_option.clone();
        let shroud_that_would_be_selected_index_option = if let Some(mouse_pos) = mouse_pos {
            self.get_shroud_that_would_be_selected_index_option(mouse_pos, rect)
        } else {
            None
        };
        let selection = self.shroud_interaction.selection();
        let callback = PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                if let Ok(mut guard) = render_data_option.lock() {
                    let render_data = guard.get_or_insert_with(|| RenderData::new(painter));
                    render_shroud_body(
                        render_data,
                        &block_container,
                        &shroud,
                        painter,
                        rect,
                        fill_color_gradient,
                        pan,
                        zoom,
                        shroud_that_would_be_selected_index_option,
                        &selection,
                    );
                }
            })),
        };
        ui.painter().add(callback);
    }
}

fn render_shroud_body(
    render_data: &mut RenderData,
    block_container: &BlockContainer,
    shroud: &[ShroudLayerContainer],
    painter: &Painter,
    rect: Rect,
    fill_color_gradient: f32,
    pan: Pos2,
    zoom: f32,
    shroud_that_would_be_selected_index_option: Option<usize>,
    selection: &[usize],
) {
    let mut render_fill_vertices_buffer = Vec::<RenderVertex>::new();
    let mut render_outline_vertices_buffer = Vec::<RenderVertex>::new();
    let mut shroud_rendering_data = shroud.to_owned();
    if block_container.visible {
        let block_as_shroud_layer_container = get_block_as_shroud_layer_container(block_container);
        shroud_rendering_data.push(block_as_shroud_layer_container);
    }
    let shroud_rendering_data = shroud_rendering_data
        .iter()
        .enumerate()
        .sorted_by(
            |(_, shroud_layer_container_1), (_, shroud_layer_container_2)| {
                let z1 = shroud_layer_container_1
                    .shroud_layer
                    .offset
                    .as_ref()
                    .unwrap()
                    .z
                    .to_f32();
                let z2 = shroud_layer_container_2
                    .shroud_layer
                    .offset
                    .as_ref()
                    .unwrap()
                    .z
                    .to_f32();
                z1.partial_cmp(&z2).unwrap_or(Ordering::Equal)
            },
        )
        .collect::<Vec<_>>();

    if !shroud_rendering_data.is_empty() {
        let mut current_z = shroud_rendering_data[0]
            .1
            .shroud_layer
            .offset
            .as_ref()
            .unwrap()
            .z
            .to_f32();
        let mut next_outline_render_start_index = usize::default();
        unsafe {
            let gl = painter.gl();
            gl.use_program(Some(render_data.program));
            gl.bind_vertex_array(Some(render_data.vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(render_data.vbo));
        }
        shroud_rendering_data.iter().enumerate().for_each(
            |(pipeline_index, (index, shroud_layer_container))| {
                let offset = shroud_layer_container.shroud_layer.offset.as_ref().unwrap();

                let is_clipping_and_on_top = offset.z.to_f32() == current_z
                    && pipeline_index == shroud_rendering_data.len() - 1;
                if is_clipping_and_on_top {
                    polygon_fill_logic(
                        &mut render_fill_vertices_buffer,
                        block_container,
                        rect,
                        shroud_layer_container.get_shroud_layer_vertices(),
                        offset,
                        shroud_layer_container.shroud_layer.color_1.unwrap(),
                        shroud_layer_container.shroud_layer.color_2.unwrap(),
                        &shroud_layer_container.shape_id,
                        fill_color_gradient,
                        pan,
                        zoom,
                    );
                }

                let is_above_last = offset.z.to_f32() > current_z;
                let is_on_top = pipeline_index == shroud_rendering_data.len() - 1;
                if is_above_last || is_on_top {
                    shroud_rendering_data[next_outline_render_start_index..pipeline_index]
                        .iter()
                        .for_each(|(index, shroud_layer_container)| {
                            let is_hovered =
                                Some(*index) == shroud_that_would_be_selected_index_option;
                            let is_selected = selection.contains(index);
                            let selection_type_option = match (is_hovered, is_selected) {
                                (true, _) => Some(SelectionType::Hovered),
                                (false, true) => Some(SelectionType::Selected),
                                _ => None,
                            };
                            polygon_line_logic(
                                &mut render_outline_vertices_buffer,
                                block_container,
                                rect,
                                shroud_layer_container.get_shroud_layer_vertices(),
                                shroud_layer_container.shroud_layer.offset.as_ref().unwrap(),
                                shroud_layer_container.shroud_layer.line_color.unwrap(),
                                selection_type_option,
                                pan,
                                zoom,
                            );
                        });
                    next_outline_render_start_index = pipeline_index;
                }

                let is_not_clipping_and_on_top = offset.z.to_f32() > current_z
                    && pipeline_index == shroud_rendering_data.len() - 1;
                let is_below_top = pipeline_index != shroud_rendering_data.len() - 1;
                if is_not_clipping_and_on_top || is_below_top {
                    polygon_fill_logic(
                        &mut render_fill_vertices_buffer,
                        block_container,
                        rect,
                        shroud_layer_container.get_shroud_layer_vertices(),
                        offset,
                        shroud_layer_container.shroud_layer.color_1.unwrap(),
                        shroud_layer_container.shroud_layer.color_2.unwrap(),
                        &shroud_layer_container.shape_id,
                        fill_color_gradient,
                        pan,
                        zoom,
                    );
                }
                current_z = offset.z.to_f32();

                if is_on_top {
                    let is_hovered = Some(*index) == shroud_that_would_be_selected_index_option;
                    let is_selected = selection.contains(index);
                    let selection_type_option = match (is_hovered, is_selected) {
                        (true, _) => Some(SelectionType::Hovered),
                        (false, true) => Some(SelectionType::Selected),
                        _ => None,
                    };
                    polygon_line_logic(
                        &mut render_outline_vertices_buffer,
                        block_container,
                        rect,
                        shroud_layer_container.get_shroud_layer_vertices(),
                        shroud_layer_container.shroud_layer.offset.as_ref().unwrap(),
                        shroud_layer_container.shroud_layer.line_color.unwrap(),
                        selection_type_option,
                        pan,
                        zoom,
                    );
                }
                if is_above_last || is_on_top {
                    let gl = painter.gl();
                    render_fills(&render_fill_vertices_buffer, gl);
                    render_lines(&render_outline_vertices_buffer, gl);
                    render_fill_vertices_buffer.clear();
                    render_outline_vertices_buffer.clear();
                }
            },
        );
    }
}

fn render_lines(render_outline_vertices: &[RenderVertex], gl: &Arc<eframe::glow::Context>) {
    unsafe {
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(render_outline_vertices),
            STATIC_DRAW,
        );
        gl.draw_arrays(
            LINES,
            0,
            (render_outline_vertices.len()).try_into().unwrap(),
        );
    }
}

fn render_fills(render_fill_vertices: &[RenderVertex], gl: &Arc<eframe::glow::Context>) {
    unsafe {
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(render_fill_vertices),
            STATIC_DRAW,
        );
        gl.draw_arrays(
            TRIANGLES,
            0,
            (render_fill_vertices.len()).try_into().unwrap(),
        )
    };
}

fn get_block_as_shroud_layer_container(block_container: &BlockContainer) -> ShroudLayerContainer {
    ShroudLayerContainer {
        shroud_layer: if block_container.shape_id == "SQUARE" {
            ShroudLayer {
                size: Some(do2d_square_size_from_verts(&block_container.vertices)),
                offset: Some(do3d_float_from(-5.0, 0.0, 0.0)),
                ..Default::default()
            }
        } else {
            ShroudLayer {
                size: Some(do2d_size_from_verts(&block_container.vertices)),
                offset: Some(do3d_float_from(0.0, 0.0, 0.0)),
                ..Default::default()
            }
        },
        shape_id: block_container.shape_id.clone(),
        vertices: block_container.vertices.clone(),
        ..Default::default()
    }
}
