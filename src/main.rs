pub mod board;
pub mod consts;
pub mod geometry;
pub mod pattern;
pub mod pattern_search;
pub mod state;
pub mod testing;
pub mod threat_space_search;
pub mod todos;

use consts::{BLACK, WHITE};
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

    // let mut s = get_state(&["f6", "h6", "g7", "h7", "h8", "g11"], &["e5", "h5", "g6", "l6", "f7", "g8"], BLACK, true);

    // *** Working on adding Victoria's games from the 4th Computer Olympiad in London (5-11 August, 1992). ***

    // 1. Victoria (B) vs. Neuron (W)

    // let mut s = get_state(
    //     &["h8", "i7", "g9", "j6", "h6", "g6", "g8", "e8", "f7", "e6", "e5", "d6", "f4", "g4"],
    //     &["i9", "h7", "f10", "k5", "j8", "i6", "g7", "f8", "d9", "d5", "e7", "f6", "g3", "i4"],
    //     BLACK,
    //     true,
    // );

    // Potential Win Vars:
    // e4, e3
    // f5, e4, e3
    // e3, e4, d4
    // e7, c6, b6, e3, e4, d4

    // 2. Victoria (B) vs. Zero Club (W)

    // let mut s = get_state(
    //     &["h8", "g7", "f6", "g6", "i7", "f9"],
    //     &["h7", "i8", "i9", "i6", "g8", "f7"],
    //     BLACK,
    //     true,
    // );

    // Potential Win Vars:
    // e6, e5, e7, g5, g4, f5
    // e5, e6, e7, g5, g4, f5

    // 3. Victoria (B) vs. Xokk (W)

    // let mut s = get_state(
    //     &["h8", "g7", "i9", "h7", "h6", "i5"],
    //     &["g9", "f6", "j10", "i7", "h9", "f8"],
    //     BLACK,
    //     true,
    // );

    // Potential Win Vars:
    // h4, j6, i6, k4, j4
    // j4, h4, k4, i6, j6
    // j4, h4, j6, j5, k4
    // j4, h4, j6, j5, i6
    // j4, h4, j6, i6, k4
    // j4, h4, j6, i6, j5

    // 4. Neuron (B) vs. Victoria (W)

    // let mut s = get_state(
    //     &["h8", "g7", "i9", "i8", "f9", "i11", "h11", "h9", "j5"],
    //     &["h7", "f6", "g8", "i6", "g9", "i10", "j11", "k4", "j7"],
    //     BLACK,
    //     true,
    // );

    // Potential Win Vars:
    // j10, h12
    // h12, j10
    // i12, h12, j10
    // h10, g11, f11, g10
    // g10, f11, f12, h10
    // g10, f11, f12, g11
    // g10, f11, g11, f12
    // g10, f11, g11, h10
    // h10, f12, f11, g10, e8
    // h10, f12, f11, g10, i12
    // g10, f11, f10, g11, h12
    // g10, f11, f10, h12, j10
    // g10, f11, f10, h12, g11
    // g10, f11, e11, f12, g13, h12
    // g10, f11, e11, f12, g13, j10
    // g10, f11, f10, h12, g13, g11 (FAILS!)
    // g10, f11, g11, j8, l8, j10, k9

    // 5. Victoria (B) vs. Polygon (W)

    // let mut s = get_state(
    //     &["h8", "i7", "i6", "g6", "h6", "j8", "j7"],
    //     &["g7", "g9", "h9", "i8", "f6", "k9", "i9"],
    //     BLACK,
    //     true,
    // );

    // Potential Win Vars:
    // j9, k8, l7

    // 6. Xokk (B) vs. Victoria (W)

    // let mut s = get_state(
    //     &["h8", "g7", "f8", "e9", "d10", "g6", "g9", "j8", "g8", "d8", "f10"],
    //     &["h7", "f6", "i8", "h6", "c11", "g5", "i7", "f5", "g10", "e8"],
    //     WHITE,
    //     true,
    // );

    // Potential Win Vars:
    // f4, f3

    // WORKING HERE!
    let mut s = get_state(
        &["h8", "g7", "f8", "e9", "d10", "g6", "g9", "j8", "g8", "d8", "f10"],
        &["h7", "f6", "i8", "h6", "c11", "g5", "i7", "f5", "g10", "e8"],
        WHITE,
        true,
    );

    println!("{}", s);

    let n = 1;

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
    for v in potential_win_vars.iter() {
        println!("{:?}", variation_to_algebraic(v));
    }

    // animate_variation(&mut s.board, s.turn, &potential_win_vars[0]);

    // testing::test_pattern_search_fns();
}
