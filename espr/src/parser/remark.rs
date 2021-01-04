use nom::{
    branch::alt, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult, Parser,
};

#[derive(Debug, Clone)]
pub struct Reamrk {
    tag: Option<Vec<String>>,
    remark: String,
}

pub fn embedded_remark_begin(input: &str) -> IResult<&str, ()> {
    tag("(*").map(|_| ()).parse(input)
}

pub fn embedded_remark_end(input: &str) -> IResult<&str, String> {
    tuple((many1(char('*')), char(')')))
        .map(|(stars, lparen)| {
            format!("{}{}", stars.iter().collect::<String>(), lparen)
                .trim_end_matches("*)")
                .to_string()
        })
        .parse(input)
}

pub fn embedded_remark(input: &str) -> IResult<&str, String> {
    tuple((embedded_remark_begin, embedded_remark_end))
        .map(|(_begin, end)| end)
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn embedded_remark_begin() {
        let (res, _) = super::embedded_remark_begin("(*").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::embedded_remark_begin("(**").finish().unwrap();
        assert_eq!(res, "*");
    }

    #[test]
    fn embedded_remark_end() {
        let (res, stars) = super::embedded_remark_end("***)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "**");
    }

    #[test]
    fn embedded_remark() {
        let (res, stars) = super::embedded_remark("(*****)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "***");
    }
}
