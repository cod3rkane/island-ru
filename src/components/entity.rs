use crate::components::{
    mesh::Mesh, physics::Physics, texture::Texture, tile::Tile, tile::TileType,
};
use nalgebra_glm::{vec2, vec3, Vec3};

#[derive(Debug, PartialEq)]
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
                -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0, 0.14902,
                0.901961, 0.545098, 1.0, 0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let rows = 12;
        let columns = 12;
        let tile_width = 0.404;
        let tile_height = 0.405;
        let mut tiles: Vec<Tile> = vec![];

        for i in 0..rows {
            for j in 0..columns {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                let tile_type: TileType = if i == 0 {
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

        Entity {
            physics: Some(world_physics),
            mesh: _square,
            tiles: Some(tiles),
            texture: None,
        }
    }
}
