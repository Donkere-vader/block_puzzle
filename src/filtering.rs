use std::collections::HashSet;

use crate::{Rotation, World, WORLD_HEIGHT, WORLD_WIDTH};

pub fn world_id(world: &World) -> String {
    let mut ids = Vec::new();

    for rotation in [
        Rotation::Zero,
        Rotation::Ninety,
        Rotation::OneEighty,
        Rotation::TwoSeventy,
    ] {
        let mut rotation_id = String::new();
        let mut mirror_id = String::new();
        let mut diagonal_id = String::new();

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let (transformed_x, transformed_y) = match rotation {
                    Rotation::Zero => (x, y),
                    Rotation::Ninety => (y, WORLD_HEIGHT - 1 - x),
                    Rotation::OneEighty => (WORLD_WIDTH - 1 - x, WORLD_HEIGHT - 1 - y),
                    Rotation::TwoSeventy => (WORLD_WIDTH - 1 - y, x),
                };

                rotation_id.push_str(&format!("{:0>2}", world[transformed_y][transformed_x]));
                mirror_id.push_str(&format!(
                    "{:0>2}",
                    world[WORLD_HEIGHT - 1 - transformed_y][transformed_x]
                ));
                diagonal_id.push_str(&format!("{:0>2}", world[transformed_x][transformed_y]));
            }
        }

        ids.push(rotation_id);
        ids.push(mirror_id);
        ids.push(diagonal_id);
    }

    ids.into_iter().min().unwrap()
}

pub fn filter_out_duplicates(solutions: &[World]) -> Vec<World> {
    let mut solution_ids = HashSet::new();
    let mut filtered_solutions = Vec::new();

    for solution in solutions.iter() {
        let solution_id = world_id(solution);
        if !solution_ids.contains(&solution_id) {
            solution_ids.insert(solution_id);
            filtered_solutions.push(*solution);
        }
    }

    filtered_solutions
}
