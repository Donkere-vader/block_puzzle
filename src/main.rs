mod block;
mod draw;
mod game;
mod validators;

use block::Block;
use draw::draw_world;
use game::Game;
use validators::{valid_inventory, Valid, valid_pos};

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

#[derive(Debug)]
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
                let new_point = (
                    (cursor.0 as i32 + point.0),
                    (cursor.1 as i32 + point.1),
                );

                if valid_pos(new_point) == Valid::Valid && game.world[new_point.1 as usize][new_point.0 as usize] != 0 {
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

fn main() {
    let inventory: Inventory = vec![
        Block::Line(1),
        Block::Line(2),
        Block::Line(3),
        Block::Line(4),
        Block::Line(5),
        Block::Line(6),
        Block::Line(7),
        Block::Line(8),
        Block::Line(9),
        Block::Line(10),
        Block::Line(11),
        Block::Line(12),
        Block::Line(13),
        Block::Line(14),
        Block::Line(15),
        Block::Line(16),
    ];

    if let Valid::Invalid(reason) = valid_inventory(&inventory) {
        panic!("Invalid inventory: {}", reason);
    }

    let mut game = Game::new(inventory);
    draw_world(&game.world);

    while !game.inventory.is_empty() {
        let mut placements = next_possible_placements(&game);
        let placement = placements.remove(placements.len() - 1);
        println!("{:?}", placement);

        game.inventory = game.inventory.into_iter()
            .filter(|b| b.get_block_id() != placement.block.get_block_id())
            .collect();

        game.place_block(placement);

        draw_world(&game.world);
    }
}
