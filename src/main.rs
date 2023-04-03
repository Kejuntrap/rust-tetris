mod block;
mod game;

use game::{Position, TetrisBoard, DELTA};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    println!("\x1b[2J\x1b[H\x1b[?25l");
    let tet = Arc::new(Mutex::new(TetrisBoard::new()));
    TetrisBoard::debug_draw(&tet.lock().unwrap()); //draw
    {
        let tet = Arc::clone(&tet);
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(DELTA)); // wait for delta
                                                                   //free fall
                let mut tet = tet.lock().unwrap();

                let new_pos = Position {
                    x: tet.block_position.x,
                    y: tet.block_position.y + 1,
                };
                if !tet.is_collision(&new_pos) {
                    tet.block_position = new_pos; // free fall and position update
                } else {
                    //cannot move downward anymore
                    tet.block_fixing(); // ライン固定
                    tet.erase_lines(); // ライン消去
                    if tet.next_block().is_err() {
                        // ブロック生成不可能になったらGame Over
                        tet.gameover();
                        break;
                    }
                }
                TetrisBoard::debug_draw(&tet);
            }
        });
    }

    // key input
    let g = Getch::new();
    loop {
        match g.getch() {
            //キー入力
            Ok(Key::Char('q')) | Ok(Key::Esc) => {
                break;
            }
            Ok(Key::Left) => {
                let mut tet = tet.lock().unwrap();
                let new_pos = Position {
                    x: tet
                        .block_position
                        .x
                        .checked_sub(1)
                        .unwrap_or_else(|| tet.block_position.x), // 符号なしでマイナスにならないようにする
                    y: tet.block_position.y,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos) {
                    tet.block_position = new_pos;
                }
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Right) => {
                let mut tet = tet.lock().unwrap();
                let new_pos = Position {
                    x: tet.block_position.x + 1,
                    y: tet.block_position.y,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos) {
                    tet.block_position = new_pos;
                }
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Down) => {
                let mut tet = tet.lock().unwrap();
                let new_pos = Position {
                    x: tet.block_position.x,
                    y: tet.block_position.y + 1,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos) {
                    tet.block_position = new_pos;
                }
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Up) => {
                let mut tet = tet.lock().unwrap();
                let mut _tmp = 0;
                let mut new_pos: Position;
                loop {
                    new_pos = Position {
                        x: tet.block_position.x,
                        y: tet.block_position.y + _tmp,
                    };
                    if TetrisBoard::is_collision(
                        &tet,
                        &Position {
                            x: tet.block_position.x,
                            y: new_pos.y + 1,
                        },
                    ) {
                        break;
                    }
                    _tmp += 1;
                }
                tet.block_position = new_pos;
                tet.add_score(_tmp as i32);
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Char(' ')) => {
                let mut tet = tet.lock().unwrap();

                tet.rotate();

                let new_pos = Position {
                    x: tet.block_position.x,
                    y: tet.block_position.y,
                };
                if !TetrisBoard::is_collision(&tet, &new_pos) {
                    tet.block_position = new_pos;
                } else if let Ok(new_pos) = TetrisBoard::super_rotation(&tet) {
                    tet.block_position = new_pos;
                } else {
                    tet.rotate_undo();
                }
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Char('h')) => {
                // ホールド
                let mut tet = tet.lock().unwrap();
                if tet.hold_block().is_err() {
                    // ブロック生成不可能になったらGame Over
                    tet.gameover();
                    break;
                }
            }
            _ => (),
        }
    }
    quit();
}

pub fn quit() {
    println!("\x1b[?25h");
}
