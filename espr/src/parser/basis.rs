use nom::{
    branch::*, character::complete::*, error::VerboseError, multi::*, sequence::*, IResult, Parser,
};

/// 128 letter = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` |`m` | `n` | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` |`y` | `z` .
pub fn letter(input: &str) -> IResult<&str, char, VerboseError<&str>> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

/// 124 digit = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> IResult<&str, char, VerboseError<&str>> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

/// 143 simple_id = [letter] { [letter] | [digit] | `_` } .
pub fn simple_id(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    tuple((letter, many0(alt((letter, digit, char('_'))))))
        .map(|(head, tail)| format!("{}{}", head, tail.into_iter().collect::<String>()))
        .parse(input)
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

        // Capital is not allowed
        assert!(super::letter("H").finish().is_err());
        // Number is not allowed
        assert!(super::letter("2").finish().is_err());
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
    fn simple_id_valid() {
        let (residual, id) = super::simple_id("h").finish().unwrap();
        assert_eq!(id, "h");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("homhom").finish().unwrap();
        assert_eq!(id, "homhom");
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
        // Capital is not allowed
        assert!(super::simple_id("HomHom").finish().is_err());
        // `_` cannot use as first
        assert!(super::simple_id("_homhom").finish().is_err());
        // digit cannot use as first
        assert!(super::simple_id("1homhom").finish().is_err());
        // Empty is invlaid
        assert!(super::simple_id("").finish().is_err());
    }
}
