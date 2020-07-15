use crate::components::{ physics::Physics, mesh::Mesh };

pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Mesh,
}

impl Entity {
    pub fn newSquare() -> Entity {
        let _triangle = Mesh {
            vertices: vec![
                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
            ],
            indices: vec![
                0, 1, 2,
                0, 2, 3,
            ],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
            ],
        };

        Entity {
            physics: None,
            mesh: _triangle,
        }
    }
}
