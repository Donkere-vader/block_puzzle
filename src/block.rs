use crate::Offset;

#[derive(Debug)]
pub struct BlockGeometry {
    pub points: Vec<Offset>, // points relative to block's anchor
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {
    Line(usize),
    Square(usize),
    Baton(usize),
    Bridge(usize),
    Plus(usize),
    Corner(usize),
    T(usize),
    L(usize),
    DoubleL(usize),
    Cursor(usize),
    Stairs(usize),
    Z(usize),
    Weird(usize),
}

impl Block {
    pub fn get_block_id(&self) -> usize {
        match self {
            Self::Line(id) => *id,
            Self::Square(id) => *id,
            Self::Baton(id) => *id,
            Self::Bridge(id) => *id,
            Self::Plus(id) => *id,
            Self::Corner(id) => *id,
            Self::T(id) => *id,
            Self::L(id) => *id,
            Self::DoubleL(id) => *id,
            Self::Cursor(id) => *id,
            Self::Stairs(id) => *id,
            Self::Z(id) => *id,
            Self::Weird(id) => *id,
        }
    }

    pub fn get_geometries(&self) -> Vec<BlockGeometry> {
        match self {
            Self::Line(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0), (4, 0)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (0, 3), (0, 4)],
                },
            ],
            Self::Square(_) => vec![BlockGeometry {
                points: vec![(1, 0), (1, 1), (0, 1)],
            }],
            Self::Baton(_) => vec![
                BlockGeometry {
                    points: vec![(-2, 1), (-1, 1), (0, 1), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (1, 1), (2, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0), (2, 1)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (0, 2), (0, 3)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (0, 2), (0, 3)],
                },
                BlockGeometry {
                    points: vec![(-1, 2), (0, 1), (0, 2), (0, 3)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 2), (0, 2), (0, 3)],
                },
            ],
            Self::Bridge(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (0, 1), (2, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (0, 2), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 1), (2, 0), (0, 1), (2, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (0, 2), (1, 2)],
                },
            ],
            Self::Plus(_) => vec![BlockGeometry {
                points: vec![(-1, 1), (0, 1), (1, 1), (0, 2)],
            }],
            Self::Corner(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (0, 1), (0, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (2, 1), (2, 2)],
                },
                BlockGeometry {
                    points: vec![(-2, 2), (-1, 2), (0, 2), (0, 1)],
                },
                BlockGeometry {
                    points: vec![(0, 2), (1, 2), (2, 2), (0, 1)],
                },
            ],
            Self::T(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (1, 1), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(-2, 1), (-1, 1), (0, 1), (0, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (-1, 2), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (1, 1), (2, 1)],
                },
            ],
            Self::L(_) => vec![
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (0, 3), (1, 3)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (0, 3), (-1, 3)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0), (0, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (3, 0), (3, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (1, 2), (1, 3)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (0, 2), (0, 3)],
                },
                BlockGeometry {
                    points: vec![(-3, 1), (-2, 1), (-1, 1), (0, 1)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (2, 1), (3, 1)],
                },
            ],
            Self::DoubleL(_) => vec![
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (2, 1), (2, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (-2, 1), (-1, 1), (-2, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (0, 2), (-1, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (1, 2), (2, 2)],
                },
            ],
            Self::Cursor(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (0, 1), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 1), (0, 1), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (1, 1), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (1, 1), (0, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (-1, 1), (0, 1), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (-1, 0), (0, 1), (1, 1)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (0, 2), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (-1, 2), (0, 2)],
                },
            ],
            Self::Stairs(_) => vec![
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (-2, 2), (-1, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (1, 2), (2, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (0, 1), (-1, 1), (-1, 2)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (2, 1), (2, 2)],
                },
            ],
            Self::Z(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (2, 1), (3, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (-2, 1), (-1, 1), (0, 1)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (-1, 2), (-1, 3)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (1, 2), (1, 3)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (2, 1), (3, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (2, 0), (-1, 1), (0, 1)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (-1, 2), (-1, 3)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (0, 2), (1, 2), (1, 3)],
                },
            ],
            Self::Weird(_) => vec![
                BlockGeometry {
                    points: vec![(1, 0), (1, 1), (1, 2), (2, 1)],
                },
                BlockGeometry {
                    points: vec![(1, 0), (-1, 1), (0, 1), (0, 2)],
                },
                BlockGeometry {
                    points: vec![(-2, 1), (-1, 1), (0, 1), (-1, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (2, 1), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (0, 2), (1, 2)],
                },
                BlockGeometry {
                    points: vec![(0, 1), (1, 1), (-1, 2), (0, 2)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (1, 1), (-1, 2)],
                },
                BlockGeometry {
                    points: vec![(-1, 1), (0, 1), (1, 1), (1, 2)],
                },
            ],
        }
    }
}
