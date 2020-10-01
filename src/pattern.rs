//! Define struct to represent threat patterns, and related functions (search etc.).

use crate::consts::{EMPTY, GEN_ELEMS, GEN_ELEMS_TO_NAMES, MAX_DEFCON, MDFIT, NOT_OWN, OWN, SIDE_LEN, WALL_ENEMY};
use crate::geometry::{point_set_on_line, Point};
use crate::pattern_search::{
    defcon_from_degree, degree, one_step_from_straight_threat, search_board, search_board_next_sq, search_point, search_point_next_sq,
    search_point_own, search_point_own_next_sq, Match,
};
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use lazy_static::lazy_static;
use ndarray::prelude::*;
use std::fmt;

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

        // Critical Squares are the places where if the oppenent plays,
        // then the threat is mitigated.
        // Checks on critical_sqs.
        let mut critical_sqs_new = critical_sqs.to_owned();
        critical_sqs_new.sort();
        critical_sqs_new.dedup();
        assert_eq!(critical_sqs, critical_sqs_new);

        let length = pattern.len();
        assert!(length <= SIDE_LEN);

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

        let own_sqs = pattern
            .iter()
            .enumerate()
            .filter(|x| *x.1 == OWN)
            .map(|x| x.0 as isize)
            .collect::<Vec<isize>>();

        let other_empty_sqs = pattern
            .iter()
            .enumerate()
            .filter(|x| *x.1 == EMPTY && !critical_sqs.iter().any(|&y| y == (x.0 as isize)))
            .map(|x| x.0 as isize)
            .collect::<Vec<isize>>();

        // Add entry for empty_sqs. critical_sqs appear first.
        let empty_sqs = critical_sqs.iter().chain(other_empty_sqs.iter()).cloned().collect::<Vec<isize>>();

        let defcon = defcon_from_degree(degree(&pattern));

        let immediate = if defcon < 2 { true } else { one_step_from_straight_threat(&pattern) };

        // Checks on data fields.
        assert!(!pattern.is_empty());
        assert!((0..=MAX_DEFCON).contains(&defcon));

        // Check on empty_sqs that they need to be useful.
        let curr_degree = degree(&pattern);
        for esq in empty_sqs.iter() {
            let mut next_pattern = pattern.to_owned();
            next_pattern[*esq as usize] = OWN;
            assert_eq!(degree(&next_pattern), curr_degree + 1);
        }

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
            .collect::<FnvHashSet<String>>()
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

    /// Patterns by defcon.
    pub static ref PATTERNS_BY_DEFCON: FnvHashMap<usize, Vec<&'static Pattern>> = {
        let mut m: FnvHashMap<usize, Vec<&'static Pattern>> = FnvHashMap::default();

        for p in PATTERNS.iter() {
            m.entry(p.defcon)
                .or_insert_with(Vec::<&'static Pattern>::new)
                .push(p);
        }

        m
    };

    /// Patterns by name.
    pub static ref PATTERNS_BY_NAME: FnvHashMap<String, &'static Pattern> = {
        let mut m: FnvHashMap<String, &'static Pattern> = FnvHashMap::default();

        for p in PATTERNS.iter() {
            assert!(!m.contains_key(&p.name));
            m.insert(String::from(&p.name), p);
        }

        m
    };

    /// Immediate/High Priority PATTERNS.
    pub static ref PATTERNS_I: Vec<&'static Pattern> = {
        PATTERNS.iter().filter(|x| x.immediate).copied().collect::<Vec<&'static Pattern>>()
    };

    /// NonImmediate/Low Priority PATTERNS.
    pub static ref PATTERNS_NI: Vec<&'static Pattern> = {
        PATTERNS.iter().filter(|x| !x.immediate).copied().collect::<Vec<&'static Pattern>>()
    };

    /// Patterns by priority.
    pub static ref PATTERNS_BY_PRI: FnvHashMap<ThreatPri, &'static Vec<&'static Pattern>> = {
        let mut m: FnvHashMap<ThreatPri, &'static Vec<&'static Pattern>> = FnvHashMap::default();
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
    pub critical_sqs: FnvHashSet<Point>,
}

impl Threat {
    pub fn new(m: Match, pattern: &Pattern) -> Self {
        Self {
            m,
            pidx: pattern.index,
            defcon: pattern.defcon,
            critical_sqs: point_set_on_line(m.0, m.1, &pattern.critical_sqs),
        }
    }
}

/// Get all pattern matches on the board.
pub fn search_all_board(board: &Array2<u8>, color: u8, pri: ThreatPri) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_board(board, &p.pattern, color) {
            threats.push(Threat::new(m, p));
        }
    }

    threats
}

/// Get all pattern matches including the given point.
pub fn search_all_point(board: &Array2<u8>, color: u8, point: Point, pri: ThreatPri) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_point(board, &p.pattern, color, point) {
            threats.push(Threat::new(m, p));
        }
    }

    threats
}

/// Get all pattern matches including the given point as an own_sq.
pub fn search_all_point_own(board: &Array2<u8>, color: u8, point: Point, pri: ThreatPri) -> Vec<Threat> {
    let mut threats: Vec<Threat> = Vec::new();

    for p in PATTERNS_BY_PRI[&pri] {
        for m in search_point_own(board, &p.pattern, color, point, &p.own_sqs) {
            threats.push(Threat::new(m, p));
        }
    }

    threats
}

/// Get all next_sqs on the board.
pub fn search_all_board_get_next_sqs(board: &Array2<u8>, color: u8, pri: ThreatPri) -> FnvHashSet<Point> {
    let mut nsqs: FnvHashSet<Point> = FnvHashSet::default();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_board_next_sq(board, &p.pattern, color) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}

/// Get all next_sqs including the given point.
pub fn search_all_point_get_next_sqs(board: &Array2<u8>, color: u8, point: Point, pri: ThreatPri) -> FnvHashSet<Point> {
    let mut nsqs: FnvHashSet<Point> = FnvHashSet::default();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_point_next_sq(board, &p.pattern, color, point) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}

/// Get all next_sqs including the given point as an own_sq.
pub fn search_all_point_own_get_next_sqs(board: &Array2<u8>, color: u8, point: Point, pri: ThreatPri) -> FnvHashSet<Point> {
    let mut nsqs: FnvHashSet<Point> = FnvHashSet::default();

    for p in PATTERNS_BY_PRI[&pri] {
        for x in search_point_own_next_sq(board, &p.pattern, color, point, &p.own_sqs) {
            nsqs.insert(x.0);
        }
    }

    nsqs
}
