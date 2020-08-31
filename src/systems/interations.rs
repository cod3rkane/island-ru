use crate::components::tile::*;
use crate::core::game_state::GameState;
use pathfinding::prelude::{absdiff, bfs};

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
}
