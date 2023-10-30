use crate::{Offset, Position};

#[derive(Debug)]
pub struct BlockGeometry {
    pub points: Vec<Offset>, // points relative to block's anchor
}

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Line(usize),
    Square(usize),
}

impl Block {
    pub fn get_block_id(&self) -> usize {
        match self {
            Self::Line(id) => *id,
            Self::Square(id) => *id,
        }
    }

    pub fn get_geometries(&self) -> Vec<BlockGeometry> {
        match self {
            Self::Line(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (0, 3)],
                },
            ],
            Self::Square(_) => vec![BlockGeometry {
                points: vec![(1, 0), (1, 1), (0, 1)],
            }],
        }
    }
}
