//! A place for general FIXMEs and TODOs.

// *** TODO SOON ***
// TODO: "win" should probably be renamed to "potenial_win",
//       as we are not looking at the opponent's counter attack possibilities?
// TODO: Add Victoria's games as test positions.
// TODO: Change Threat.m to Threat.match.
// TODO: Too many garbage variations when using LowPri threats!
//       0) Check that the comprehensive threat handling is efficient.
//       1) Dedupe all_threats!
//       2) Make a note in search node when we cut off variation due to the opponent's potential win!
//       3) Sort win variations by length? Shorter is better.
//       4) Handling of win pattern consistently in threat space search.
//       5) Add "TSS" to the names of types in threat space search.
// TODO: Update python impl!
// TODO: Fix critical_sqs pattern in tss_next_sq.
// TODO: Fix the line wrapping logic in cargo fmt to have longer lines.
// TODO: Refactor potential loss code.
// TODO: Implement a transposition table. How do we cope with having different last_sqs?
//       A Zobrit hash? Or just use a dict? What's the most efficient data structure?
//       A hash that depends on position as well as latest move?
// TODO: We don't actually need critical sqs for NON_IMMEDIATE threats.
//       Should we keep them as they are currently?
// TODO: Use Yixin to check for best moves in test positions.
//       Use it to guide the development effort and debugging.
// TODO: Check all test positions in paper. Implement regtests based on them.
// TODO: Remove unnecessary fields from threat data.
// TODO: Need function to search using a given sets of points as own sqs!
//       Something efficient and proper, not some hack.
// TODO: Need fn to search along direction at point?
// TODO: Can we have a better way to input position?

// *** Miscellaneous ***
// TODO: Take point as a tuple and not two separate arguments!

// *** Patterns and Threats; Pattern Matching ***
// TODO: Implement function to calculate intersection of pattern matches. What for though?

// *** Position Evaluation (Static/Semi-Static) ***
// TODO: Implement an evaluation scale.
// TODO: Use all patterns to assign a value to the position (including P_2_D?).
// TODO: null-move heuristic to see any short-term attacks?
// TODO: Use Positional Evaluation for quiet/positions not determined by TSS.
// TODO: Negamax Search?

// *** Profiling and Testing ***
// TODO: Create tests for algebraic related fns, state construction etc.

// *** Cleanup ***
// TODO: Remove unused functions (search related etc.).
// TODO: Remove unnecessary constants.

// *** State Represenation ***
// TODO: Stricter check on win status to make sure no multiple wins for a given player?
// TODO: Implement code to update status after move is made!
// TODO: Calculate and store Rich State?

// *** Standard Gomoku Implementation ***
// TODO: We can rely on the win pattern to differentiate between Standard and Freestyle.
// TODO: Specialize threats for Standard case.
// TODO: Control Freestyle or Standard via a global flag? Or better to do via State?
// TODO: Implement Standard Gomoku.

// *** Swap2 Implementation ***
// TODO: Implement Swap2 (and update state initialization, relevant checks, and code).
