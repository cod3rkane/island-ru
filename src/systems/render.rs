use gl;
extern crate nalgebra_glm as glm;

use crate::components::tile::*;
use crate::core::buffers::{BufferRenderType};
use crate::core::game_state::GameState;
use std::ffi::{CString};

pub fn render_system(game_state: &mut GameState) {
    let aspect: f32 = (game_state.window_width / game_state.window_height) as f32;
    game_state.projection_matrix = glm::perspective(aspect, 45.0, 0.1, 100.0);

    for buffer in &mut game_state.buffers {
        match buffer.render_type {
            BufferRenderType::DrawElements => {
                buffer.bind();

                buffer.vertices_vbo.bind();
                // 12 equal the number of vertices used to draw a rect
                // @TODO: maybe that'll change in the near future
                buffer.vertices_vbo.set_data(
                    (game_state.entities.len() * 12 * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    std::ptr::null(),
                );
                buffer.vertices_vbo.set_vertex_attr(
                    0,
                    3,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                );

                buffer.colors_vbo.bind();
                // @TODO: texture coordinates comes here
                buffer.colors_vbo.set_data(
                    (game_state.entities.len() * 16 * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    std::ptr::null(),
                );
                buffer.colors_vbo.set_vertex_attr(
                    2,
                    4,
                    (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                );

                buffer.indices_vbo.bind();
                buffer.indices_vbo.set_data(
                    (game_state.entities.len() * 6 * std::mem::size_of::<f32>()) as isize,
                    std::ptr::null(),
                );

                let mut indices_num: isize = 0;
                let mut indice_max: isize = 0;
                for (i, e) in game_state.entities.iter().enumerate() {
                    buffer.vertices_vbo.bind();
                    let mut vertices_t: Vec<f32> = vec![];
                    let mut index: usize = 0;
                    let mut _position: glm::Vec4 = glm::vec4(0.0, 0.0, 0.0, 0.0);

                    loop {
                        if index == e.mesh.vertices.len() {
                            break;
                        }

                        _position = e.physics.as_ref().unwrap().transform * glm::vec4(
                            e.mesh.vertices[index],
                            e.mesh.vertices[index + 1],
                            e.mesh.vertices[index + 2],
                            1.0,
                        );

                        vertices_t.push(_position.x);
                        vertices_t.push(_position.y);
                        vertices_t.push(_position.z);

                        index += 3;
                    }

                    buffer.vertices_vbo.set_sub_data(
                        (i * e.mesh.vertices.len() * std::mem::size_of::<f32>()) as isize,
                        (e.mesh.vertices.len() * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        vertices_t.as_ptr() as *const gl::types::GLvoid,
                    );

                    buffer.colors_vbo.bind();
                    buffer.colors_vbo.set_sub_data(
                        (i * e.mesh.colors.len() * std::mem::size_of::<f32>()) as isize,
                        (e.mesh.colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        e.mesh.colors.as_ptr() as *const gl::types::GLvoid,
                    );

                    let mut indices: Vec<i32> = vec![];

                    buffer.indices_vbo.bind();
                    for indice in e.mesh.indices.iter() {
                        let indice_index: i32 = match i {
                            0 => *indice,
                            _ => *indice + indice_max as i32,
                        };

                        indices.push(indice_index);
                    }
                    let test: Vec<i32> = match i {
                        0 => vec![0, 1, 2, 0, 2, 3],
                        _ => vec![4, 5, 6, 4, 6, 7],
                    };

                    buffer.indices_vbo.set_sub_data(
                        (i * e.mesh.indices.len() * std::mem::size_of::<f32>()) as isize,
                        (e.mesh.indices.len() * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        test.as_ptr() as *const gl::types::GLvoid,
                    );

                    indices_num = e.mesh.indices.len() as isize + indices_num;
                    indice_max = *e.mesh.indices.iter().max().unwrap() as isize + 1;
                }

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
                        indices_num as i32,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );

                    gl::Disable(gl::BLEND);
                }
            }
            BufferRenderType::DrawElementsInstanced => {
                if game_state.world.is_some() {
                    buffer.bind();

                    buffer.vertices_vbo.bind();
                    buffer.vertices_vbo.set_data(
                        (game_state.world.as_ref().unwrap().mesh.vertices.len() * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        game_state.world.as_ref().unwrap().mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                    );
                    buffer.vertices_vbo.set_vertex_attr(
                        0,
                        3,
                        (0 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                    );

                    buffer.colors_vbo.bind();
                    const VEC4_SIZE: i32 = (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
                    let tiles_len = game_state
                        .world
                        .as_ref()
                        .unwrap()
                        .tiles
                        .as_ref()
                        .unwrap()
                        .len();
                    buffer.colors_vbo.set_vertex_attr_pointer(
                        1,
                        4,
                        (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        std::ptr::null(),
                    );
                    buffer.colors_vbo.set_data(
                        ((tiles_len * 4) * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        std::ptr::null(),
                    );

                    buffer.textures_vbo.unwrap().bind();
                    buffer.textures_vbo.unwrap().set_vertex_attr_pointer(
                        6,
                        2,
                        (2 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        std::ptr::null(),
                    );
                    buffer.textures_vbo.unwrap().set_data(
                        ((tiles_len * 8) * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        std::ptr::null(),
                    );

                    for (i, tile) in game_state
                        .world
                        .as_ref()
                        .unwrap()
                        .tiles
                        .as_ref()
                        .unwrap()
                        .iter()
                        .enumerate()
                    {
                        let offset = match i {
                            0 => 0 as isize,
                            _ => (i * VEC4_SIZE as usize) as isize,
                        };
                        let color: Vec<f32> = match tile.kind {
                            TileType::DIRT => vec![0.54902, 0.290196, 0.168627, 1.0],
                            TileType::GRASS => vec![0.25098, 0.909804, 0.411765, 1.0],
                            _ => vec![1.0, 0.121569, 0.152941, 1.0],
                        };

                        buffer.colors_vbo.bind();
                        buffer.colors_vbo.set_sub_data(
                            offset,
                            VEC4_SIZE as gl::types::GLsizeiptr,
                            (color.as_ptr()) as *const gl::types::GLvoid,
                        );
                        buffer.textures_tbo.unwrap().bind(0);
                        buffer.textures_vbo.unwrap().bind();
                        buffer.textures_vbo.unwrap().set_sub_data(
                            (i * tile.texture_coordinates.len() * std::mem::size_of::<f32>() as usize) as isize,
                            (tile.texture_coordinates.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                            (tile.texture_coordinates.as_ptr()) as *const gl::types::GLvoid,
                        );
                    }
                    buffer.textures_tbo.unwrap().attach_buffer(buffer.textures_vbo.unwrap().id);

                    buffer.indices_vbo.bind();
                    buffer.indices_vbo.set_data(
                        (game_state.world.as_ref().unwrap().mesh.indices.len()
                            * std::mem::size_of::<f32>())
                            as gl::types::GLsizeiptr,
                        game_state.world.as_ref().unwrap().mesh.indices.as_ptr()
                            as *const gl::types::GLvoid,
                    );

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
                        (1 * VEC4_SIZE) as *const gl::types::GLvoid,
                    );
                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        4,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        (2 * VEC4_SIZE) as *const gl::types::GLvoid,
                    );
                    buffer.transformations_vbo.unwrap().set_vertex_attr_pointer(
                        5,
                        4,
                        (16 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                        (3 * VEC4_SIZE) as *const gl::types::GLvoid,
                    );

                    unsafe {
                        gl::VertexAttribDivisor(1, 1);
                        gl::VertexAttribDivisor(2, 1);
                        gl::VertexAttribDivisor(3, 1);
                        gl::VertexAttribDivisor(4, 1);
                        gl::VertexAttribDivisor(5, 1);
                    }

                    let view_matrix_t: glm::Mat4 = game_state.view_matrix
                        * game_state
                            .world
                            .as_ref()
                            .unwrap()
                            .physics
                            .unwrap()
                            .transform;

                    unsafe {
                        gl::UseProgram(game_state.world_shader.program_id);
                        let view_id = gl::GetUniformLocation(
                            game_state.world_shader.program_id,
                            CString::new("view_matrix").expect("view_matrix").as_ptr(),
                        );
                        gl::UniformMatrix4fv(view_id, 1, gl::FALSE, view_matrix_t.as_ptr());
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

                        let texture_tbo_loc = gl::GetUniformLocation(game_state.world_shader.program_id, CString::new("texture_tbo").expect("texture_tbo").as_ptr());
                        buffer.textures_tbo.unwrap().bind(0);
                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::Uniform1i(texture_tbo_loc, 0);

                        gl::ActiveTexture(gl::TEXTURE1);
                        gl::BindTexture(gl::TEXTURE_2D, game_state.world.as_ref().unwrap().texture.as_ref().unwrap().id);
                        gl::Uniform1i(gl::GetUniformLocation(game_state.world_shader.program_id, CString::new("texture1").expect("texture1").as_ptr()), 1);

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
                        gl::BindTexture(gl::TEXTURE_2D, 0);
                    }
                }
            },
        }
    }

    unsafe {
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }
}

pub fn render_system_clean(game_state: &mut GameState) {
    for buffer in &mut game_state.buffers {
        buffer.vertices_vbo.clean();
        buffer.colors_vbo.clean();
        buffer.indices_vbo.clean();

        buffer.clean();
        buffer.unbind();
    }
}
