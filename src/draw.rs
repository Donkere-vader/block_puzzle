use crate::{
    validators::{valid_pos, Valid},
    Offset, Position, World, COLORS, WORLD_HEIGHT, WORLD_WIDTH,
};
use colored::Colorize;

fn border_on_offset(world: &World, pos: Position, same_as: &usize, offset: Offset) -> bool {
    let new_pos = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);

    if valid_pos(new_pos) != Valid::Valid {
        return true;
    }

    world[new_pos.1 as usize][new_pos.0 as usize] != *same_as
}

pub fn draw_world(world: &World) {
    let mut image_map = [[" "; WORLD_WIDTH * 4]; WORLD_HEIGHT * 3];
    let mut color_map = [[(0, 0, 0); WORLD_WIDTH]; WORLD_HEIGHT];

    for (y, row) in world.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                continue;
            }
            color_map[y][x] = COLORS[(value - 1) % 7];

            let borders = [
                border_on_offset(world, (x, y), value, (0, -1)),
                border_on_offset(world, (x, y), value, (1, 0)),
                border_on_offset(world, (x, y), value, (0, 1)),
                border_on_offset(world, (x, y), value, (-1, 0)),
            ];

            if borders[1] {
                image_map[y * 3][x * 4 + 3] = "🭵";
                image_map[y * 3 + 1][x * 4 + 3] = "🭵";
                image_map[y * 3 + 2][x * 4 + 3] = "🭵";
            }

            if borders[3] {
                image_map[y * 3][x * 4] = "🭰";
                image_map[y * 3 + 1][x * 4] = "🭰";
                image_map[y * 3 + 2][x * 4] = "🭰";
            }

            if borders[0] {
                image_map[y * 3][x * 4 + 1] = "🭶";
                image_map[y * 3][x * 4 + 2] = "🭶";
                if borders[3] {
                    image_map[y * 3][x * 4] = "🭽";
                } else {
                    image_map[y * 3][x * 4] = "🭶";
                }
                if borders[1] {
                    image_map[y * 3][x * 4 + 3] = "🭾";
                } else {
                    image_map[y * 3][x * 4 + 3] = "🭶";
                }
            }

            if borders[2] {
                image_map[y * 3 + 2][x * 4 + 1] = "🭻";
                image_map[y * 3 + 2][x * 4 + 2] = "🭻";
                if borders[3] {
                    image_map[y * 3 + 2][x * 4] = "🭼";
                } else {
                    image_map[y * 3 + 2][x * 4] = "🭻";
                }
                if borders[1] {
                    image_map[y * 3 + 2][x * 4 + 3] = "🭿";
                } else {
                    image_map[y * 3 + 2][x * 4 + 3] = "🭻";
                }
            }
        }
    }

    for (y, row) in image_map.into_iter().enumerate() {
        for (x, value) in row.into_iter().enumerate() {
            let color = color_map[y / 3][x / 4];
            print!(
                "{}",
                value
                    .to_string()
                    .on_truecolor(color.0, color.1, color.2)
                    .truecolor(0, 0, 0)
            );
        }
        println!();
    }
}
