use crate::components::entity::Entity;
use crate::components::shader::{ Shader };
use crate::core::buffers::*;
use crate::core::shader::{ create_shader };

use nalgebra_glm::{ vec3, Mat4, mat4, translate };

pub struct GameState {
    pub entities: Vec<Entity>,
    pub buffers: Vec<Buffer>,
    pub current_shader: Shader,
    pub world_shader: Shader,
    pub buffer_data: BufferData, // @TODO: this guy disappear, it will be inside the Buffer now.
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub window_width: i32,
    pub window_height: i32,
    pub world: Option<Entity>,
}

pub fn initial_game_state() -> GameState {
    let _initial_buffer: Buffer = Buffer::new(BufferRenderType::DrawElements);
    let _world_buffer: Buffer = Buffer::new(BufferRenderType::DrawElementsInstanced);
    let current_shader: Shader = create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");
    let world_shader: Shader = create_shader("src/resources/vertex_world.glsl", "src/resources/fragment.glsl");
    let mut _triangle_a = Entity::new_square(vec3(4.0, 0.0, 0.0));
    _triangle_a.physics.as_mut().unwrap().scale(vec3(0.2, 0.2, 0.2));
    let mut _triangle_b = Entity::new_square(vec3(2.4, 0.0, 0.0));
    _triangle_b.physics.as_mut().unwrap().scale(vec3(0.2, 0.2, 0.2));
    let mut _view_matrix: Mat4 = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    _view_matrix = translate(&mut _view_matrix, &vec3(0.0, 0.0, -5.0));
    let mut _projection_matrix: Mat4 = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    GameState {
        entities: vec![_triangle_a, _triangle_b],
        buffers: vec![_world_buffer, _initial_buffer],
        current_shader,
        world_shader,
        buffer_data: BufferData::new(),
        view_matrix: _view_matrix,
        projection_matrix: _projection_matrix,
        window_width: 0,
        window_height: 0,
        world: Some(Entity::new_world(vec3(0.0, 0.0, 0.0))),
    }
}
