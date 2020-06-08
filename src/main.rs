#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ndarray;

pub mod consts;
pub mod geometry;
pub mod pattern_search;

use ndarray::prelude::*;
use pattern_search::get_pattern;

fn main() {
    let mut pattern: Array1<u8> = Array::zeros(56);
    pattern[17] = 45;
    let p2 = get_pattern(&pattern, 2);
    println!("{:?}", p2[17]);
}
