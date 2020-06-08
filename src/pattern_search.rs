//! Functions to search for patterns on the board.

use ndarray::prelude::*;
use consts::{BLACK, WHITE, EMPTY, STONE, NUM_DIRECTIONS, WIN_LENGTH, OWN, MAX_DEFCON};
use geometry::{increments, index_bounds, index_bounds_incl};

pub fn get_pattern(gen_pattern: &Array1<u8>, color: u8) -> Array1<u8> {
    let mut pattern = gen_pattern.to_owned();

    match color {
        BLACK => return pattern,
        WHITE => {
            for val in pattern.iter_mut() {
                *val = if *val & STONE == 0 || *val & STONE == STONE { *val } else { *val ^ STONE };
            }

            return pattern;
        },
        _ => panic!("Invalid color!"),
    }
}



// @njit
// def dedupe_matches(matches):
//     """Remove duplicates from matches."""

//     i = 0
//     n = len(matches)

//     while i < n - 1:
//         val = matches[i]
//         j = i + 1

//         while j < n:
//             if matches[j] == val or matches[j] == val[::-1]:
//                 del matches[j]
//                 n -= 1
//             else:
//                 j += 1

//         i += 1


// @njit
// def search_board(board, gen_pattern, color):
//     """Search for a 1d pattern on a 2d board."""

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     matches = []
//     for d in range(NUM_DIRECTIONS):
//         (row_inc, col_inc) = increments(d)
//         (row_min, row_max) = index_bounds(side, length, row_inc)
//         (col_min, col_max) = index_bounds(side, length, col_inc)

//         for i in range(row_min, row_max):
//             for j in range(col_min, col_max):
//                 for k in range(length):
//                     if not pattern[k] & board[i + row_inc * k, j + col_inc * k]:
//                         break
//                 else:
//                     # Store Ordered Line Segment, a -> b, where the pattern lies.
//                     a = (i, j)
//                     b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                     matches.append((a, b))

//     dedupe_matches(matches)
//     return matches


// @njit
// def search_point(board, gen_pattern, color, point):
//     """Search for a 1d pattern on a 2d board including the given point."""

//     (x, y) = point

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     matches = []
//     for d in range(NUM_DIRECTIONS):
//         (row_inc, col_inc) = increments(d)
//         (s_min, s_max) = index_bounds_incl(side, length, x, y, row_inc, col_inc)

//         for h in range(s_min, s_max):
//             (i, j) = (x + row_inc * h, y + col_inc * h)

//             for k in range(length):
//                 if not pattern[k] & board[i + row_inc * k, j + col_inc * k]:
//                     break
//             else:
//                 # Store Ordered Line Segment, a -> b, where the pattern lies.
//                 a = (i, j)
//                 b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                 matches.append((a, b))

//     dedupe_matches(matches)
//     return matches


// @njit
// def search_point_own(board, gen_pattern, color, point, own_sqs):
//     """Search for a 1d pattern on a 2d board including the given point as an own_sq."""

//     (x, y) = point

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     matches = []

//     # We are searching for patterns including the given point as an "own_sq".
//     if board[point] == color:
//         for d in range(NUM_DIRECTIONS):
//             (row_inc, col_inc) = increments(d)
//             (s_min, s_max) = index_bounds_incl(side, length, x, y, row_inc, col_inc)

//             for own_sq in own_sqs:
//                 if s_min <= -own_sq < s_max:
//                     (i, j) = (x - row_inc * own_sq, y - col_inc * own_sq)

//                     for k in range(length):
//                         if not pattern[k] & board[i + row_inc * k, j + col_inc * k]:
//                             break
//                     else:
//                         # Store Ordered Line Segment, a -> b, where the pattern lies.
//                         a = (i, j)
//                         b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                         matches.append((a, b))

//     dedupe_matches(matches)
//     return matches


// @njit
// def dedupe_next_sq_match_pairs(pairs):
//     """Remove duplicates from next_sq_matche pairs."""

//     i = 0
//     n = len(pairs)

//     while i < n - 1:
//         a = pairs[i]
//         j = i + 1

