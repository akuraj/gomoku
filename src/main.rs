#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ndarray;

pub mod consts;
pub mod geometry;
pub mod pattern_search;
pub mod pattern;
pub mod board;

use board::{get_board, new_board, algebraic_to_point, board_to_str};

fn main() {
    let x = ["a4", "b6"];
    let y = ["a5", "b7"];

    let board = get_board(&x, &y);

    println!("{}", board_to_str(&board));
}
