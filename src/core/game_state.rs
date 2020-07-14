use crate::components::entity::Entity;

pub struct GameState {
    pub entities: Vec<Option<Entity>>,
}

pub fn initial_game_state() -> GameState {
    GameState {
        entities: vec![],
    }
}
