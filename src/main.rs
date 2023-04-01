const TETRIS_WIDTH:usize=13;
const TETRIS_HEIGHT:usize=21;
const BOARDSIZE:usize=TETRIS_HEIGHT*TETRIS_WIDTH;
const DELTA:u64=1000;

use std::{thread, time};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};


#[derive(Clone,Copy,Debug)]
enum TetrisBlocks{
    I,
    O,
    S,
    Z,
    T,
    J,
    L,
}
type BlockShape =  [[usize; 4]; 4];

const BLOCKS: [BlockShape; 7] = [
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
    [       // T
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
];

struct Position{
    x: usize,
    y: usize,
}

#[derive(Clone,Copy,Debug)]
struct TetrisBoard{
    tetris_board: [usize;BOARDSIZE],
    score:i32,
}

impl TetrisBoard{
    pub fn new() -> Self{
        let mut _b:[usize; BOARDSIZE] = [0; BOARDSIZE];
        for i in 0..BOARDSIZE{

            if i%TETRIS_WIDTH == 0 || i%TETRIS_WIDTH == TETRIS_WIDTH-1 || i/TETRIS_WIDTH == TETRIS_HEIGHT-1 {
                _b[i] = 1;
            }
            else{
                _b[i] = 0;
            }
        }
        TetrisBoard{
            tetris_board: _b,   // うえから0,1,2かな高さは
            score:0,
        }
    }

    pub fn draw(&self) {
        println!("\x1b[2J\x1b[H\x1b[?25l");
        for i in 0..TETRIS_HEIGHT{ //y
           match Self::is_bottom(i){
                true => {  //x
                    for j in 0..TETRIS_WIDTH{
                        match Self::is_yokohasikko(j){
                            true => print!("[]"),
                            false => print!("{}{}",(64u8+(j as u8)) as char,(64u8+(j as u8)) as char),
                        }
                    }
                },
                false => {                 
                    for j in 0..TETRIS_WIDTH{
                        match Self::is_yokohasikko(j){
                            false => {
                                match self.tetris_board[i*TETRIS_WIDTH+j]{
                                    0 => print!("__"),
                                    _ => print!("[]"),
                                }
                            },
                            true => {
                                print!("{i:>02}");
                            }
                        }
                    }
                },
            }
            println!("");   // 改行
        }
    }

    pub fn debug_draw(&self,pos: &Position) {
        let mut field_buffer = self.tetris_board.clone();

        for y in 0..4{
            for x in 0..4{
                if BLOCKS[TetrisBlocks::I as usize][y][x] == 1 {
                    field_buffer[(y+pos.y)*TETRIS_WIDTH + x + pos.x] = 1;
                }
            }
        }

        println!("\x1b[2J\x1b[H\x1b[?25l");
        for i in 0..TETRIS_HEIGHT{ //y
           match Self::is_bottom(i){
                true => {  //x
                    for j in 0..TETRIS_WIDTH{
                        match Self::is_yokohasikko(j){
                            true => print!("[]"),
                            false => print!("{}{}",(64u8+(j as u8)) as char,(64u8+(j as u8)) as char),
                        }
                    }
                },
                false => {                 
                    for j in 0..TETRIS_WIDTH{
                        match Self::is_yokohasikko(j){
                            false => {
                                match field_buffer[i*TETRIS_WIDTH+j]{
                                    0 => print!("  "),
                                    _ => print!("[]"),
                                }
                            },
                            true => {
                                print!("{i:>02}");
                            }
                        }
                    }
                },
            }
            println!("");   // 改行
        }
    }

    fn is_yokohasikko(i: usize) -> bool{
        if i % TETRIS_WIDTH == 0 || i % TETRIS_WIDTH == TETRIS_WIDTH - 1{       // 両端であるかの判定
            return true;
        }
        return false;
    }

    fn is_bottom(i: usize) -> bool{     //底面であるかの判定
        if i % TETRIS_HEIGHT == TETRIS_HEIGHT -1 {
            return true;
        }
        return false;
    }

    fn is_collision(&self, pos: &Position, block: TetrisBlocks) -> bool{
        for y in 0..4{
            for x in 0..4{
                if self.tetris_board[(y+pos.y)*TETRIS_WIDTH+x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
                    return true;
                }
            }
        }
        return false;
    }
}


fn main() {
    println!("Hello, world!");
    let tet:TetrisBoard = TetrisBoard::new();
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    tet.debug_draw(&pos.lock().unwrap());  //draw

    {
        let pos = Arc::clone(&pos);
        let _ = thread::spawn(move || {
            loop{
                thread::sleep(time::Duration::from_millis(DELTA));   // wait for delta
                //free fall
                let mut pos = pos.lock().unwrap();
                let new_pos = Position{
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !tet.is_collision(&new_pos, TetrisBlocks::I) {
                    *pos = new_pos;
                }
                tet.debug_draw(&pos);
            }
        });
    }

    // key input 
    let g = Getch::new();
    loop{
        match g.getch(){        //キー入力
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h");
                return;
            },
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let new_pos = Position{
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !tet.is_collision(&new_pos, TetrisBlocks::I){
                    *pos = new_pos;
                }
                tet.debug_draw(&pos);
            },
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let new_pos = Position{
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !tet.is_collision(&new_pos, TetrisBlocks::I){
                    *pos = new_pos;
                }
                tet.debug_draw(&pos);
            },
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let new_pos = Position{
                    x: pos.x ,
                    y: pos.y + 1,
                };
                if !tet.is_collision(&new_pos, TetrisBlocks::I){
                    *pos = new_pos;
                }
                tet.debug_draw(&pos);
            },
            _ => (),
        }
    }
   
}
