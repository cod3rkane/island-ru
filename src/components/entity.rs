extern crate image;
use image::GenericImage;
use image::GenericImageView;
use std::path::Path;
use crate::components::{
    mesh::Mesh, physics::Physics, tile::Tile, tile::TileType,
};
use crate::core::{ texture::Texture };
use nalgebra_glm::{vec2, vec3, Vec3};

#[derive(Clone)]
pub struct Entity {
    pub physics: Option<Physics>,
    pub mesh: Mesh,
    pub tiles: Option<Vec<Tile>>,
    pub texture: Option<Texture>,
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
        }
    }

    pub fn new_world(position: Vec3) -> Entity {
        let _square = Mesh {
            vertices: vec![
                1.0, 1.0, 0.0,
                1.0, -1.0, 0.0,
                -1.0, -1.0, 0.0,
                -1.0, 1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0, 0.14902,
                0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let rows = 12;
        let columns = 12;
        let tile_width = 0.41;
        let tile_height = 0.41;
        let mut tiles: Vec<Tile> = vec![];

        for i in 0..rows {
            for j in 0..columns {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                let mut tile_type: TileType = if i == 0 || i == 11 && j == 11 {
                    TileType::DIRT
                } else {
                    TileType::GRASS
                };
                tiles.push(Tile::new(
                    tile_type,
                    &mut Physics::new(vec3(x, y, 0.0)),
                    vec2(i as f32, j as f32),
                ));
            }
        }
        let mut world_physics = Physics::new(position);
        world_physics.rotate_z(-135.0);
        let grass_img = image::open(&Path::new("src/resources/textures/tiles-textures.png"))
            .expect("Failed to load texture!");
        let texture: Texture = Texture::new(Box::new(grass_img));

        Entity {
            physics: Some(world_physics),
            mesh: _square,
            tiles: Some(tiles),
            texture: Some(texture),
        }
    }
}
