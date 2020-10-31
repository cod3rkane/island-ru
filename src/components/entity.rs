use crate::components::{
    mesh::Mesh, physics::Physics, tile::*, tile::TileType, worker::Worker, world::*,
};
use crate::core::texture::Texture;
use nalgebra_glm::{vec2, vec3, Vec2, Vec3};
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

    pub fn new_worker(position: Vec3, texture: &Texture) -> Entity {
        let _triangle = Mesh {
            vertices: vec![
                1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, 1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
                0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let mut p = Physics::new(position);
        p.scale(vec3(0.5, 0.5, 0.0));

        Entity {
            physics: Some(p),
            mesh: _triangle,
            tiles: None,
            texture: Some(texture),
            worker: Some(Worker::new(
                texture.get_texture_coord_from_size(TileType::WORKER_16x16 as usize, 16),
            )),
        }
    }

    pub fn new_world(position: Vec3, texture: &Texture) -> Entity {
        let _square = Mesh {
            vertices: vec![
                1.0, 1.0, 0.0,
                1.0, -1.0, 0.0,
                -1.0, -1.0, 0.0,
                -1.0, 1.0, 0.0,
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            colors: vec![
                0.14902, 0.901961, 0.545098, 1.0,
            ],
        };
        let rows = 16;
        let columns = 16;
        let tile_width = 2.0;
        let tile_height = 1.0;
        let seed_id: i64 = 985;
        let mut tiles: Vec<Tile> = vec![];
        let mut tiles_items: Vec<Tile> = vec![];
        let map_noise = create_random_noise(columns, rows, seed_id, 2.28, 1.24);
        let tree_noise = create_random_noise(columns, rows, seed_id, 8.88, 1.24);
        let rocks_noise = create_random_noise(columns, rows, seed_id, 60.88, 1.8);

        for i in 0..rows {
            for j in (0..columns).rev() {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                // let screen_x = (x + y) * (tile_width / 2.0);
                // let screen_y = (x - y) * (tile_height / 2.0);
                // let screen_x = i as f32 * tile_width;
                // let screen_y = j as f32 * tile_height;
                let screen_x = (i as f32 * tile_width / 2.0) + (j as f32 * tile_width / 2.0);
                let screen_y = (j as f32 * tile_height / 2.0) - (i as f32 * tile_height / 2.0);
                let n: f64 = map_noise[j as usize * columns as usize + i as usize];

                let mut tile_type: TileType = if n < 0.4 {
                    TileType::WATER
                } else if n < 0.45 {
                    TileType::SAND
                } else if n < 0.8 {
                    TileType::GRASS
                } else if n < 0.91 {
                    TileType::DIRT
                } else if n < 0.98 {
                    TileType::SNOW
                } else {
                    TileType::GRASS
                };

                let mut tile: Tile = Tile::new(
                    tile_type,
                    &mut Physics::new(vec3(screen_x, screen_y, 0.0)),
                    GridPos(i as i32, j as i32),
                    texture.get_tile_coord(tile_type as usize),
                );

                if tile_type == TileType::WATER {
                    tile.physics.is_obstacle = true;
                }

                tiles.push(tile);
            }
        }

        for i in 0..rows {
            for j in (0..columns).rev() {
                let x = (j as f32) * tile_width;
                let y = (i as f32) * tile_height;
                let screen_x = (x + y) * (tile_width / 2.0);
                let screen_y = (x - y) * (tile_height / 2.0);
                let n: f64 = tree_noise[j as usize * columns as usize + i as usize];

                if n > 0.2 {
                    let tst: Option<&Tile> = tiles
                        .iter()
                        .find(|&t| t.grid_pos.0 == i as i32 && t.grid_pos.1 == j as i32);
                    let mut ps = Physics::new(vec3(screen_x, screen_y + 0.888, 0.0));
                    match tst.unwrap().kind {
                        TileType::GRASS => {
                            let grid_tile = tiles.iter_mut().find(|t| t.grid_pos == GridPos(i as i32, j as i32)).unwrap();
                            ps.is_obstacle = true;
                            grid_tile.physics.is_obstacle = true;
                            tiles_items.push(Tile::new(
                                TileType::TREE,
                                &mut ps,
                                GridPos(i as i32, j as i32),
                                texture.get_tile_coord(TileType::TREE as usize),
                            ));
                        }
                        _ => (),
                    }
                }

                let n_rocks: f64 = rocks_noise[j as usize * columns as usize + i as usize];
                if n_rocks > 0.4 {
                    let t: Option<&Tile> = tiles
                        .iter()
                        .find(|&t| t.grid_pos.0 == i as i32 && t.grid_pos.1 == j as i32);
                    let mut ps = Physics::new(vec3(screen_x + 0.22, screen_y + 0.12, 0.0));
                    ps.scale(vec3(0.6, 0.6, 0.0));
                    match t.unwrap().kind {
                        TileType::GRASS => {
                            let t_tree: Option<&Tile> = tiles_items.iter().find(|&t| {
                                t.kind == TileType::TREE
                                    && t.grid_pos.0 == i as i32
                                    && t.grid_pos.1 == j as i32
                            });
                            if t_tree.is_none() {
                                let grid_tile = tiles.iter_mut().find(|t| t.grid_pos == GridPos(i as i32, j as i32)).unwrap();
                                ps.is_obstacle = true;
                                grid_tile.physics.is_obstacle = true;
                                tiles_items.push(Tile::new(
                                    TileType::ROCK,
                                    &mut ps,
                                    GridPos(i as i32, j as i32),
                                    texture.get_tile_coord(TileType::ROCK_1 as usize),
                                ));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        let grid_tile = tiles.iter_mut().find(|t| t.grid_pos == GridPos(0, 0)).unwrap();
        let mut tile: Tile = Tile::new(
            TileType::SELECTED_32,
            &mut Physics::new(vec3(grid_tile.physics.position.x, grid_tile.physics.position.y, 0.0)),
            GridPos(0, 0),
            texture.get_tile_coord(TileType::SELECTED_32 as usize),
        );

        tiles.push(tile);

        let mut world_physics = Physics::new(position);
        tiles.append(&mut tiles_items);

        Entity {
            physics: Some(world_physics),
            mesh: _square,
            tiles: Some(tiles),
            texture: Some(texture),
            worker: None,
        }
    }
}
