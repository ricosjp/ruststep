//! Parser combinators extended for STEP exchange structure
//!
//! This is helper submodule for writting a parser like as WSN definitions.
//!
//! Token separators in exchange structure is one of
//!
//! - space
//! - explicit print control directives (`\N\` and `\F\` )
//! - comments
//!
//! and combinators in this submodule responsible for handling them.

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, none_of},
    combinator::{not, opt, peek, value},
    error::VerboseError,
    multi::{many0, many1},
    sequence::tuple,
    IResult, Parser,
};

/// Parse result
pub type ParseResult<'a, X> = IResult<&'a str, X, VerboseError<&'a str>>;

/// Alias of `nom::Parser`
pub trait ExchangeParser<'a, X>: Clone + nom::Parser<&'a str, X, VerboseError<&'a str>> {}

impl<'a, X, T> ExchangeParser<'a, X> for T where
    T: Clone + nom::Parser<&'a str, X, VerboseError<&'a str>>
{
}

pub fn char_<'a>(c: char) -> impl ExchangeParser<'a, char> {
    move |input| {
        let (input, c) = nom::character::complete::char(c)(input)?;
        Ok((input, c))
    }
}

pub fn tag_<'a>(name: &'static str) -> impl ExchangeParser<'a, &'a str> {
    move |input| {
        let (input, c) = nom::bytes::complete::tag(name)(input)?;
        Ok((input, c))
    }
}

pub fn opt_<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Option<O>> {
    move |input| {
        let (input, c) = nom::combinator::opt(f.clone())(input)?;
        Ok((input, c))
    }
}

/// Comment
///
/// A comment shall be encoded as a solidus asterisk `/*`
/// followed by any number of characters from the basic alphabet,
/// and terminated by an asterisk solidus `*/`
///
/// These comments are dropped while parsing. Do not passed to following convert step.
///
pub fn comment(input: &str) -> ParseResult<String> {
    let internal = alt((
        none_of("*"),
        tuple((char('*'), peek(not(char('/'))))).map(|(star, _not_slash)| star),
    ));
    tuple((tag("/*"), many0(internal), tag("*/")))
        .map(|(_start, c, _end)| c.into_iter().collect())
        .parse(input)
}

/// Comments with front/back spaces, or multi-space at least 1 char
///
/// - This never matches to empty string.
/// - Drop matched comments and spaces
///
/// FIXME
/// ------
/// - support explicit print control directives
///
pub fn separator(input: &str) -> ParseResult<()> {
    let comment = many1(tuple((multispace0, comment, multispace0))).map(|_| ());
    alt((comment, value((), multispace1))).parse(input)
}

pub fn many0_<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        let (input, first) = opt(f.clone()).parse(input)?;
        if first.is_none() {
            return Ok((input, Vec::new()));
        };
        let (input, tail) = many0(tuple((ignorable, f.clone())).map(|(_sep, v)| v)).parse(input)?;
        let first = vec![first.unwrap()];
        let list = first.into_iter().chain(tail).collect();
        Ok((input, list))
    }
}

pub fn many1_<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        tuple((f.clone(), many0(tuple((ignorable, f.clone())))))
            .map(|(first, tail)| {
                let first = vec![first];
                let tail = tail.into_iter().map(|(_sep, val)| val);
                first.into_iter().chain(tail).collect()
            })
            .parse(input)
    }
}

pub fn ignorable(input: &str) -> ParseResult<()> {
    let comment = many1(tuple((multispace0, comment, multispace0))).map(|_| ());
    alt((comment, value((), multispace0))).parse(input)
}

pub fn separated<'a, O>(c: char, f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        tuple((
            f.clone(),
            many0(
                tuple((ignorable, char(c), ignorable, f.clone()))
                    .map(|(_sep1, _char, _sep2, value)| value),
            ),
        ))
        .map(|(first, mut tails)| {
            let mut values = vec![first];
            values.append(&mut tails);
            values
        })
        .parse(input)
    }
}

pub fn comma_separated<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    separated(',', f)
}

/// Sequence of separated tokens
pub fn tuple_<'a, O, List: Tuple<'a, O>>(mut l: List) -> impl ExchangeParser<'a, O> {
    move |input| l.parse(input)
}

/// helper for [tuple_]
pub trait Tuple<'a, O>: Clone {
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, O>;
}

/// Expand `tuple_gen!(f1, f2, f3)` to `tuple((f1, ignorable, tuple((f2, ignorable, f3))))`
macro_rules! tuple_gen {
    ($head:ident, $($tail:ident),*) => {
        tuple(($head.clone(), ignorable, tuple_gen!($($tail),*)))
    };
    ($head:ident) => {
        $head.clone()
    };
}

/// Expand `match_gen!(o1, o2, o3)` to `(o1, _, (o2, _, o3))`
macro_rules! match_gen {
    ($head:ident, $($tail:ident),*) => {
        ($head, _, match_gen!($($tail),*))
    };
    ($head:ident) => {
        $head
    };
}

