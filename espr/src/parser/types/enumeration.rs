use super::{
    super::{combinator::*, identifier::*},
    *,
};

/// 211 enumeration_items = `(` [enumeration_id] { `,` [enumeration_id] } `)` .
pub fn enumeration_items(input: &str) -> ParseResult<Vec<String>> {
    tuple((char('('), comma_separated(enumeration_id), char(')')))
        .map(|(_open, enums, _close)| enums)
        .parse(input)
}

/// 213 enumeration_type = \[ EXTENSIBLE \] ENUMERATION \[ ( OF [enumeration_items] ) | enumeration_extension \] .
pub fn enumeration_type(input: &str) -> ParseResult<EnumerationType> {
    // FIXME enumeration_extension
    tuple((
        opt(tag("EXTENSIBLE")),
        tag("ENUMERATION"),
        tag("OF"),
        enumeration_items,
    ))
    .map(|(extensiblility, _start, _of, items)| EnumerationType {
        extensiblity: if extensiblility.is_some() {
            Extensiblity::Extensible
        } else {
            Extensiblity::None
        },
        items,
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn enumeration_type() {
        let (residual, (e, _remark)) =
            super::enumeration_type("ENUMERATION OF (up, down, left, right)")
                .finish()
                .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            e,
            super::EnumerationType {
                extensiblity: super::Extensiblity::None,
                items: vec![
                    "up".to_string(),
                    "down".to_string(),
                    "left".to_string(),
                    "right".to_string()
                ]
            }
        );
    }

    #[test]
    fn extensible() {
        let (residual, (e, _remark)) =
            super::enumeration_type("EXTENSIBLE ENUMERATION OF (up, down, left, right)")
                .finish()
                .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            e,
            super::EnumerationType {
                extensiblity: super::Extensiblity::Extensible,
                items: vec![
                    "up".to_string(),
                    "down".to_string(),
                    "left".to_string(),
                    "right".to_string()
                ]
            }
        );

        // GENERIC_ENTITY is only supported for SELECT
        assert!(super::enumeration_type(
            "EXTENSIBLE GENERIC_ENTITY ENUMERATION OF (up, down, left, right)"
        )
        .finish()
        .is_err());
    }
}
