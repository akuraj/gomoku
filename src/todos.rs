//! A place for general FIXMEs and TODOs.

// *** TODO SOON ***
// TODO: Too many garbage variations when using LowPri threats!
//       0) Create a TSS which fully takes into account the opponent's threats.
//          Handle different possible min_defcon combinations among the threats.
//          Maintain, update, pass-down threats (both own and opp).
//       1) Make a note in search node when we cut off variation due to the opponent's potential win!
//       2) Sort win variations by length? Shorter is better.
// TODO: Remove "threats" and "critical_sqs" from SearchNode?
//       They are only useful for debugging purposes, right?
//       Or maybe we can change the threats etc. data that we store?
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
