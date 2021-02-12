//! Parser for basic alphabet definitions

use super::combinator::*;
use nom::{
    branch::alt,
    character::complete::{char, satisfy},
    Parser,
};

/// LATIN_CODEPOINT = SPACE | DIGIT | LOWER | UPPER | SPECIAL | REVERSE_SOLIDUS | APOSTROPHE
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

/// SPACE = ` ` .
pub fn space(input: &str) -> ParseResult<char> {
    char(' ')(input)
}

/// DIGIT = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

/// LOWER = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h`
///       | `i` | `j` | `k` | `l` | `m` | `n` | `o` | `p`
///       | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x`
///       | `y` | `z` .
pub fn lower(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

/// UPPER = `A` | `B` | `C` | `D` | `E` | `F` | `G` | `H`
///       | `I` | `J` | `K` | `L` | `M` | `N` | `O` | `P`
///       | `Q` | `R` | `S` | `T` | `U` | `V` | `W` | `X`
///       | `Y` | `Z` | `_` .
pub fn upper(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'A'..='Z' | '_')).parse(input)
}

/// SPECIAL  = `!` | `"` | `*` | `$` | `%` | `&` | `.` | `#` | `+` | `,`  | `-` | `(` | `)` | `?` | `/` | `:` | `;` | `<`  | `=` | `>` | `@` | `[` | `]` | `{` | `|` | `}`  | `^` | \` | `~` .
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

/// REVERSE_SOLIDUS = `\\` .
pub fn reverse_solidus(input: &str) -> ParseResult<char> {
    char('\\')(input)
}

/// APOSTROPHE = `'` .
pub fn apostrophe(input: &str) -> ParseResult<char> {
    char('\'')(input)
}
