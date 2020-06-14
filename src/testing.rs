use crate::board::new_board;
use crate::consts::{COLORS, EMPTY, NUM_DIRECTIONS, SIDE_LEN, WIN_LENGTH};
use crate::geometry::Point;
use crate::geometry::{increments, point_is_on_line, point_on_line};
use crate::pattern::PATTERNS;
use crate::pattern_search::{
    apply_pattern, get_pattern, idx, matches_are_equal, next_sq_matches_are_subset, search_board,
    search_board_next_sq, search_point, search_point_next_sq, search_point_own,
    search_point_own_next_sq, Match, NSQMatch,
};
use ndarray::prelude::*;


use std::time::Instant;

pub fn subtest_search_board(
    board: &Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    start: Point,
    end: Point,
) {
    let expected_matches: Vec<Match> = Vec::from([(start, end)]);
    let matches = search_board(board, gen_pattern, color);
    assert!(matches_are_equal(&matches, &expected_matches));
}

pub fn subtest_search_point(
    board: &Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    start: Point,
    end: Point,
) {
    let expected_matches: Vec<Match> = Vec::from([(start, end)]);
    for x in 0..SIDE_LEN {
        for y in 0..SIDE_LEN {
            let point = (x as isize, y as isize);
            let matches = search_point(board, gen_pattern, color, point);

            if point_is_on_line(point, start, end, true) {
                assert!(matches_are_equal(&matches, &expected_matches));
            } else {
                assert!(matches.is_empty());
            }
        }
    }
}

pub fn subtest_search_point_own(
    board: &Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    own_sqs: &[isize],
    start: Point,
    end: Point,
) {
    let expected_matches: Vec<Match> = Vec::from([(start, end)]);
    for x in 0..SIDE_LEN {
        for y in 0..SIDE_LEN {
            let point = (x as isize, y as isize);
            let matches = search_point_own(board, gen_pattern, color, point, own_sqs);
            if board[(x, y)] == color && point_is_on_line(point, start, end, true) {
                assert!(matches_are_equal(&matches, &expected_matches));
            } else {
                assert!(matches.is_empty());
            }
        }
    }
}

pub fn subtest_search_board_next_sq(
    board: &mut Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    own_sqs: &[isize],
    defcon: usize,
    start: Point,
    end: Point,
) {
    for own_sq in own_sqs {
        let test_sq = point_on_line(start, end, *own_sq);
        let expected_ns_matches: Vec<NSQMatch> = Vec::from([(test_sq, (start, end))]);

        let stored_val = board[(test_sq.0 as usize, test_sq.1 as usize)];
        board[(test_sq.0 as usize, test_sq.1 as usize)] = EMPTY;
        let ns_matches = search_board_next_sq(board, gen_pattern, color);
        board[(test_sq.0 as usize, test_sq.1 as usize)] = stored_val;

        assert!(next_sq_matches_are_subset(
            &expected_ns_matches,
            &ns_matches
        ));

        if WIN_LENGTH - defcon > 2 {
            for nsm in ns_matches {
                assert!(point_is_on_line(nsm.0, start, end, false));
            }
        }
    }
}

pub fn subtest_search_point_next_sq(
    board: &mut Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    own_sqs: &[isize],
    defcon: usize,
    start: Point,
    end: Point,
) {
    for own_sq in own_sqs {
        let test_sq = point_on_line(start, end, *own_sq);
        let expected_ns_matches: Vec<NSQMatch> = Vec::from([(test_sq, (start, end))]);

        for x in 0..SIDE_LEN {
            for y in 0..SIDE_LEN {
                let point = (x as isize, y as isize);

                let stored_val = board[(test_sq.0 as usize, test_sq.1 as usize)];
                board[(test_sq.0 as usize, test_sq.1 as usize)] = EMPTY;
                let ns_matches = search_point_next_sq(board, gen_pattern, color, point);
                board[(test_sq.0 as usize, test_sq.1 as usize)] = stored_val;

                if point_is_on_line(point, start, end, true) {
                    assert!(next_sq_matches_are_subset(
                        &expected_ns_matches,
                        &ns_matches
                    ));
                } else if point_is_on_line(point, start, end, false) {
                } else if WIN_LENGTH - defcon > 2 {
                    assert!(ns_matches.is_empty());
                }

                if WIN_LENGTH - defcon > 2 {
                    for nsm in ns_matches {
                        assert!(point_is_on_line(nsm.0, start, end, false));
                    }
                }
            }
        }
    }
}

