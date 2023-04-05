use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy)]
pub enum TetrisBlocks {
    NONE,
    WALL,
    I,
    O,
    S,
    Z,
    T,
    J,
    L,
    GHOST,
}
impl Clone for TetrisBlocks {
    fn clone(&self) -> Self {
        match self {
            TetrisBlocks::NONE => TetrisBlocks::NONE,
            TetrisBlocks::WALL => TetrisBlocks::WALL,
            TetrisBlocks::I => TetrisBlocks::I,
            TetrisBlocks::O => TetrisBlocks::O,
            TetrisBlocks::S => TetrisBlocks::S,
            TetrisBlocks::Z => TetrisBlocks::Z,
            TetrisBlocks::T => TetrisBlocks::T,
            TetrisBlocks::J => TetrisBlocks::J,
            TetrisBlocks::L => TetrisBlocks::L,
            TetrisBlocks::GHOST => TetrisBlocks::GHOST,
        }
    }
}
impl fmt::Debug for TetrisBlocks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TetrisBlocks::NONE => write!(f, "N"),
            TetrisBlocks::WALL => write!(f, "W"),
            TetrisBlocks::I => write!(f, "I"),
            TetrisBlocks::O => write!(f, "O"),
            TetrisBlocks::S => write!(f, "S"),
            TetrisBlocks::Z => write!(f, "Z"),
            TetrisBlocks::T => write!(f, "T"),
            TetrisBlocks::J => write!(f, "J"),
            TetrisBlocks::L => write!(f, "L"),
            TetrisBlocks::GHOST => write!(f, "G"),
        }
    }
}

impl Distribution<TetrisBlocks> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrisBlocks {
        match rng.gen_range(2..=8) {
            0 => TetrisBlocks::NONE,
            1 => TetrisBlocks::WALL,
            2 => TetrisBlocks::I,
            3 => TetrisBlocks::O,
            4 => TetrisBlocks::S,
            5 => TetrisBlocks::Z,
            6 => TetrisBlocks::T,
            7 => TetrisBlocks::J,
            8 => TetrisBlocks::L,
            9 => TetrisBlocks::GHOST,
            _ => TetrisBlocks::I, // 一応
        }
    }
}
use tetris_blocks::{I, J, L, O, S, T, Z};
pub type BlockColor = usize;
pub mod tetris_blocks {
    pub const NONE: super::BlockColor = 0;
    pub const WALL: super::BlockColor = 1;
    pub const I: super::BlockColor = 2;
    pub const O: super::BlockColor = 3;
    pub const S: super::BlockColor = 4;
    pub const Z: super::BlockColor = 5;
    pub const J: super::BlockColor = 6;
    pub const L: super::BlockColor = 7;
    pub const T: super::BlockColor = 8;
    pub const GHOST: super::BlockColor = 9;
}
pub const COLOR_TABLE: [&str; 10] = [
    "\x1b[48;2;000;000;000m  ", // 何もなし
    "\x1b[48;2;127;127;127m__", // 壁
    "\x1b[48;2;000;000;255m__", // I
    "\x1b[48;2;000;255;000m__", // O
    "\x1b[48;2;000;255;255m__", // S
    "\x1b[48;2;255;000;000m__", // Z
    "\x1b[48;2;255;000;255m__", // J
    "\x1b[48;2;255;127;000m__", // L
    "\x1b[48;2;255;255;000m__", // T
    "\x1b[48;2;000;000;000m[]", // ゴースト
];

pub type BlockShape = [[usize; 4]; 4];
pub trait Rotate {
    fn rotate(&self, _rotate_angle: u8) -> Self;
}
impl Rotate for BlockShape {
    fn rotate(&self, _rotate_angle: u8) -> BlockShape {
        let mut _r: BlockShape = [[0; 4]; 4];
        match _rotate_angle {
            0 => {
                return *self;
            }
            1 => {
                for y in 0..4 {
                    for x in 0..4 {
                        _r[3 - x][y] = self[y][x];
                    }
                }
                return _r;
            }
            2 => {
                for y in 0..4 {
                    for x in 0..4 {
                        _r[y][x] = self[3 - y][3 - x];
                    }
                }
                return _r;
            }
            3 => {
                for y in 0..4 {
                    for x in 0..4 {
                        _r[y][x] = self[3 - x][y];
                    }
                }
                return _r;
            }
            _ => Self::rotate(self, _rotate_angle % 4),
        }
    }
}
pub const BLOCKS: [BlockShape; 9] = [
    [
        // NONE
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // WALL
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // I
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [I, I, I, I],
        [0, 0, 0, 0],
    ],
    [
        // O
        [0, 0, 0, 0],
        [0, O, O, 0],
        [0, O, O, 0],
        [0, 0, 0, 0],
    ],
    [
        // S
        [0, 0, 0, 0],
        [0, S, S, 0],
        [S, S, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // Z
        [0, 0, 0, 0],
        [Z, Z, 0, 0],
        [0, Z, Z, 0],
        [0, 0, 0, 0],
    ],
    [
        // T
        [0, 0, 0, 0],
        [0, T, 0, 0],
        [T, T, T, 0],
        [0, 0, 0, 0],
    ],
    [
        // J
        [0, 0, 0, 0],
        [J, 0, 0, 0],
        [J, J, J, 0],
        [0, 0, 0, 0],
    ],
    [
        // L
        [0, 0, 0, 0],
        [0, 0, L, 0],
        [L, L, L, 0],
        [0, 0, 0, 0],
    ],
];
