use nalgebra_glm::{ Vec2, vec3 };
use crate::components::{ physics::Physics };

#[derive(Clone, Copy)]
pub enum TileType {
    GRASS,
    WATER,
    DIRT,
    SAND,
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileType,
    pub physics: Physics,
    pub grid_pos: Vec2,
    pub texture_coordinates: Vec<f32>,
}

impl Tile {
    pub fn new(kind: TileType, physics: &mut Physics, pos: Vec2) -> Tile {
        // @TODO: using TileType to get the coordinates from out coordinates handler
        physics.scale(vec3(0.2, 0.2, 0.0));

        Tile {
            kind,
            physics: *physics,
            grid_pos: pos,
            texture_coordinates: vec![
                0.0, 1.0,
                1.0, 1.0,
                0.0, 0.0,
                1.0, 0.0,
            ],
        }
    }
}

