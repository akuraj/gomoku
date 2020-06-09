//! Functions related to the geometry of the problem.

use std::cmp::{min, max};
use num::{signum, abs};
use ndarray::prelude::*;
use std::collections::HashSet;

pub type Point = (i8, i8);

pub fn increment(d: i8) -> i8 {
    if d % 4 == 0 {
        return 0;
    } else if d % 8 < 4 {
        return 1
    } else {
        return -1;
    }
}

pub fn increments(d: i8) -> (i8, i8) {
    return (increment(d), increment(d + 2));
}

pub fn index_bounds(side: i8, length: i8, increment: i8) -> (i8, i8) {
    if length <= side {
        match increment {
            -1 => return (length - 1, side),
            0 => return (0, side),
            1 => return (0, side - length + 1),
            _ => panic!("Invalid increment!"),
        }
    } else {
        return (0, 0);
    }
}

pub fn index_bounds_incl(side: i8, length: i8, x: i8, y: i8, row_inc: i8, col_inc: i8) -> (i8, i8) {
    let mut row_b = side;
    let mut row_f = side;
    match row_inc {
        -1 => {row_f = x + 1; row_b = side - row_f;},
        0 => {},
        1 => {row_b = x; row_f = side - row_b;},
        _ => panic!("Invalid row_inc!"),
    }

    let mut col_b = side;
    let mut col_f = side;
    match col_inc {
        -1 => {col_f = y + 1; col_b = side - col_f;},
        0 => {},
        1 => {col_b = y; col_f = side - col_b;},
        _ => panic!("Invalid col_inc!"),
    }

    let back = min(row_b, col_b);
    let front = min(row_f, col_f);

    return (-min(back, length - 1), min(front, length) - (length - 1));
}

pub fn point_is_on_line(point: Point, start: Point, end: Point, segment_only: bool) -> bool {
    let dx1 = point.0 - start.0;
    let dy1 = point.1 - start.1;
    let dx2 = point.0 - end.0;
    let dy2 = point.1 - end.1;

    return (dx1 * dy2 == dx2 * dy1) && (!segment_only || (dx1 * dx2 <= 0 && dy1 * dy2 <= 0));
}

pub fn point_on_line(start: Point, end: Point, i: i8) -> Point {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    assert!(dx * dy == 0 || abs(dx) == abs(dy));

    return (start.1 + signum(dx) * i, start.1 + signum(dy) * i);
}

pub fn is_normal_line(start: Point, end: Point) -> bool {
    let adx = abs(end.0 - start.0);
    let ady = abs(end.1 - start.1);
    return (adx * ady == 0 || adx == ady) && (adx + ady > 0);
}

pub fn chebyshev_distance(start: Point, end: Point) -> i8 {
    let adx = abs(end.0 - start.0);
    let ady = abs(end.1 - start.1);
    return max(adx, ady);
}

pub fn point_set_on_line(start: Point, end: Point, idxs: &Array1<i8>) -> HashSet<Point> {
    let mut point_set: HashSet<Point> = HashSet::new();

    for idx in idxs.iter() {
        point_set.insert(point_on_line(start, end, *idx));
    }

    return point_set;
}

pub fn slope_intercept(start: Point, end: Point) -> (i8, i8, i8) {
    assert!(is_normal_line(start, end));

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    if dx == 0 {
        return (0, 1, -start.0);
    } else {
        let slope = signum(dx) * signum(dy);
        return (1, slope, start.1 - slope * start.0);
    }
}

pub fn point_idx_on_line(point: Point, line: (i8, i8, i8)) -> i8 {
    let (x, y) = point;

    assert!(line.0 * point.1 == line.1 * point.0 + line.2);

    if line.0 != 0 {
        return point.0;
    } else {
        return point.1;
    }
}
