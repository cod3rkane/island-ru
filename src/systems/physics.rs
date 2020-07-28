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

            item.t_vertices[index] = _position.x;
            index += 1;
            item.t_vertices[index] = _position.y;
            index += 1;
            item.t_vertices[index] = _position.z;
            index += 1;
        }

        game_state.buffer_data.vertices.extend(item.t_vertices.iter().cloned());
        let indices_size = match game_state.buffer_data.indices.iter().max() {
            Some(max) => *max + 1,
            None => 0,
        };
        for i in &item.mesh.indices {
            game_state.buffer_data.indices.push(*i + indices_size);
        }

        game_state.buffer_data.colors.extend(item.mesh.colors.iter().cloned());
    }
}
