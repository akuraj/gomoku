//! Functions to search for patterns on the board.

#![allow(clippy::many_single_char_names)]

use crate::consts::{BLACK, EMPTY, MAX_DEFCON, NUM_DIRECTIONS, OWN, STONE, WHITE, WIN_LENGTH};
use crate::geometry::{increments, index_bounds, index_bounds_incl, Point};
use ndarray::prelude::*;
use std::cmp::max;

pub type Match = (Point, Point);
pub type NSQMatch = (Point, Match);

pub fn get_pattern(gen_pattern: &Array1<u8>, color: u8) -> Array1<u8> {
    let mut pattern = gen_pattern.to_owned();

    match color {
        BLACK => { pattern }
        WHITE => {
            for val in pattern.iter_mut() {
                *val = if *val & STONE == 0 || *val & STONE == STONE {
                    *val
                } else {
                    *val ^ STONE
                };
            }

            pattern
        }
        _ => panic!("Invalid color!"),
    }
}

pub fn dedupe_matches(matches: &mut Vec<Match>) {
    let mut i: usize = 0;
    let mut n = matches.len();

    if n > 0 {
        while i < n - 1 {
            let a = matches[i];
            let mut j = i + 1;

            while j < n {
                let b = matches[j];
                if b == a || (b.0 == a.1 && b.1 == a.0) {
                    matches.remove(j);
                    n -= 1;
                } else {
                    j += 1;
                }
            }

            i += 1;
        }
    }
}

#[inline(always)]
pub fn idx(start: isize, increment: isize, steps: usize) -> isize {
    start + increment * (steps as isize)
}

pub fn search_board(board: &Array2<u8>, gen_pattern: &Array1<u8>, color: u8) -> Vec<Match> {
    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut matches: Vec<Match> = Vec::new();
    for d in 0..NUM_DIRECTIONS {
        let (row_inc, col_inc) = increments(d as isize);
        let (row_min, row_max) = index_bounds(side as isize, length as isize, row_inc);
        let (col_min, col_max) = index_bounds(side as isize, length as isize, col_inc);

        for i in row_min..row_max {
            for j in col_min..col_max {
                let mut found = true;

                for k in 0..length {
                    if pattern[k]
                        & board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)]
                        == 0
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    let a = (i, j);
                    let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                    matches.push((a, b));
                }
            }
        }
    }

    dedupe_matches(&mut matches);
    matches
}

pub fn search_point(
    board: &Array2<u8>,
    gen_pattern: &Array1<u8>,
    color: u8,
    point: Point,
) -> Vec<Match> {
    let (x, y) = point;

    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut matches: Vec<Match> = Vec::new();
    for d in 0..NUM_DIRECTIONS {
        let (row_inc, col_inc) = increments(d as isize);
        let (s_min, s_max) =
            index_bounds_incl(side as isize, length as isize, x, y, row_inc, col_inc);

        for h in s_min..s_max {
            let (i, j) = (x + row_inc * h, y + col_inc * h);

            let mut found = true;

            for k in 0..length {
                if pattern[k] & board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)]
                    == 0
                {
                    found = false;
                    break;
                }
            }

            if found {
                let a = (i, j);
                let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                matches.push((a, b));
            }
        }
    }

    dedupe_matches(&mut matches);
    matches
}

pub fn search_point_own(
    board: &Array2<u8>,
    gen_pattern: &Array1<u8>,
    color: u8,
    point: Point,
    own_sqs: &Array1<isize>,
) -> Vec<Match> {
    let (x, y) = point;

    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut matches: Vec<Match> = Vec::new();

    if board[(x as usize, y as usize)] == color {
        for d in 0..NUM_DIRECTIONS {
            let (row_inc, col_inc) = increments(d as isize);
            let (s_min, s_max) =
                index_bounds_incl(side as isize, length as isize, x, y, row_inc, col_inc);

            for own_sq in own_sqs.iter() {
                if s_min <= (-*own_sq) && (-*own_sq) < s_max {
                    let (i, j) = (x - row_inc * (*own_sq), y - col_inc * (*own_sq));

                    let mut found = true;

                    for k in 0..length {
                        if pattern[k]
                            & board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)]
                            == 0
                        {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        let a = (i, j);
                        let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                        matches.push((a, b));
                    }
                }
            }
        }
    }

    dedupe_matches(&mut matches);
    matches
}

