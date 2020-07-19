use gl;
extern crate nalgebra_glm as glm;

use crate::core::game_state::GameState;
use std::ffi::{ CStr };

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

pub fn render_system(game_state: &mut GameState) {
    // @TODO: we need to loop trough all buffers
    // @TODO: also, we need to get all entities and create a big chunk of gathered meshes.
    game_state.buffers.get(0).unwrap().bind();

    game_state.buffers.get(0).unwrap().vertices_vbo.bind();
    game_state.buffers.get(0).unwrap().vertices_vbo.set_data(
        (game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.vertices.as_ptr() as *const gl::types::GLvoid
    );
    game_state.buffers.get(0).unwrap().vertices_vbo.set_vertex_attr(0, 3, (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei);

    game_state.buffers.get(0).unwrap().colors_vbo.bind();
    game_state.buffers.get(0).unwrap().colors_vbo.set_data(
        (game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.colors.as_ptr() as *const gl::types::GLvoid,
    );
    game_state.buffers.get(0).unwrap().colors_vbo.set_vertex_attr(1, 4, (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei);

    game_state.buffers.get(0).unwrap().indices_vbo.bind();
    game_state.buffers.get(0).unwrap().indices_vbo.set_data(
        (game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.indices.as_ptr() as *const gl::types::GLvoid,
    );

    let mut _transform: glm::Mat4 = glm::mat4(
        1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
    );
    _transform = glm::translate(&mut _transform, &mut glm::vec3(0.5, -0.5, 0.0));
    _transform = glm::scale(&mut _transform, &mut glm::vec3(0.5, 0.5, 0.5));


    let _transform_loc: i32 =  unsafe { gl::GetUniformLocation(game_state.current_shader.program_id, c_str!("transform").as_ptr()) };

    unsafe {
        gl::UniformMatrix4fv(
            _transform_loc,
            1,
            gl::FALSE,
            _transform.as_ptr(),
        );

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::UseProgram(game_state.current_shader.program_id);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::Disable(gl::BLEND);
    }
}

pub fn render_system_clean(game_state: &mut GameState) {
    game_state.buffers.get(0).unwrap().vertices_vbo.clean();
    game_state.buffers.get(0).unwrap().colors_vbo.clean();
    game_state.buffers.get(0).unwrap().indices_vbo.clean();
    game_state.buffers.get(0).unwrap().clean();
}
