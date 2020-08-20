use crate::components::{
    mesh::Mesh, physics::Physics, tile::Tile, tile::TileType, worker::Worker, world::*,
};
use crate::core::texture::Texture;
use nalgebra_glm::{vec2, vec3, Vec3};
extern crate opensimplex;

#[derive(Clone)]
pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Mesh,
    pub tiles: Option<Vec<Tile>>,
    pub texture: Option<*const Texture>,
    pub worker: Option<Worker>,
}

impl Entity {
    pub fn new_square(position: Vec3) -> Entity {
        let _triangle = Mesh {
            vertices: vec![-0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 0.0, 0.0, 0.0, -0.5, 0.0],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0, 0.14902,
                0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0,
            ],
        };

        Entity {
            physics: Some(Physics::new(position)),
            mesh: _triangle,
            tiles: None,
            texture: None,
            worker: None,
        }
    }

    pub fn new_player(position: Vec3, texture: &Texture) -> Entity {
        let _triangle = Mesh {
            vertices: vec![
                0.2, 1.0, 0.0, 0.2, -2.0, 0.0, -1.0, -2.0, 0.0, -1.0, 1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0, 0.14902,
                0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0,
            ],
        };

        Entity {
            physics: Some(Physics::new(position)),
            mesh: _triangle,
            tiles: None,
            texture: Some(texture),
            worker: Some(Worker::new(texture.get_texture_coord_from_size(63, 16))),
        }
    }

    pub fn new_world(position: Vec3, texture: &Texture) -> Entity {
        let _square = Mesh {
            vertices: vec![
                1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, 1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0, 0.14902,
                0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let rows = 126;
        let columns = 126;
        let tile_width = 0.4;
        let tile_height = 0.4;
        let mut tiles: Vec<Tile> = vec![];
        let noise = opensimplex::OsnContext::new(8847).unwrap();
        const FREQUENCY_NOISE: f64 = 1.54;

        for i in 0..rows {
            for j in 0..columns {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                //let n = noise.get_value(j, i);
                let nx: f64 = j as f64 / columns as f64 - 0.7;
                let ny: f64 = i as f64 / rows as f64 - 0.5;
                let d: f64 = 2.0 * nx.abs().max (ny.abs());
                let e: f64 = noise.noise2(FREQUENCY_NOISE * nx, FREQUENCY_NOISE * ny) + noise.noise2(0.54 * nx, 0.54 * ny);
                let n = (1.0 + e - d.powf(4.0)) / 2.0;
                //let n = e.powf(0.59);

                let mut tile_type: TileType = if n < 0.4 {
                    TileType::WATER
                } else if n < 0.45 {
                    TileType::SAND
                } else if n < 1.0 {
                    TileType::GRASS
                } else if n < 0.72 {
                    TileType::DIRT
                } else if n < 0.74 {
                    TileType::SNOW
                } else {
                    TileType::GRASS
                };
                tiles.push(Tile::new(
                    tile_type,
                    &mut Physics::new(vec3(x, y, 0.0)),
                    vec2(i as f32, j as f32),
                    texture.get_tile_coord(tile_type as usize),
                ));
            }
        }

        let mut world_physics = Physics::new(position);
        world_physics.rotate_z(-135.0);

        Entity {
            physics: Some(world_physics),
            mesh: _square,
            tiles: Some(tiles),
            texture: Some(texture),
            worker: None,
        }
    }
}
