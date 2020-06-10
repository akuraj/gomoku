// from time import sleep
use ndarray::prelude::*;
use consts::{STONE, MAX_DEFCON, ANIMATION_TIMESTEP_SECS};
use board::{set_sq, clear_sq, board_to_str};
use pattern::{ThreatPri, search_all_board, search_all_point_own,
              search_all_board_get_next_sqs,
              search_all_point_own_get_next_sqs};
use geometry::Point;
use pattern::Threat;
use std::collections::HashSet;
use std::time::Duration;
use std::thread;

// @unique
// class SearchStatus(IntEnum):
//     """Enum to represent the status of threat space search."""

//     QUIET = auto()
//     UNQUIET = auto()
//     WIN = auto()
//     LOSS = auto()

#[derive(Clone,Debug)]
pub struct SearchNode {
    pub next_sq: Option<Point>,
    pub threats: Vec<Threat>,
    pub critical_sqs: Option<HashSet<Point>>,
    pub potential_win: bool,
    pub children: Vec<SearchNode>,
}

impl SearchNode {
    pub fn new(next_sq: Option<Point>, threats: Vec<Threat>, critical_sqs: Option<HashSet<Point>>,
               potential_win: bool, children: Vec<SearchNode>) -> Self {
//     # # Only keep potentially winning children.
//     # children = [x for x in children if x["potential_win"]]

        Self {
            next_sq: next_sq,
            threats: threats,
            critical_sqs: critical_sqs,
            potential_win: potential_win,
            children: children,
        }
    }
}

pub fn tss_next_sq(board: &mut Array2<u8>, color: u8, next_sq: Point) -> SearchNode {
    set_sq(board, color, next_sq);

    let threats = search_all_point_own(board, color, next_sq, ThreatPri::IMMEDIATE);
    let num_threats = threats.len();
    let critical_sqs: HashSet<Point> = if num_threats > 0 {
        // FIXME: Reduce?
        let mut csqs_temp: HashSet<Point> = threats[0].critical_sqs.iter().cloned().collect();
        for i in 1..num_threats {
            csqs_temp = csqs_temp.intersection(&threats[i].critical_sqs).cloned().collect::<HashSet<Point>>();
        }
        csqs_temp
    } else {
        HashSet::<Point>::new()
    };

    let mut potential_win = num_threats > 0 && critical_sqs.len() == 0;
    let mut children = Vec::<SearchNode>::new();

    for csq in critical_sqs.iter() {
        set_sq(board, color ^ STONE, *csq);

        let threats_csq = search_all_point_own(board, color ^ STONE,
                                               *csq, ThreatPri::IMMEDIATE);
        let num_threats_csq = threats_csq.len();
        let critical_sqs_csq: HashSet<Point> = if num_threats_csq > 0 {
            // FIXME: Reduce?
            let mut csqs_temp: HashSet<Point> = threats_csq[0].critical_sqs.iter().cloned().collect();
            for i in 1..num_threats_csq {
                csqs_temp = csqs_temp.intersection(&threats_csq[i].critical_sqs).cloned().collect::<HashSet<Point>>();
            }
            csqs_temp
        } else {
            HashSet::<Point>::new()
        };

        let potential_win_csq = num_threats_csq > 0 && critical_sqs_csq.len() == 0;

        clear_sq(board, color ^ STONE, *csq);

        if potential_win_csq {
            potential_win = false;
            clear_sq(board, color, next_sq);
            return SearchNode::new(Some(next_sq), threats, Some(critical_sqs), potential_win, children);
        }
    }

    if num_threats > 0 && !potential_win {
        for csq in critical_sqs.iter() {
            set_sq(board, color ^ STONE, *csq);
        }

        let nsqs = search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::IMMEDIATE);
        children = nsqs.iter().map(|x| tss_next_sq(board, color, *x)).collect();
        potential_win = children.iter().any(|x| x.potential_win);

        if !potential_win {
            let nsqs_other = search_all_point_own_get_next_sqs(board, color, next_sq, ThreatPri::NON_IMMEDIATE);
            let children_other: Vec<SearchNode> = nsqs_other.iter().map(|x| tss_next_sq(board, color, *x)).collect();
            potential_win = children_other.iter().any(|x| x.potential_win);
            children.extend(children_other);
        }

        for csq in critical_sqs.iter() {
            clear_sq(board, color ^ STONE, *csq);
        }
    }

    clear_sq(board, color, next_sq);
    return SearchNode::new(Some(next_sq), threats, Some(critical_sqs), potential_win, children);
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

pub fn tss_board(board: &mut Array2<u8>, color: u8) -> SearchNode {
    let threats = search_all_board(board, color, ThreatPri::IMMEDIATE);
    let mut potential_win = threats.len() > 0;
    let mut children = Vec::<SearchNode>::new();

    if !potential_win {
        let nsqs = search_all_board_get_next_sqs(board, color, ThreatPri::IMMEDIATE);
        children = nsqs.iter().map(|x| tss_next_sq(board, color, *x)).collect();
        potential_win = children.iter().any(|x| x.potential_win);
    }

    return SearchNode::new(None, threats, None, potential_win, children);
}

pub fn potential_win_variations(node: &SearchNode) -> Vec<Vec<(Point, HashSet<Point>)>> {
    let mut variations: Vec<Vec<(Point, HashSet<Point>)>> = Vec::new();

    if node.potential_win {
        let mut node_var: Vec<(Point, HashSet<Point>)> = Vec::new();
        if node.next_sq.is_some() {
            node_var.push((node.next_sq.unwrap(), node.critical_sqs.clone().unwrap()));
        }

        if node.children.len() > 0 {
            for child in node.children.iter() {
                if child.potential_win {
                    let child_variations = potential_win_variations(child);
                    for child_var in child_variations {
                        let mut child_var_next = node_var.clone();
                        child_var_next.extend(child_var);
                        variations.push(child_var_next);
                    }
                }
            }
        } else {
            variations.push(node_var);
        }
    }

    return variations;
}

pub fn animate_variation(board: &mut Array2<u8>, color: u8, variation: &Vec<(Point, HashSet<Point>)>) {
    let sleep_duration = Duration::from_secs(ANIMATION_TIMESTEP_SECS);

    println!("{}", board_to_str(board));
    thread::sleep(sleep_duration);

    for item in variation.iter() {
        set_sq(board, color, item.0);

        println!("{}", board_to_str(board));
        thread::sleep(sleep_duration);

        for csq in item.1.iter() {
            set_sq(board, color ^ STONE, *csq);
        }

        println!("{}", board_to_str(board));
        thread::sleep(sleep_duration);
    }

    for item in variation.iter() {
        clear_sq(board, color, item.0);

        for csq in item.1.iter() {
            clear_sq(board, color ^ STONE, *csq);
        }
    }
}