macro_rules! impl_tuple {
    ($($F:ident),*; $($O:ident),*; $($f:ident),*; $($o:ident),*) => {
        impl<'a, $($F),*, $($O),*> Tuple<'a, ($($O),*)> for ($($F),*)
        where
            $( $F: ExchangeParser<'a, $O> ),*
        {
            fn parse(&mut self, input: &'a str) -> ParseResult<'a, ($($O),*)> {
                let ($($f),*) = self;
                tuple_gen!($($f),*)
                    .map(|match_gen!($($o),*)| ($($o),*))
                    .parse(input)
            }
        }
    };
}

impl_tuple!(
    F1, F2;
    O1, O2;
    f1, f2;
    o1, o2
);
impl_tuple!(
    F1, F2, F3;
    O1, O2, O3;
    f1, f2, f3;
    o1, o2, o3
);
impl_tuple!(
    F1, F2, F3, F4;
    O1, O2, O3, O4;
    f1, f2, f3, f4;
    o1, o2, o3, o4
);
impl_tuple!(
    F1, F2, F3, F4, F5;
    O1, O2, O3, O4, O5;
    f1, f2, f3, f4, f5;
    o1, o2, o3, o4, o5
);
impl_tuple!(
    F1, F2, F3, F4, F5, F6;
    O1, O2, O3, O4, O5, O6;
    f1, f2, f3, f4, f5, f6;
    o1, o2, o3, o4, o5, o6
);
impl_tuple!(
    F1, F2, F3, F4, F5, F6, F7;
    O1, O2, O3, O4, O5, O6, O7;
    f1, f2, f3, f4, f5, f6, f7;
    o1, o2, o3, o4, o5, o6, o7
);
impl_tuple!(
    F1, F2, F3, F4, F5, F6, F7, F8;
    O1, O2, O3, O4, O5, O6, O7, O8;
    f1, f2, f3, f4, f5, f6, f7, f8;
    o1, o2, o3, o4, o5, o6, o7, o8
);
impl_tuple!(
    F1, F2, F3, F4, F5, F6, F7, F8, F9;
    O1, O2, O3, O4, O5, O6, O7, O8, O9;
    f1, f2, f3, f4, f5, f6, f7, f8, f9;
    o1, o2, o3, o4, o5, o6, o7, o8, o9
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::basic::digit;
    use nom::Finish;

    #[test]
    fn comment() {
        let (res, c) = super::comment("/*🦀*/").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(c, "🦀");

        let (res, c) = super::comment("/* vim * vim */").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(c, " vim * vim ");
    }

    #[test]
    fn separator() {
        let (res, _sep) = super::separator("/* comment */").finish().unwrap();
        assert_eq!(res, "");

        let (res, _sep) = super::separator("/* comment1 */ /* comment2 */")
            .finish()
            .unwrap();
        assert_eq!(res, "");

        let (res, _sep) = super::separator(" ").finish().unwrap();
        assert_eq!(res, "");

        assert!(super::separator("").finish().is_err());
    }

    fn tuple_digit(input: &str) -> ParseResult<(char, char)> {
        tuple_((digit, digit)).parse(input)
    }

    #[test]
    fn tuple() {
        let (res, (a, b)) = tuple_digit("1 /* comment */ 2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(a, '1');
        assert_eq!(b, '2');
    }

    #[test]
    fn tuple_trailing_space() {
        // does not match to trailing space
        let (res, (a, b)) = tuple_digit("1 /* comment */ 2 ").finish().unwrap();
        assert_eq!(res, " ");
        assert_eq!(a, '1');
        assert_eq!(b, '2');
    }

    #[test]
    fn tuple_head_space() {
        // does not match to head space
        assert!(tuple_digit(" 1 /* comment */ 2").finish().is_err());
    }

    fn many0_digit(input: &str) -> ParseResult<Vec<char>> {
        many0_(digit).parse(input)
    }

    #[test]
    fn many0() {
        let (res, digits) = many0_digit("1 /* comment */ 2 3").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2', '3']);

        // match to empty
        let (res, digits) = many0_digit("").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &[]);

        let (res, digits) = many1_digit("1").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1']);

        // does not match to trailing space
        let (res, digits) = many0_digit("1 /* comment */ 2 ").finish().unwrap();
        assert_eq!(res, " ");
        assert_eq!(digits, &['1', '2']);

        // does not match to head space
        let (res, digits) = many0_digit(" 1 /* comment */ 2").finish().unwrap();
        assert_eq!(res, " 1 /* comment */ 2"); // match to nothing
        assert_eq!(digits, &[]);
    }

    fn many1_digit(input: &str) -> ParseResult<Vec<char>> {
        many1_(digit).parse(input)
    }

    #[test]
    fn many1() {
        let (res, digits) = many1_digit("1 /* comment */ 2 3").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2', '3']);

        // does not match to empty
        assert!(many1_digit("").finish().is_err());

        let (res, digits) = many1_digit("1").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1']);

        // does not match to trailing space
        let (res, digits) = many1_digit("1 /* comment */ 2 ").finish().unwrap();
        assert_eq!(res, " ");
        assert_eq!(digits, &['1', '2']);

        // does not match to head space
        assert!(many1_digit(" 1 /* comment */ 2").finish().is_err());
    }

    #[test]
    fn ignorable() {
        let (res, _) = super::ignorable("").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::ignorable(" ").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::ignorable("  ").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::ignorable("/* comment */").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::ignorable("/* comment */ ").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::ignorable(" /* comment */ ").finish().unwrap();
        assert_eq!(res, "");
    }

    fn comma_digit(input: &str) -> ParseResult<Vec<char>> {
        comma_separated(digit).parse(input)
    }

    #[test]
    fn comma() {
        let (res, digits) = comma_digit("1,2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2']);

        let (res, digits) = comma_digit("1 ,2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2']);

        let (res, digits) = comma_digit("1, 2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2']);

        let (res, digits) = comma_digit("1 , 2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(digits, &['1', '2']);
    }
}
