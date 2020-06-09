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
use pattern::{PATTERNS, PATTERNS_BY_DEFCON, PATTERNS_BY_NAME, PATTERNS_I, PATTERNS_NI};

use std::collections::HashSet;

fn main() {
    for p in PATTERNS_NI.iter() {
        println!("{}", *p);
    }
}
