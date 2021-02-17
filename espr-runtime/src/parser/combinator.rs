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
    character::complete::{char, multispace1, none_of},
    combinator::{not, peek, value},
    error::VerboseError,
    multi::{many0, many1},
    sequence::tuple,
    IResult, Parser,
};

/// Parse result
pub type ParseResult<'a, X> = IResult<&'a str, X, VerboseError<&'a str>>;

/// Specialized trait of `nom::Parser`
pub trait ExchangeParser<'a, X>: nom::Parser<&'a str, X, VerboseError<&'a str>> + Clone {}

impl<'a, X, T> ExchangeParser<'a, X> for T where
    T: nom::Parser<&'a str, X, VerboseError<&'a str>> + Clone
{
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

pub fn separator(input: &str) -> ParseResult<()> {
    // FIXME support explicit print control directives
    let space = value((), multispace1);
    let comment = value((), comment);
    alt((space, comment)).parse(input)
}

pub fn many0_<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        many0(tuple((separator, f.clone())))
            .map(|seq| seq.into_iter().map(|(_sep, val)| val).collect())
            .parse(input)
    }
}

pub fn many1_<'a, O>(f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        many1(tuple((separator, f.clone())))
            .map(|seq| seq.into_iter().map(|(_sep, val)| val).collect())
            .parse(input)
    }
}

pub fn separated<'a, O>(c: char, f: impl ExchangeParser<'a, O>) -> impl ExchangeParser<'a, Vec<O>> {
    move |input| {
        tuple((
            f.clone(),
            many0(
                tuple((separator, char(c), separator, f.clone()))
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

pub fn tuple_<'a, O, List: Tuple<'a, O>>(mut l: List) -> impl ExchangeParser<'a, O> {
    move |input| l.parse(input)
}

pub trait Tuple<'a, O>: Clone {
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, O>;
}

macro_rules! impl_tuple {
    ($($F:ident),*; $($O:ident),*; $($f:ident),*; $($o:ident),*) => {

        impl<'a, $($F),*, $($O),*> Tuple<'a, ($($O),*)> for ($($F),*)
        where
            $( $F: ExchangeParser<'a, $O> ),*
        {
            fn parse(&mut self, input: &'a str) -> ParseResult<'a, ($($O),*)> {
                let ($($f),*) = self;

                $(
                let (input, _spacer) = separator(input)?;
                let (input, $o) = nom::Parser::parse($f, input)?;
                )*

                Ok((input, ($($o),*,)))
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
    use nom::Finish;

    #[test]
    fn comment_emoji() {
        let (res, c) = super::comment("/*🦀*/").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(c, "🦀");
    }

    #[test]
    fn comment_astr() {
        let (res, c) = super::comment("/* vim * vim */").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(c, " vim * vim ");
    }
}
