#[macro_use]
extern crate lazy_static;

pub mod consts;

use consts::OWN;
use consts::GEN_ELEMS_TO_NAMES;

fn main() {
    println!("{}", GEN_ELEMS_TO_NAMES[&OWN]);
}
