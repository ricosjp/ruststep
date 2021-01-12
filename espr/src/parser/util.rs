use super::remark::*;
use nom::{
    bytes::complete::*, character::complete::*, error::Error, multi::*, sequence::*, IResult,
    Parser,
};

pub use nom::branch::alt;

pub type ParseResult<'a, Output> = IResult<&'a str, (Output, Vec<Remark>), Error<&'a str>>;

/// Specialized trait of `nom::Parser` to capturing remarks
pub trait EsprParser<'a, Output>:
    Parser<&'a str, (Output, Vec<Remark>), Error<&'a str>>
    + FnMut(&'a str) -> ParseResult<'a, Output>
    + Clone
{
    fn remarked_map<G, O2>(
        self,
        g: G,
    ) -> nom::Map<
        Self,
        Box<dyn Fn((Output, Vec<Remark>)) -> (O2, Vec<Remark>)>,
        (Output, Vec<Remark>),
    >
    where
        G: Fn(Output) -> O2 + 'static,
    {
        nom::Parser::map(
            self,
            Box::new(move |(output, remarks)| (g(output), remarks)),
        )
    }
}

impl<'a, Output, T: FnMut(&'a str) -> ParseResult<'a, Output> + Clone> EsprParser<'a, Output>
    for T
{
}

/// Lift up nom parser into [EsprParser] by adding empty remark.
///
/// Be sure that `Vec::new` does not allocates memory until any member will be pushed.
/// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new
pub fn remarked<'a, O, F>(f: F) -> impl EsprParser<'a, O>
where
    F: Parser<&'a str, O, Error<&'a str>> + Clone,
{
    move |input| f.clone().map(|out| (out, Vec::new())).parse(input)
}

pub fn remarked_tag<'a>(tag_str: &'static str) -> impl EsprParser<'a, &'a str> {
    move |input: &'a str| {
        let (input, tag) = tag(tag_str)(input)?;
        Ok((input, (tag, Vec::new())))
    }
}

pub fn remarked_char<'a>(c: char) -> impl EsprParser<'a, char> {
    move |input| {
        let (input, c) = char(c)(input)?;
        Ok((input, (c, Vec::new())))
    }
}

pub fn spaced_many0<'a, O>(f: impl EsprParser<'a, O>) -> impl EsprParser<'a, Vec<O>> {
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
    move |input| {
        let comma_with_remark =
            tuple((spaces_or_remarks, char(','), spaces_or_remarks)).map(|(mut l, _, mut r)| {
                l.append(&mut r);
                l
            });
        tuple((f.clone(), many0(pair(comma_with_remark, f.clone()))))
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
pub fn remarked_tuple<'a, O, List: Tuple<'a, O>>(mut l: List) -> impl EsprParser<'a, O> {
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
                let (input, ($o, mut r)) = $f.parse(input)?;
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

#[cfg(test)]
mod tests {
    use crate::parser::basis;
    use nom::Finish;

    #[test]
    fn comma_separated() {
        let (res, (values, remarks)) =
            super::comma_separated(super::remarked(basis::digit))("1, (* 2, *) 3, 4")
                .finish()
                .unwrap();
        assert_eq!(res, "");
        assert_eq!(values, ['1', '3', '4']);
        assert_eq!(remarks.len(), 1);
        assert_eq!(remarks[0].remark, "2,");
    }
}
