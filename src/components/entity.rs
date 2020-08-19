use crate::components::{
    mesh::Mesh, physics::Physics, tile::Tile, tile::TileType, worker::Worker, world::*,
};
use crate::core::texture::Texture;
use nalgebra_glm::{vec2, vec3, Vec3};
extern crate noise;
use noise::{ utils::*, * };

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
        const CURRENT_SEED: u32 = 5;
        const CONTINENT_FREQUENCY: f64 = 1.00;
        const CONTINENT_LACUNARITY: f64 = 2.208984375;
        const SEA_LEVEL: f64 = 0.0;
        const RIVER_DEPTH: f64 = 0.0234375;
        let base_continent = Fbm::new()
            .set_seed(CURRENT_SEED)
            .set_frequency(CONTINENT_FREQUENCY)
            .set_persistence(0.5)
            .set_lacunarity(CONTINENT_LACUNARITY)
            .set_octaves(14);
        let perlin = Perlin::new();
        let clam = Clamp::new(&base_continent)
            .set_lower_bound(0.0)
            .set_upper_bound(1.0);
        let noise: NoiseMap = PlaneMapBuilder::new(&clam)
            .set_size(rows, columns)
            .set_x_bounds(0.0, 3.0)
            .set_y_bounds(0.0, 3.0)
            .build();

        noise.write_to_file("perlin.png");

        for i in 0..rows {
            for j in 0..columns {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                let n = noise.get_value(j, i);

                let mut tile_type: TileType = if n < 0.1 {
                    TileType::WATER
                } else if n < 0.2 {
                    TileType::SAND
                } else if n < 0.5 {
                    TileType::GRASS
                } else if n < 0.7 {
                    TileType::DIRT
                } else {
                    TileType::SNOW
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
