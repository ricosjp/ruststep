use nom::{character::complete::*, error::Error, multi::*, sequence::*, IResult, Parser};

pub fn spaced_many0<'a, O, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, Error<&'a str>>
where
    F: Parser<&'a str, O, Error<&'a str>>,
{
    separated_list0(multispace0, f)
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
