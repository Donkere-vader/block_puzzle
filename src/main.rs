mod block;
mod draw;
mod game;
mod validators;

use std::{collections::HashSet, thread, io::stdin};

use stopwatch::Stopwatch;

use block::Block;
use game::Game;
use validators::{valid_inventory, valid_pos, Valid};

use crate::draw::draw_world;

const WORLD_HEIGHT: usize = 8;
const WORLD_WIDTH: usize = 8;
const COLORS: [(u8, u8, u8); 7] = [
    (241, 196, 15),
    (41, 128, 185),
    (230, 126, 34),
    (231, 76, 60),
    (46, 204, 113),
    (155, 89, 182),
    (26, 188, 156),
];

type Position = (usize, usize);
type Offset = (i32, i32);
type World = [[usize; WORLD_WIDTH]; WORLD_HEIGHT];
type Inventory = Vec<Block>; // blocks to choose from

#[derive(Debug, Clone)]
struct Placement {
    block: Block,
    rotation: usize,
    anchor_pos: Position,
}

fn next_possible_placements(game: &Game) -> Vec<Placement> {
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
                    block: block.clone(),
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
    return cursor_position(world).is_none();
}

fn add_possibilities(game: &mut Game, solved_worlds: &mut Vec<World>) {
    if world_filled(&game.world) {
        solved_worlds.push(game.world.clone());
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

fn world_id(world: &World) -> String {
    let center = (
        ((WORLD_WIDTH - 1) as f64 / 2.0),
        ((WORLD_HEIGHT - 1) as f64 / 2.0),
    );
    let mut ids = Vec::new();

    let layers = center.0.ceil().max(center.1.ceil()) as i32;
    for rotation in vec![0, 1, 2 ,3] {
        let mut rotation_id = String::new();
        let mut mirror_id = String::new();

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
            }
        }
    }

    ids.into_iter().min().unwrap()
}

fn filter_out_duplicates(solutions: &Vec<World>) -> Vec<World> {
    let mut solution_ids = HashSet::new();
    let mut filtered_solutions = Vec::new();

    for solution in solutions.iter() {
        let solution_id = world_id(&solution);
        if !solution_ids.contains(&solution_id) {
            solution_ids.insert(solution_id);
            filtered_solutions.push(solution.clone());
        }
    }

    filtered_solutions
}

fn calculate_parallel(inventory: Inventory) -> Vec<World> {
    if let Valid::Invalid(reason) = valid_inventory(&inventory) {
        panic!("Invalid inventory: {}", reason);
    }

    let game = Game::new(inventory);

    let mut threads = vec![];
    println!(
        "spawning {} threads...",
        next_possible_placements(&game).len()
    );

    for p in next_possible_placements(&game) {
        let mut g = game.clone();
        g.place_block(p);

        threads.push(thread::spawn(move || {
            let mut solved_worlds = Vec::new();
            add_possibilities(&mut g, &mut solved_worlds);

            solved_worlds
        }));
    }

    let mut solved_worlds = Vec::new();
    let n_threads = threads.len();
    for (i, t) in threads.into_iter().enumerate() {
        let mut found_in_thread = t.join().unwrap();
        println!(
            "Thread {}/{} finished and found {} solutions!",
            i + 1,
            n_threads,
            found_in_thread.len()
        );
        solved_worlds.append(&mut found_in_thread);
    }

    solved_worlds
}

fn calculate_sync(inventory: Inventory) -> Vec<World> {
    if let Valid::Invalid(reason) = valid_inventory(&inventory) {
        panic!("Invalid inventory: {}", reason);
    }

    let mut game = Game::new(inventory);

    let mut solved_worlds = Vec::new();
    add_possibilities(&mut game, &mut solved_worlds);

    solved_worlds
}

fn main() {
    let inventory: Inventory = vec![
        Block::Line(1),
        Block::Bridge(2),
        Block::T(3),
        Block::Weird(4),
        Block::Corner(5),
        Block::DoubleL(6),
        Block::Square(7),
        Block::Baton(8),
        Block::Cursor(9),
        Block::Plus(10),
        Block::Z(11),
        Block::Stairs(12),
        Block::L(13),
    ];

    let mut stopwatch = Stopwatch::new();
    stopwatch.start();

    let solved_worlds = calculate_parallel(inventory);
    // let solved_worlds = calculate_sync(inventory);

    stopwatch.stop();

    println!(
        "Found {} unique solution(s) in {:?}",
        solved_worlds.len() / 8,
        stopwatch.elapsed()
    );

    for s in solved_worlds.iter() {
        println!("Press enter to view (another) solution");
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        draw_world(&s);
    }
}
