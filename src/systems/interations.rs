use crate::components::tile::*;
use crate::core::game_state::GameState;
use pathfinding::prelude::{absdiff, astar};

fn neighbors(node: &GridPos, neighborhood: &Vec<GridPos>) -> Vec<(GridPos, u32)> {
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
    let mut result: Vec<(GridPos, u32)> = vec![];

    for dir in dirs {
        // 36x36 our MAP size @TODO use GRID_SIZE constant
        if dir.0 < 36 && dir.1 < 36 {
            let weight: u32 = match neighborhood.iter().find(|n| **n == dir).is_some() {
                true => 1,
                _ => 99999
            };

            result.push((dir, weight));
        }
    }

    result
}

pub fn interations_system(game_state: &mut GameState, delta_time: f32) {
    static GOAL: GridPos = GridPos(6, 7);
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

    let rs = astar(
        &GridPos(7, 0),
        |p| neighbors(p, &all_nodes),
        |p| p.distance(&GOAL) / 3,
        |p| *p == GOAL,
    );
    println!("here {:?}", rs);
}
