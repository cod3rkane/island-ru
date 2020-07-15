use crate::components::entity::Entity;
use crate::components::shader::{ Shader };
use crate::core::buffers::*;
use crate::core::shader::{ create_shader };

pub struct GameState {
    pub entities: Vec<Option<Entity>>,
    pub buffers: Vec<Buffer>,
    pub current_shader: Shader,
}

pub fn initial_game_state() -> GameState {
    let initial_buffer: Buffer = Buffer::new();
    let current_shader: Shader = create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");

    GameState {
        entities: vec![Some(Entity::new_square())],
        buffers: vec![initial_buffer],
        current_shader,
    }
}
