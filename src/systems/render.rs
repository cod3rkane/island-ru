use gl;

use crate::core::game_state::GameState;

pub fn render_system(game_state: &mut GameState) {
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

    unsafe {
        gl::UseProgram(game_state.current_shader.program_id);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub fn render_system_clean(game_state: &mut GameState) {
    game_state.buffers.get(0).unwrap().vertices_vbo.clean();
    game_state.buffers.get(0).unwrap().colors_vbo.clean();
    game_state.buffers.get(0).unwrap().indices_vbo.clean();
    game_state.buffers.get(0).unwrap().clean();
}
