use super::{basis::*, remark::*, util::*};
use nom::{bytes::complete::*, character::complete::*, sequence::*, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    type_id: String,
    underlying_type: String,
}

/// 332 underlying_type = concrete_types | constructed_types .
/// 193 concrete_types = aggregation_types | simple_types | type_ref.
/// 198 constructed_types = enumeration_type | select_type .
/// 213 enumeration_type = [ EXTENSIBLE ] ENUMERATION [ ( OF enumeration_items ) | enumeration_extension ] .
/// 302 select_type = [ EXTENSIBLE [ GENERIC_ENTITY ] ] SELECT [ select_list | select_extension ] .
/// 301 select_list = `(` named_types { `,` named_types } `)` .
/// 300 select_extension = BASED_ON type_ref [ WITH select_list ] .
/// 258 named_types = entity_ref | type_ref .
pub fn underlying_type(input: &str) -> ParseResult<String> {
    // FIXME
    remarked(simple_id)(input)
}

/// 327 type_decl = TYPE type_id `=` underlying_type `;` [ where_clause ] END_TYPE `;` .
pub fn type_decl(input: &str) -> ParseResult<Type> {
    tuple((
        tag("TYPE"),
        spaces_or_remarks,
        simple_id, // type_id
        spaces_or_remarks,
        char('='),
        spaces_or_remarks,
        underlying_type,
        spaces_or_remarks,
        char(';'),
        spaces_or_remarks,
        tag("END_TYPE"),
        spaces_or_remarks,
        char(';'),
    ))
    .map(
        |(
            _start,
            mut remarks,
            type_id,
            mut r1,
            _equal,
            mut r2,
            (underlying_type, mut r3),
            mut r4,
            _semicoron1,
            mut r5,
            _end,
            mut r6,
            _semicoron2,
        )| {
            remarks.append(&mut r1);
            remarks.append(&mut r2);
            remarks.append(&mut r3);
            remarks.append(&mut r4);
            remarks.append(&mut r5);
            remarks.append(&mut r6);
            (
                Type {
                    type_id,
                    underlying_type,
                },
                remarks,
            )
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn type_decl() {
        let (residual, (ty, _remark)) = super::type_decl("TYPE my_type = STRING; END_TYPE;")
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            ty,
            super::Type {
                type_id: "my_type".to_string(),
                underlying_type: "STRING".to_string()
            }
        );
    }
}
