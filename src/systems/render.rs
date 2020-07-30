use gl;
extern crate nalgebra_glm as glm;

use crate::components::entity::Entity;
use crate::components::tile::*;
use crate::core::buffers::{Buffer, BufferRenderType};
use crate::core::game_state::GameState;
use std::ffi::{CStr, CString};

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub fn render_system(game_state: &mut GameState) {
    let aspect: f32 = (game_state.window_width / game_state.window_height) as f32;
    game_state.projection_matrix = glm::perspective(aspect, 45.0, 0.1, 100.0);

    for buffer in &mut game_state.buffers {
        match buffer.render_type {
            BufferRenderType::DRAW_ELEMENTS => {
                buffer.bind();

                buffer.vertices_vbo.bind();
                buffer.vertices_vbo.set_data(
                    (game_state.buffer_data.vertices.len() * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    game_state.buffer_data.vertices.as_ptr() as *const gl::types::GLvoid,
                );
                buffer.vertices_vbo.set_vertex_attr(
                    0,
                    3,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                );

                buffer.colors_vbo.bind();
                buffer.colors_vbo.set_data(
                    (game_state.buffer_data.colors.len() * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    game_state.buffer_data.colors.as_ptr() as *const gl::types::GLvoid,
                );
                buffer.colors_vbo.set_vertex_attr(
                    1,
                    4,
                    (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                );

                buffer.indices_vbo.bind();
                buffer.indices_vbo.set_data(
                    (game_state.buffer_data.indices.len()
                        * std::mem::size_of::<gl::types::GLfloat>())
                        as gl::types::GLsizeiptr,
                    game_state.buffer_data.indices.as_ptr() as *const gl::types::GLvoid,
                );

                unsafe {
                    gl::UseProgram(game_state.current_shader.program_id);
                    let view_id = gl::GetUniformLocation(
                        game_state.current_shader.program_id,
                        CString::new("view_matrix").expect("view_matrix").as_ptr(),
                    );
                    gl::UniformMatrix4fv(view_id, 1, gl::FALSE, game_state.view_matrix.as_ptr());
                    let projection_id = gl::GetUniformLocation(
                        game_state.current_shader.program_id,
                        CString::new("projection_matrix")
                            .expect("projection_matrix")
                            .as_ptr(),
                    );
                    gl::UniformMatrix4fv(
                        projection_id,
                        1,
                        gl::FALSE,
                        game_state.projection_matrix.as_ptr(),
                    );
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

                    gl::DrawElements(
                        gl::TRIANGLES,
                        game_state.buffer_data.indices.len() as i32,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );

                    gl::Disable(gl::BLEND);
                }
            }
            BufferRenderType::DRAW_ELEMENTS_INSTANCED => {
                if game_state.world.is_some() {
                    buffer.bind();

                    buffer.vertices_vbo.bind();
                    buffer.vertices_vbo.set_data(
                        (game_state.world.as_ref().unwrap().mesh.vertices.len()
                            * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        game_state.world.as_ref().unwrap().mesh.vertices.as_ptr()
                            as *const gl::types::GLvoid,
                    );
                    buffer.vertices_vbo.set_vertex_attr(
                        0,
                        3,
                        (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                    );

                    buffer.colors_vbo.bind();
                    buffer.colors_vbo.set_data(
                        (game_state.world.as_ref().unwrap().mesh.colors.len()
                            * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        game_state.world.as_ref().unwrap().mesh.colors.as_ptr()
                            as *const gl::types::GLvoid,
                    );
                    buffer.colors_vbo.set_vertex_attr(
                        1,
                        4,
                        (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                    );

                    buffer.indices_vbo.bind();
                    buffer.indices_vbo.set_data(
                        (game_state.world.as_ref().unwrap().mesh.indices.len()
                            * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        game_state.world.as_ref().unwrap().mesh.indices.as_ptr()
                            as *const gl::types::GLvoid,
                    );

                    let vec4_size = (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
                    let mat4_size = (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
                    buffer.transformations_vbo.unwrap().bind();
                    buffer.transformations_vbo.unwrap().set_data(
                        (game_state
                            .world
                            .as_ref()
                            .unwrap()
                            .tiles
                            .as_ref()
                            .unwrap()
                            .len()) as isize
                            * mat4_size as isize,
                        std::ptr::null(),
                    );

                    let items = game_state
                        .world
                        .as_ref()
                        .unwrap()
                        .tiles
                        .as_ref()
                        .unwrap()
                        .iter()
                        .enumerate();

                    for (i, tile) in items {
                        let offset = match i {
                            0 => 0 as isize,
                            _ => (i * mat4_size as usize) as isize,
                        };
                        buffer.transformations_vbo.unwrap().set_sub_data(
                            offset,
                            (mat4_size) as gl::types::GLsizeiptr,
                            (tile.physics.transform.as_ptr()) as *const gl::types::GLvoid,
                        );
                    }

                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        2,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        std::ptr::null(),
                    );
                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        3,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        (1 * vec4_size) as *const gl::types::GLvoid,
                    );
                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        4,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        (2 * vec4_size) as *const gl::types::GLvoid,
                    );
                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        5,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        (3 * vec4_size) as *const gl::types::GLvoid,
                    );

                    unsafe {
                        gl::VertexAttribDivisor(2, 1);
                        gl::VertexAttribDivisor(3, 1);
                        gl::VertexAttribDivisor(4, 1);
                        gl::VertexAttribDivisor(5, 1);
                    }

                    unsafe {
                        gl::UseProgram(game_state.world_shader.program_id);
                        let view_id = gl::GetUniformLocation(
                            game_state.world_shader.program_id,
                            CString::new("view_matrix").expect("view_matrix").as_ptr(),
                        );
                        gl::UniformMatrix4fv(
                            view_id,
                            1,
                            gl::FALSE,
                            game_state.view_matrix.as_ptr(),
                        );
                        let projection_id = gl::GetUniformLocation(
                            game_state.world_shader.program_id,
                            CString::new("projection_matrix")
                                .expect("projection_matrix")
                                .as_ptr(),
                        );
                        gl::UniformMatrix4fv(
                            projection_id,
                            1,
                            gl::FALSE,
                            game_state.projection_matrix.as_ptr(),
                        );
                        gl::Enable(gl::BLEND);
                        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

                        gl::DrawElementsInstanced(
                            gl::TRIANGLES,
                            game_state.world.as_ref().unwrap().mesh.indices.len() as i32,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                            game_state
                                .world
                                .as_ref()
                                .unwrap()
                                .tiles
                                .as_ref()
                                .unwrap()
                                .len() as i32,
                        );

                        gl::Disable(gl::BLEND);
                    }
                }
            }
            _ => {}
        }
    }

    unsafe {
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }

    game_state.buffer_data.vertices.clear();
    game_state.buffer_data.indices.clear();
    game_state.buffer_data.colors.clear();
}

pub fn render_system_clean(game_state: &mut GameState) {
    game_state.buffers.get(0).unwrap().vertices_vbo.clean();
    game_state.buffers.get(0).unwrap().colors_vbo.clean();
    game_state.buffers.get(0).unwrap().indices_vbo.clean();
    game_state.buffers.get(0).unwrap().clean();
}
