use gl;
extern crate nalgebra_glm as glm;

use crate::core::game_state::GameState;
use crate::components::entity::{ Entity };
use std::ffi::{ CStr };

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

pub fn render_system(game_state: &mut GameState) {
    unsafe {
        gl::UseProgram(game_state.current_shader.program_id);
    }

    for buffer in &mut game_state.buffers {
        buffer.bind();

        // @TODO: Maybe we need to filter the chunks by type: e.g(3D entities, 2D entities)
        let (_vertices, _indices, _colors) = get_big_chunk_from_entities(&mut game_state.entities);
        
        buffer.vertices_vbo.bind();
        buffer.vertices_vbo.set_data(
            (_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            _vertices.as_ptr() as *const gl::types::GLvoid,
        );
        buffer.vertices_vbo.set_vertex_attr(0, 3, (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei);
    
        buffer.colors_vbo.bind();
        buffer.colors_vbo.set_data(
            (_colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            _colors.as_ptr() as *const gl::types::GLvoid,
        );
        buffer.colors_vbo.set_vertex_attr(1, 4, (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei);
    
        buffer.indices_vbo.bind();
        buffer.indices_vbo.set_data(
            (_indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            _indices.as_ptr() as *const gl::types::GLvoid,
        );
    
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    
            gl::DrawElements(gl::TRIANGLES, _indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
    
            gl::Disable(gl::BLEND);
        }
    }

    unsafe {
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }
}

fn get_big_chunk_from_entities(entities: &mut Vec<Entity>) -> (Vec<f32>, Vec<i32>, Vec<f32>) {
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<i32> = vec![];
    let mut colors: Vec<f32> = vec![];

    for e in entities {
        for v in e.mesh.vertices.clone() {
            vertices.push(v);
        }

        let indices_size = match indices.iter().max() {
            Some(max) => *max + 1,
            None => 0,
        };

        for i in e.mesh.indices.clone() {
            indices.push(i + indices_size);
        }

        for c in e.mesh.colors.clone() {
            colors.push(c);
        }
    }

    (vertices, indices, colors)
}

pub fn render_system_clean(game_state: &mut GameState) {
    game_state.buffers.get(0).unwrap().vertices_vbo.clean();
    game_state.buffers.get(0).unwrap().colors_vbo.clean();
    game_state.buffers.get(0).unwrap().indices_vbo.clean();
    game_state.buffers.get(0).unwrap().clean();
}
