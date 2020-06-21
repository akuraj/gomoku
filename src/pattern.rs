//! Define struct to represent threat patterns, and related functions (search etc.).

use crate::consts::{EMPTY, GEN_ELEMS, GEN_ELEMS_TO_NAMES, MDFIT, NOT_OWN, OWN, WALL_ENEMY};
use crate::geometry::{point_set_on_line, Point};
use crate::pattern_search::{
    defcon_from_degree, degree, one_step_from_straight_threat, search_board, search_board_next_sq,
    search_point, search_point_next_sq, search_point_own, search_point_own_next_sq, Match,
};
use lazy_static::lazy_static;
use ndarray::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

// FIXME: Use map, fold, filter everywhere below when mapping/accumulating!

/// Pattern: Used to represent threat patterns.
#[derive(Clone, Debug)]
pub struct Pattern {
    pub pattern: Vec<u8>,
    pub critical_sqs: Vec<isize>,
    pub own_sqs: Vec<isize>,
    pub name: String,
    pub index: usize,
    pub empty_sqs: Vec<isize>,
    pub defcon: usize,
    pub immediate: bool,
}

#[allow(clippy::collapsible_if)]
impl Pattern {
    pub fn new(pattern: Vec<u8>, critical_sqs: Vec<isize>, name: String, index: usize) -> Self {
        // Make sure elemnts of the pattern are valid.
        for elem in pattern.iter() {
            assert!(GEN_ELEMS.iter().any(|&x| x == *elem));
            assert!(*elem == OWN || (*elem & OWN == 0));
        }

        // FIXME: port below code.
        // Critical Squares are the places where if the oppenent plays,
        // then the threat is mitigated.
        // Checks on critical_sqs.
        // critical_sqs.sort()
        // critical_sqs_uniq = list(set(critical_sqs))
        // critical_sqs_uniq.sort()
        // assert critical_sqs == critical_sqs_uniq

        let length = pattern.len();
        for sq in critical_sqs.iter() {
            // sq must be EMPTY for it to be critical.
            let squ = *sq as usize;
            assert!((0..length).contains(&squ) && pattern[squ] == EMPTY);
        }

        // Check size of name.
        assert!(!name.is_empty());

        // Check that any OWN or EMPTY squares in the pattern are contiguous,
        // i.e., OWN/EMPTY is not interrupted by any other kind of square.
        // This is what a normal/useful pattern would like.
        let mut oe_start = false;
        let mut oe_end = false;
        for v in pattern.iter() {
            if *v == OWN || *v == EMPTY {
                if !oe_start {
                    oe_start = true;
                }

                assert!(!oe_end, "Non-contiguous OWN/EMPTY squares!");
            } else {
                if oe_start && !oe_end {
                    oe_end = true;
                }
            }
        }

        let mut own_sqs: Vec<isize> = Vec::new();
        for (i, v) in pattern.iter().enumerate() {
            if *v == OWN {
                own_sqs.push(i as isize);
            }
        }

        let mut other_empty_sqs: Vec<isize> = Vec::new();
        for (i, v) in pattern.iter().enumerate() {
            if *v == EMPTY && !critical_sqs.iter().any(|&x| x == (i as isize)) {
                other_empty_sqs.push(i as isize);
            }
        }

        // FIXME: Fix the below garbage: should use extend or something instead!
        // Add entry for empty_sqs. critical_sqs appear first.
        let mut empty_sqs: Vec<isize> = Vec::new();
        for v in critical_sqs.iter() {
            empty_sqs.push(*v);
        }
        for v in other_empty_sqs.iter() {
            empty_sqs.push(*v);
        }

        let defcon = defcon_from_degree(degree(&pattern));

        let immediate = if defcon < 2 {
            true
        } else {
            one_step_from_straight_threat(&pattern)
        };

        // FIXME: port below code.
        // # Checks on data fields.
        // assert self.pattern.ndim == 1
        // assert self.pattern.size > 0
        // assert self.critical_sqs.ndim == 1
        // assert self.own_sqs.ndim == 1
        // assert self.empty_sqs.ndim == 1
        // assert self.defcon in DEFCON_RANGE

        // FIXME: port below code.
        // # Check on empty_sqs that they need to be useful.
        // curr_degree = degree(self.pattern)
        // for esq in self.empty_sqs:
        //     next_pattern = np.array(self.pattern, dtype=np.byte)
        //     next_pattern[esq] = OWN
        //     assert degree(next_pattern) == curr_degree + 1

        Self {
            pattern,
            critical_sqs,
            own_sqs,
            name,
            index,
            empty_sqs,
            defcon,
            immediate,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push('\n');
        output.push_str("pattern: ");
        for x in self.pattern.iter() {
            output.push_str(GEN_ELEMS_TO_NAMES[x]);
            output.push(' ');
        }
        output.push('\n');
        output.push_str(&format!("defcon: {}\n", self.defcon));
        output.push_str(&format!("immediate: {}\n", self.immediate));
        output.push_str(&format!("critical_sqs: {:?}\n", self.critical_sqs));
        output.push_str(&format!("own_sqs: {:?}\n", self.own_sqs));
        output.push_str(&format!("empty_sqs: {:?}\n", self.empty_sqs));
        output.push_str(&format!("name: {}\n", self.name));
        output.push_str(&format!("index: {}\n", self.index));

        write!(f, "{}", output)
    }
}

/// Enum to represent the priority of a Threat.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ThreatPri {
    All,
    Immediate,
    NonImmediate,
}

