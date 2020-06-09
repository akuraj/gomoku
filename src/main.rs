#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ndarray;

pub mod consts;
pub mod geometry;
pub mod pattern_search;
pub mod pattern;

use ndarray::prelude::*;
use pattern_search::get_pattern;
use consts::{GEN_ELEMS, OWN};

fn main() {
    println!("{:?}", GEN_ELEMS.iter().any(|&x| x == 77));
}
