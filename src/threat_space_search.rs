//! Implements Threat Space Search.

use crate::board::{board_to_str, clear_sq, set_sq};
use crate::consts::{ANIMATION_TIMESTEP_SECS, STONE};
use crate::geometry::Point;
use crate::pattern::Threat;
use crate::pattern::{
    search_all_board, search_all_board_get_next_sqs, search_all_point_own,
    search_all_point_own_get_next_sqs, ThreatPri,
};
use ndarray::prelude::*;
// use rayon::prelude::*;
use crate::board::point_to_algebraic;
use fnv::FnvHashSet;
use reduce::Reduce;
use std::thread;
use std::time::Duration;

// @unique
// class SearchStatus(IntEnum):
//     """Enum to represent the status of threat space search."""

//     QUIET = auto()
//     UNQUIET = auto()
//     WIN = auto()
//     LOSS = auto()

/// A tree that represents the result of a Threat Space Search.
#[derive(Clone, Debug)]
pub struct SearchNode {
    pub next_sq: Option<Point>,
    pub threats: Vec<Threat>,
    pub critical_sqs: Option<FnvHashSet<Point>>,
    pub potential_win: bool,
    pub children: Vec<SearchNode>,
}

impl SearchNode {
    #[inline(always)]
    pub fn new(
        next_sq: Option<Point>,
        threats: Vec<Threat>,
        critical_sqs: Option<FnvHashSet<Point>>,
        potential_win: bool,
        children: Vec<SearchNode>,
    ) -> Self {
        Self {
            next_sq,
            threats,
            critical_sqs,
            potential_win,
            children,
        }
    }
}

/// Threat Space Search for a given next_sq.
pub fn tss_next_sq(board: &mut Array2<u8>, color: u8, next_sq: Point) -> SearchNode {
    set_sq(board, color, next_sq);

    let threats = search_all_point_own(board, color, next_sq, ThreatPri::Immediate);
    let num_threats = threats.len();
    let critical_sqs: FnvHashSet<Point> = if num_threats > 0 {
        threats
            .iter()
            .map(|x| x.critical_sqs.to_owned())
            .reduce(|a, b| a.intersection(&b).copied().collect::<FnvHashSet<Point>>())
            .unwrap()
    } else {
        FnvHashSet::<Point>::default()
    };

    let mut potential_win = num_threats > 0 && critical_sqs.is_empty();
    let mut children = Vec::<SearchNode>::new();

    // If the opponent is potentially winning by playing at one of the critical_sqs,
    // then this variation is assumed to not be winning.
    for csq in critical_sqs.iter() {
        set_sq(board, color ^ STONE, *csq);

        let threats_csq = search_all_point_own(board, color ^ STONE, *csq, ThreatPri::Immediate);
        let num_threats_csq = threats_csq.len();
        let critical_sqs_csq: FnvHashSet<Point> = if num_threats_csq > 0 {
            threats_csq
                .iter()
                .map(|x| x.critical_sqs.to_owned())
                .reduce(|a, b| a.intersection(&b).copied().collect::<FnvHashSet<Point>>())
                .unwrap()
        } else {
            FnvHashSet::<Point>::default()
        };

        let potential_win_csq = num_threats_csq > 0 && critical_sqs_csq.is_empty();

        clear_sq(board, color ^ STONE, *csq);

        if potential_win_csq {
            potential_win = false;
            clear_sq(board, color, next_sq);
            return SearchNode::new(
                Some(next_sq),
                threats,
                Some(critical_sqs),
                potential_win,
                children,
            );
        }
    }

    // If next_sq produces no threats or we've found a potential win, we stop.
    if num_threats > 0 && !potential_win {
        for csq in critical_sqs.iter() {
            set_sq(board, color ^ STONE, *csq);
        }

        let nsqs = search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::Immediate);
        children = nsqs.iter().map(|x| tss_next_sq(board, color, *x)).collect();
        potential_win = children.iter().any(|x| x.potential_win);

        if !potential_win {
            let nsqs_other =
                search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::NonImmediate);
            let children_other: Vec<SearchNode> = nsqs_other
                .iter()
                .map(|x| tss_next_sq(board, color, *x))
                .collect();
            potential_win = children_other.iter().any(|x| x.potential_win);
            children.extend(children_other);
        }

        for csq in critical_sqs.iter() {
            clear_sq(board, color ^ STONE, *csq);
        }
    }

    clear_sq(board, color, next_sq);
    SearchNode::new(
        Some(next_sq),
        threats,
        Some(critical_sqs),
        potential_win,
        children,
    )
}

