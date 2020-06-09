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
use consts::OWN;
use pattern::PATTERNS;

fn main() {
    for x in PATTERNS.iter() {
        println!("{}", x);
    }
}
