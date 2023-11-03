use crate::{
    game::Game,
    validators::{valid_pos, Valid},
    Placement, Position, World,
};

pub fn next_possible_placements(game: &Game) -> Vec<Placement> {
    let cursor = match cursor_position(&game.world) {
        Some(c) => c,
        None => return Vec::new(),
    };

    let mut placements = Vec::new();

    for block in game.inventory.iter() {
        for (rotation, geometry) in block.get_geometries().iter().enumerate() {
            let mut possible = true;
            for point in geometry.points.iter() {
                let new_point = ((cursor.0 as i32 + point.0), (cursor.1 as i32 + point.1));

                if valid_pos(new_point) != Valid::Valid
                    || game.world[new_point.1 as usize][new_point.0 as usize] != 0
                {
                    possible = false;
                    break;
                }
            }

            if possible {
                placements.push(Placement {
                    block: *block,
                    rotation,
                    anchor_pos: cursor,
                })
            }
        }
    }

    placements
}

fn cursor_position(world: &World) -> Option<Position> {
    for (y, row) in world.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                return Some((x, y));
            }
        }
    }

    None
}

fn world_filled(world: &World) -> bool {
    cursor_position(world).is_none()
}

pub fn add_possibilities(game: &mut Game, solved_worlds: &mut Vec<World>) {
    if world_filled(&game.world) {
        solved_worlds.push(game.world);
        return;
    }

    let possible_placements = next_possible_placements(game);
    if possible_placements.is_empty() {
        return;
    }

    for placement in possible_placements.into_iter() {
        game.place_block(placement);
        add_possibilities(game, solved_worlds);
        game.pop_block();
    }
}
