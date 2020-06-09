//! Define struct to represent threat patterns, and related functions (search etc.).

use ndarray::prelude::*;
use consts::{GEN_ELEMS, EMPTY, MAX_DEFCON, OWN, WALL_ENEMY, NOT_OWN, GEN_ELEMS_TO_NAMES, MDFIT};
use geometry::{point_set_on_line, Point};
use pattern_search::{search_board, search_point, search_point_own,
                     search_board_next_sq, search_point_next_sq,
                     search_point_own_next_sq, one_step_from_straight_threat,
                     degree, defcon_from_degree, Match, NSQMatch};
use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct Pattern {
    pattern: Array1<u8>,
    critical_sqs: Array1<i8>,
    own_sqs: Array1<i8>,
    name: String,
    index: i8,
    empty_sqs: Array1<i8>,
    defcon: i8,
    immediate: bool,
}

impl Pattern {
    pub fn new(pattern: Array1<u8>, critical_sqs: Array1<i8>, name: String, index: i8) -> Self {
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
            assert!( (0 <= *sq && *sq < (length as i8)) && pattern[*sq as usize] == EMPTY);
        }

        assert!(name.len() > 0);

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

        let mut own_sqs: Vec<i8> = Vec::new();
        for (i, v) in pattern.iter().enumerate() {
            if *v == OWN {
                own_sqs.push(i as i8);
            }
        }

        let mut other_empty_sqs: Vec<i8> = Vec::new();
        for (i, v) in pattern.iter().enumerate() {
            if *v == EMPTY && !critical_sqs.iter().any(|&x| x == (i as i8)) {
                other_empty_sqs.push(i as i8);
            }
        }

        let mut empty_sqs: Vec<i8> = Vec::new();
        for v in critical_sqs.iter() {
            empty_sqs.push(*v);
        }
        for v in other_empty_sqs.iter() {
            empty_sqs.push(*v);
        }

        let defcon = defcon_from_degree(degree(&pattern));

        let immediate = if defcon < 2 { true } else { one_step_from_straight_threat(&pattern) };

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
            pattern: pattern,
            critical_sqs: critical_sqs,
            own_sqs: Array1::from(own_sqs),
            name: name,
            index: index,
            empty_sqs: Array1::from(empty_sqs),
            defcon: defcon,
            immediate: immediate,
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

#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub enum ThreatPri {
    ALL,
    IMMEDIATE,
    NON_IMMEDIATE,
}

