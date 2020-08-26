use nalgebra_glm::{ Vec2, vec3 };
use crate::components::{ physics::Physics };

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    GRASS = 12,
    SAND = 14,
    DIRT = 8,
    WATER = 13,
    WATER_1 = 10,
    WATER_2 = 11,
    WORKER_16x16 = 63,
    WARRIOR_16 = 62,
    SNOW = 122,
    TREE = 1,
    ROCK = 0,
    ROCK_1 = 2,
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileType,
    pub physics: Physics,
    pub grid_pos: Vec2,
    pub texture_coordinates: Vec<f32>,
}

impl Tile {
    pub fn new(kind: TileType, physics: &mut Physics, pos: Vec2, texture_coods: Vec<f32>) -> Tile {
        physics.scale(vec3(0.4, 0.4, 0.0));

        Tile {
            kind,
            physics: *physics,
            grid_pos: pos,
            texture_coordinates: texture_coods,
        }
    }
}

