use crate::components::{ physics::Physics, mesh::Mesh };

pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Option<Vec<Mesh>>,
}
