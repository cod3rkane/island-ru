use crate::components::entity::Entity;
use crate::components::shader::{ Shader };
use crate::core::buffers::*;
use crate::core::shader::{ create_shader };

use nalgebra_glm::{ vec3 };

pub struct GameState {
    pub entities: Vec<Entity>,
    pub buffers: Vec<Buffer>,
    pub current_shader: Shader,
}

pub fn initial_game_state() -> GameState {
    let initial_buffer: Buffer = Buffer::new();
    let current_shader: Shader = create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");

    GameState {
        entities: vec![Entity::new_square(vec3(0.5, -0.5, 0.0)), Entity::new_square(vec3(-0.8, -0.5, 0.0))],
        buffers: vec![initial_buffer],
        current_shader,
    }
}
