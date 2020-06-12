use crate::board::{board_to_str, get_board};
use crate::consts::{ACT_ELEMS_TO_NAMES, BLACK, COLORS, EMPTY, SIDE_LEN, WALL, WHITE};
use crate::pattern::P_WIN;
use crate::pattern_search::search_board;
use ndarray::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    ONGOING,
    BLACK_WON,
    WHITE_WON,
}

#[derive(Clone, Debug)]
pub struct State {
    pub board: Array2<u8>,
    pub turn: u8,
    pub status: Status,
}

impl State {
    pub fn new(board: Array2<u8>, turn: u8, strict_stone_count: bool) -> Self {
        let shape = board.shape();
        assert!(shape.len() == 2);
        assert!(shape[0] == SIDE_LEN && shape[1] == SIDE_LEN);
        assert!(turn == BLACK || turn == WHITE);

        let mut black_total: usize = 0;
        let mut white_total: usize = 0;

        for i in 0..SIDE_LEN {
            for j in 0..SIDE_LEN {
                let val = board[(i, j)];
                if (i == 0 || i == (SIDE_LEN - 1)) || (j == 0 || j == (SIDE_LEN - 1)) {
                    assert_eq!(val, WALL);
                } else if val == BLACK {
                    black_total += 1;
                } else if val == WHITE {
                    white_total += 1;
                } else if val == EMPTY {
                } else {
                    panic!("Invalid item on board: {}", val);
                }
            }
        }

        if strict_stone_count {
            if black_total == white_total + 1 {
                assert_eq!(turn, WHITE);
            } else if black_total == white_total {
                assert_eq!(turn, BLACK);
            } else {
                panic!(
                    "Invalid number of stones: Black: {}, White: {}",
                    black_total, white_total
                );
            }
        }

        let mut status = Status::ONGOING;
        let b_wins_found = search_board(&board, &P_WIN.pattern, BLACK);
        let w_wins_found = search_board(&board, &P_WIN.pattern, WHITE);
        let black_won = !b_wins_found.is_empty();
        let white_won = !w_wins_found.is_empty();

        if black_won && white_won {
            panic!("Both BLACK and WHITE cannot have won!");
        } else if black_won {
            status = Status::BLACK_WON;
            assert_eq!(turn, WHITE);
        } else if white_won {
            status = Status::WHITE_WON;
            assert_eq!(turn, BLACK);
        }

        Self {
            board,
            turn,
            status,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("\nboard:\n{}", board_to_str(&self.board)));
        output.push_str(&format!(
            "turn: {}\n",
            ACT_ELEMS_TO_NAMES.get(&self.turn).unwrap()
        ));
        output.push_str(&format!("status: {:?}\n", self.status));

        write!(f, "{}", output)
    }
}

pub fn get_state(blacks: &[&str], whites: &[&str], turn: u8, strict_stone_count: bool) -> State {
    return State::new(get_board(blacks, whites), turn, strict_stone_count);
}
