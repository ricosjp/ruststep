use super::{basis::simple_id, combinator::RawParseResult};
use crate::ast::Remark;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::*, character::complete::*, combinator::opt, multi::*,
    sequence::*, Parser,
};

fn begin(input: &str) -> RawParseResult<()> {
    tag("(*").map(|_| ()).parse(input)
}

fn end(input: &str) -> RawParseResult<String> {
    tuple((many1(char('*')), char(')')))
        .map(|(stars, lparen)| {
            format!("{}{}", stars.iter().collect::<String>(), lparen)
                .trim_end_matches("*)")
                .to_string()
        })
        .parse(input)
}

fn middle_star(input: &str) -> RawParseResult<String> {
    tuple((many1(char('*')), none_of("*)")))
        .map(|(stars, c)| format!("{}{}", stars.iter().collect::<String>(), c))
        .parse(input)
}

/// Quoted string like `\`*)\``
fn quoted(input: &str) -> RawParseResult<String> {
    tuple((char('`'), many0(none_of("`")), char('`')))
        .map(|(_quote_start, chars, _quote_end)| format!("`{}`", chars.iter().collect::<String>()))
        .parse(input)
}

/// String which does not include `*` and \`
fn non_quoted(input: &str) -> RawParseResult<String> {
    many1(none_of("`*"))
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
}

/// 999 embedded_remark
///
/// Extended the original definition
///
/// ```text
/// 145 embedded_remark = `(*` [ remark_tag ] { ( not_paren_star { not_paren_star } )
///                                           | lparen_then_not_lparen_star
///                                           | ( `*` { `*` } )
///                                           | not_rparen_star_then_rparen
///                                           | embedded_remark
///                                           } `*)` .
/// ```
///
/// because it cannot parse the example in ISO 10303-11 correctly:
///
/// ```text
/// (* The `(*` symbol starts a remark, and the `*)` symbol ends it *)
/// ```
///
pub fn embedded_remark(input: &str) -> RawParseResult<Remark> {
    tuple((
        begin,
        multispace0,
        opt(remark_tag),
        multispace0,
        many0(alt((non_quoted, quoted, middle_star))),
        end,
    ))
    .map(|(_begin, _sp1, tag, _sp2, chars, end)| Remark {
        tag,
        remark: format!("{}{}", chars.iter().join(""), end)
            .trim()
            .to_string(),
    })
    .parse(input)
}

/// 999 tail_remark
///
/// Extended the original definition
///
/// ```text
/// 149 tail_remark = `--` [ remark_tag ] { \a | \s | \x9 | \xA | \xD } \n .
/// ```
///
/// to support `\r\n` case and unicode string
pub fn tail_remark(input: &str) -> RawParseResult<Remark> {
    tuple((
        tag("--"),
        multispace0,
        opt(remark_tag),
        not_line_ending,
        line_ending,
    ))
    .map(
        |(_start, _sp, tag, chars, _newline): (_, _, _, &str, _)| Remark {
            tag,
            remark: chars.trim().to_string(),
        },
    )
    .parse(input)
}

/// 147 remark_tag = `"` remark_ref { `.` remark_ref } `"` .
///
/// `remark_ref` is replaced by `simple_id` because it should be handled by following ir
/// analysis phase.
pub fn remark_tag(input: &str) -> RawParseResult<Vec<String>> {
    delimited(char('"'), separated_list1(char('.'), simple_id), char('"')).parse(input)
}

/// Match to spaces or remarks
pub fn spaces_or_remarks(input: &str) -> RawParseResult<Vec<Remark>> {
    tuple((
        many0(tuple((multispace0, alt((embedded_remark, tail_remark))))),
        multispace0,
    ))
    .map(|(contains, _endspace)| {
        contains
            .into_iter()
            .map(|(_space, remark)| remark)
            .collect()
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn begin() {
        let (res, _) = super::begin("(*").finish().unwrap();
        assert_eq!(res, "");

        let (res, _) = super::begin("(**").finish().unwrap();
        assert_eq!(res, "*");
    }

    #[test]
    fn end() {
        let (res, stars) = super::end("***)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars, "**");
    }

    #[test]
    fn middle_star() {
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
        assert_eq!(stars.remark, "`*)` should be ignored");
    }

    #[test]
    fn simple() {
        let (res, stars) = super::embedded_remark("(* aaa *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars.remark, "aaa");
    }

    #[test]
    fn stars() {
        let (res, stars) = super::embedded_remark("(*****)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars.remark, "***");

        let (res, stars) = super::embedded_remark("(* *** *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars.remark, "***");

        let (res, stars) = super::embedded_remark("(* ****)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars.remark, "***");
    }

    #[test]
    fn middle_stars() {
        let (res, stars) = super::embedded_remark("(* a * b *)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(stars.remark, "a * b");
    }

    #[test]
    fn tag() {
        let (res, remark) = super::embedded_remark("(* \"some.tag\" a * b *)")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(
            remark.tag,
            Some(vec!["some".to_string(), "tag".to_string()])
        );
        assert_eq!(remark.remark, "a * b");
    }

    #[test]
    fn tail_remark() {
        let (res, remark) = super::tail_remark("-- aaa\nbbb").finish().unwrap();
        assert_eq!(res, "bbb");
        assert_eq!(remark.tag, None);
        assert_eq!(remark.remark, "aaa");

        let (res, remark) = super::tail_remark("-- \"some.tag\" aaa\nbbb")
            .finish()
            .unwrap();
        assert_eq!(res, "bbb");
        assert_eq!(
            remark.tag,
            Some(vec!["some".to_string(), "tag".to_string()])
        );
        assert_eq!(remark.remark, "aaa");
    }

    #[test]
    fn remark_tag() {
        let (res, tag) = super::remark_tag(r#""some.name.space""#).finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            tag,
            vec!["some".to_string(), "name".to_string(), "space".to_string()]
        );
    }

    #[test]
    fn spaces_or_remarks() {
        let (res, remarks) = super::spaces_or_remarks("").finish().unwrap();
        assert_eq!(res, "");
        assert!(remarks.is_empty());

        let (res, remarks) = super::spaces_or_remarks(
            r#"
            -- some comment
            (* embedded comment *)
            "#,
        )
        .finish()
        .unwrap();
        assert_eq!(res, "");
        dbg!(&remarks);
        assert_eq!(remarks.len(), 2);
    }
}
