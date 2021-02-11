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
    character::complete::{multispace1, none_of},
    combinator::value,
    error::VerboseError,
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};
use std::marker::PhantomData;

/// Parse result
pub type ParseResult<'a, X> = IResult<&'a str, X, VerboseError<&'a str>>;

/// Specialized trait of `nom::Parser`
pub trait ExchangeParser<'a, X>: nom::Parser<&'a str, X, VerboseError<&'a str>> + Clone {
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, X> {
        nom::Parser::parse(self, input)
    }

    /// Apply `f` to `X`, not to remarks
    fn map<F: Fn(X) -> Y, Y>(self, f: F) -> Map<'a, Self, X, Y, F> {
        Map {
            parser: self,
            f,
            phantom: PhantomData,
        }
    }
}

impl<'a, X, T> ExchangeParser<'a, X> for T where
    T: nom::Parser<&'a str, X, VerboseError<&'a str>> + Clone
{
}

pub struct Map<'a, P, X, Y, F> {
    parser: P,
    f: F,
    phantom: PhantomData<&'a dyn Fn(X) -> Y>,
}

impl<'a, P: Clone, X, Y, F: Clone> Clone for Map<'a, P, X, Y, F> {
    fn clone(&self) -> Self {
        Map {
            parser: self.parser.clone(),
            f: self.f.clone(),
            phantom: PhantomData,
        }
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
pub fn comment(input: &str) -> ParseResult<()> {
    value((), tuple((tag("/*"), many0(none_of("/*")), tag("*/")))).parse(input)
}

pub fn separator(input: &str) -> ParseResult<()> {
    // FIXME support explicit print control directives
    let space = value((), multispace1);
    alt((space, comment)).parse(input)
}
