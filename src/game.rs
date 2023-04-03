use std::fmt;

use crate::block::Rotate;
use crate::block::TetrisBlocks;
use crate::block::BLOCKS;
use crate::block::BlockShape;
use crate::block::{BlockColor, tetris_blocks, COLOR_TABLE,
    tetris_blocks::WALL as W,
    tetris_blocks::NONE as NONE,
    tetris_blocks::GHOST as GHOST,
};

pub const BASE_WIDTH: usize = 10;
pub const BASE_HEIGHT: usize = 20;
pub const EDGE_WIDTH: usize = 3;
pub const TETRIS_WIDTH: usize = BASE_WIDTH + EDGE_WIDTH * 2;
pub const TETRIS_HEIGHT: usize = BASE_HEIGHT + EDGE_WIDTH;
pub const BOARDSIZE: usize = TETRIS_HEIGHT * TETRIS_WIDTH;
pub const DELTA: u64 = 1000;

pub const LINE_BASE_SCORE:i32 = 100;

pub type Field = [BlockColor; BOARDSIZE];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ x : {} ,  y : {} }}", self.x, self.y)
    }
}
impl Position{
    pub fn init() -> Self{
        Position{x:4,y:0}
    }
}

#[derive(Clone, Copy)]
pub struct TetrisBoard {
    pub tetris_board: Field,
    pub score: i32,
    pub block_placed: i32,
    pub block_rotate: u8,
    pub block_now_shape: TetrisBlocks,
    pub block_next_three: [TetrisBlocks; 3],
    pub block_position: Position,
    pub ghost: Position,
    pub block_hold: TetrisBlocks,
    pub hold_rotate: u8,
}
impl fmt::Debug for TetrisBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Score:{}. #of block placed:{}. Rotate_angle:{}.\nNext 3 blocks:{:?},{:?},{:?}.\nHold block:{:?}",
            self.score, self.block_placed, self.block_rotate,self.block_next_three[0],self.block_next_three[1],self.block_next_three[2],self.block_hold,
        )
    }
}

// TODO: ブロックの色付け?
// TODO:　ブロックのオフセット 回転の時
// TODO:　ホールド
// TODO:　次のブロックの表示

#[allow(clippy::needless_return)]
impl TetrisBoard {
    pub fn new() -> Self {
        let mut _b = Self::init_board();
        let mut _r: u8 = 0;
        let mut _p: Position = Position::init();
        let _blockshape =  rand::random::<TetrisBlocks>();
        let mut _next_blocks: [TetrisBlocks;3] = [TetrisBlocks::I; 3];
        for _i in 0..3{
            _next_blocks[_i] = rand::random::<TetrisBlocks>();
        }
        TetrisBoard {
            tetris_board: _b, // うえから0,1,2かな高さは
            score: 0,
            block_placed: 0,
            block_rotate: _r,
            block_now_shape: _blockshape,
            block_position: _p,
            block_next_three: _next_blocks,
            ghost: Self::calc_init_ghost(_b,_r,_p,_blockshape),
            block_hold: TetrisBlocks::NONE,
            hold_rotate: 0,
        }
    }

    pub fn calc_init_ghost(_board: Field,_rotate: u8,_pos: Position, _blks: TetrisBlocks) -> Position{
        let mut new_pos: Position;
        let mut _tmp = 0;
        let _b: BlockShape = BLOCKS[_blks as usize].rotate(_rotate);
        loop {
            new_pos = Position {
                x: _pos.x,
                y: _pos.y + _tmp,
            };
            let mut _result:bool = false;
            for y in 0..4 {
                for x in 0..4 {
                    if y +new_pos.y + 1>= TETRIS_HEIGHT || x + new_pos.x >= TETRIS_WIDTH {
                        continue;
                    }
                    if _board[(y + new_pos.y + 1) * TETRIS_WIDTH + x + new_pos.x] != NONE
                        && _b[y][x] != NONE
                    {
                        _result = true;
                    }
                }
            }
            if _result{
                break;
            }else{
                _tmp += 1;
            }
            
        }
        return new_pos;
    }


    pub fn rotate(&mut self) {
        //! 回転させる関数 基本的に左回り
        self.block_rotate += 1;
        self.block_rotate %= 4;
    }

