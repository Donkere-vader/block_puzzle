use std::collections::HashSet;

use crate::{Inventory, WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Debug, PartialEq)]
pub enum Valid {
    Valid,
    Invalid(String),
}

pub fn valid_inventory(inventory: &Inventory) -> Valid {
    let mut unique_ids: HashSet<usize> = HashSet::new();

    for block in inventory.iter() {
        let id = block.get_block_id();
        if id == 0 {
            return Valid::Invalid("Id 0 not allowed".to_string());
        }
        if unique_ids.contains(&id) {
            return Valid::Invalid(format!("Duplicate id {} found", id));
        }
        unique_ids.insert(id);
    }

    Valid::Valid
}

pub fn valid_pos(pos: (i32, i32)) -> Valid {
    if 0 <= pos.0 && pos.0 < WORLD_WIDTH as i32 && 0 <= pos.1 && pos.1 < WORLD_HEIGHT as i32 {
        Valid::Valid
    } else {
        Valid::Invalid("Not in range".to_string())
    }
}