// # # TODO: Change name of fn.
// # def search_status(threats_own, threats_opp):
// #     md_own = reduce(min, [t["defcon"] for t in threats_own], MAX_DEFCON)
// #     md_opp = reduce(min, [t["defcon"] for t in threats_opp], MAX_DEFCON)

// #     if md_opp == MAX_DEFCON:
// #         if md_own == MAX_DEFCON:
// #             return (SearchStatus.QUIET, [])
// #         else:
// #             return (SearchStatus.WIN, [])
// #     else:

// #     elif md_own != MAX_DEFCON and md_opp == MAX_DEFCON:

// #     pass

// /// Thread safe version of tss_next_sq.
// pub fn tss_next_sq_safe(board: &Array2<u8>, color: u8, next_sq: Point) -> SearchNode {
//     let mut board_clone = board.to_owned();
//     tss_next_sq(&mut board_clone, color, next_sq)
// }

/// Threat Space Search for the whole board.
pub fn tss_board(board: &mut Array2<u8>, color: u8) -> SearchNode {
    let threats = search_all_board(board, color, ThreatPri::Immediate);
    let mut potential_win = !threats.is_empty();
    let mut children = Vec::<SearchNode>::new();

    if !potential_win {
        let nsqs = search_all_board_get_next_sqs(board, color, ThreatPri::Immediate);
        // children = nsqs.par_iter().map(|x| tss_next_sq_safe(board, color, *x)).collect();
        children = nsqs.iter().map(|x| tss_next_sq(board, color, *x)).collect();
        potential_win = children.iter().any(|x| x.potential_win);
    }

    SearchNode::new(None, threats, None, potential_win, children)
}

/// Extract all potential win variations from SearchNode.
pub fn potential_win_variations(node: &SearchNode) -> Vec<Vec<(Point, FnvHashSet<Point>)>> {
    let mut variations: Vec<Vec<(Point, FnvHashSet<Point>)>> = Vec::new();

    if node.potential_win {
        let mut node_var: Vec<(Point, FnvHashSet<Point>)> = Vec::new();
        if node.next_sq.is_some() {
            node_var.push((node.next_sq.unwrap(), node.critical_sqs.to_owned().unwrap()));
        }

        if !node.children.is_empty() {
            for child in node.children.iter() {
                if child.potential_win {
                    let child_variations = potential_win_variations(child);
                    for child_var in child_variations {
                        let mut child_var_next = node_var.to_owned();
                        child_var_next.extend(child_var);
                        variations.push(child_var_next);
                    }
                }
            }
        } else {
            variations.push(node_var);
        }
    }

    variations
}

/// Animate a given variation on the board.
pub fn animate_variation(
    board: &mut Array2<u8>,
    color: u8,
    variation: &[(Point, FnvHashSet<Point>)],
) {
    let sleep_duration = Duration::from_secs(ANIMATION_TIMESTEP_SECS);

    println!("{}", board_to_str(board));
    thread::sleep(sleep_duration);

    for item in variation.iter() {
        set_sq(board, color, item.0);

        println!("next_sq: {}", point_to_algebraic(item.0));
        println!("{}", board_to_str(board));
        thread::sleep(sleep_duration);

        for csq in item.1.iter() {
            set_sq(board, color ^ STONE, *csq);
        }

        if !item.1.is_empty() {
            let csqs_str = item
                .1
                .iter()
                .map(|&x| point_to_algebraic(x))
                .reduce(|a, b| a + ", " + &b)
                .unwrap();

            println!("critical_sqs: {}", csqs_str);
            println!("{}", board_to_str(board));
            thread::sleep(sleep_duration);
        }
    }

    for item in variation.iter() {
        clear_sq(board, color, item.0);

        for csq in item.1.iter() {
            clear_sq(board, color ^ STONE, *csq);
        }
    }
}

pub fn variation_to_algebraic(
    variation: &[(Point, FnvHashSet<Point>)],
) -> Vec<(String, Vec<String>)> {
    variation
        .iter()
        .map(|x| {
            (
                point_to_algebraic(x.0),
                x.1.iter().map(|&y| point_to_algebraic(y)).collect(),
            )
        })
        .collect()
}