lazy_static! {
    pub static ref P_WIN: Pattern = Pattern::new(
        Vec::<u8>::from([OWN, OWN, OWN, OWN, OWN]),
        Vec::<isize>::new(),
        String::from("P_WIN"),
        0
    );
    pub static ref P_4_ST: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, OWN, OWN, OWN, OWN, EMPTY]),
        Vec::<isize>::new(),
        String::from("P_4_ST"),
        1
    );
    pub static ref P_4_A: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, OWN, EMPTY]),
        Vec::<isize>::from([5]),
        String::from("P_4_A"),
        2
    );
    pub static ref P_4_B: Pattern = Pattern::new(
        Vec::<u8>::from([NOT_OWN, OWN, OWN, OWN, EMPTY, OWN]),
        Vec::<isize>::from([4]),
        String::from("P_4_B"),
        3
    );
    pub static ref P_4_C: Pattern = Pattern::new(
        Vec::<u8>::from([NOT_OWN, OWN, OWN, EMPTY, OWN, OWN, NOT_OWN]),
        Vec::<isize>::from([3]),
        String::from("P_4_C"),
        4
    );
    pub static ref P_3_ST: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY]),
        Vec::<isize>::from([1, 5]),
        String::from("P_3_ST"),
        5
    );
    pub static ref P_3_A: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY]),
        Vec::<isize>::from([1, 5, 6]),
        String::from("P_3_A"),
        6
    );
    pub static ref P_3_B: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, OWN, OWN, EMPTY, OWN, EMPTY]),
        Vec::<isize>::from([0, 3, 5]),
        String::from("P_3_B"),
        7
    );
    pub static ref P_3_C: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, EMPTY, EMPTY]),
        Vec::<isize>::from([4, 5]),
        String::from("P_3_C"),
        8
    );
    pub static ref P_3_D: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, OWN, OWN, EMPTY, OWN, EMPTY]),
        Vec::<isize>::from([3, 5]),
        String::from("P_3_D"),
        9
    );
    pub static ref P_3_E: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, OWN, EMPTY, OWN, OWN, EMPTY]),
        Vec::<isize>::from([2, 5]),
        String::from("P_3_E"),
        10
    );
    pub static ref P_3_F: Pattern = Pattern::new(
        Vec::<u8>::from([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, WALL_ENEMY]),
        Vec::<isize>::from([1, 5]),
        String::from("P_3_F"),
        11
    );
    pub static ref P_3_G: Pattern = Pattern::new(
        Vec::<u8>::from([NOT_OWN, OWN, OWN, EMPTY, EMPTY, OWN]),
        Vec::<isize>::from([3, 4]),
        String::from("P_3_G"),
        12
    );
    pub static ref P_3_H: Pattern = Pattern::new(
        Vec::<u8>::from([NOT_OWN, OWN, EMPTY, OWN, EMPTY, OWN, NOT_OWN]),
        Vec::<isize>::from([2, 4]),
        String::from("P_3_H"),
        13
    );
    pub static ref P_2_A: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, EMPTY, EMPTY]),
        Vec::<isize>::from([0, 1, 4, 5]),
        String::from("P_2_A"),
        14
    );
    pub static ref P_2_B: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, EMPTY, OWN, EMPTY, OWN, EMPTY, EMPTY]),
        Vec::<isize>::from([0, 1, 3, 5, 6]),
        String::from("P_2_B"),
        15
    );
    pub static ref P_2_C: Pattern = Pattern::new(
        Vec::<u8>::from([EMPTY, OWN, EMPTY, EMPTY, OWN, EMPTY]),
        Vec::<isize>::from([0, 2, 3, 5]),
        String::from("P_2_C"),
        16
    );

    /// All defined patterns.
    pub static ref PATTERNS: Vec<&'static Pattern> = {
        let patterns: [&'static Pattern; 17] = [
            &(*P_WIN),
            &(*P_4_ST),
            &(*P_4_A),
            &(*P_4_B),
            &(*P_4_C),
            &(*P_3_ST),
            &(*P_3_A),
            &(*P_3_B),
            &(*P_3_C),
            &(*P_3_D),
            &(*P_3_E),
            &(*P_3_F),
            &(*P_3_G),
            &(*P_3_H),
            &(*P_2_A),
            &(*P_2_B),
            &(*P_2_C),
        ];

        for (i, p) in patterns.iter().enumerate() {
            assert_eq!(i, p.index);
        }

        let num_names = patterns
            .iter()
            .map(|&x| String::from(&x.name))
            .collect::<HashSet<String>>()
            .len();
        assert_eq!(num_names, patterns.len());

        let max_defcon_imm =
            patterns.iter().fold(
                usize::MIN,
                |a, b| if b.immediate { a.max(b.defcon) } else { a },
            );
        assert_eq!(max_defcon_imm, MDFIT);

        Vec::from(patterns)
    };
    pub static ref NUM_PTNS: usize = PATTERNS.len();
    pub static ref PATTERNS_BY_DEFCON: HashMap<usize, Vec<&'static Pattern>> = {
        let mut m: HashMap<usize, Vec<&'static Pattern>> = HashMap::new();

        for p in PATTERNS.iter() {
            m.entry(p.defcon)
                .or_insert_with(Vec::<&'static Pattern>::new)
                .push(p);
        }

        m
    };
    pub static ref PATTERNS_BY_NAME: HashMap<String, &'static Pattern> = {
        let mut m: HashMap<String, &'static Pattern> = HashMap::new();

        for p in PATTERNS.iter() {
            assert!(!m.contains_key(&p.name));
            m.insert(String::from(&p.name), p);
        }

        m
    };

    /// Immediate/High Priority PATTERNS.
    pub static ref PATTERNS_I: Vec<&'static Pattern> = {
        let mut patterns: Vec<&'static Pattern> = Vec::new();

        for p in PATTERNS.iter() {
            if p.immediate {
                patterns.push(p);
            }
        }

        patterns
    };

    /// NonImmediate/Low Priority PATTERNS.
    pub static ref PATTERNS_NI: Vec<&'static Pattern> = {
        let mut patterns: Vec<&'static Pattern> = Vec::new();

        for p in PATTERNS.iter() {
            if !p.immediate {
                patterns.push(p);
            }
        }

        patterns
    };
    pub static ref PATTERNS_BY_PRI: HashMap<ThreatPri, &'static Vec<&'static Pattern>> = {
        let mut m: HashMap<ThreatPri, &'static Vec<&'static Pattern>> = HashMap::new();
        m.insert(ThreatPri::All, &PATTERNS);
        m.insert(ThreatPri::Immediate, &PATTERNS_I);
        m.insert(ThreatPri::NonImmediate, &PATTERNS_NI);
        m
    };
}

