mod block;

const TETRIS_WIDTH: usize = 17;
const TETRIS_HEIGHT: usize = 23;
const BOARDSIZE: usize = TETRIS_HEIGHT * TETRIS_WIDTH;
const DELTA: u64 = 1000;
const EDGE_WIDTH: usize = 3;

use block::{BlockShape, Rotate, TetrisBlocks, BLOCKS};
use getch_rs::{Getch, Key};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::{thread, time};

type Field = [usize; BOARDSIZE];

struct Position {
    x: usize,
    y: usize,
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ x : {} ,  y : {} }}", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct TetrisBoard {
    tetris_board: Field,
    score: i32,
    block_placed: i32,
    block_rotate: u8,
}
impl fmt::Debug for TetrisBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Score:{}. #of block placed:{}. Rotate_angle: {}",
            self.score, self.block_placed, self.block_rotate
        )
    }
}

// TODO: ブロックの色付け?
// TODO:　ブロックのオフセット 回転の時
// TODO:　ホールド
// TODO:　次のブロックの表示

impl TetrisBoard {
    fn new() -> Self {
        let mut _b = Self::init_board();
        let mut _r: u8 = 0;
        TetrisBoard {
            tetris_board: _b, // うえから0,1,2かな高さは
            score: 0,
            block_placed: 0,
            block_rotate: _r,
        }
    }

    fn rotate(&mut self) {
        self.block_rotate += 1;
        self.block_rotate %= 4;
    }

    fn rotate_undo(&mut self) {
        self.block_rotate += 3;
        self.block_rotate %= 4;
    }
    fn add_score(&mut self, pts: i32) {
        self.score += pts;
    }

    fn init_board() -> Field {
        let mut _b: Field = [0; BOARDSIZE];
        for i in 0..BOARDSIZE {
            if i % TETRIS_WIDTH == EDGE_WIDTH - 1
                || i % TETRIS_WIDTH == TETRIS_WIDTH - EDGE_WIDTH
                || i / TETRIS_WIDTH == TETRIS_HEIGHT - EDGE_WIDTH
            {
                _b[i] = 1;
            } else {
                _b[i] = 0;
            }
        }
        return _b;
    }

    pub fn debug_draw(field: &TetrisBoard, pos: &Position, block: TetrisBlocks) {
        let mut field_buffer = field.tetris_board.clone();
        let _b: BlockShape = BLOCKS[block as usize].rotate(field.block_rotate);

        for y in 0..4 {
            for x in 0..4 {
                if _b[y][x] == 1 {
                    field_buffer[(y + pos.y) * TETRIS_WIDTH + x + pos.x] = 1;
                }
            }
        }

        println!("\x1b[2J\x1b[H\x1b[?25l");
        for i in 0..=TETRIS_HEIGHT - EDGE_WIDTH {
            //y
            for j in EDGE_WIDTH - 1..=TETRIS_WIDTH - EDGE_WIDTH {
                match field_buffer[i * TETRIS_WIDTH + j] {
                    0 => print!(". "),
                    _ => print!("[]"),
                }
            }
            println!(""); // 改行
        }
        println!("Now Block is {:?}", block);
        println!("Now Score is {:?}", field);
        println!("Now Pointing area is {:?}", pos);
    }

