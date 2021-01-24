use super::remark::*;
use nom::{error::VerboseError, multi::*, sequence::*, IResult};
use std::marker::PhantomData;

/// Parse result without remarks
pub type RawParseResult<'a, RawOutput> = IResult<&'a str, RawOutput, VerboseError<&'a str>>;
/// Parse result with remarks
pub type ParseResult<'a, Output> = RawParseResult<'a, (Output, Vec<Remark>)>;

pub struct Map<'a, P, O1, O2, F> {
    parser: P,
    f: F,
    phantom: PhantomData<&'a dyn Fn(O1) -> O2>,
}

// not use `#[derive(Close)]` becuase it requires `O1, O2: Clone`
impl<'a, P: Clone, O1, O2, F: Clone> Clone for Map<'a, P, O1, O2, F> {
    fn clone(&self) -> Self {
        Map {
            parser: self.parser.clone(),
            f: self.f.clone(),
            phantom: PhantomData,
        }
    }
}

impl<'a, P, O1, O2, F> nom::Parser<&'a str, (O2, Vec<Remark>), VerboseError<&'a str>>
    for Map<'a, P, O1, O2, F>
where
    P: EsprParser<'a, O1>,
    F: Fn(O1) -> O2,
{
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, O2> {
        let (input, (out, remarks)) = nom::Parser::parse(&mut self.parser, input)?;
        let out = (self.f)(out);
        Ok((input, (out, remarks)))
    }
}

/// Specialized trait of `nom::Parser` to capturing remarks
pub trait EsprParser<'a, Output>:
    nom::Parser<&'a str, (Output, Vec<Remark>), VerboseError<&'a str>> + Clone
{
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, Output> {
        nom::Parser::parse(self, input)
    }

    /// Apply `f` to `Output`, not to remarks
    fn map<F, O2>(self, f: F) -> Map<'a, Self, Output, O2, F>
    where
        F: Fn(Output) -> O2,
    {
        Map {
            parser: self,
            f,
            phantom: PhantomData,
        }
    }
}

impl<'a, Output, T> EsprParser<'a, Output> for T where
    T: nom::Parser<&'a str, (Output, Vec<Remark>), VerboseError<&'a str>> + Clone
{
}

/// Lift up nom parser into [EsprParser] by adding empty remark.
///
/// Be sure that `Vec::new` does not allocates memory until any member will be pushed.
/// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new
pub fn remarked<'a, O, F>(f: F) -> impl EsprParser<'a, O>
where
    F: nom::Parser<&'a str, O, VerboseError<&'a str>> + Clone,
{
    use nom::Parser;
    move |input| f.clone().map(|out| (out, Vec::new())).parse(input)
}

pub fn opt<'a, O, F>(f: F) -> impl EsprParser<'a, Option<O>>
where
    F: EsprParser<'a, O>,
{
    move |input| -> ParseResult<'a, Option<O>> {
        let (input, value) = nom::Parser::parse(&mut nom::combinator::opt(f.clone()), input)?;
        if let Some((value, remarks)) = value {
            Ok((input, (Some(value), remarks)))
        } else {
            Ok((input, (None, Vec::new())))
        }
    }
}

pub fn value<'a, O, V, F>(value: V, mut f: F) -> impl EsprParser<'a, V>
where
    V: Clone,
    F: EsprParser<'a, O>,
{
    move |input| {
        let (input, (_matched, remarks)) = nom::Parser::parse(&mut f, input)?;
        Ok((input, (value.clone(), remarks)))
    }
}

pub fn tag<'a>(tag_str: &'static str) -> impl EsprParser<'a, &'a str> {
    move |input: &'a str| {
        let (input, tag) = nom::bytes::complete::tag(tag_str)(input)?;
        Ok((input, (tag, Vec::new())))
    }
}

pub fn is_not<'a>(pattern: &'static str) -> impl EsprParser<'a, &'a str> {
    move |input: &'a str| {
        let (input, tag) = nom::bytes::complete::is_not(pattern)(input)?;
        Ok((input, (tag, Vec::new())))
    }
}

pub fn char<'a>(c: char) -> impl EsprParser<'a, char> {
    move |input| {
        let (input, c) = nom::character::complete::char(c)(input)?;
        Ok((input, (c, Vec::new())))
    }
}

pub fn spaces(input: &str) -> ParseResult<()> {
    let (input, remarks) = spaces_or_remarks(input)?;
    Ok((input, ((), remarks)))
}

pub fn satisfy<'a, F>(f: F) -> impl EsprParser<'a, char>
where
    F: Fn(char) -> bool + Clone,
{
    use nom::Parser;
    move |input: &'a str| {
        let (input, c) = nom::character::complete::satisfy(f.clone()).parse(input)?;
        Ok((input, (c, Vec::new())))
    }
}

