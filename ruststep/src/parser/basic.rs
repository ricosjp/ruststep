//! Parser for basic alphabets defined in the table 1 of ISO-10303-21

use super::combinator::*;
use nom::{
    branch::alt,
    character::complete::{char, satisfy},
    Parser,
};

/// latin_codepoint = [space] | [digit] | [lower] | [upper] | [special] | [reverse_solidus] | [apostrophe]
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

/// space = ` ` .
pub fn space(input: &str) -> ParseResult<char> {
    char(' ')(input)
}

/// digit = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

/// lower = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h`
///       | `i` | `j` | `k` | `l` | `m` | `n` | `o` | `p`
///       | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x`
///       | `y` | `z` .
pub fn lower(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

/// upper = `A` | `B` | `C` | `D` | `E` | `F` | `G` | `H`
///       | `I` | `J` | `K` | `L` | `M` | `N` | `O` | `P`
///       | `Q` | `R` | `S` | `T` | `U` | `V` | `W` | `X`
///       | `Y` | `Z` | `_` .
pub fn upper(input: &str) -> ParseResult<char> {
    satisfy(|c| matches!(c, 'A'..='Z' | '_')).parse(input)
}

/// special  = `!` | `"` | `*` | `$` | `%` | `&` | `.` | `#` | `+` | `,`  | `-` | `(` | `)` | `?` | `/` | `:` | `;` | `<`  | `=` | `>` | `@` | `[` | `]` | `{` | `|` | `}`  | `^` | \` | `~` .
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

/// reverse_solidus = `\\` .
pub fn reverse_solidus(input: &str) -> ParseResult<char> {
    char('\\')(input)
}

/// apostrophe = `'` .
pub fn apostrophe(input: &str) -> ParseResult<char> {
    char('\'')(input)
}
