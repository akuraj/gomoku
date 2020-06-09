//! Define struct to represent threat patterns, and related functions (search etc.).

use ndarray::prelude::*;
use consts::{GEN_ELEMS, EMPTY, MAX_DEFCON, OWN, WALL_ENEMY, NOT_OWN, GEN_ELEMS_TO_NAMES, MDFIT};
use geometry::point_set_on_line;
use pattern_search::{search_board, search_point, search_point_own,
                     search_board_next_sq, search_point_next_sq,
                     search_point_own_next_sq, one_step_from_straight_threat,
                     degree, defcon_from_degree};
use std::fmt;

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

lazy_static! {
    pub static ref P_WIN: Pattern = Pattern::new(Array1::from(Vec::<u8>::from([OWN, OWN, OWN, OWN, OWN])),
                                                 Array1::from(Vec::<i8>::from([])),
                                                 String::from("P_WIN"),
                                                 0);
}

// # Threat patterns (including low priority threats).
// P_4_ST = Pattern([EMPTY, OWN, OWN, OWN, OWN, EMPTY], [], "P_4_ST")
// P_4_A = Pattern([WALL_ENEMY, OWN, OWN, OWN, OWN, EMPTY], [5], "P_4_A")
// P_4_B = Pattern([NOT_OWN, OWN, OWN, OWN, EMPTY, OWN], [4], "P_4_B")
// P_4_C = Pattern([NOT_OWN, OWN, OWN, EMPTY, OWN, OWN, NOT_OWN], [3], "P_4_C")
// P_3_ST = Pattern([EMPTY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY], [1, 5], "P_3_ST")
// P_3_A = Pattern([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, EMPTY], [1, 5, 6], "P_3_A")
// P_3_B = Pattern([EMPTY, OWN, OWN, EMPTY, OWN, EMPTY], [0, 3, 5], "P_3_B")
// P_3_C = Pattern([WALL_ENEMY, OWN, OWN, OWN, EMPTY, EMPTY], [4, 5], "P_3_C")
// P_3_D = Pattern([WALL_ENEMY, OWN, OWN, EMPTY, OWN, EMPTY], [3, 5], "P_3_D")
// P_3_E = Pattern([WALL_ENEMY, OWN, EMPTY, OWN, OWN, EMPTY], [2, 5], "P_3_E")
// P_3_F = Pattern([WALL_ENEMY, EMPTY, OWN, OWN, OWN, EMPTY, WALL_ENEMY], [1, 5], "P_3_F")
// P_3_G = Pattern([NOT_OWN, OWN, OWN, EMPTY, EMPTY, OWN], [3, 4], "P_3_G")
// P_3_H = Pattern([NOT_OWN, OWN, EMPTY, OWN, EMPTY, OWN, NOT_OWN], [2, 4], "P_3_H")
// P_2_A = Pattern([EMPTY, EMPTY, OWN, OWN, EMPTY, EMPTY], [0, 1, 4, 5], "P_2_A")
// P_2_B = Pattern([EMPTY, EMPTY, OWN, EMPTY, OWN, EMPTY, EMPTY], [0, 1, 3, 5, 6], "P_2_B")
// P_2_C = Pattern([EMPTY, OWN, EMPTY, EMPTY, OWN, EMPTY], [0, 2, 3, 5], "P_2_C")

// # NOTE: Put all the patterns defined above in this list.
// PATTERNS = [P_WIN, P_4_ST, P_4_A, P_4_B, P_4_C, P_3_ST, P_3_A, P_3_B,
//             P_3_C, P_3_D, P_3_E, P_3_F, P_3_G, P_3_H,
//             P_2_A, P_2_B, P_2_C]

// # Setting indices of PATTERNS.
// for i, p in enumerate(PATTERNS):
//     p.index = i

// NUM_PTNS = len(PATTERNS)

// PATTERNS_BY_DEFCON = dict()
// for p in PATTERNS:
//     if p.defcon in PATTERNS_BY_DEFCON:
//         PATTERNS_BY_DEFCON[p.defcon].append(p)
//     else:
//         PATTERNS_BY_DEFCON[p.defcon] = [p]

// PATTERNS_BY_NAME = dict()
// for p in PATTERNS:
//     assert p.name not in PATTERNS_BY_NAME
//     PATTERNS_BY_NAME[p.name] = p

// # Immediate/High Priority PATTERNS.
// PATTERNS_I = [x for x in PATTERNS if x.immediate]

// # Low Priority PATTERNS.
// PATTERNS_NI = [x for x in PATTERNS if not x.immediate]

// # Check against MDFIT.
// assert MDFIT == max([p.defcon for p in PATTERNS if p.immediate])


// # *** THREAT PRIORITY ENUM ***


// @unique
// class ThreatPri(IntEnum):
//     """Enum to represent the priority of a Threat."""

//     ALL = auto()
//     IMMEDIATE = auto()
//     NON_IMMEDIATE = auto()


// PATTERNS_BY_PRI = dict()
// PATTERNS_BY_PRI[ThreatPri.ALL] = PATTERNS
// PATTERNS_BY_PRI[ThreatPri.IMMEDIATE] = PATTERNS_I
// PATTERNS_BY_PRI[ThreatPri.NON_IMMEDIATE] = PATTERNS_NI


// # *** PATTERN SEARCH FUNCTIONS ***


// def threat_item(match, pattern):
//     """Threat: the where and the what."""

//     return {"match": match,
//             "pidx": pattern.index,
//             "defcon": pattern.defcon,
//             "critical_sqs": point_set_on_line(match[0],
//                                               match[1],
//                                               pattern.critical_sqs)}


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