pub fn spaced_many0<'a, O>(f: impl EsprParser<'a, O>) -> impl EsprParser<'a, Vec<O>> {
    use nom::Parser;
    move |input| {
        many0(pair(spaces_or_remarks, f.clone()))
            .map(|pairs| {
                let mut outputs = Vec::new();
                let mut remarks = Vec::new();
                for (mut r1, (out, mut r2)) in pairs {
                    outputs.push(out);
                    remarks.append(&mut r1);
                    remarks.append(&mut r2);
                }
                (outputs, remarks)
            })
            .parse(input)
    }
}

pub fn space_separated<'a, O>(f: impl EsprParser<'a, O>) -> impl EsprParser<'a, Vec<O>> {
    use nom::Parser;
    move |input| {
        many1(pair(spaces_or_remarks, f.clone()))
            .map(|pairs| {
                let mut outputs = Vec::new();
                let mut remarks = Vec::new();
                for (mut r1, (out, mut r2)) in pairs {
                    outputs.push(out);
                    remarks.append(&mut r1);
                    remarks.append(&mut r2);
                }
                (outputs, remarks)
            })
            .parse(input)
    }
}

pub fn comma_separated<'a, O>(f: impl EsprParser<'a, O>) -> impl EsprParser<'a, Vec<O>> {
    use nom::Parser;
    move |input| {
        let comma_with_remark =
            nom::sequence::tuple((spaces_or_remarks, char(','), spaces_or_remarks)).map(
                |(mut l, _, mut r)| {
                    l.append(&mut r);
                    l
                },
            );
        nom::sequence::tuple((f.clone(), many0(pair(comma_with_remark, f.clone()))))
            .map(|((first, mut r0), pairs)| {
                let mut output = vec![first];
                let mut remarks = Vec::new();
                remarks.append(&mut r0);
                for (mut r1, (out, mut r2)) in pairs {
                    remarks.append(&mut r1);
                    remarks.append(&mut r2);
                    output.push(out);
                }
                (output, remarks)
            })
            .parse(input)
    }
}

/// Merge tupled EsprParser into a single EsprParser
pub fn tuple<'a, O, List: Tuple<'a, O>>(mut l: List) -> impl EsprParser<'a, O> {
    move |input| l.parse(input)
}

pub trait Tuple<'a, O>: Clone {
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, O>;
}

macro_rules! impl_tuple {
    ($($F:ident),*; $($O:ident),*; $($f:ident),*; $($o:ident),*) => {

        impl<'a, $($F),*, $($O),*> Tuple<'a, ($($O),*)> for ($($F),*)
        where
            $( $F: EsprParser<'a, $O> ),*
        {
            fn parse(&mut self, input: &'a str) -> ParseResult<'a, ($($O),*)> {
                let mut remarks = Vec::new();

                let ($($f),*) = self;

                $(
                let (input, ($o, mut r)) = nom::Parser::parse($f, input)?;
                remarks.append(&mut r);
                let (input, mut r) = spaces_or_remarks(input)?;
                remarks.append(&mut r);
                )*

                Ok((input, (($($o),*), remarks)))
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

pub fn alt<'a, O, List: Alt<'a, O>>(l: List) -> impl EsprParser<'a, O> {
    move |input| Alt::choice(l.clone(), input)
}

pub trait Alt<'a, O>: Clone {
    fn choice(self, input: &'a str) -> ParseResult<'a, O>;
}

macro_rules! impl_alg {
    ($($F:ident),*) => {
        impl<'a, $($F),*, O> Alt<'a, O> for ($($F),*)
        where
            $( $F: EsprParser<'a, O> ),*
        {
            fn choice(self, input: &'a str) -> ParseResult<'a, O> {
                use nom::Parser;
                nom::branch::alt(self).parse(input)
            }
        }
    };
}

impl_alg!(F1, F2);
impl_alg!(F1, F2, F3);
impl_alg!(F1, F2, F3, F4);
impl_alg!(F1, F2, F3, F4, F5);
impl_alg!(F1, F2, F3, F4, F5, F6);
impl_alg!(F1, F2, F3, F4, F5, F6, F7);
impl_alg!(F1, F2, F3, F4, F5, F6, F7, F8);
impl_alg!(F1, F2, F3, F4, F5, F6, F7, F8, F9);

#[cfg(test)]
mod tests {
    use crate::parser::basis;
    use nom::{Finish, Parser};

    #[test]
    fn comma_separated() {
        let (res, (values, remarks)) = super::comma_separated(super::remarked(basis::digit))
            .parse("1, (* 2, *) 3, 4")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(values, ['1', '3', '4']);
        assert_eq!(remarks.len(), 1);
        assert_eq!(remarks[0].remark, "2,");
    }
}
