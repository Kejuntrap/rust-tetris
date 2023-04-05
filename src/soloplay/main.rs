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
                let mut tet = tet.lock().unwrap();
                // 自由落下とボードの更新
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
                tet.debug_draw();
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
                tet.move_left(1);
                tet.ghost_pos(); // ゴーストの計算
                tet.debug_draw();
            }
            Ok(Key::Right) => {
                let mut tet = tet.lock().unwrap();
                tet.move_right(1);
                tet.ghost_pos(); // ゴーストの計算
                tet.debug_draw();
            }
            Ok(Key::Down) => {
                let mut tet = tet.lock().unwrap();
                tet.move_down(1);
                tet.ghost_pos(); // ゴーストの計算
                tet.debug_draw();
            }
            Ok(Key::Up) => {
                let mut tet = tet.lock().unwrap();
                tet.hard_drop();        // すぐ次のブロックの処理になるのでゴースト計算をしなくてよい
                tet.debug_draw();
            }
            Ok(Key::Char(' ')) => {
                let mut tet = tet.lock().unwrap();

                tet.rotate();
                tet.check_rotate();
                tet.ghost_pos(); // ゴーストの計算
                tet.debug_draw();
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
