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
use threat_space_search::{animate_variation, potential_win_variations, tss_board, variation_to_algebraic};

fn main() {
    // // NOTE: WHITE is clearly winning here, but TSS finds a win for black
    // //       because it doesn't consider the threats that the opponent can make.
    // let mut s = get_state(
    //     &["g9", "h9", "j10", "k11", "a6", "o6"],
    //     &["b5", "c4", "d3", "d2", "c2", "n5", "m4", "l3", "l2", "m2"],
    //     BLACK,
    //     false,
    // );

    // let mut s = get_state(
    //     &[
    //         "a1", "a2", "a3", "a13", "a14", "a15", "b1", "b15", "c1", "c15", "f14", "g13", "i9",
    //         "i10", "m1", "m15", "n1", "n15", "o1", "o2", "o3", "o13", "o14", "o15",
    //     ],
    //     &["i6", "i13", "j10"],
    //     BLACK,
    //     false,
    // );

    // let mut s = get_state(
    //     &[
    //         "f5", "g5", "h5", "g6", "g7", "h7", "i7", "h8", "h9", "g9", "i9",
    //     ],
    //     &[
    //         "g4", "e5", "f6", "h6", "j6", "f7", "j7", "f8", "g8", "i8", "f9",
    //     ],
    //     BLACK,
    //     true,
    // );

    // let mut s = get_state(
    //     &["g10", "h8", "i7", "j7", "j9"],
    //     &["g7", "g8", "g9", "i9", "k8"],
    //     BLACK,
    //     true,
    // );

    let mut s = get_state(&["f6", "h6", "g7", "h7", "h8", "g11"], &["e5", "h5", "g6", "l6", "f7", "g8"], BLACK, true);

    println!("{}", s);

    let n = 1000;

    for _ in 0..n {
        tss_board(&mut s.board, s.turn);
    }

    let start = Instant::now();

    for _ in 0..n {
        tss_board(&mut s.board, s.turn);
    }

    println!("Time taken: {} seconds", (start.elapsed().as_nanos() as f32) / 1e9);

    let node = tss_board(&mut s.board, s.turn);
    let potential_win_vars = potential_win_variations(&node);
    println!("{}", potential_win_vars.len());
    // println!("{:?}", variation_to_algebraic(&potential_win_vars[0]));
    // animate_variation(&mut s.board, s.turn, &potential_win_vars[0]);

    // testing::test_pattern_search_fns();
}
