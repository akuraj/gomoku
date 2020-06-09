#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ndarray;

pub mod consts;
pub mod geometry;
pub mod pattern_search;
pub mod pattern;
pub mod board;
pub mod state;

use consts::BLACK;
use state::get_state;
use board::algebraic_to_point;

fn main() {
    let s = get_state(&["i10"],//&["j5", "j6", "i10", "i11"],
                      &[],
                      BLACK,
                      false );

    println!("{}", s);
    // println!("{}", algebraic_to_point("j5"));
}
