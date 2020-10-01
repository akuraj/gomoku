//! Implements a struct to represent State. Also implements related methods.

use crate::board::{board_to_str, get_board};
use crate::consts::{ACT_ELEMS_TO_NAMES, BLACK, EMPTY, SIDE_LEN, WALL, WHITE};
use crate::pattern::P_WIN;
use crate::pattern_search::search_board;
use ndarray::prelude::*;
use std::fmt;

/// Enum to represent the current status of the game.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Ongoing,
    BlackWon,
    WhiteWon,
}

/// Game State.
#[derive(Clone, Debug)]
pub struct State {
    pub board: Array2<u8>,
    pub turn: u8,
    pub status: Status,
}

impl State {
    pub fn new(board: Array2<u8>, turn: u8, strict_stone_count: bool) -> Self {
        // State Integrity Checks.
        let shape = board.shape();
        assert!(shape.len() == 2);
        assert!(shape[0] == SIDE_LEN && shape[1] == SIDE_LEN);
        assert!(turn == BLACK || turn == WHITE);

        let mut black_total: usize = 0;
        let mut white_total: usize = 0;

        for i in 0..SIDE_LEN {
            for j in 0..SIDE_LEN {
                match board[(i, j)] {
                    WALL => assert!(i == 0 || i == (SIDE_LEN - 1) || j == 0 || j == (SIDE_LEN - 1)),
                    BLACK => black_total += 1,
                    WHITE => white_total += 1,
                    EMPTY => (),
                    _ => panic!("Invalid item on board: {}", board[(i, j)]),
                }
            }
        }

        if strict_stone_count {
            match black_total - white_total {
                1 => assert_eq!(turn, WHITE),
                0 => assert_eq!(turn, BLACK),
                _ => panic!("Invalid number of stones: Black: {}, White: {}", black_total, white_total),
            }
        }

        // Calculate game status.
        let mut status = Status::Ongoing;
        let b_wins_found = search_board(&board, &P_WIN.pattern, BLACK);
        let w_wins_found = search_board(&board, &P_WIN.pattern, WHITE);
        let black_won = !b_wins_found.is_empty();
        let white_won = !w_wins_found.is_empty();

        if black_won && white_won {
            panic!("Both BLACK and WHITE cannot have won!");
        } else if black_won {
            status = Status::BlackWon;
            assert_eq!(turn, WHITE);
        } else if white_won {
            status = Status::WhiteWon;
            assert_eq!(turn, BLACK);
        }

        Self { board, turn, status }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("\nboard:{}", board_to_str(&self.board)));
        output.push_str(&format!("turn: {}\n", ACT_ELEMS_TO_NAMES.get(&self.turn).unwrap()));
        output.push_str(&format!("status: {:?}\n", self.status));

        write!(f, "{}", output)
    }
}

/// Return State object.
pub fn get_state(blacks: &[&str], whites: &[&str], turn: u8, strict_stone_count: bool) -> State {
    State::new(get_board(blacks, whites), turn, strict_stone_count)
}
