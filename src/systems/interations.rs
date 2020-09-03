use crate::components::tile::*;
use crate::core::game_state::GameState;
use nalgebra_glm::{vec4, Vec4, Mat4, inverse, translate, vec3, Vec3};
use pathfinding::prelude::{absdiff, bfs};

extern crate nalgebra_glm as glm;

fn neighbors(node: &GridPos, neighborhood: &Vec<GridPos>) -> Vec<GridPos> {
    let &GridPos(x, y) = node;
    let dirs: Vec<GridPos> = vec![
        GridPos(x + 1, y + 2),
        GridPos(x + 1, y - 2),
        GridPos(x - 1, y + 2),
        GridPos(x - 1, y - 2),
        GridPos(x + 2, y + 1),
        GridPos(x + 2, y - 1),
        GridPos(x - 2, y + 1),
        GridPos(x - 2, y - 1),
    ];
    let mut result: Vec<GridPos> = vec![];

    for dir in dirs {
        // 36x36 our MAP size @TODO use GRID_SIZE constant
        if dir.0 < 36 && dir.1 < 36 {
            if neighborhood.iter().find(|n| **n == dir).is_some() {
                result.push(dir);
            }
        }
    }

    result
}

pub fn interations_system(game_state: &mut GameState, delta_time: f32) {
    static GOAL: GridPos = GridPos(6, 12);
    let all_nodes: Vec<GridPos> = game_state
        .world
        .as_ref()
        .unwrap()
        .tiles
        .as_ref()
        .unwrap()
        .iter()
        .filter(|t| !t.physics.is_obstacle)
        .map(|t| t.grid_pos)
        .collect();

    let rs = bfs(&GridPos(7, 0), |p| neighbors(p, &all_nodes), |p| *p == GOAL);
    //println!("here {:?}", rs);
    let tile_width = 0.7 / 2.0;
    let tile_height = 0.75 / 2.0;

    let grid_tile: &Tile = game_state
        .world
        .as_ref()
        .unwrap()
        .tiles
        .as_ref()
        .unwrap()
        .iter()
        .find(|t| t.grid_pos == GridPos(0, 34))
        .unwrap();
    let p = grid_tile.physics.position;

    let aspect: f32 = (game_state.window_width / game_state.window_height) as f32;
    let projection_matrix = glm::perspective(aspect, 45.0, 0.1, 100.0);
    let view_matrix = game_state.view_matrix;

    // Normalized Device Coords
    let x = (2.0 * (game_state.mouse_pos.x / game_state.viewport_width as f32)) - 1.0;
    let y = 1.0 - (2.0 * (game_state.mouse_pos.y / game_state.viewport_height as f32));

    // Clip Space
    let clip_coords: Vec4 = vec4(x, y, -1.0, 1.0);

    // eye Space
    let inverted_projection_matrix: Mat4 = inverse(&projection_matrix);
    let eye_coords: Vec4 = inverted_projection_matrix * clip_coords;

    // world space
    let inverted_view_matrix: Mat4 = inverse(&view_matrix);
    let ray_world: Vec4 = inverted_view_matrix * eye_coords;
    let mouse_pos: Vec3 = vec3(ray_world.x, ray_world.y, ray_world.z);

    // Grid Pos
    let grid_transform = game_state.world.as_ref().unwrap().physics.unwrap().transform;
    let t: Vec4 = projection_matrix * vec4(p.x, p.y, p.z, 1.0);
    let t1: Vec4 = view_matrix * grid_transform * t;
    let grid_pos: Vec3 = vec3(t1.x, t1.y, t1.z);

    println!("TILE: {:?} Mouse: {:?}", grid_pos, mouse_pos);
}
