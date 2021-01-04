use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser};

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
    tuple((
        embedded_remark_begin,
        multispace0,
        many0(none_of("*")),
        embedded_remark_end,
    ))
    .map(|(_begin, _sp1, chars, end)| format!("{}{}", chars.iter().collect::<String>(), end))
    .map(|s| s.trim().to_string())
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
    fn embedded_remark_simple() {
        let (res, stars) = super::embedded_remark("(* aaa *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "aaa");
    }

    #[test]
    fn embedded_remark_stars() {
        let (res, stars) = super::embedded_remark("(*****)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "***");
    }
}