pub fn dedupe_next_sq_match_pairs(pairs: &mut Vec<NSQMatch>) {
    let mut i: usize = 0;
    let mut n = pairs.len();

    if n > 0 {
        while i < n - 1 {
            let a = pairs[i];
            let mut j = i + 1;

            while j < n {
                let b = pairs[j];
                if (a.0 == b.0) && (a.1 == b.1 || ((a.1).0 == (b.1).1 && (a.1).1 == (b.1).0)) {
                    pairs.remove(j);
                    n -= 1;
                } else {
                    j += 1;
                }
            }

            i += 1;
        }
    }
}

pub fn search_board_next_sq(
    board: &Array2<u8>,
    gen_pattern: &Array1<u8>,
    color: u8,
) -> Vec<NSQMatch> {
    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut next_sq_match_pairs: Vec<NSQMatch> = Vec::new();
    for d in 0..NUM_DIRECTIONS {
        let (row_inc, col_inc) = increments(d as isize);
        let (row_min, row_max) = index_bounds(side as isize, length as isize, row_inc);
        let (col_min, col_max) = index_bounds(side as isize, length as isize, col_inc);

        for i in row_min..row_max {
            for j in col_min..col_max {
                let mut found_next_sq = false;
                let mut k_next_sq: isize = -1;
                let mut found = true;

                for k in 0..length {
                    let p_val = pattern[k];
                    let b_val = board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)];

                    if p_val & b_val == 0 {
                        if !found_next_sq && p_val == color && b_val == EMPTY {
                            found_next_sq = true;
                            k_next_sq = k as isize;
                        } else {
                            found = false;
                            break;
                        }
                    }
                }

                if found && found_next_sq {
                    let a = (i, j);
                    let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                    let next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq);
                    next_sq_match_pairs.push((next_sq, (a, b)))
                }
            }
        }
    }

    dedupe_next_sq_match_pairs(&mut next_sq_match_pairs);
    next_sq_match_pairs
}

pub fn search_point_next_sq(
    board: &Array2<u8>,
    gen_pattern: &Array1<u8>,
    color: u8,
    point: Point,
) -> Vec<NSQMatch> {
    let (x, y) = point;

    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut next_sq_match_pairs: Vec<NSQMatch> = Vec::new();
    for d in 0..NUM_DIRECTIONS {
        let (row_inc, col_inc) = increments(d as isize);
        let (s_min, s_max) =
            index_bounds_incl(side as isize, length as isize, x, y, row_inc, col_inc);

        for h in s_min..s_max {
            let (i, j) = (x + row_inc * h, y + col_inc * h);

            let mut found_next_sq = false;
            let mut k_next_sq: isize = -1;
            let mut found = true;

            for k in 0..length {
                let p_val = pattern[k];
                let b_val = board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)];

                if p_val & b_val == 0 {
                    if !found_next_sq && p_val == color && b_val == EMPTY {
                        found_next_sq = true;
                        k_next_sq = k as isize;
                    } else {
                        found = false;
                        break;
                    }
                }
            }

            if found && found_next_sq {
                let a = (i, j);
                let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                let next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq);
                next_sq_match_pairs.push((next_sq, (a, b)))
            }
        }
    }

    dedupe_next_sq_match_pairs(&mut next_sq_match_pairs);
    next_sq_match_pairs
}

