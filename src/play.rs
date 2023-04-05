use getch_rs::{Getch, Key};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::game::{Position, TetrisBoard};

/*
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
                tet.move_left(2);       // ここ量
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Right) => {
                let mut tet = tet.lock().unwrap();
                tet.move_right(2);      // ここ量
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Down) => {
                let mut tet = tet.lock().unwrap();
                tet.move_down(2);
                tet.ghost_pos(); // ゴーストの計算
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Up) => {
                let mut tet = tet.lock().unwrap();
                tet.hard_drop();
                TetrisBoard::debug_draw(&tet);
            }
            Ok(Key::Char(' ')) => {
                let mut tet = tet.lock().unwrap();

                tet.rotate();
                tet.check_rotate();
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
*/

pub fn autoplay() {
    //! 自動化処理

    let _ = thread::spawn(|| {
        let mut game: TetrisBoard = TetrisBoard::new();
        loop {
            //todo!();
            ai1(&mut game);
            thread::sleep(time::Duration::from_millis(10));
        }
    });

    // キー入力処理
    let g = Getch::new();
    loop {
        // `q`キーで終了
        if let Ok(Key::Char('q')) = g.getch() {
            break;
        }
    }

    // 終了処理
    quit();
}

pub fn quit() {
    println!("\x1b[?25h");
}
pub fn ai1(game: &mut TetrisBoard) {
    let mut rng = rand::thread_rng();
    // hold
    if rng.gen_range(0..5) == 0 {
        if game.hold_block().is_err() {
            game.gameover();
        }
    }
    // ランダムに回転
    for _ in 0..rng.gen_range(0..=3) {
        game.rotate();
    }
    // ランダムに横移動
    let diff: isize = rng.gen_range(-4..=5);
    if diff < 0 {
        game.move_left(diff.abs() as usize);
    } else {
        game.move_right(diff.abs() as usize);
    }
    // ハードドロップ
    game.hard_drop();

    game.block_fixing(); // ライン固定
    game.erase_lines(); // ライン消去
    if game.next_block().is_err() {
        // ブロック生成不可能になったらGame Over
        game.gameover();
    }
    game.debug_draw();
}