//         while j < n:
//             b = pairs[j]
//             if a[0] == b[0] and (a[1] == b[1] or a[1] == b[1][::-1]):
//                 del pairs[j]
//                 n -= 1
//             else:
//                 j += 1

//         i += 1


// @njit
// def search_board_next_sq(board, gen_pattern, color):
//     """Search for a 1d pattern on a 2d board.

//     Returns "next_sq"s and the corresponding pattern matches (as in above functions)
//     as a list of (next_sq, match) pairs.

//     In existing terminology, "point" is a "rest" square,
//     and "next_sq" is the "gain" square.
//     """

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     next_sq_match_pairs = []
//     for d in range(NUM_DIRECTIONS):
//         (row_inc, col_inc) = increments(d)
//         (row_min, row_max) = index_bounds(side, length, row_inc)
//         (col_min, col_max) = index_bounds(side, length, col_inc)

//         for i in range(row_min, row_max):
//             for j in range(col_min, col_max):
//                 found_next_sq = False
//                 k_next_sq = -1

//                 for k in range(length):
//                     p_val = pattern[k]
//                     b_val = board[i + row_inc * k, j + col_inc * k]

//                     if not p_val & b_val:
//                         if not found_next_sq and p_val == color and b_val == EMPTY:
//                             found_next_sq = True
//                             k_next_sq = k
//                         else:
//                             break
//                 else:
//                     if found_next_sq:
//                         # Store Ordered Line Segment, a -> b, where the pattern lies.
//                         a = (i, j)
//                         b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                         next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq)
//                         next_sq_match_pairs.append((next_sq, (a, b)))

//     dedupe_next_sq_match_pairs(next_sq_match_pairs)
//     return next_sq_match_pairs


// @njit
// def search_point_next_sq(board, gen_pattern, color, point):
//     """Search for a 1d pattern on a 2d board including the given point.

//     Returns "next_sq"s and the corresponding pattern matches (as in above functions)
//     as a list of (next_sq, match) pairs.

//     In existing terminology, "point" is a "rest" square,
//     and "next_sq" is the "gain" square.
//     """

//     (x, y) = point

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     next_sq_match_pairs = []
//     for d in range(NUM_DIRECTIONS):
//         (row_inc, col_inc) = increments(d)
//         (s_min, s_max) = index_bounds_incl(side, length, x, y, row_inc, col_inc)

//         for h in range(s_min, s_max):
//             (i, j) = (x + row_inc * h, y + col_inc * h)

//             found_next_sq = False
//             k_next_sq = -1

//             for k in range(length):
//                 p_val = pattern[k]
//                 b_val = board[i + row_inc * k, j + col_inc * k]

//                 if not p_val & b_val:
//                     if not found_next_sq and p_val == color and b_val == EMPTY:
//                         found_next_sq = True
//                         k_next_sq = k
//                     else:
//                         break
//             else:
//                 if found_next_sq:
//                     # Store Ordered Line Segment, a -> b, where the pattern lies.
//                     a = (i, j)
//                     b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                     next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq)
//                     next_sq_match_pairs.append((next_sq, (a, b)))

//     dedupe_next_sq_match_pairs(next_sq_match_pairs)
//     return next_sq_match_pairs


// @njit
// def search_point_own_next_sq(board, gen_pattern, color, point, own_sqs):
//     """Search for a 1d pattern on a 2d board including the given point as an own_sq.

//     Returns "next_sq"s and the corresponding pattern matches (as in above functions)
//     as a list of (next_sq, match) pairs.

//     In existing terminology, "point" is a "rest" square,
//     and "next_sq" is the "gain" square.
//     """

//     (x, y) = point

//     side = board.shape[0]
//     pattern = get_pattern(gen_pattern, color)
//     length = pattern.size

//     next_sq_match_pairs = []

//     # We are searching for patterns including the given point as an "own_sq".
//     if board[point] == color:
//         for d in range(NUM_DIRECTIONS):
//             (row_inc, col_inc) = increments(d)
//             (s_min, s_max) = index_bounds_incl(side, length, x, y, row_inc, col_inc)

//             for own_sq in own_sqs:
//                 if s_min <= -own_sq < s_max:
//                     (i, j) = (x - row_inc * own_sq, y - col_inc * own_sq)

