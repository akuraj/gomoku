//! Functions related to the geometry of the problem.

use std::cmp::min;

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

// def point_is_on_line(point, start, end, segment_only):
//     dx1 = point[0] - start[0]
//     dy1 = point[1] - start[1]
//     dx2 = point[0] - end[0]
//     dy2 = point[1] - end[1]
//     return dx1 * dy2 == dx2 * dy1 and (not segment_only or (dx1 * dx2 <= 0 and dy1 * dy2 <= 0))

// @njit
// def point_on_line(start, end, i):
//     """For lines along one of the 8 directions, returns the i'th point from start."""

//     dx = end[0] - start[0]
//     dy = end[1] - start[1]
//     assert dx * dy == 0 or abs(dx) == abs(dy)
//     return (start[0] + signum(dx) * i, start[1] + signum(dy) * i)


// @njit
// def is_normal_line(start, end):
//     """A line along one of the 8 directions is a normal line."""

//     adx = abs(end[0] - start[0])
//     ady = abs(end[1] - start[1])
//     return (adx * ady == 0 or adx == ady) and adx + ady > 0


// @njit
// def chebyshev_distance(start, end):
//     adx = abs(end[0] - start[0])
//     ady = abs(end[1] - start[1])
//     return max(adx, ady)


// @njit
// def point_set_on_line(start, end, idxs):
//     """ Set of points, as specified by idxs, on the line spanning start and end.

//     See point_on_line for more info.
//     """

//     # pylint: disable=R1718

//     return set([point_on_line(start, end, i) for i in idxs])


// @njit
// def slope_intercept(start, end):
//     """Modified slope intercept form.

//     Returns (a, b, c), where,

//     a * y = b * x + c

//     Normalized such that a is 1 if line is not vertical,
//     and in the case of a vertical line, (a, b) = (0, 1).
//     """

//     assert is_normal_line(start, end)

//     dx = end[0] - start[0]
//     dy = end[1] - start[1]

//     if dx == 0:
//         return (0, 1, -start[0])
//     else:
//         slope = signum(dx) * signum(dy)
//         return (1, slope, start[1] - slope * start[0])


// @njit
// def point_idx_on_line(point, line):
//     """Unique identifier for a point on a given line.

//     (x, y) = point

//     Check the that point is on the given line,
//     and return x if line is not vertical, and y otherwise.
//     """

//     assert line[0] * point[1] == line[1] * point[0] + line[2]
//     return point[0] if line[0] else point[1]
