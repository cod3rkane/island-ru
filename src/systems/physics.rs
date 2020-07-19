use crate::core::game_state::GameState;
extern crate nalgebra_glm as glm;

pub fn physics_system(game_state: &mut GameState) {
    for item in &mut game_state.entities {
        if item.is_some() {
            let mut index: usize = 0;

            match item.as_mut() {
                Some(x) => {
                    loop {
                        let mut _position: glm::Vec4 = glm::vec4(0.0, 0.0, 0.0, 0.01);
                        if index == x.mesh.vertices.len() {
                            break;
                        }
        
                        _position = x.physics.as_ref().unwrap().transform * glm::vec4(
                            x.mesh.vertices[index],
                         x.mesh.vertices[index + 1],
                         x.mesh.vertices[index + 2],
                            0.01,
                        );
        
                        x.mesh.vertices[index] = _position.x;
                        index += 1;
                        x.mesh.vertices[index] = _position.y;
                        index += 1;
                        x.mesh.vertices[index] = _position.z;
                        index += 1;
                    }
                },
                _ => {},
            }
        }
    }

    // println!("here: {:?}", game_state.entities.get(0).unwrap().as_ref().unwrap().mesh.vertices);
}
