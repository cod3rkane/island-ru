use nalgebra_glm::{ Vec3, vec3, Mat4, mat4 };
use crate::components::physics::{ Physics };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    GRASS,
    WATER,
    DIRT,
    SAND,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub kind: TileType,
    pub physics: Physics,
}

impl Tile {
    pub fn new(kind: TileType, physics: &mut Physics) -> Tile {
        physics.scale(vec3(0.2, 0.2, 0.0));
        Tile {
            kind,
            physics: *physics,
        }
    }
}

