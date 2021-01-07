use super::remark::*;
use nom::{character::complete::*, error::Error, multi::*, sequence::*, IResult, Parser};

type ParseResult<'a, Output> = IResult<&'a str, (Output, Vec<Remark>), Error<&'a str>>;

fn collect_pairs<O>(iter: impl IntoIterator<Item = (Vec<Remark>, O)>) -> (Vec<O>, Vec<Remark>) {
    let mut outputs = Vec::new();
    let mut remarks = Vec::new();
    for (mut r, out) in iter {
        outputs.push(out);
        remarks.append(&mut r);
    }
    (outputs, remarks)
}

pub fn spaced_many0<'a, O, F>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, Vec<O>>
where
    F: Parser<&'a str, O, Error<&'a str>> + Clone,
{
    move |input| {
        many0(pair(spaces_or_remarks, f.clone()))
            .map(|pairs| collect_pairs(pairs))
            .parse(input)
    }
}

pub fn space_separated<'a, O, F>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, Vec<O>>
where
    F: Parser<&'a str, O, Error<&'a str>> + Clone,
{
    move |input| {
        many1(pair(spaces_or_remarks, f.clone()))
            .map(|pairs| collect_pairs(pairs))
            .parse(input)
    }
}

pub fn comma_separated<'a, O, F>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, Vec<O>>
where
    F: Parser<&'a str, O, Error<&'a str>> + Clone,
{
    move |input| {
        let comma_with_remark =
            tuple((spaces_or_remarks, char(','), spaces_or_remarks)).map(|(mut l, _, mut r)| {
                l.append(&mut r);
                l
            });
        tuple((f.clone(), many0(pair(comma_with_remark, f.clone()))))
            .map(|(first, pairs)| {
                let mut output = vec![first];
                let mut remarks = Vec::new();
                for (mut r, out) in pairs {
                    remarks.append(&mut r);
                    output.push(out);
                }
                (output, remarks)
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::basis;
    use nom::Finish;

    #[test]
    fn comma_separated() {
        let (res, (values, remarks)) = super::comma_separated(basis::digit)("1, (* 2, *) 3, 4")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(values, ['1', '3', '4']);
        assert_eq!(remarks.len(), 1);
        assert_eq!(remarks[0].remark, "2,");
    }
}