    fn is_collision(&self, pos: &Position, block: TetrisBlocks) -> bool {
        // 当たり判定
        let _b: BlockShape = BLOCKS[block as usize].rotate(self.block_rotate);
        for y in 0..4 {
            for x in 0..4 {
                if y + pos.y < TETRIS_HEIGHT && x + pos.x < TETRIS_WIDTH {
                    if self.tetris_board[(y + pos.y) * TETRIS_WIDTH + x + pos.x] != 0
                        && _b[y][x] != 0
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");
    let tet = Arc::new(Mutex::new(TetrisBoard::new()));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<TetrisBlocks>()));
    TetrisBoard::debug_draw(
        &tet.lock().unwrap(),
        &pos.lock().unwrap(),
        *block.lock().unwrap(),
    ); //draw

    {
        let pos = Arc::clone(&pos);
        let tet = Arc::clone(&tet);
        let block = Arc::clone(&block);
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(DELTA)); // wait for delta
                                                                   //free fall
                let mut pos = pos.lock().unwrap();
                let mut tet = tet.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut erase_lines = 0;
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos, *block) {
                    *pos = new_pos; // free fall and position update
                } else {
                    //cannot move downward anymore
                    let _b: BlockShape = BLOCKS[*block as usize].rotate(tet.block_rotate);
                    for y in 0..4 {
                        for x in 0..4 {
                            if _b[y][x] != 0 {
                                tet.tetris_board[(y + pos.y) * TETRIS_WIDTH + x + pos.x] = _b[y][x];
                                // block を固定
                            }
                        }
                    }
                    for y in 1..TETRIS_HEIGHT - EDGE_WIDTH {
                        let mut is_erase_line = true;
                        for x in EDGE_WIDTH..TETRIS_WIDTH - EDGE_WIDTH {
                            if tet.tetris_board[y * TETRIS_WIDTH + x] == 0 {
                                is_erase_line = false;
                                break;
                            }
                        }
                        if is_erase_line {
                            erase_lines += 1;
                            for y2 in (0..y).rev() {
                                for x2 in 0..TETRIS_WIDTH {
                                    tet.tetris_board[(y2 + 1) * TETRIS_WIDTH + x2] =
                                        tet.tetris_board[(y2) * TETRIS_WIDTH + x2];
                                }
                            }
                        }
                    }

                    *pos = Position { x: 4, y: 0 };
                    tet.block_placed += 1;
                    tet.add_score(erase_lines * erase_lines * 100);
                    tet.block_rotate = 0;
                    *block = rand::random(); // ブロックが固定されたら変数を変えて出てくるブロックを変える
                }
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
        });
    }

    // key input
    let g = Getch::new();
    loop {
        match g.getch() {
            //キー入力
            Ok(Key::Char('q')) | Ok(Key::Esc) => {
                println!("\x1b[?25h");
                return;
            }
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let tet = tet.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = match pos.x {
                    1..=TETRIS_WIDTH => Position {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                    _ => Position {
                        // 左端より端にくるとき0に強制的にする
                        x: 0,
                        y: pos.y,
                    },
                };
                if !TetrisBoard::is_collision(&tet, &new_pos, *block) {
                    *pos = new_pos;
                }
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let tet = tet.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = match pos.x + 1 {
                    0..=TETRIS_WIDTH => Position {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    _ => Position {
                        // 右端のさらに右なら動かさない
                        x: pos.x,
                        y: pos.y,
                    },
                };
                if !TetrisBoard::is_collision(&tet, &new_pos, *block) {
                    *pos = new_pos;
                }
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let tet = tet.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos, *block) {
                    *pos = new_pos;
                }
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
            Ok(Key::Up) => {
                let mut pos = pos.lock().unwrap();
                let mut tet = tet.lock().unwrap();
                let block = block.lock().unwrap();
                let mut _tmp = 0;
                let mut new_pos: Position;
                loop {
                    new_pos = Position {
                        x: pos.x,
                        y: pos.y + _tmp,
                    };
                    if TetrisBoard::is_collision(
                        &tet,
                        &Position {
                            x: pos.x,
                            y: new_pos.y + 1,
                        },
                        *block,
                    ) {
                        break;
                    }
                    _tmp += 1;
                }
                *pos = new_pos;
                tet.add_score(_tmp as i32);
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
            Ok(Key::Char(' ')) => {
                let mut pos = pos.lock().unwrap();
                let mut tet = tet.lock().unwrap();
                let block = block.lock().unwrap();

                tet.rotate();

                let new_pos = Position { x: pos.x, y: pos.y };
                if !TetrisBoard::is_collision(&tet, &new_pos, *block) {
                    *pos = new_pos;
                } else {
                    tet.rotate_undo();
                }
                TetrisBoard::debug_draw(&tet, &pos, *block);
            }
            _ => (),
        }
    }
}
