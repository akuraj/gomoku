use consts::{SIDE_LEN_ACT, SIDE_LEN, EMPTY, WALL, BLACK, WHITE, COLORS,
             ACT_ELEMS_TO_CHRS, SPL_ELEM_CHR, RADIX};
use std::char;
use geometry::Point;
use ndarray::prelude::*;
use std::collections::HashSet;

pub fn row_idx_to_num(x: usize) -> usize {
    assert!(1 <= x && x <= SIDE_LEN_ACT);
    return SIDE_LEN_ACT + 1 - x;
}

pub fn row_num_to_idx(x: usize) -> usize {
    assert!(1 <= x && x <= SIDE_LEN_ACT);
    return SIDE_LEN_ACT + 1 - x;
}

pub fn col_idx_to_chr(x: usize) -> char {
    assert!(1 <= x && x <= SIDE_LEN_ACT);
    return char::from_u32('a'.to_digit(RADIX).unwrap() + (x as u32) - 1).unwrap();
}

pub fn col_chr_to_idx(x: char) -> usize {
    let idx = (x.to_digit(RADIX).unwrap() - 'a'.to_digit(RADIX).unwrap() + 1) as usize;
    assert!(1 <= idx && idx <= SIDE_LEN_ACT);
    return idx;
}

pub fn point_to_algebraic(x: Point) -> String {
    let row_num = row_idx_to_num(x.0 as usize);
    let col_chr = col_idx_to_chr(x.1 as usize);
    return format!("{}{}", row_num, col_chr);
}

pub fn algebraic_to_point(x: &str) -> Point {
    let c: Vec<char> = x.chars().collect();
    let col_idx = col_chr_to_idx(c[0]) as i8;
    let row_idx = row_num_to_idx((c[1]).to_digit(RADIX).unwrap() as usize) as i8;
    return (row_idx, col_idx);
}

pub fn new_board() -> Array2<u8> {
    let mut board: Array2<u8> = Array::from_elem((SIDE_LEN, SIDE_LEN), EMPTY);

    for wall in [0, SIDE_LEN - 1].iter() {
        for i in 0..SIDE_LEN {
            board[(*wall, i)] = WALL;
            board[(i, *wall)] = WALL;
        }
    }

    return board;
}

pub fn get_board(blacks: &[&str], whites: &[&str]) -> Array2<u8> {
    let blacks_set = blacks.iter().map(|&x| String::from(x)).collect::<HashSet<String>>();
    let whites_set = whites.iter().map(|&x| String::from(x)).collect::<HashSet<String>>();
    let common: HashSet<String> = blacks_set.intersection(&whites_set).cloned().collect();
    assert!(common.len() == 0);

    let mut board = new_board();

    for elem in blacks.iter() {
        let p = algebraic_to_point(elem);
        board[(p.0 as usize, p.1 as usize)] = BLACK;
    }

    for elem in whites.iter() {
        let p = algebraic_to_point(elem);
        board[(p.0 as usize, p.1 as usize)] = WHITE;
    }

    return board;
}

//     for elem in blacks:
//         board[algebraic_to_point(elem)] = BLACK

//     for elem in whites:
//         board[algebraic_to_point(elem)] = WHITE

//     return board


// def board_to_str(board):
//     """Representation of the board as a string."""

//     board_repr = ""

//     for i in range(board.shape[0]):
//         num_str = "  "
//         if 1 <= i <= SIDE_LEN_ACT:
//             num_str = str(row_idx_to_num(i))
//             if len(num_str) == 2:
//                 pass
//             elif len(num_str) == 1:
//                 num_str = " " + num_str
//             else:
//                 raise Exception(f"Invalid index: {i}!")

//         board_repr += num_str + " "

//         for j in range(board.shape[1]):
//             if board[i][j] in ACT_ELEMS_TO_CHRS:
//                 board_repr += ACT_ELEMS_TO_CHRS[board[i][j]]
//             else:
//                 board_repr += SPL_ELEM_CHR

//             board_repr += " "

//         board_repr += "\n"

//     board_repr += "     "

//     for i in range(1, SIDE_LEN_ACT + 1):
//         board_repr += col_idx_to_chr(i)
//         board_repr += " "

//     board_repr += "\n\n"

//     return board_repr


// @njit
// def set_sq(board, color, point):
//     """Sets given square on board to given color."""

//     assert color in COLORS
//     assert board[point] == EMPTY
//     board[point] = color


// @njit
// def clear_sq(board, color, point):
//     """Clears given square on board of given color."""

//     assert color in COLORS
//     assert board[point] == color
//     board[point] = EMPTY
