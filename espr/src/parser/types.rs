use super::{basis::*, remark::*, util::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    type_id: String,
    underlying_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectType {
    extensiblity: Extensiblity,
    types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensiblity {
    None,
    Extensible,
    GenericEntity,
}

/// 301 select_list = `(` named_types { `,` named_types } `)` .
///
/// `named_types` is replaced by `simple_id` because it will be handled in semantics analysis phase
pub fn select_list(input: &str) -> ParseResult<Vec<String>> {
    tuple((
        char('('),
        spaces_or_remarks,
        comma_separated(remarked(simple_id)),
        spaces_or_remarks,
        char(')'),
    ))
    .map(|(_start, mut remarks, (ids, mut r1), mut r2, _end)| {
        remarks.append(&mut r1);
        remarks.append(&mut r2);
        (ids, remarks)
    })
    .parse(input)
}

/// 300 select_extension = BASED_ON type_ref [ WITH select_list ] .
pub fn select_extension(input: &str) -> ParseResult<(String, Vec<String>)> {
    let with = tuple((tag("WITH"), spaces_or_remarks, select_list)).map(
        |(_with, mut remarks, (list, mut r1))| {
            remarks.append(&mut r1);
            (list, remarks)
        },
    );
    tuple((tag("BASED_ON"), spaces_or_remarks, simple_id, opt(with)))
        .map(|(_based_on, mut remarks, id, opt)| {
            if let Some((list, mut r1)) = opt {
                remarks.append(&mut r1);
                ((id, list), remarks)
            } else {
                ((id, Vec::new()), remarks)
            }
        })
        .parse(input)
}

/// 302 select_type = [ EXTENSIBLE [ GENERIC_ENTITY ] ] SELECT [ select_list | select_extension ] .
pub fn select_type(input: &str) -> ParseResult<SelectType> {
    // FIXME support select_extension
    let extensiblity = tuple((
        tag("EXTENSIBLE"),
        opt(tuple((spaces_or_remarks, tag("GENERIC_ENTITY")))),
    ))
    .map(|(_extensible, opt)| {
        if let Some((remarks, _generic_entity)) = opt {
            (Extensiblity::GenericEntity, remarks)
        } else {
            (Extensiblity::Extensible, Vec::new())
        }
    });
    tuple((
        opt(tuple((extensiblity, spaces_or_remarks))),
        tag("SELECT"),
        spaces_or_remarks,
        select_list,
    ))
    .map(|(opt, _select, mut r2, (types, mut r3))| {
        let ((extensiblity, mut remarks), mut r1) =
            opt.unwrap_or(((Extensiblity::None, Vec::new()), Vec::new()));
        remarks.append(&mut r1);
        remarks.append(&mut r2);
        remarks.append(&mut r3);
        (
            SelectType {
                extensiblity,
                types,
            },
            remarks,
        )
    })
    .parse(input)
}

/// 332 underlying_type = concrete_types | constructed_types .
/// 193 concrete_types = aggregation_types | simple_types | type_ref.
/// 198 constructed_types = enumeration_type | select_type .
/// 213 enumeration_type = [ EXTENSIBLE ] ENUMERATION [ ( OF enumeration_items ) | enumeration_extension ] .
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

/// Parsed result of `width_spec`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

/// 8.1 Simple data types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleType {
    /// 8.1.1 Number data type
    Number,
    /// 8.1.2 Real data type
    Real,
    /// 8.1.3 Integer data type
    Integer,
    /// 8.1.4 Logical data type
    Logical,
    /// 8.1.5 Boolen data type
    Boolen,
    /// 8.1.6 String data type
    String_ { width_spec: Option<WidthSpec> },
    /// 8.1.7 Binary data type
    Binary { width_spec: Option<WidthSpec> },
}

/// 307 simple_types = binary_type | boolean_type | integer_type | logical_type | number_type | real_type | string_type .
pub fn simple_types(input: &str) -> ParseResult<SimpleType> {
    alt((
        number_type,
        real_type,
        integer_type,
        logical_type,
        boolen_type,
        string_type,
        binary_type,
    ))
    .parse(input)
}

/// 261 number_type = NUMBER .
pub fn number_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Number, tag("NUMBER")).parse(input)
}

/// 278 real_type = REAL \[ `(` precision_spec `)` \] .
///
/// 268 precision_spec = numeric_expression .
pub fn real_type(input: &str) -> ParseResult<SimpleType> {
    // FIXME precision_spec is not supported
    value(SimpleType::Real, tag("REAL")).parse(input)
}

/// 241 integer_type = INTEGER .
pub fn integer_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Integer, tag("INTEGER")).parse(input)
}

/// 256 logical_type = LOGICAL .
pub fn logical_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Logical, tag("LOGICAL")).parse(input)
}

/// 182 boolean_type = BOOLEAN .
pub fn boolen_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Boolen, tag("BOOLEN")).parse(input)
}

/// 311 string_type = STRING \[ width_spec \] .
pub fn string_type(input: &str) -> ParseResult<SimpleType> {
    tuple((tag("STRING"), opt(width_spec)))
        .map(|(_, width_spec)| SimpleType::String_ { width_spec })
        .parse(input)
}

/// 181 binary_type = BINARY \[ width_spec \] .
pub fn binary_type(input: &str) -> ParseResult<SimpleType> {
    tuple((tag("BINARY"), opt(width_spec)))
        .map(|(_, width_spec)| SimpleType::Binary { width_spec })
        .parse(input)
}

/// 341 width_spec = `(` width `)` \[ FIXED \] .
pub fn width_spec(input: &str) -> ParseResult<WidthSpec> {
    // FIXME Should use `numeric_expression` parser
    tuple((char('('), is_not(")"), char(')'), opt(tag("FIXED"))))
        .map(|(_lparen, width, _rparen, fixed)| {
            let width = width.parse::<usize>().unwrap(); // FIXME should raise error instead of panic
            WidthSpec {
                width,
                fixed: fixed.is_some(),
            }
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{SimpleType, WidthSpec};
    use nom::Finish;

    #[test]
    fn string() {
        let (res, (string, _remarks)) = super::string_type("STRING").finish().unwrap();
        assert_eq!(string, SimpleType::String_ { width_spec: None });
        assert_eq!(res, "");

        let (res, (string, _remarks)) = super::string_type("STRING (10)").finish().unwrap();
        assert_eq!(
            string,
            SimpleType::String_ {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false,
                })
            }
        );
        assert_eq!(res, "");

        let (res, (string, _remarks)) = super::string_type("STRING (10) FIXED").finish().unwrap();
        assert_eq!(
            string,
            SimpleType::String_ {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: true,
                }),
            }
        );
        assert_eq!(res, "");
    }

    #[test]
    fn binary() {
        let (res, (binary, _remarks)) = super::binary_type("BINARY").finish().unwrap();
        assert_eq!(binary, SimpleType::Binary { width_spec: None });
        assert_eq!(res, "");

        let (res, (binary, _remarks)) = super::binary_type("BINARY (10)").finish().unwrap();
        assert_eq!(
            binary,
            SimpleType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false
                })
            }
        );
        assert_eq!(res, "");

        let (res, (binary, _remarks)) = super::binary_type("BINARY (10) FIXED").finish().unwrap();
        assert_eq!(
            binary,
            SimpleType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: true
                })
            }
        );
        assert_eq!(res, "");
    }

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
