//! Defines required constants that will be used in this project.

use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

pub const SIDE_LEN_ACT: usize = 15;
pub const SIDE_LEN: usize = SIDE_LEN_ACT + 2; // Including the walls.

// Actual elements.
// All the actual elements defined below must be part of ACT_ELEMS,
// as well as ACT_ELEMS_TO_CHRS and ACT_ELEMS_TO_NAMES.
pub const EMPTY: u8 = 1 << 0;
pub const BLACK: u8 = 1 << 1;
pub const WHITE: u8 = 1 << 2;
pub const WALL: u8 = 1 << 3;

pub const ACT_ELEMS: [u8; 4] = [EMPTY, BLACK, WHITE, WALL];
pub const NUM_ACT_ELEMS: usize = ACT_ELEMS.len();

pub const COLORS: [u8; 2] = [BLACK, WHITE];

pub const BLACK_CIRCLE: char = '●';
pub const WHITE_CIRCLE: char = '○';
pub const EMPTY_CHR: char = '+';
pub const WALL_CHR: char = ' ';

// NOTE: Need to switch strone colors if you are using a dark theme in your console.
pub const SWITCH_DISPLAY_COLORS: bool = true;

// For printing non-standard elements. Useful for debugging.
pub const SPL_ELEM_CHR: char = '!';

// Define ACT_ELEMS_TO_CHRS and ACT_ELEMS_TO_NAMES.
// Also check that ACT_ELEMS are unique and defined without mutual overlap of bits.
lazy_static! {
    pub static ref ACT_ELEMS_TO_CHRS: HashMap<u8, char> = {
        // Checks on ACT_ELEMS.
        for i in 0..NUM_ACT_ELEMS {
            for j in (i + 1)..NUM_ACT_ELEMS {
                assert!(ACT_ELEMS[i] != ACT_ELEMS[j]);
                assert!(ACT_ELEMS[i] & ACT_ELEMS[j] == 0);
            }
        }

        let mut m = HashMap::new();
        m.insert(EMPTY, EMPTY_CHR);
        m.insert(BLACK, if SWITCH_DISPLAY_COLORS { WHITE_CIRCLE } else { BLACK_CIRCLE });
        m.insert(WHITE, if SWITCH_DISPLAY_COLORS { BLACK_CIRCLE } else { WHITE_CIRCLE });
        m.insert(WALL, WALL_CHR);

        // Make sure all ACT_ELEMS are represented and corresponding chars are unique.
        assert_eq!(m.keys().cloned().collect::<HashSet<u8>>(),
                   ACT_ELEMS.iter().cloned().collect::<HashSet<u8>>());
        assert_eq!(m.values().cloned().collect::<HashSet<char>>().len(), ACT_ELEMS.len());

        return m;
    };

    pub static ref ACT_ELEMS_TO_NAMES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(EMPTY, "EMPTY");
        m.insert(BLACK, "BLACK");
        m.insert(WHITE, "WHITE");
        m.insert(WALL, "WALL");

        // Make sure all ACT_ELEMS are represented and corresponding names are unique.
        assert_eq!(m.keys().cloned().collect::<HashSet<u8>>(),
                   ACT_ELEMS.iter().cloned().collect::<HashSet<u8>>());
        assert_eq!(m.values().cloned().collect::<HashSet<&str>>().len(), ACT_ELEMS.len());

        return m;
    };
}

// NOTE: Generic Patterns are specified from BLACK's POV.
// NOTE: Allowed values include the below plus EMPTY and WALL (see GEN_ELEMS).
// NOTE: If you define a new generic element, please add it to GEN_ELEMS
//       as well as GEN_ELEMS_TO_NAMES.
pub const OWN: u8 = BLACK;
pub const ENEMY: u8 = WHITE;
pub const STONE: u8 = OWN | ENEMY;
pub const ANY: u8 = EMPTY | STONE | WALL;
pub const NOT_EMPTY: u8 = ANY ^ EMPTY;
pub const NOT_WALL: u8 = ANY ^ WALL;
pub const NOT_STONE: u8 = ANY ^ STONE;
pub const NOT_OWN: u8 = ANY ^ OWN;
pub const WALL_ENEMY: u8 = WALL | ENEMY;

pub const GEN_ELEMS: [u8; 11] = [
    EMPTY, WALL, OWN, ENEMY, STONE, ANY, NOT_EMPTY, NOT_WALL, NOT_STONE, NOT_OWN, WALL_ENEMY,
];

lazy_static! {
    pub static ref GEN_ELEMS_TO_NAMES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(EMPTY, "EMPTY");
        m.insert(WALL, "WALL");
        m.insert(OWN, "OWN");
        m.insert(ENEMY, "ENEMY");
        m.insert(STONE, "STONE");
        m.insert(ANY, "ANY");
        m.insert(NOT_EMPTY, "NOT_EMPTY");
        m.insert(NOT_WALL, "NOT_WALL");
        m.insert(NOT_STONE, "NOT_STONE");
        m.insert(NOT_OWN, "NOT_OWN");
        m.insert(WALL_ENEMY, "WALL_ENEMY");

        // Make sure all GEN_ELEMS are represented and corresponding names are unique.
        assert_eq!(m.keys().cloned().collect::<HashSet<u8>>(),
                   GEN_ELEMS.iter().cloned().collect::<HashSet<u8>>());
        assert_eq!(m.values().cloned().collect::<HashSet<&str>>().len(), GEN_ELEMS.len());

        return m;
    };
}

pub const NUM_DIRECTIONS: usize = 8;

// WIN_LENGTH is the length of a winning sequence.
// Some things implicitly assume a win length of 5, for example, threat pattern definitions.
// Don't change WIN_LENGTH without making all other relevant changes everywhere else in the project.
pub const WIN_LENGTH: usize = 5;

// If defcon is x, then game will be over in x moves if no action is taken. 0 is game over.
// Effectively, the maximum distance away from winning.
pub const MAX_DEFCON: usize = WIN_LENGTH;

// Max defcon for immediate threat.
pub const MDFIT: usize = 2;

// Used for Unicode character conversion.
pub const RADIX: u32 = 36;

pub const ANIMATION_TIMESTEP_SECS: u64 = 2;