/// Threat: the where and the what.
#[derive(Clone, Debug)]
pub struct Threat {
    pub m: Match,
    pub pidx: usize,
    pub defcon: usize,
    pub critical_sqs: HashSet<Point>,
}

pub fn threat_item(m: Match, pattern: &Pattern) -> Threat {
    Threat {
        m,
        pidx: pattern.index,
        defcon: pattern.defcon,
        critical_sqs: point_set_on_line(m.0, m.1, &pattern.critical_sqs),
    }
}

pub fn search_all_board(board: &Array2<u8>, color: u8, pri: ThreatPri) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_board(board, &p.pattern, color) {
            threats.push(threat_item(m, p));
        }
    }

    threats

    // PATTERNS_BY_PRI[&pri]
    // .iter()
    // .map(|p| {
    //     search_board(board, &p.pattern, color)
    //         .into_iter()
    //         .map(move |m| threat_item(m, p))
    // })
    // .flatten()
    // .collect::<Vec<Threat>>()
}

pub fn search_all_point(
    board: &Array2<u8>,
    color: u8,
    point: Point,
    pri: ThreatPri,
) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_point(board, &p.pattern, color, point) {
            threats.push(threat_item(m, p));
        }
    }

    threats
}

pub fn search_all_point_own(
    board: &Array2<u8>,
    color: u8,
    point: Point,
    pri: ThreatPri,
) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_point_own(board, &p.pattern, color, point, &p.own_sqs) {
            threats.push(threat_item(m, p));
        }
    }

    threats
}

pub fn search_all_board_get_next_sqs(
    board: &Array2<u8>,
    color: u8,
    pri: ThreatPri,
) -> HashSet<Point> {
    let mut nsqs: HashSet<Point> = HashSet::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_board_next_sq(board, &p.pattern, color) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}

pub fn search_all_point_get_next_sqs(
    board: &Array2<u8>,
    color: u8,
    point: Point,
    pri: ThreatPri,
) -> HashSet<Point> {
    let mut nsqs: HashSet<Point> = HashSet::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_point_next_sq(board, &p.pattern, color, point) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}

pub fn search_all_point_own_get_next_sqs(
    board: &Array2<u8>,
    color: u8,
    point: Point,
    pri: ThreatPri,
) -> HashSet<Point> {
    let mut nsqs: HashSet<Point> = HashSet::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_point_own_next_sq(board, &p.pattern, color, point, &p.own_sqs) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}
