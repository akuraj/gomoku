pub mod board;
pub mod consts;
pub mod geometry;
pub mod pattern;
pub mod pattern_search;
pub mod state;
pub mod testing;
pub mod threat_space_search;
pub mod todos;

use consts::BLACK;
use state::get_state;
use std::time::Instant;
use threat_space_search::{
    animate_variation, tss_board, variation_to_algebraic, winning_variations,
};

fn main() {
    // let mut s = get_state(
    //     &[
    //         "a1", "a2", "a3", "a13", "a14", "a15", "b1", "b15", "c1", "c15", "f14", "g13", "i9",
    //         "i10", "m1", "m15", "n1", "n15", "o1", "o2", "o3", "o13", "o14", "o15",
    //     ],
    //     &["i6", "i13", "j10"],
    //     BLACK,
    //     false,
    // );

    let mut s = get_state(
        &[
            "f5", "g5", "h5", "g6", "g7", "h7", "i7", "h8", "h9", "g9", "i9",
        ],
        &[
            "g4", "e5", "f6", "h6", "j6", "f7", "j7", "f8", "g8", "i8", "f9",
        ],
        BLACK,
        true,
    );

    // let mut s = get_state(
    //     &["g10", "h8", "i7", "j7", "j9"],
    //     &["g7", "g8", "g9", "i9", "k8"],
    //     BLACK,
    //     true,
    // );

    // let mut s = get_state(
    //     &["f6", "h6", "g7", "h7", "h8", "g11"],
    //     &["e5", "h5", "g6", "l6", "f7", "g8"],
    //     BLACK,
    //     true,
    // );

    // let mut s = get_state(&["j5", "j6", "i10", "i11"], &[], BLACK, false);

    println!("{}", s);

    let n = 100;

    for _ in 0..n {
        tss_board(&mut s.board, s.turn);
    }

    let start = Instant::now();

    for _ in 0..n {
        tss_board(&mut s.board, s.turn);
    }

    println!(
        "Time taken: {} seconds",
        (start.elapsed().as_nanos() as f32) / 1e9
    );

    let node = tss_board(&mut s.board, s.turn);
    let win_vars = winning_variations(&node);
    println!("{}", win_vars.len());
    // animate_variation(&mut s.board, s.turn, &win_vars[0]);
    // println!("{:?}", variation_to_algebraic(&win_vars[0]));

    // testing::test_pattern_search_fns();
}
