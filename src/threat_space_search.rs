//! Implements Threat Space Search.

use crate::board::{board_to_str, clear_sq, set_sq};
use crate::consts::{ANIMATION_TIMESTEP_SECS, MAX_DEFCON, STONE};
use crate::geometry::{point_is_on_line, Point};
use crate::pattern::Threat;
use crate::pattern::{
    search_all_board, search_all_board_get_next_sqs, search_all_point, search_all_point_own, search_all_point_own_get_next_sqs, ThreatPri,
};
use ndarray::prelude::*;
// use rayon::prelude::*;
use crate::board::point_to_algebraic;
use fnv::FnvHashSet;
use std::thread;
use std::time::Duration;

/// A tree that represents the result of a Threat Space Search.
#[derive(Clone, Debug)]
pub struct SearchNode {
    pub next_sq: Option<Point>,
    pub critical_sqs: Option<FnvHashSet<Point>>,
    pub potential_win: bool,
    pub children: Vec<SearchNode>,
}

impl SearchNode {
    #[inline(always)]
    pub fn new(next_sq: Option<Point>, critical_sqs: Option<FnvHashSet<Point>>, potential_win: bool, children: Vec<SearchNode>) -> Self {
        Self {
            next_sq,
            critical_sqs,
            potential_win,
            children,
        }
    }
}

/// Threat Space Search for a given next_sq.
pub fn tss_next_sq(board: &mut Array2<u8>, color: u8, next_sq: Point, all_threats_init: &[Threat], opp_all_threats_init: &[Threat]) -> SearchNode {
    set_sq(board, color, next_sq);

    // Create all_threats for self and opponent, and update them.
    // 1. Remove threats including next_sq.
    // 2. Compute new threats including next_sq.
    let mut all_threats = all_threats_init
        .iter()
        .filter(|x| !point_is_on_line(next_sq, x.m.0, x.m.1, true))
        .cloned()
        .collect::<Vec<Threat>>();
    all_threats.extend(search_all_point(board, color, next_sq, ThreatPri::Immediate));

    let mut opp_all_threats = opp_all_threats_init
        .iter()
        .filter(|x| !point_is_on_line(next_sq, x.m.0, x.m.1, true))
        .cloned()
        .collect::<Vec<Threat>>();
    opp_all_threats.extend(search_all_point(board, color ^ STONE, next_sq, ThreatPri::Immediate));

    // NOTE: If we are potentially losing, we will early return.

    // Check if we are potentially losing, by looking at the updated lists of all threats.
    let mut min_defcon = all_threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));
    let mut opp_min_defcon = opp_all_threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));
    let mut potential_loss = !opp_all_threats.is_empty() && opp_min_defcon <= min_defcon;
    if potential_loss {
        clear_sq(board, color, next_sq);
        return SearchNode::new(Some(next_sq), Some(FnvHashSet::<Point>::default()), false, Vec::<SearchNode>::new());
    }

    let threats = search_all_point_own(board, color, next_sq, ThreatPri::Immediate);

    // We will consider those of our threats which are more immediate than all of our opponent's threats.
    let pressing_threats = threats.iter().filter(|x| x.defcon < opp_min_defcon).cloned().collect::<Vec<Threat>>();

    let critical_sqs: FnvHashSet<Point> = if !pressing_threats.is_empty() {
        pressing_threats
            .iter()
            .map(|x| x.critical_sqs.to_owned())
            .reduce(|a, b| a.intersection(&b).copied().collect::<FnvHashSet<Point>>())
            .unwrap()
    } else {
        FnvHashSet::<Point>::default()
    };

    for csq in critical_sqs.iter() {
        set_sq(board, color ^ STONE, *csq);
    }

    // If we have any critical_sqs, update lists of all threats.
    // Also check if we are potentially losing after the critical_sqs are covered by the opponent.
    if !critical_sqs.is_empty() {
        all_threats = all_threats
            .iter()
            .filter(|x| !critical_sqs.iter().any(|p| point_is_on_line(*p, x.m.0, x.m.1, true)))
            .cloned()
            .collect::<Vec<Threat>>();

        opp_all_threats = opp_all_threats
            .iter()
            .filter(|x| !critical_sqs.iter().any(|p| point_is_on_line(*p, x.m.0, x.m.1, true)))
            .cloned()
            .collect::<Vec<Threat>>();

        for csq in critical_sqs.iter() {
            all_threats.extend(search_all_point(board, color, *csq, ThreatPri::Immediate));
            opp_all_threats.extend(search_all_point(board, color ^ STONE, *csq, ThreatPri::Immediate));
        }

        min_defcon = all_threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));
        opp_min_defcon = opp_all_threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));

        // If opp_min_defcon is 0, then the opponent has potentially won!
        potential_loss = opp_min_defcon == 0;
        if potential_loss {
            for csq in critical_sqs.iter() {
                clear_sq(board, color ^ STONE, *csq);
            }
            clear_sq(board, color, next_sq);
            return SearchNode::new(Some(next_sq), Some(FnvHashSet::<Point>::default()), false, Vec::<SearchNode>::new());
        }

        // We will consider those of the opponent's threats which are more immediate than all of our threats.
        let opp_pressing_threats = opp_all_threats.iter().filter(|x| x.defcon < min_defcon).cloned().collect::<Vec<Threat>>();
        let opp_critical_sqs: FnvHashSet<Point> = if !opp_pressing_threats.is_empty() {
            opp_pressing_threats
                .iter()
                .map(|x| x.critical_sqs.to_owned())
                .reduce(|a, b| a.intersection(&b).copied().collect::<FnvHashSet<Point>>())
                .unwrap()
        } else {
            FnvHashSet::<Point>::default()
        };

        potential_loss = !opp_pressing_threats.is_empty() && opp_critical_sqs.is_empty();
        if potential_loss {
            for csq in critical_sqs.iter() {
                clear_sq(board, color ^ STONE, *csq);
            }
            clear_sq(board, color, next_sq);
            return SearchNode::new(Some(next_sq), Some(FnvHashSet::<Point>::default()), false, Vec::<SearchNode>::new());
        }
    }

    let mut potential_win = !pressing_threats.is_empty() && critical_sqs.is_empty();
    let mut children = Vec::<SearchNode>::new();

    // If next_sq produces no threats or we've found a potential win, we won't go any deeper.
    if !threats.is_empty() && !potential_win {
        let nsqs = search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::Immediate);
        children = nsqs
            .iter()
            .map(|x| tss_next_sq(board, color, *x, &all_threats, &opp_all_threats))
            .collect();
        potential_win = children.iter().any(|x| x.potential_win);

        if !potential_win {
            let nsqs_other = search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::NonImmediate);
            let children_other: Vec<SearchNode> = nsqs_other
                .iter()
                .map(|x| tss_next_sq(board, color, *x, &all_threats, &opp_all_threats))
                .collect();
            potential_win = children_other.iter().any(|x| x.potential_win);
            children.extend(children_other);
        }
    }

    for csq in critical_sqs.iter() {
        clear_sq(board, color ^ STONE, *csq);
    }
    clear_sq(board, color, next_sq);

    SearchNode::new(Some(next_sq), Some(critical_sqs), potential_win, children)
}

