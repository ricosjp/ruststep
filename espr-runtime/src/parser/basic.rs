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

use super::combinator::*;
use nom::{
    branch::alt,
    character::complete::{char, satisfy},
    Parser,
};

pub fn latin_codepoint(input: &str) -> ParseResult<char> {
    alt((
        space,
        digit,
        lower,
        upper,
        special,
        reverse_solidus,
        apostrophe,
    ))
    .parse(input)
}

pub fn space(input: &str) -> ParseResult<char> {
    char(' ')(input)
}

pub fn digit(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

pub fn lower(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

pub fn upper(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'A'..='Z' | '_')).parse(input)
}

pub fn special(input: &str) -> ParseResult<char> {
    satisfy(|c| {
        matches!(
            c,
            '!' | '"'
                | '*'
                | '$'
                | '%'
                | '&'
                | '.'
                | '#'
                | '+'
                | ','
                | '-'
                | '('
                | ')'
                | '?'
                | '/'
                | ':'
                | ';'
                | '<'
                | '='
                | '>'
                | '@'
                | '['
                | ']'
                | '{'
                | '|'
                | '}'
                | '^'
                | '`'
                | '~'
        )
    })
    .parse(input)
}

pub fn reverse_solidus(input: &str) -> ParseResult<char> {
    char('\\')(input)
}

pub fn apostrophe(input: &str) -> ParseResult<char> {
    char('\'')(input)
}
