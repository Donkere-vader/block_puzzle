use crate::{block::BlockGeometry, Inventory, Placement, Position, World};

#[derive(Debug)]
pub struct Game {
    pub world: World,
    pub placements: Vec<Placement>,
    pub inventory: Inventory,
}

impl Game {
    pub fn new(inventory: Inventory) -> Self {
        Game {
            world: Default::default(),
            placements: Vec::new(),
            inventory,
        }
    }

    fn set_values(&mut self, anchor_pos: &Position, geometry: &BlockGeometry, value: usize) {
        self.world[anchor_pos.1][anchor_pos.0] = value;

        for point in geometry.points.iter() {
            let new_point = (
                (anchor_pos.0 as i32 + point.0) as usize,
                (anchor_pos.1 as i32 + point.1) as usize,
            );
            self.world[new_point.1][new_point.0] = value;
        }
    }

    pub fn place_block(&mut self, placement: Placement) {
        let geometries = placement.block.get_geometries();

        let geometry = geometries.get(placement.rotation).unwrap();

        self.set_values(
            &placement.anchor_pos,
            geometry,
            placement.block.get_block_id(),
        );

        self.placements.push(placement);
    }

    pub fn pop_block(&mut self) -> Option<Placement> {
        let placement = self.placements.pop()?;

        let geometries = placement.block.get_geometries();

        let geometry = geometries.get(placement.rotation).unwrap();

        self.set_values(&placement.anchor_pos, geometry, 0);

        Some(placement)
    }
}
