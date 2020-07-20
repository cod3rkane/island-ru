use crate::core::game_state::GameState;
extern crate nalgebra_glm as glm;

pub fn physics_system(game_state: &mut GameState, deltatime: f64) {
    for item in &mut game_state.entities {
        let mut index: usize = 0;
        let mut _position: glm::Vec4 = glm::vec4(0.0, 0.0, 0.0, 0.0);
        item.physics.as_mut().unwrap().rotate_z(0.01);

        loop {
            if index == item.mesh.vertices.len() {
                break;
            }

            _position = item.physics.as_ref().unwrap().transform * glm::vec4(
                item.mesh.vertices[index],
                item.mesh.vertices[index + 1],
                item.mesh.vertices[index + 2],
                0.01,
            );

            item.vertices[index] = _position.x;
            index += 1;
            item.vertices[index] = _position.y;
            index += 1;
            item.vertices[index] = _position.z;
            index += 1;
        }
    }
}
