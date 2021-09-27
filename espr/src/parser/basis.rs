use super::{combinator::RawParseResult, reserved::is_reserved};
use nom::{branch::*, character::complete::*, multi::*, sequence::*, Parser};

/// 128 letter = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` |`m` | `n` | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` |`y` | `z` .
pub fn letter(input: &str) -> RawParseResult<char> {
    satisfy(|c| matches!(c, 'A'..='Z' | 'a'..='z')).parse(input)
}

/// 124 digit = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> RawParseResult<char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

/// 127 hex_digit = [digit] | `a` | `b` | `c` | `d` | `e` | `f` .
pub fn hex_digit(input: &str) -> RawParseResult<u8> {
    let hex_letter = satisfy(|c| matches!(c, 'A'..='Z' | 'a'..='f'));
    alt((digit, hex_letter))
        .map(|c| c.to_digit(16).unwrap() as u8)
        .parse(input)
}

/// 136 octet = [hex_digit] [hex_digit] .
pub fn octet(input: &str) -> RawParseResult<u8> {
    tuple((hex_digit, hex_digit))
        .map(|(u, l)| {
            assert!(u < 16);
            assert!(l < 16);
            u * 16 + l
        })
        .parse(input)
}

/// 126 encoded_character = [octet] [octet] [octet] [octet] .
pub fn encoded_character(input: &str) -> RawParseResult<[u8; 4]> {
    tuple((octet, octet, octet, octet))
        .map(|(a, b, c, d)| [a, b, c, d])
        .parse(input)
}

/// 140 encoded_string_literal = `"` [encoded_character] { [encoded_character] } `"` .
pub fn encoded_string_literal(input: &str) -> RawParseResult<String> {
    tuple((char('"'), many1(encoded_character), char('"')))
        .map(|(_openq, chars, _closeq)| {
            let raw_chars: Vec<u8> = chars.iter().map(|c| c.iter()).flatten().cloned().collect();
            String::from_utf8(raw_chars).expect("non UTF8 input")
        })
        .parse(input)
}

/// 144 simple_string_literal = \q { ( \q \q ) | not_quote | \s | \x9 | \xA | \xD } \q .
pub fn simple_string_literal(input: &str) -> RawParseResult<String> {
    tuple((char('\''), many0(none_of("'")), char('\'')))
        .map(|(_open, chars, _close)| chars.into_iter().collect())
        .parse(input)
}

/// 143 simple_id = [letter] { [letter] | [digit] | `_` } .
/// According to the standard, identifiers cannot be reserved keywords.
pub fn simple_id(input: &str) -> RawParseResult<String> {
    if let Ok((input, id)) = tuple((letter, many0(alt((letter, digit, char('_'))))))
        .map(|(head, tail)| format!("{}{}", head, tail.into_iter().collect::<String>()))
        .parse(input)
    {
        if is_reserved(id.as_str()) {
            Err(nom::Err::Error(nom::error::VerboseError {
                errors: Vec::new(),
            }))
        } else {
            Ok((input, id))
        }
    } else {
        Err(nom::Err::Error(nom::error::VerboseError {
            errors: Vec::new(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn letter() {
        let (residual, l) = super::letter("h").finish().unwrap();
        assert_eq!(l, 'h');
        assert_eq!(residual, "");

        let (residual, l) = super::letter("abc").finish().unwrap();
        assert_eq!(l, 'a');
        assert_eq!(residual, "bc");

        let (residual, l) = super::letter("H").finish().unwrap();
        assert_eq!(l, 'H');
        assert_eq!(residual, "");

        let (residual, l) = super::letter("Hi").finish().unwrap();
        assert_eq!(l, 'H');
        assert_eq!(residual, "i");

        // Number is not allowed
        assert!(super::letter("2").finish().is_err());

        // Symbols are not allowed
        assert!(super::letter("\\").finish().is_err());
    }

    #[test]
    fn digit() {
        let (residual, l) = super::digit("123").finish().unwrap();
        assert_eq!(l, '1');
        assert_eq!(residual, "23");

        // Alphabets are not allowed
        assert!(super::digit("h").finish().is_err());
    }

    #[test]
    fn hex_digit() {
        let (residual, l) = super::hex_digit("a23").finish().unwrap();
        assert_eq!(l, 10);
        assert_eq!(residual, "23");

        let (residual, l) = super::hex_digit("F23").finish().unwrap();
        assert_eq!(l, 15);
        assert_eq!(residual, "23");

        assert!(super::hex_digit("x").finish().is_err());
    }

    #[test]
    fn encoded_character() {
        let (residual, l) = super::encoded_character("a0b1c2d3").finish().unwrap();
        assert_eq!(l, [0xa0, 0xb1, 0xc2, 0xd3]);
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_id_valid() {
        let (residual, id) = super::simple_id("h").finish().unwrap();
        assert_eq!(id, "h");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("homhom").finish().unwrap();
        assert_eq!(id, "homhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("homHom").finish().unwrap();
        assert_eq!(id, "homHom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("ho_mhom").finish().unwrap();
        assert_eq!(id, "ho_mhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("h10o_1mh2om").finish().unwrap();
        assert_eq!(id, "h10o_1mh2om");
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_id_invalid() {
        // `_` cannot use as first
        assert!(super::simple_id("_homhom").finish().is_err());
        // digit cannot use as first
        assert!(super::simple_id("1homhom").finish().is_err());
        // Empty is invalid
        assert!(super::simple_id("").finish().is_err());
        // IDs cannot consist of reserved keywords
        assert!(super::simple_id("end").finish().is_err());
        assert!(super::simple_id("end_entity").finish().is_err());
    }
}
