//! Defines required constants that will be used in this project.

use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

/// The actual side length.
pub const SIDE_LEN_ACT: usize = 15;

/// The side length including the walls on both sides.
pub const SIDE_LEN: usize = SIDE_LEN_ACT + 2;

/// Represents an empty point.
pub const EMPTY: u8 = 1;

/// Represents a black stone.
pub const BLACK: u8 = 1 << 1;

/// Represents a white stone.
pub const WHITE: u8 = 1 << 2;

/// Represents the wall.
pub const WALL: u8 = 1 << 3;

/// List of all the constants representing actual elements.
pub const ACT_ELEMS: [u8; 4] = [EMPTY, BLACK, WHITE, WALL];

pub const COLORS: [u8; 2] = [BLACK, WHITE];

/// Represents a black stone. Display colors may be switched via a flag.
pub const BLACK_CIRCLE: char = '●';

/// Represents a white stone. Display colors may be switched via a flag.
pub const WHITE_CIRCLE: char = '○';

/// Represents an empty point on the board.
pub const EMPTY_CHR: char = '+';

/// Represents the wall.
pub const WALL_CHR: char = ' ';

/// Flag to control display color switching.
/// Need to switch strone colors if you are using a dark theme in your console.
pub const SWITCH_DISPLAY_COLORS: bool = true;

/// For printing non-standard elements. Useful for debugging.
pub const SPL_ELEM_CHR: char = '!';

// Define ACT_ELEMS_TO_CHRS and ACT_ELEMS_TO_NAMES.
// Also check that ACT_ELEMS are unique and defined without mutual overlap of bits.
lazy_static! {
    /// Map from actual element to display character.
    pub static ref ACT_ELEMS_TO_CHRS: HashMap<u8, char> = {
        // Checks on ACT_ELEMS.
        for (i, a) in ACT_ELEMS.iter().enumerate() {
            for b in ACT_ELEMS.iter().skip(i + 1) {
                assert!(a != b && a & b == 0);
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

        m
    };

    /// Map from actual element to display name.
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

        m
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

/// Generic elements are specified from black's POV.
/// Used to represent elements of a generic pattern.
pub const GEN_ELEMS: [u8; 11] = [
    EMPTY, WALL, OWN, ENEMY, STONE, ANY, NOT_EMPTY, NOT_WALL, NOT_STONE, NOT_OWN, WALL_ENEMY,
];

lazy_static! {
    /// Map from generic element to display name.
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

        m
    };
}

/// 4 cardinal directions + 4 ordinal directions.
pub const NUM_DIRECTIONS: usize = 8;

/// WIN_LENGTH is the length of a winning sequence.
/// Some things implicitly assume a win length of 5, for example, threat pattern definitions.
/// Don't change WIN_LENGTH without making all other relevant changes everywhere else in the project.
pub const WIN_LENGTH: usize = 5;

/// If defcon is x, then game will be over in x moves if no action is taken. 0 is game over.
/// Effectively, the maximum distance away from winning.
pub const MAX_DEFCON: usize = WIN_LENGTH;

/// Max defcon for an immediate threat.
pub const MDFIT: usize = 2;

/// Used for Unicode character conversion.
pub const RADIX: u32 = 36;

/// The timestep for animation of a variation.
pub const ANIMATION_TIMESTEP_SECS: u64 = 2;
