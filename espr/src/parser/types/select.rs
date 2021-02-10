use super::{
    super::{combinator::*, types::named_types},
    *,
};

/// Output of [select_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectType {
    pub extensiblity: Extensiblity,
    pub types: Vec<String>,
}

/// 301 select_list = `(` [named_types] { `,` [named_types] } `)` .
pub fn select_list(input: &str) -> ParseResult<Vec<String>> {
    tuple((char('('), comma_separated(named_types), char(')')))
        .map(|(_start, ids, _end)| ids)
        .parse(input)
}

/// 300 select_extension = BASED_ON [type_ref] \[ WITH [select_list] \] .
pub fn select_extension(input: &str) -> ParseResult<(String, Vec<String>)> {
    let with = tuple((tag("WITH"), select_list)).map(|(_with, list)| list);
    tuple((tag("BASED_ON"), type_ref, opt(with)))
        .map(|(_based_on, id, opt)| (id, opt.unwrap_or(Vec::new())))
        .parse(input)
}

/// 302 select_type = \[ EXTENSIBLE \[ GENERIC_ENTITY \] \] SELECT \[ [select_list] | [select_extension] \] .
pub fn select_type(input: &str) -> ParseResult<SelectType> {
    // FIXME support select_extension

    // `GENERIC_ENTITY` only appears in `select_type` declaration.
    let extensiblity = tuple((
        tag("EXTENSIBLE"),
        opt(tuple((spaces, tag("GENERIC_ENTITY")))),
    ))
    .map(|(_extensible, opt)| {
        if opt.is_some() {
            Extensiblity::GenericEntity
        } else {
            Extensiblity::Extensible
        }
    });

    tuple((
        opt(tuple((extensiblity, spaces))),
        tag("SELECT"),
        select_list,
    ))
    .map(|(opt, _select, types)| {
        if let Some((extensiblity, _spaces)) = opt {
            SelectType {
                extensiblity,
                types,
            }
        } else {
            SelectType {
                extensiblity: Extensiblity::None,
                types,
            }
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn select() {
        let (res, (s, _remarks)) = super::select_type("SELECT (a, b)").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(s.extensiblity, Extensiblity::None);
        assert_eq!(s.types[0], "a");
        assert_eq!(s.types[1], "b");
    }
}
