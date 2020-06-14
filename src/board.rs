//! Functions related to the board and its representation.

use crate::consts::{
    ACT_ELEMS_TO_CHRS, BLACK, EMPTY, RADIX, SIDE_LEN, SIDE_LEN_ACT, SPL_ELEM_CHR, WALL, WHITE,
};
use crate::geometry::Point;
use ndarray::prelude::*;
use std::char;
use std::collections::HashSet;

/// Get the display row number from the internal row index.
pub fn row_idx_to_num(x: usize) -> usize {
    assert!(1 <= x && x <= SIDE_LEN_ACT);
    SIDE_LEN_ACT + 1 - x
}

/// Get the internal row index from the display row number.
pub use row_idx_to_num as row_num_to_idx;

/// Get the display column character from the internal column number.
pub fn col_idx_to_chr(x: usize) -> char {
    assert!(1 <= x && x <= SIDE_LEN_ACT);
    char::from_u32(97 + (x as u32) - 1).unwrap()
}

/// Get the internal column number from the display column character.
pub fn col_chr_to_idx(x: char) -> usize {
    let idx = (x.to_digit(RADIX).unwrap() - 'a'.to_digit(RADIX).unwrap() + 1) as usize;
    assert!(1 <= idx && idx <= SIDE_LEN_ACT);
    idx
}

/// Get algebraic representation of point.
pub fn point_to_algebraic(x: Point) -> String {
    let row_num = row_idx_to_num(x.0 as usize);
    let col_chr = col_idx_to_chr(x.1 as usize);
    format!("{}{}", row_num, col_chr)
}

/// Get the point from its algebraic representation.
pub fn algebraic_to_point(x: &str) -> Point {
    let col_idx = col_chr_to_idx(x.chars().next().unwrap()) as isize;
    let row_num: usize = x.chars().skip(1).collect::<String>().parse().unwrap();
    let row_idx = row_num_to_idx(row_num) as isize;
    (row_idx, col_idx)
}

/// Get new board.
pub fn new_board() -> Array2<u8> {
    let mut board: Array2<u8> = Array::from_elem((SIDE_LEN, SIDE_LEN), EMPTY);

    for wall in [0, SIDE_LEN - 1].iter() {
        for i in 0..SIDE_LEN {
            board[(*wall, i)] = WALL;
            board[(i, *wall)] = WALL;
        }
    }

    board
}

/// Get board from lists of points of blacks and whites.
pub fn get_board(blacks: &[&str], whites: &[&str]) -> Array2<u8> {
    let blacks_set = blacks
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>();
    let whites_set = whites
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>();
    let common: HashSet<String> = blacks_set.intersection(&whites_set).cloned().collect();
    assert!(common.is_empty());

    let mut board = new_board();

    for elem in blacks.iter() {
        let p = algebraic_to_point(elem);
        board[(p.0 as usize, p.1 as usize)] = BLACK;
    }

    for elem in whites.iter() {
        let p = algebraic_to_point(elem);
        board[(p.0 as usize, p.1 as usize)] = WHITE;
    }

    board
}

/// Representation of the board as a string.
pub fn board_to_str(board: &Array2<u8>) -> String {
    let shape = board.shape();

    let mut board_repr = String::new();
    board_repr.push('\n');

    for i in 0..shape[0] {
        let mut num_str = String::from("  ");
        if 1 <= i && i <= SIDE_LEN_ACT {
            num_str = row_idx_to_num(i).to_string();
            match num_str.len() {
                2 => {}
                1 => {
                    let temp_str = String::from(&num_str);
                    num_str = String::from(" ");
                    num_str.push_str(&temp_str);
                }
                _ => panic!("Invalid index: {}", i),
            }
        }

        board_repr.push_str(&num_str);
        board_repr.push(' ');

        for j in 0..shape[1] {
            let val = board[(i, j)];
            if ACT_ELEMS_TO_CHRS.contains_key(&val) {
                board_repr.push(*ACT_ELEMS_TO_CHRS.get(&val).unwrap());
            } else {
                board_repr.push(SPL_ELEM_CHR);
            }

            board_repr.push(' ');
        }

        board_repr.push('\n');
    }

    board_repr.push_str("     ");

    for i in 1..(SIDE_LEN_ACT + 1) {
        board_repr.push(col_idx_to_chr(i));
        board_repr.push(' ');
    }

    board_repr.push_str("\n\n");

    board_repr
}

/// Sets the given square on the board to the given color.
pub fn set_sq(board: &mut Array2<u8>, color: u8, point: Point) {
    assert!(color == BLACK || color == WHITE);
    let p = (point.0 as usize, point.1 as usize);
    assert_eq!(board[p], EMPTY);
    board[p] = color;
}

/// Clears the given square on the board of the given color.
pub fn clear_sq(board: &mut Array2<u8>, color: u8, point: Point) {
    assert!(color == BLACK || color == WHITE);
    let p = (point.0 as usize, point.1 as usize);
    assert_eq!(board[p], color);
    board[p] = EMPTY;
}