    pub fn rotate_undo(&mut self) {
        //! 回転を戻す
        self.block_rotate += 3;
        self.block_rotate %= 4;
    }
    pub fn add_score(&mut self, pts: i32) {
        //! スコアを加算させる
        self.score += pts;
    }

    pub fn init_board() -> Field {
        //! ボードの初期化
        let mut _b: Field = [NONE; BOARDSIZE];
        for i in 0..BOARDSIZE {
            if i % TETRIS_WIDTH == EDGE_WIDTH - 1
                || i % TETRIS_WIDTH == TETRIS_WIDTH - EDGE_WIDTH
                || i / TETRIS_WIDTH == TETRIS_HEIGHT - EDGE_WIDTH
            {
                _b[i] = W;      // 壁の領域に壁を設置する
            } else {
                _b[i] = NONE;
            }
        }
        return _b;
    }

    pub fn debug_draw(field: &TetrisBoard) {
        //! 盤面を描画する関数 Debug情報込みで余計なものも多い
        let mut field_buffer = field.tetris_board.clone();
        let _b: BlockShape = BLOCKS[field.block_now_shape as usize].rotate(field.block_rotate);
        let mut _x:usize;   // 一次的変数
        let mut _y:usize;   // 一次的変数
        let mut _board_position:usize;  // 一次的変数
        let _g = field.ghost;
        for y in 0..4 {
            for x in 0..4 {
                if _b[y][x] != NONE {
                    _x = x + field.block_position.x;
                    _y = y + field.block_position.y;
                    _board_position = _y * TETRIS_WIDTH + _x;
                    field_buffer[_board_position] = _b[y][x];
                    field_buffer[(_g.y+y)*TETRIS_WIDTH+_g.x+x] = GHOST;
                }
            }
        }

        println!("\x1b[H");
        for i in 0..=TETRIS_HEIGHT - EDGE_WIDTH {
            //y
            for j in EDGE_WIDTH - 1..=TETRIS_WIDTH - EDGE_WIDTH {
                print!("{}", COLOR_TABLE[field_buffer[i * TETRIS_WIDTH + j]]);
            }
            println!(); // 改行
        }
        println!("Now Block is {:?}", field.block_now_shape);
        println!("{:?}", field);
        println!("Now Pointing area is {:?}", field.block_position);
    }

