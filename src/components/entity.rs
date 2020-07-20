use crate::components::{ physics::Physics, mesh::Mesh };
use nalgebra_glm::{ Vec3 };

pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Mesh,
    pub vertices: Vec<f32>,
}

impl Entity {
    pub fn new_square(position: Vec3) -> Entity {
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
            physics: Some(Physics::new(position)),
            mesh: _triangle,
            vertices: vec![
                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
            ],
        }
    }
}
