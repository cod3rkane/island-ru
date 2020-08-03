use nalgebra_glm::{ Vec2, vec3 };
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
    pub grid_pos: Vec2,
}

impl Tile {
    pub fn new(kind: TileType, physics: &mut Physics, pos: Vec2) -> Tile {
        physics.scale(vec3(0.2, 0.2, 0.0));
        Tile {
            kind,
            physics: *physics,
            grid_pos: pos,
        }
    }
}

