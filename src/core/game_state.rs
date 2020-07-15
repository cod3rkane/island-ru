use crate::components::entity::Entity;
use crate::core::buffers::*;

pub struct GameState {
    pub entities: Vec<Option<Entity>>,
    pub buffers: Vec<Buffer>,
}

pub fn initial_game_state() -> GameState {
    let mut initial_buffer: Buffer = Buffer::new();

    GameState {
        entities: vec![],
        buffers: vec![initial_buffer],
    }
}
