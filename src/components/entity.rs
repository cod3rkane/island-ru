use crate::components::{ physics::Physics, mesh::Mesh, tile::Tile, tile::TileType };
use nalgebra_glm::{ Vec3, vec3 };

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Mesh,
    pub t_vertices: Vec<f32>,
    pub tiles: Option<Vec<Tile>>,
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
            t_vertices: vec![
                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
            ],
            tiles: None,
        }
    }

    pub fn new_world(position: Vec3) -> Entity {
        let _square = Mesh {
            vertices: vec![
                -1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                1.0, 0.0, 0.0,
                0.0, -1.0, 0.0,
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
        let _tile: Tile = Tile::new(TileType::GRASS, &mut Physics::new(vec3(0.4, 0.0, 0.0)));
        let _tile2: Tile = Tile::new(TileType::GRASS, &mut Physics::new(vec3(0.0, 0.0, 0.0)));

        Entity {
            physics: Some(Physics::new(position)),
            mesh: _square,
            t_vertices: vec![
                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
            ],
            tiles: Some(vec![_tile, _tile2]),
        }
    }
}