    pub fn is_collision(&self, pos: &Position) -> bool {
        //! 当たり判定の関数
        let _b: BlockShape = BLOCKS[self.block_now_shape as usize].rotate(self.block_rotate);
        for y in 0..4 {
            for x in 0..4 {
                if y + pos.y >= TETRIS_HEIGHT || x + pos.x >= TETRIS_WIDTH {
                    continue;
                }
                if self.tetris_board[(y + pos.y) * TETRIS_WIDTH + x + pos.x] != NONE
                    && _b[y][x] != NONE
                {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn ghost_pos(&mut self) {
        //! ゴーストの座標を返す
        let mut new_pos: Position;
        let mut _tmp = 0;
        loop {
            new_pos = Position {
                x: self.block_position.x,
                y: self.block_position.y + _tmp,
            };
            if TetrisBoard::is_collision(
                &self,
                &Position {
                    x: self.block_position.x,
                    y: new_pos.y + 1,
                },
            ) {
                break;
            }
            _tmp += 1;
        }
        self.ghost = new_pos;
    }

    pub fn erase_lines(&mut self){
        //! ラインを消去する関数
        let mut erase_lines = 0;
        for y in 1..TETRIS_HEIGHT - EDGE_WIDTH {
            let mut is_erase_line = true;
            for x in EDGE_WIDTH..TETRIS_WIDTH - EDGE_WIDTH {
                if self.tetris_board[y * TETRIS_WIDTH + x] == NONE {
                    is_erase_line = false;
                    break;
                }
            }
            if is_erase_line {
                erase_lines += 1;
                for y2 in (0..y).rev() {
                    for x2 in 0..TETRIS_WIDTH {
                        self.tetris_board[(y2 + 1) * TETRIS_WIDTH + x2] =
                            self.tetris_board[(y2) * TETRIS_WIDTH + x2];
                    }
                }
            }
        }
        self.add_score(erase_lines * erase_lines * LINE_BASE_SCORE);
    }

    pub fn next_block(&mut self) -> Result<(), ()>{
        //! 次に表示させるブロックの処理を行う関数
        self.block_placed += 1;
        self.block_rotate = 0;
        self.block_position = Position::init();       // 次のブロックの処理
        self.block_now_shape = self.block_next_three[0]; // ブロックが固定されたら変数を変えて出てくるブロックを変える

        let mut _new_three_array:[TetrisBlocks; 3] = [TetrisBlocks::I; 3];
        _new_three_array[0] = self.block_next_three[1];
        _new_three_array[1] = self.block_next_three[2];
        _new_three_array[2] = rand::random::<TetrisBlocks>();
        self.block_next_three = _new_three_array;       // 1つずらす

        self.ghost_pos();

        if self.is_collision(&self.block_position){
            Err(())
        }else{
            Ok(())
        }
    }

    pub fn hold_block(&mut self) -> Result<(), ()>{
        //! ホールドに関する関数
        
        if self.block_hold as usize != TetrisBlocks::NONE as usize {    //HOLDをすでにしてるなら
            let _tmpblock = self.block_now_shape;
            let _tmprotate: u8= self.block_rotate;

            self.block_now_shape = self.block_hold;
            self.block_rotate = self.hold_rotate;
            
            self.block_hold = _tmpblock;
            self.block_rotate = _tmprotate;

            self.block_position = Position::init();

        }else {         // 始めてHOLDなら
            self.block_hold = self.block_now_shape;
            self.hold_rotate = self.block_rotate;

            self.block_position = Position::init();       // 次のブロックの処理
            self.block_now_shape = self.block_next_three[0]; // ブロックが固定されたら変数を変えて出てくるブロックを変える

            let mut _new_three_array:[TetrisBlocks; 3] = [TetrisBlocks::I; 3];
            _new_three_array[0] = self.block_next_three[1];
            _new_three_array[1] = self.block_next_three[2];
            _new_three_array[2] = rand::random::<TetrisBlocks>();
            self.block_next_three = _new_three_array;       // 1つずらす            
        }

        self.ghost_pos();       //ゴースト表示
        if self.is_collision(&self.block_position){
            Err(())
        }else{
            Ok(())
        }
    }

    pub fn gameover(&self){
        TetrisBoard::debug_draw(self);
        println!("GAMEOVER");
        println!("press `q` key to exit");
    }

    pub fn block_fixing(&mut self){
        //! ブロックの固定を行う関数
        let _b: BlockShape = BLOCKS[self.block_now_shape as usize].rotate(self.block_rotate);
        let gy = self.block_position.y;
        let gx = self.block_position.x;
        for y in 0..4 {
            for x in 0..4 {
                if _b[y][x] != NONE {
                    self.tetris_board[(y + gy) * TETRIS_WIDTH + x + gx ] = _b[y][x];
                    // block を固定
                }
            }
        }
    }

    pub fn super_rotation(&self) -> Result<Position, ()> {
        //! スーパーローテートの関数
        let pos = self.block_position;
        let diff_pos = [
            // 上
            Position {
                x: pos.x,
                y: pos.y.checked_sub(1).unwrap_or(pos.y),
            },
            // 右
            Position {
                x: pos.x + 1,
                y: pos.y,
            },
            // 下
            Position {
                x: pos.x,
                y: pos.y + 1,
            },
            // 左
            Position {
                x: pos.x.checked_sub(1).unwrap_or(pos.x),
                y: pos.y,
            },
            //さらに上
            Position {
                x: pos.x,
                y: pos.y.checked_sub(2).unwrap_or(pos.y),
            },
            // さらに右
            Position {
                x: pos.x + 2,
                y: pos.y,
            },
            // さらに下
            Position {
                x: pos.x,
                y: pos.y + 2,
            },
            // さらに左
            Position {
                x: pos.x.checked_sub(2).unwrap_or(pos.x),
                y: pos.y,
            },
        ];
        for _pos in diff_pos {
            if ! Self::is_collision(&self, &_pos) {
                return Ok(_pos);
            }
        }
        Err(())
    }
}

