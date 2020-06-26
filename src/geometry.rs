//! Functions related to the geometry of the problem.

use num::{abs, signum};
use std::cmp::{max, min};
use std::collections::HashSet;

/// Represents a point on the board.
pub type Point = (isize, isize);

/// Row increment when moving in direction 'd'.
pub fn increment(d: usize) -> isize {
    if d % 4 == 0 {
        0
    } else if d % 8 < 4 {
        1
    } else {
        -1
    }
}

/// Row and Column increments when moving in direction 'd'.
pub fn increments(d: usize) -> (isize, isize) {
    (increment(d), increment(d + 2))
}

/// Possible starting indices of the pattern.
pub fn index_bounds(side: isize, length: isize, increment: isize) -> (isize, isize) {
    if length <= side {
        match increment {
            -1 => (length - 1, side),
            0 => (0, side),
            1 => (0, side - length + 1),
            _ => panic!("Invalid increment!"),
        }
    } else {
        (0, 0)
    }
}

/// Possible relative starting indices of the pattern.
pub fn index_bounds_incl(
    side: isize,
    length: isize,
    x: isize,
    y: isize,
    row_inc: isize,
    col_inc: isize,
) -> (isize, isize) {
    let mut row_b = side;
    let mut row_f = side;
    match row_inc {
        -1 => {
            row_f = x + 1;
            row_b = side - row_f;
        }
        0 => {}
        1 => {
            row_b = x;
            row_f = side - row_b;
        }
        _ => panic!("Invalid row_inc!"),
    }

    let mut col_b = side;
    let mut col_f = side;
    match col_inc {
        -1 => {
            col_f = y + 1;
            col_b = side - col_f;
        }
        0 => {}
        1 => {
            col_b = y;
            col_f = side - col_b;
        }
        _ => panic!("Invalid col_inc!"),
    }

    let back = min(row_b, col_b);
    let front = min(row_f, col_f);
    (-min(back, length - 1), min(front, length) - (length - 1))
}

/// Check if the point is on the given line/line segment.
pub fn point_is_on_line(point: Point, start: Point, end: Point, segment_only: bool) -> bool {
    let dx1 = point.0 - start.0;
    let dy1 = point.1 - start.1;
    let dx2 = point.0 - end.0;
    let dy2 = point.1 - end.1;
    (dx1 * dy2 == dx2 * dy1) && (!segment_only || (dx1 * dx2 <= 0 && dy1 * dy2 <= 0))
}

/// For lines along one of the 8 directions, returns the i'th point from start.
pub fn point_on_line(start: Point, end: Point, i: isize) -> Point {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    assert!(dx * dy == 0 || abs(dx) == abs(dy));
    (start.0 + signum(dx) * i, start.1 + signum(dy) * i)
}

/// Check if the two points represent a normal line.
/// A line along one of the 8 directions is a normal line.
pub fn is_normal_line(start: Point, end: Point) -> bool {
    let adx = abs(end.0 - start.0);
    let ady = abs(end.1 - start.1);
    (adx * ady == 0 || adx == ady) && (adx + ady > 0)
}

/// Chebyshev distance between the two points.
pub fn chebyshev_distance(start: Point, end: Point) -> isize {
    let adx = abs(end.0 - start.0);
    let ady = abs(end.1 - start.1);
    max(adx, ady)
}

/// Set of points, as specified by idxs, on the line spanning start and end.
/// See point_on_line for more info.
pub fn point_set_on_line(start: Point, end: Point, idxs: &[isize]) -> HashSet<Point> {
    idxs.iter()
        .map(|x| point_on_line(start, end, *x))
        .collect::<HashSet<Point>>()
}

/// Modified slope intercept form.
///
/// Returns (a, b, c), where,
///
/// a * y = b * x + c
///
/// Normalized such that a is 1 if line is not vertical,
/// and in the case of a vertical line, (a, b) = (0, 1).
pub fn slope_intercept(start: Point, end: Point) -> (isize, isize, isize) {
    assert!(is_normal_line(start, end));

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    if dx == 0 {
        (0, 1, -start.0)
    } else {
        let slope = signum(dx) * signum(dy);
        (1, slope, start.1 - slope * start.0)
    }
}

/// Unique identifier for a point on a given line.
///
/// (x, y) = point
///
/// Check the that point is on the given line,
/// and return x if line is not vertical, and y otherwise.
pub fn point_idx_on_line(point: Point, line: (isize, isize, isize)) -> isize {
    let (x, y) = point;
    assert!(line.0 * y == line.1 * x + line.2);
    if line.0 != 0 {
        x
    } else {
        y
    }
}
