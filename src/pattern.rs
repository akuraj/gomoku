//! Define struct to represent threat patterns, and related functions (search etc.).

use crate::consts::{
    EMPTY, GEN_ELEMS, GEN_ELEMS_TO_NAMES, MDFIT, NOT_OWN, OWN, WALL_ENEMY,
};
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

#[derive(Clone, Debug)]
pub struct Pattern {
    pub pattern: Array1<u8>,
    pub critical_sqs: Array1<isize>,
    pub own_sqs: Array1<isize>,
    pub name: String,
    pub index: isize,
    pub empty_sqs: Array1<isize>,
    pub defcon: usize,
    pub immediate: bool,
}

impl Pattern {
    pub fn new(
        pattern: Array1<u8>,
        critical_sqs: Array1<isize>,
        name: String,
        index: isize,
    ) -> Self {
        for elem in pattern.iter() {
            assert!(GEN_ELEMS.iter().any(|&x| x == *elem));
            assert!(*elem == OWN || (*elem & OWN == 0));
        }

        // FIXME: port below code.
        // critical_sqs.sort()
        // critical_sqs_uniq = list(set(critical_sqs))
        // critical_sqs_uniq.sort()
        // assert critical_sqs == critical_sqs_uniq

        let length = pattern.len();
        for sq in critical_sqs.iter() {
            assert!((0 <= *sq && *sq < (length as isize)) && pattern[*sq as usize] == EMPTY);
        }

        assert!(!name.is_empty());

        let mut oe_start = false;
        let mut oe_end = false;
        for v in pattern.iter() {
            if *v == OWN || *v == EMPTY {
                if !oe_start {
                    oe_start = true;
                }

                assert!(!oe_end);
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
            own_sqs: Array1::from(own_sqs),
            name,
            index,
            empty_sqs: Array1::from(empty_sqs),
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
        output.push_str(&format!("critical_sqs: {}\n", self.critical_sqs));
        output.push_str(&format!("own_sqs: {}\n", self.own_sqs));
        output.push_str(&format!("empty_sqs: {}\n", self.empty_sqs));
        output.push_str(&format!("name: {}\n", self.name));
        output.push_str(&format!("index: {}\n", self.index));

        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ThreatPri {
    ALL,
    IMMEDIATE,
    NON_IMMEDIATE,
}

lazy_static! {
    pub static ref P_WIN: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([OWN, OWN, OWN, OWN, OWN])),
        Array1::from(Vec::<isize>::from([])),
        String::from("P_WIN"),
        0
    );
    pub static ref P_4_ST: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([EMPTY, OWN, OWN, OWN, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([])),
        String::from("P_4_ST"),
        1
    );
    pub static ref P_4_A: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([5])),
        String::from("P_4_A"),
        2
    );
    pub static ref P_4_B: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([NOT_OWN, OWN, OWN, OWN, EMPTY, OWN])),
        Array1::from(Vec::<isize>::from([4])),
        String::from("P_4_B"),
        3
    );
    pub static ref P_4_C: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([
            NOT_OWN, OWN, OWN, EMPTY, OWN, OWN, NOT_OWN
        ])),
        Array1::from(Vec::<isize>::from([3])),
        String::from("P_4_C"),
        4
    );
    pub static ref P_3_ST: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY])),
        Array1::from(Vec::<isize>::from([1, 5])),
        String::from("P_3_ST"),
        5
    );
    pub static ref P_3_A: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([
            WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY
        ])),
        Array1::from(Vec::<isize>::from([1, 5, 6])),
        String::from("P_3_A"),
        6
    );
    pub static ref P_3_B: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([EMPTY, OWN, OWN, EMPTY, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([0, 3, 5])),
        String::from("P_3_B"),
        7
    );
    pub static ref P_3_C: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, EMPTY, EMPTY])),
        Array1::from(Vec::<isize>::from([4, 5])),
        String::from("P_3_C"),
        8
    );
    pub static ref P_3_D: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, EMPTY, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([3, 5])),
        String::from("P_3_D"),
        9
    );
    pub static ref P_3_E: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, EMPTY, OWN, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([2, 5])),
        String::from("P_3_E"),
        10
    );
    pub static ref P_3_F: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([
            WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, WALL_ENEMY
        ])),
        Array1::from(Vec::<isize>::from([1, 5])),
        String::from("P_3_F"),
        11
    );
    pub static ref P_3_G: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([NOT_OWN, OWN, OWN, EMPTY, EMPTY, OWN])),
        Array1::from(Vec::<isize>::from([3, 4])),
        String::from("P_3_G"),
        12
    );
    pub static ref P_3_H: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([
            NOT_OWN, OWN, EMPTY, OWN, EMPTY, OWN, NOT_OWN
        ])),
        Array1::from(Vec::<isize>::from([2, 4])),
        String::from("P_3_H"),
        13
    );
    pub static ref P_2_A: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, EMPTY, EMPTY])),
        Array1::from(Vec::<isize>::from([0, 1, 4, 5])),
        String::from("P_2_A"),
        14
    );
    pub static ref P_2_B: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([
            EMPTY, EMPTY, OWN, EMPTY, OWN, EMPTY, EMPTY
        ])),
        Array1::from(Vec::<isize>::from([0, 1, 3, 5, 6])),
        String::from("P_2_B"),
        15
    );
    pub static ref P_2_C: Pattern = Pattern::new(
        Array1::from(Vec::<u8>::from([EMPTY, OWN, EMPTY, EMPTY, OWN, EMPTY])),
        Array1::from(Vec::<isize>::from([0, 2, 3, 5])),
        String::from("P_2_C"),
        16
    );
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
            assert_eq!(i as isize, p.index);
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

        return Vec::from(patterns);
    };
    pub static ref NUM_PTNS: usize = PATTERNS.len();
    pub static ref PATTERNS_BY_DEFCON: HashMap<usize, Vec<&'static Pattern>> = {
        let mut m: HashMap<usize, Vec<&'static Pattern>> = HashMap::new();

        for p in PATTERNS.iter() {
            m.entry(p.defcon).or_insert(Vec::<&'static Pattern>::new()).push(p);
        }

        return m;
    };
    pub static ref PATTERNS_BY_NAME: HashMap<String, &'static Pattern> = {
        let mut m: HashMap<String, &'static Pattern> = HashMap::new();

        for p in PATTERNS.iter() {
            assert!(!m.contains_key(&p.name));
            m.insert(String::from(&p.name), p);
        }

        return m;
    };
    pub static ref PATTERNS_I: Vec<&'static Pattern> = {
        let mut patterns: Vec<&'static Pattern> = Vec::new();

        for p in PATTERNS.iter() {
            if p.immediate {
                patterns.push(p);
            }
        }

        return patterns;
    };
    pub static ref PATTERNS_NI: Vec<&'static Pattern> = {
        let mut patterns: Vec<&'static Pattern> = Vec::new();

        for p in PATTERNS.iter() {
            if !p.immediate {
                patterns.push(p);
            }
        }

        return patterns;
    };
    pub static ref PATTERNS_BY_PRI: HashMap<ThreatPri, &'static Vec<&'static Pattern>> = {
        let mut m: HashMap<ThreatPri, &'static Vec<&'static Pattern>> = HashMap::new();
        m.insert(ThreatPri::ALL, &PATTERNS);
        m.insert(ThreatPri::IMMEDIATE, &PATTERNS_I);
        m.insert(ThreatPri::NON_IMMEDIATE, &PATTERNS_NI);
        return m;
    };
}

#[derive(Clone, Debug)]
pub struct Threat {
    pub m: Match,
    pub pidx: isize,
    pub defcon: usize,
    pub critical_sqs: HashSet<Point>,
}

pub fn threat_item(m: Match, pattern: &Pattern) -> Threat {
    return Threat {
        m,
        pidx: pattern.index,
        defcon: pattern.defcon,
        critical_sqs: point_set_on_line(m.0, m.1, &pattern.critical_sqs),
    };
}

pub fn search_all_board(board: &Array2<u8>, color: u8, pri: ThreatPri) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_board(board, &p.pattern, color) {
            threats.push(threat_item(m, p));
        }
    }

    return threats;
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

    return threats;
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

    return threats;
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

    return nsqs;
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

    return nsqs;
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

    return nsqs;
}