//                     found_next_sq = False
//                     k_next_sq = -1

//                     for k in range(length):
//                         p_val = pattern[k]
//                         b_val = board[i + row_inc * k, j + col_inc * k]

//                         if not p_val & b_val:
//                             if not found_next_sq and p_val == color and b_val == EMPTY:
//                                 found_next_sq = True
//                                 k_next_sq = k
//                             else:
//                                 break
//                     else:
//                         if found_next_sq:
//                             # Store Ordered Line Segment, a -> b, where the pattern lies.
//                             a = (i, j)
//                             b = (i + row_inc * (length - 1), j + col_inc * (length - 1))
//                             next_sq = (i + row_inc * k_next_sq, j + col_inc * k_next_sq)
//                             next_sq_match_pairs.append((next_sq, (a, b)))

//     dedupe_next_sq_match_pairs(next_sq_match_pairs)
//     return next_sq_match_pairs


// @njit
// def apply_pattern(board, pattern, point, d):
//     """Apply given pattern at given point in given direction.

//     Returns True if application was succesful.
//     Else, returns False.
//     If application fails then board is unchanged.

//     We only check that the length fits at that point.
//     We will apply a non-standard element as it is,
//     including overwriting a wall with any element whatsoever.

//     Useful for testing purposes.
//     """

//     (x, y) = point

//     side = board.shape[0]
//     length = pattern.size

//     (row_inc, col_inc) = increments(d)

//     can_apply = True
//     for k in range(length):
//         (i, j) = (x + row_inc * k, y + col_inc * k)
//         if i not in range(side) or j not in range(side):
//             can_apply = False
//             break

//     if can_apply:
//         for k in range(length):
//             # NOTE: If it's a non-standard element, we just apply it as is.
//             board[x + row_inc * k, y + col_inc * k] = pattern[k]

//     return can_apply


// @njit
// def matches_are_subset(x, y):
//     """Check if x is a subset of y."""

//     for a in x:
//         for b in y:
//             if a in (b, b[::-1]):
//                 break
//         else:
//             return False

//     return True


// @njit
// def matches_are_equal(x, y):
//     # pylint: disable=W1114
//     return matches_are_subset(x, y) and matches_are_subset(y, x)


// @njit
// def next_sq_matches_are_subset(x, y):
//     """Check if x is a subset of y."""

//     for a in x:
//         for b in y:
//             if a[0] == b[0] and a[1] in (b[1], b[1][::-1]):
//                 break
//         else:
//             return False

//     return True


// @njit
// def next_sq_matches_are_equal(x, y):
//     # pylint: disable=W1114
//     return next_sq_matches_are_subset(x, y) and next_sq_matches_are_subset(y, x)


// @njit
// def degree(gen_pattern):
//     """Maximum number of 'OWN's in a sub-sequence of length = WIN_LENGTH,
//     full of OWN/EMPTY sqs."""

//     n = len(gen_pattern)
//     max_owns = 0
//     for i in range(n - WIN_LENGTH + 1):
//         owns = 0
//         for j in range(WIN_LENGTH):
//             if gen_pattern[i + j] not in (OWN, EMPTY):
//                 break

//             if gen_pattern[i + j] == OWN:
//                 owns += 1
//         else:
//             max_owns = max(max_owns, owns)

//     return max_owns


// @njit
// def defcon_from_degree(d):
//     return MAX_DEFCON - d


// @njit
// def one_step_from_straight_threat(gen_pattern):
//     """True if a straight threat (unstoppable: a straight four for example)
//     can be achieved in one more move."""

//     n = len(gen_pattern)

//     # Length of a straight threat: WIN_LENGTH - 1 OWN's in a row,
//     # with an empty space on either side.
//     l = WIN_LENGTH + 1

//     for idx, v in enumerate(gen_pattern):
//         if v == EMPTY:
//             for i in range(n - l + 1):
//                 for j in range(l):
//                     # Straight threat pattern value.
//                     value = EMPTY if j in (0, l - 1) else OWN

//                     if i + j == idx:
//                         if value != OWN:
//                             break
//                     else:
//                         if value != gen_pattern[i + j]:
//                             break
//                 else:
//                     return True

//     return False