// /// Thread safe version of tss_next_sq.
// pub fn tss_next_sq_safe(board: &Array2<u8>, color: u8, next_sq: Point) -> SearchNode {
//     let mut board_clone = board.to_owned();
//     tss_next_sq(&mut board_clone, color, next_sq)
// }

/// Threat Space Search for the whole board.
pub fn tss_board(board: &mut Array2<u8>, color: u8) -> SearchNode {
    let threats = search_all_board(board, color, ThreatPri::Immediate);
    let opp_threats = search_all_board(board, color ^ STONE, ThreatPri::Immediate);

    let min_defcon = threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));
    let opp_min_defcon = opp_threats.iter().fold(MAX_DEFCON, |a, b| a.min(b.defcon));

    let mut potential_win = !threats.is_empty() && min_defcon <= opp_min_defcon;
    let mut children = Vec::<SearchNode>::new();

    if !potential_win {
        let nsqs = search_all_board_get_next_sqs(board, color, ThreatPri::Immediate);
        // children = nsqs.par_iter().map(|x| tss_next_sq_safe(board, color, *x)).collect();
        children = nsqs.iter().map(|x| tss_next_sq(board, color, *x, &threats, &opp_threats)).collect();
        potential_win = children.iter().any(|x| x.potential_win);
    }

    SearchNode::new(None, None, potential_win, children)
}

/// Extract all potentially winning variations from SearchNode.
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

    variations.sort_by(|a, b| a.len().cmp(&b.len()));
    variations
}

/// Animate a given variation on the board.
pub fn animate_variation(board: &mut Array2<u8>, color: u8, variation: &[(Point, FnvHashSet<Point>)]) {
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
            let csqs_str = item.1.iter().map(|&x| point_to_algebraic(x)).reduce(|a, b| a + ", " + &b).unwrap();

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

/// Output a variation in algebraic notation.
pub fn variation_to_algebraic(variation: &[(Point, FnvHashSet<Point>)]) -> Vec<(String, Vec<String>)> {
    variation
        .iter()
        .map(|x| (point_to_algebraic(x.0), x.1.iter().map(|&y| point_to_algebraic(y)).collect()))
        .collect()
}
