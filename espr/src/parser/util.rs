use super::remark::*;
use nom::{character::complete::*, error::Error, multi::*, sequence::*, IResult, Parser};

pub fn spaced_many0<'a, O, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, (Vec<O>, Vec<Remark>), Error<&'a str>>
where
    F: Parser<&'a str, O, Error<&'a str>> + Copy,
{
    move |input| {
        many0(pair(spaces_or_remarks, f))
            .map(|pairs| {
                let mut outputs = Vec::new();
                let mut remarks = Vec::new();
                for (mut remark, out) in pairs {
                    outputs.push(out);
                    remarks.append(&mut remark);
                }
                (outputs, remarks)
            })
            .parse(input)
    }
}

pub fn space_separated<'a, O, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, Error<&'a str>>
where
    F: Parser<&'a str, O, Error<&'a str>>,
{
    separated_list1(multispace1, f)
}

pub fn comma_separated<'a, O, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, Error<&'a str>>
where
    F: Parser<&'a str, O, Error<&'a str>>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}
