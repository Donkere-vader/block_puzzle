mod block;
mod discovery;
mod draw;
mod filtering;
mod game;
mod validators;

use std::{io::stdin, thread, time::Instant};

use block::Block;
use filtering::{filter_out_duplicates, world_id};
use game::Game;
use validators::{valid_inventory, Valid};

use crate::{
    discovery::{add_possibilities, next_possible_placements},
    draw::draw_world,
};

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

enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Debug, Clone)]
pub struct Placement {
    block: Block,
    rotation: usize,
    anchor_pos: Position,
}

#[allow(unused)]
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

    println!("Filtering out duplicates...");
    filter_out_duplicates(&solved_worlds)
}

#[allow(unused)]
fn calculate_sync(inventory: Inventory) -> Vec<World> {
    if let Valid::Invalid(reason) = valid_inventory(&inventory) {
        panic!("Invalid inventory: {}", reason);
    }

    let mut game = Game::new(inventory);

    let mut solved_worlds = Vec::new();
    add_possibilities(&mut game, &mut solved_worlds);

    println!("Filtering out duplicates...");
    filter_out_duplicates(&solved_worlds)
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

    // let inventory: Inventory = vec![
    //     Block::Square(1),
    //     Block::Square(2),
    //     Block::Square(3),
    //     Block::Square(4),
    // ];

    let start = Instant::now();

    let solved_worlds = calculate_parallel(inventory);
    // let solved_worlds = calculate_sync(inventory);

    let end = Instant::now();

    println!(
        "Found {} unique solution(s) in {:?}",
        solved_worlds.len(),
        end.duration_since(start),
    );

    for s in solved_worlds.iter() {
        println!("Press enter to view (another) solution");
        let mut _buf = String::new();
        stdin().read_line(&mut _buf).unwrap();
        draw_world(s);
        println!("id: {}", world_id(s));
    }
}
