use super::remark::*;
use nom::{character::complete::*, error::Error, multi::*, sequence::*, IResult, Parser};

pub type ParseResult<'a, Output> = IResult<&'a str, (Output, Vec<Remark>), Error<&'a str>>;

/// Specialized trait of `nom::Parser` to capturing remarks
pub trait EsprParser<'a, Output>: FnMut(&'a str) -> ParseResult<'a, Output> + Clone {}
impl<'a, Output, T: FnMut(&'a str) -> ParseResult<'a, Output> + Clone> EsprParser<'a, Output>
    for T
{
}

/// Lift up nom parser into [EsprParser] by adding empty remark.
pub fn remarked<'a, O, F>(f: F) -> impl EsprParser<'a, O>
where
    F: Parser<&'a str, O, Error<&'a str>> + Clone,
{
    move |input| f.clone().map(|out| (out, Vec::new())).parse(input)
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

pub trait Tuple<'a, O>: Clone {
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, O>;
}

impl<'a, F1, O1> Tuple<'a, (O1,)> for (F1,)
where
    F1: EsprParser<'a, O1>,
{
    fn parse(&mut self, input: &'a str) -> ParseResult<'a, (O1,)> {
        let mut remarks = Vec::new();
        let (input, (o1, mut r)) = self.0.parse(input)?;
        remarks.append(&mut r);
        let (input, mut r) = spaces_or_remarks(input)?;
        remarks.append(&mut r);
        Ok((input, ((o1,), remarks)))
    }
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

pub fn remarked_tuple<'a, O, List: Tuple<'a, O>>(mut l: List) -> impl EsprParser<'a, O> {
    move |input| l.parse(input)
}

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
