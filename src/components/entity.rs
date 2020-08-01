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
                -1.0, 1.0, 0.0,
                1.0, 1.0, 0.0,
                -1.0, -1.0, 0.0,
                1.0, -1.0, 0.0,
            ],
            indices: vec![
                0, 1, 2,
                1, 2, 3,
            ],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let rows = 7;
        let columns = 6;
        let tile_width = 0.404;
        let tile_height = 0.405;
        let mut tiles: Vec<Tile> = vec![];

        for i in 0..rows {
            for j in 0..columns {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                // tile_type = level_data[i][j];

                tiles.push(Tile::new(TileType::GRASS, &mut Physics::new(vec3(x, y, 0.0))));
            }
        }

        Entity {
            physics: Some(Physics::new(position)),
            mesh: _square,
            t_vertices: vec![
                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
            ],
            tiles: Some(tiles),
        }
    }
}
