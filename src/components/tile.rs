use crate::components::physics::Physics;
use nalgebra_glm::{vec3, Vec2};
use pathfinding::prelude::absdiff;

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
    SELECTED_32 = 9,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
pub struct GridPos(pub i32, pub i32);

impl GridPos {
    pub fn distance(&self, other: &GridPos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    pub fn successors(&self) -> Vec<(GridPos, u32)> {
        let &GridPos(x, y) = self;
        vec![
            GridPos(x + 1, y + 2),
            GridPos(x + 1, y - 2),
            GridPos(x - 1, y + 2),
            GridPos(x - 1, y - 2),
            GridPos(x + 2, y + 1),
            GridPos(x + 2, y - 1),
            GridPos(x - 2, y + 1),
            GridPos(x - 2, y - 1),
        ]
        .into_iter()
        .map(|p| (p, 1))
        .collect()
    }
}

#[derive(Clone, PartialEq)]
pub struct Tile {
    pub kind: TileType,
    pub physics: Physics,
    pub grid_pos: GridPos,
    pub texture_coordinates: Vec<f32>,
    pub is_selected: bool,
}

impl Tile {
    pub fn new(
        kind: TileType,
        physics: &mut Physics,
        pos: GridPos,
        texture_coods: Vec<f32>,
    ) -> Tile {
        physics.scale(vec3(0.4, 0.4, 0.0));

        Tile {
            kind,
            physics: *physics,
            grid_pos: pos,
            texture_coordinates: texture_coods,
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
}