pub fn search_point_own_next_sq(
    board: &Array2<u8>,
    gen_pattern: &Array1<u8>,
    color: u8,
    point: Point,
    own_sqs: &Array1<isize>,
) -> Vec<NSQMatch> {
    let (x, y) = point;

    let side = board.shape()[0];
    let pattern = get_pattern(gen_pattern, color);
    let length = pattern.len();

    let mut next_sq_match_pairs: Vec<NSQMatch> = Vec::new();

    if board[(x as usize, y as usize)] == color {
        for d in 0..NUM_DIRECTIONS {
            let (row_inc, col_inc) = increments(d as isize);
            let (s_min, s_max) =
                index_bounds_incl(side as isize, length as isize, x, y, row_inc, col_inc);

            for own_sq in own_sqs.iter() {
                if s_min <= (-*own_sq) && (-*own_sq) < s_max {
                    let (i, j) = (x - row_inc * (*own_sq), y - col_inc * (*own_sq));

                    let mut found_next_sq = false;
                    let mut k_next_sq: isize = -1;
                    let mut found = true;

                    for k in 0..length {
                        let p_val = pattern[k];
                        let b_val =
                            board[(idx(i, row_inc, k) as usize, idx(j, col_inc, k) as usize)];

                        if p_val & b_val == 0 {
                            if !found_next_sq && p_val == color && b_val == EMPTY {
                                found_next_sq = true;
                                k_next_sq = k as isize;
                            } else {
                                found = false;
                                break;
                            }
                        }
                    }

                    if found && found_next_sq {
                        let a = (i, j);
                        let b = (idx(i, row_inc, length - 1), idx(j, col_inc, length - 1));
                        let next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq);
                        next_sq_match_pairs.push((next_sq, (a, b)))
                    }
                }
            }
        }
    }

    dedupe_next_sq_match_pairs(&mut next_sq_match_pairs);
    next_sq_match_pairs
}

pub fn apply_pattern(board: &mut Array2<u8>, pattern: &Array1<u8>, point: Point, d: usize) -> bool {
    let (x, y) = point;

    let side = board.shape()[0];
    let length = pattern.len();

    let (row_inc, col_inc) = increments(d as isize);

    let mut can_apply = true;
    for k in 0..length {
        let (i, j) = (idx(x, row_inc, k) as usize, idx(y, col_inc, k) as usize);
        if side <= i || side <= j {
            can_apply = false;
            break;
        }
    }

    if can_apply {
        for k in 0..length {
            board[(idx(x, row_inc, k) as usize, idx(y, col_inc, k) as usize)] = pattern[k];
        }
    }

    can_apply
}

pub fn matches_are_subset(x: &[Match], y: &[Match]) -> bool {
    for a_ref in x.iter() {
        let a = *a_ref;
        let mut found = false;
        for b_ref in y.iter() {
            let b = *b_ref;
            if a == b || (a.0 == b.1 && a.1 == b.0) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    true
}

pub fn matches_are_equal(x: &[Match], y: &[Match]) -> bool {
    matches_are_subset(x, y) && matches_are_subset(y, x)
}

pub fn next_sq_matches_are_subset(x: &[NSQMatch], y: &[NSQMatch]) -> bool {
    for a_ref in x.iter() {
        let a = *a_ref;
        let mut found = false;
        for b_ref in y.iter() {
            let b = *b_ref;
            if a.0 == b.0 && (a.1 == b.1 || ((a.1).0 == (b.1).1 && (a.1).1 == (b.1).0)) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    true
}

pub fn next_sq_matches_are_equal(x: &[NSQMatch], y: &[NSQMatch]) -> bool {
    next_sq_matches_are_subset(x, y) && next_sq_matches_are_subset(y, x)
}

pub fn degree(gen_pattern: &Array1<u8>) -> usize {
    let n = gen_pattern.len();
    let mut max_owns: usize = 0;

    for i in 0..(n - WIN_LENGTH + 1) {
        let mut owns: usize = 0;
        let mut found = true;

        for j in 0..WIN_LENGTH {
            let gp_val = gen_pattern[i + j];

            if gp_val != OWN && gp_val != EMPTY {
                found = false;
                break;
            }

            if gp_val == OWN {
                owns += 1;
            }
        }

        if found {
            max_owns = max(max_owns, owns);
        }
    }

    max_owns
}

pub fn defcon_from_degree(d: usize) -> usize {
    MAX_DEFCON - d
}

#[allow(clippy::collapsible_if)]
pub fn one_step_from_straight_threat(gen_pattern: &Array1<u8>) -> bool {
    let n = gen_pattern.len();
    let l = WIN_LENGTH + 1;

    for idx in 0..n {
        let v = gen_pattern[idx];

        if v == EMPTY {
            for i in 0..(n - l + 1) {
                let mut found = true;

                for j in 0..l {
                    let value = if j == 0 || j == (l - 1) { EMPTY } else { OWN };

                    if i + j == idx {
                        if value != OWN {
                            found = false;
                            break;
                        }
                    } else {
                        if value != gen_pattern[i + j] {
                            found = false;
                            break;
                        }
                    }
                }

                if found {
                    return true;
                }
            }
        }
    }

    false
}
