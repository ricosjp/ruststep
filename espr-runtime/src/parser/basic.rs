//! Parser for basic alphabet definitions
//!
//! Table 1 — WSN defining subsets of the basic alphabet
//! -----------------------------------------------------
//!
//! ```text
//! SPACE    = " " .
//! DIGIT    = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" .
//! LOWER    = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"
//!          | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p"
//!          | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x"
//!          | "y" | "z" .
//! UPPER    = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H"
//!          | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
//!          | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X"
//!          | "Y" | "Z" | "_" .
//! SPECIAL  = "!" | """" | "*" | "$" | "%" | "&" | "." | "#" | "+" | ","  | "-" | "(" | ")" | "?" | "/" | ":" | ";" | "<"  | "=" | ">" | "@" | "[" | "]" | "{" | "|" | "}"  | "^" | "`" | "~" .
//! REVERSE_SOLIDUS  = "\" .
//! APOSTROPHE = "'" .
//! LATIN_CODEPOINT = SPACE | DIGIT | LOWER | UPPER | SPECIAL | REVERSE_SOLIDUS | APOSTROPHE
//! HIGH_CODEPOINT = (U+0080 to U+10FFFF, see 5.2)
//! ```
