use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

#[derive(Debug, Clone)]
pub struct Reamrk {
    tag: Option<Vec<String>>,
    remark: String,
}

fn begin(input: &str) -> IResult<&str, ()> {
    tag("(*").map(|_| ()).parse(input)
}

fn end(input: &str) -> IResult<&str, String> {
    tuple((many1(char('*')), char(')')))
        .map(|(stars, lparen)| {
            format!("{}{}", stars.iter().collect::<String>(), lparen)
                .trim_end_matches("*)")
                .to_string()
        })
        .parse(input)
}

fn middle_star(input: &str) -> IResult<&str, String> {
    tuple((many1(char('*')), none_of("*)")))
        .map(|(stars, c)| format!("{}{}", stars.iter().collect::<String>(), c))
        .parse(input)
}

/// Quoted string like `\`*)\``
fn quoted(input: &str) -> IResult<&str, String> {
    tuple((char('`'), many0(none_of("`")), char('`')))
        .map(|(_quote_start, chars, _quote_end)| format!("`{}`", chars.iter().collect::<String>()))
        .parse(input)
}

/// String which does not include `*` and \`
fn non_quoted(input: &str) -> IResult<&str, String> {
    many1(none_of("`*"))
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
}

pub fn embedded_remark(input: &str) -> IResult<&str, String> {
    tuple((
        begin,
        multispace0,
        many0(alt((non_quoted, quoted, middle_star))),
        end,
    ))
    .map(|(_begin, _sp1, chars, end)| format!("{}{}", chars.iter().join(""), end))
    .map(|s| s.trim().to_string())
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn quoted() {
        let (res, quoted) = super::quoted("`*)`").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(quoted, "`*)`");

        let (res, quoted) = super::quoted("` a `").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(quoted, "` a `");
    }

    #[test]
    fn quoted_end() {
        let (res, stars) = super::embedded_remark("(* `*)` should be ignored *)")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "`*)` should be ignored");
    }

    #[test]
    fn embedded_remark_begin() {
        let (res, _) = super::begin("(*").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::begin("(**").finish().unwrap();
        assert_eq!(res, "*");
    }

    #[test]
    fn embedded_remark_end() {
        let (res, stars) = super::end("***)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "**");
    }

    #[test]
    fn embedded_remark_middle_star() {
        let (res, stars) = super::middle_star("*b").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "*b");

        let (res, stars) = super::middle_star("* ").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "* ");

        assert!(super::middle_star("**").finish().is_err());
        assert!(super::middle_star("*)").finish().is_err());
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

        let (res, stars) = super::embedded_remark("(* *** *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "***");

        let (res, stars) = super::embedded_remark("(* ****)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "***");
    }

    #[test]
    fn embedded_remark_middle_stars() {
        let (res, stars) = super::embedded_remark("(* a * b *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "a * b");
    }
}