lazy_static! {
    pub static ref P_WIN: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([OWN, OWN, OWN, OWN, OWN])),
                                                 Array1::from(Vec::<i8>::from([])),
                                                 String::from("P_WIN"),
                                                 0);

    pub static ref P_4_ST: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, OWN, OWN, OWN, OWN, EMPTY])),
                                                  Array1::from(Vec::<i8>::from([])),
                                                  String::from("P_4_ST"),
                                                  1);

    pub static ref P_4_A: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, OWN, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([5])),
                                                 String::from("P_4_A"),
                                                 2);

    pub static ref P_4_B: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([NOT_OWN, OWN, OWN, OWN, EMPTY, OWN])),
                                                 Array1::from(Vec::<i8>::from([4])),
                                                 String::from("P_4_B"),
                                                 3);

    pub static ref P_4_C: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([NOT_OWN, OWN, OWN, EMPTY, OWN, OWN, NOT_OWN])),
                                                 Array1::from(Vec::<i8>::from([3])),
                                                 String::from("P_4_C"),
                                                 4);

    pub static ref P_3_ST: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY])),
                                                  Array1::from(Vec::<i8>::from([1, 5])),
                                                  String::from("P_3_ST"),
                                                  5);

    pub static ref P_3_A: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([1, 5, 6])),
                                                 String::from("P_3_A"),
                                                 6);

    pub static ref P_3_B: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, OWN, OWN, EMPTY, OWN, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([0, 3, 5])),
                                                 String::from("P_3_B"),
                                                 7);

    pub static ref P_3_C: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, OWN, EMPTY, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([4, 5])),
                                                 String::from("P_3_C"),
                                                 8);

    pub static ref P_3_D: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, OWN, EMPTY, OWN, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([3, 5])),
                                                 String::from("P_3_D"),
                                                 9);

    pub static ref P_3_E: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, OWN, EMPTY, OWN, OWN, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([2, 5])),
                                                 String::from("P_3_E"),
                                                 10);

    pub static ref P_3_F: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, WALL_ENEMY])),
                                                 Array1::from(Vec::<i8>::from([1, 5])),
                                                 String::from("P_3_F"),
                                                 11);

    pub static ref P_3_G: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([NOT_OWN, OWN, OWN, EMPTY, EMPTY, OWN])),
                                                 Array1::from(Vec::<i8>::from([3, 4])),
                                                 String::from("P_3_G"),
                                                 12);

    pub static ref P_3_H: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([NOT_OWN, OWN, EMPTY, OWN, EMPTY, OWN, NOT_OWN])),
                                                 Array1::from(Vec::<i8>::from([2, 4])),
                                                 String::from("P_3_H"),
                                                 13);

    pub static ref P_2_A: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, EMPTY, OWN, OWN, EMPTY, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([0, 1, 4, 5])),
                                                 String::from("P_2_A"),
                                                 14);

    pub static ref P_2_B: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, EMPTY, OWN, EMPTY, OWN, EMPTY, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([0, 1, 3, 5, 6])),
                                                 String::from("P_2_B"),
                                                 15);

    pub static ref P_2_C: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([EMPTY, OWN, EMPTY, EMPTY, OWN, EMPTY])),
                                                 Array1::from(Vec::<i8>::from([0, 2, 3, 5])),
                                                 String::from("P_2_C"),
                                                 16);

    pub static ref PATTERNS: Vec<&'static Pattern> = {
        let patterns: [&'static Pattern; 17] = [&(*P_WIN), &(*P_4_ST), &(*P_4_A), &(*P_4_B), &(*P_4_C), &(*P_3_ST), &(*P_3_A), &(*P_3_B),
                                                &(*P_3_C), &(*P_3_D), &(*P_3_E), &(*P_3_F), &(*P_3_G), &(*P_3_H),
                                                &(*P_2_A), &(*P_2_B), &(*P_2_C)];

        for (i, p) in patterns.iter().enumerate() {
            assert_eq!(i as i8, p.index);
        }

        let num_names = patterns.iter().map(|&x| String::from(&x.name)).collect::<HashSet<String>>().len();
        assert_eq!(num_names, patterns.len());

        let max_defcon_imm = patterns.iter().fold(i8::MIN, |a, b| if b.immediate { a.max(b.defcon) } else { a });
        assert_eq!(max_defcon_imm, MDFIT);

        return Vec::from(patterns);
    };

    pub static ref NUM_PTNS: usize = PATTERNS.len();

    pub static ref PATTERNS_BY_DEFCON: HashMap<i8, Vec<&'static Pattern>> = {
        let mut m: HashMap<i8, Vec<&'static Pattern>> = HashMap::new();

        for p in PATTERNS.iter() {
            if m.contains_key(&p.defcon) {
                m.get_mut(&p.defcon).unwrap().push(p);
            } else {
                m.insert(p.defcon, Vec::<&'static Pattern>::new());
                m.get_mut(&p.defcon).unwrap().push(p);
            }
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

#[derive(Clone,Debug)]
pub struct Threat {
    m: Match,
    pidx: i8,
    defcon: i8,
    critical_sqs: HashSet<Point>,
}

pub fn threat_item(m: Match, pattern: &Pattern) -> Threat {
    return Threat {
        m: m,
        pidx: pattern.index,
        defcon: pattern.defcon,
        critical_sqs: point_set_on_line(m.0, m.1, &pattern.critical_sqs),
    };
}

// def search_all_board(board, color, pri):
//     return [threat_item(match, p)
//             for p in PATTERNS_BY_PRI[pri]
//             for match in search_board(board, p.pattern, color)]


// def search_all_point(board, color, point, pri):
//     return [threat_item(match, p)
//             for p in PATTERNS_BY_PRI[pri]
//             for match in search_point(board, p.pattern, color, point)]


// def search_all_point_own(board, color, point, pri):
//     return [threat_item(match, p)
//             for p in PATTERNS_BY_PRI[pri]
//             for match in search_point_own(board, p.pattern, color, point, p.own_sqs)]


// def search_all_board_get_next_sqs(board, color, pri):
//     return {x[0] for p in PATTERNS_BY_PRI[pri]
//             for x in search_board_next_sq(board, p.pattern, color)}


// def search_all_point_get_next_sqs(board, color, point, pri):
//     return {x[0] for p in PATTERNS_BY_PRI[pri]
//             for x in search_point_next_sq(board, p.pattern, color, point)}


// def search_all_point_own_get_next_sqs(board, color, point, pri):
//     return {x[0] for p in PATTERNS_BY_PRI[pri]
//             for x in search_point_own_next_sq(board, p.pattern, color, point, p.own_sqs)}