pub fn subtest_search_point_own_next_sq(
    board: &mut Array2<u8>,
    gen_pattern: &[u8],
    color: u8,
    own_sqs: &[isize],
    defcon: usize,
    start: Point,
    end: Point,
) {
    for own_sq in own_sqs {
        let test_sq = point_on_line(start, end, *own_sq);
        let expected_ns_matches: Vec<NSQMatch> = Vec::from([(test_sq, (start, end))]);

        for x in 0..SIDE_LEN {
            for y in 0..SIDE_LEN {
                let point = (x as isize, y as isize);

                let stored_val = board[(test_sq.0 as usize, test_sq.1 as usize)];
                board[(test_sq.0 as usize, test_sq.1 as usize)] = EMPTY;
                let ns_matches =
                    search_point_own_next_sq(board, gen_pattern, color, point, own_sqs);
                let point_is_own_sq = board[(x, y)] == color;
                board[(test_sq.0 as usize, test_sq.1 as usize)] = stored_val;

                if point_is_own_sq && point_is_on_line(point, start, end, true) {
                    assert!(next_sq_matches_are_subset(
                        &expected_ns_matches,
                        &ns_matches
                    ));
                } else if point_is_own_sq && point_is_on_line(point, start, end, false) {
                } else if WIN_LENGTH - defcon > 2 {
                    assert!(ns_matches.is_empty());
                }

                if WIN_LENGTH - defcon > 2 {
                    for nsm in ns_matches {
                        assert!(point_is_on_line(nsm.0, start, end, false));
                    }
                }
            }
        }
    }
}

pub fn subtest_search_fns(gen_pattern: &[u8], color: u8, own_sqs: &[isize], defcon: usize) {
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    for i in 0..SIDE_LEN {
        for j in 0..SIDE_LEN {
            for d in 0..NUM_DIRECTIONS {
                let mut board = new_board();
                if apply_pattern(&mut board, &pattern, (i as isize, j as isize), d) {
                    let (row_inc, col_inc) = increments(d as isize);
                    let start = (i as isize, j as isize);
                    let end = (
                        idx(i as isize, row_inc, length - 1),
                        idx(j as isize, col_inc, length - 1),
                    );

                    subtest_search_board(&board, gen_pattern, color, start, end);
                    subtest_search_point(&board, gen_pattern, color, start, end);
                    subtest_search_point_own(&board, gen_pattern, color, own_sqs, start, end);

                    subtest_search_board_next_sq(
                        &mut board,
                        gen_pattern,
                        color,
                        own_sqs,
                        defcon,
                        start,
                        end,
                    );
                    subtest_search_point_next_sq(
                        &mut board,
                        gen_pattern,
                        color,
                        own_sqs,
                        defcon,
                        start,
                        end,
                    );
                    subtest_search_point_own_next_sq(
                        &mut board,
                        gen_pattern,
                        color,
                        own_sqs,
                        defcon,
                        start,
                        end,
                    );
                }
            }
        }
    }
}

pub fn test_search_fns() {
    let start = Instant::now();

    for color in COLORS.iter() {
        for p in PATTERNS.iter() {
            subtest_search_fns(&p.pattern, *color, &p.own_sqs, p.defcon);
        }
    }

    println!(
        "Time taken: {} seconds",
        (start.elapsed().as_nanos() as f32) / 1e9
    );
}

#[test]
pub fn test_search_fns_test() {
    test_search_fns();
}
