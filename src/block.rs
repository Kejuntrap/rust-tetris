use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy)]
pub enum TetrisBlocks{
    I,
    O,
    S,
    Z,
    T,
    J,
    L,
}
impl Clone for TetrisBlocks{
    fn clone(&self) -> Self {
        match self{
            TetrisBlocks::I => TetrisBlocks::I,
            TetrisBlocks::O => TetrisBlocks::O,
            TetrisBlocks::S => TetrisBlocks::S,
            TetrisBlocks::Z => TetrisBlocks::Z,
            TetrisBlocks::T => TetrisBlocks::T,
            TetrisBlocks::J => TetrisBlocks::J,
            TetrisBlocks::L => TetrisBlocks::L,
        }
    }   
}
impl fmt::Debug for TetrisBlocks{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            TetrisBlocks::I => write!(f,"I"),
            TetrisBlocks::O => write!(f,"O"),
            TetrisBlocks::S => write!(f,"S"),
            TetrisBlocks::Z => write!(f,"Z"),
            TetrisBlocks::T => write!(f,"T"),
            TetrisBlocks::J => write!(f,"J"),
            TetrisBlocks::L => write!(f,"L"),
        }
    }   
}

impl Distribution<TetrisBlocks> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrisBlocks {
        match rng.gen_range(0..=6) {
            0 => TetrisBlocks::I,
            1 => TetrisBlocks::O,
            2 => TetrisBlocks::S,
            3 => TetrisBlocks::Z,
            4 => TetrisBlocks::T,
            5 => TetrisBlocks::J,
            6 => TetrisBlocks::L,
            _ => TetrisBlocks::I,   // 一応
        }
    }
}

pub type BlockShape =  [[usize; 4]; 4];
pub trait Rotate{
    fn rotate(&self,_rotate_angle: u8) -> Self;
}
impl Rotate for BlockShape{
    fn rotate(&self, _rotate_angle: u8) -> BlockShape {
        match _rotate_angle{
            0 => {return *self;},
            1 => {
                let mut _r:BlockShape = [[0;4]; 4];
                for y in 0..4{
                    for x in 0..4{
                        _r[3-x][y] = self[y][x];
                    }
                }
                return _r;
            },
            2 => {
                let mut _r:BlockShape = [[0;4]; 4];
                for y in 0..4{
                    for x in 0..4{
                        _r[y][x] = self[3-y][3-x];
                    }
                }
                return _r;
            },
            3 => {
                let mut _r:BlockShape = [[0;4]; 4];
                for y in 0..4{
                    for x in 0..4{
                        _r[y][x] = self[3-x][y];
                    }
                }
                return _r;
            },
            _ => Self::rotate(&self,_rotate_angle%4),
        }
    }
}
pub const BLOCKS: [BlockShape; 7] = [
    [       // I
        [0,0,0,0],
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
    ],
    [       // O
        [0,0,0,0],
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    [       // S
        [0,0,0,0],
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
    ],
    [       // Z
        [0,0,0,0],
        [1,1,0,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    [       // T
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    [       // J
        [0,0,0,0],
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    [       // L
        [0,0,0,0],
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
];
