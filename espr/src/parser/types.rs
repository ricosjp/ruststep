use super::{basis::*, util::*};

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
    tuple((char('('), comma_separated(remarked(simple_id)), char(')')))
        .map(|(_start, ids, _end)| ids)
        .parse(input)
}

/// 300 select_extension = BASED_ON type_ref [ WITH select_list ] .
pub fn select_extension(input: &str) -> ParseResult<(String, Vec<String>)> {
    let with = tuple((tag("WITH"), select_list)).map(|(_with, list)| list);
    tuple((tag("BASED_ON"), remarked(simple_id), opt(with)))
        .map(|(_based_on, id, opt)| (id, opt.unwrap_or(Vec::new())))
        .parse(input)
}

/// 302 select_type = [ EXTENSIBLE [ GENERIC_ENTITY ] ] SELECT [ select_list | select_extension ] .
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    extensiblity: Extensiblity,
    items: Vec<String>,
}

/// 211 enumeration_items = `(` enumeration_id { `,` enumeration_id } `)` .
pub fn enumeration_items(input: &str) -> ParseResult<Vec<String>> {
    tuple((char('('), comma_separated(remarked(simple_id)), char(')')))
        .map(|(_open, enums, _close)| enums)
        .parse(input)
}

/// 213 enumeration_type = [ EXTENSIBLE ] ENUMERATION [ ( OF enumeration_items ) | enumeration_extension ] .
pub fn enumeration_type(input: &str) -> ParseResult<Enumeration> {
    // FIXME enumeration_extension
    tuple((
        opt(tag("EXTENSIBLE")),
        tag("ENUMERATION"),
        tag("OF"),
        enumeration_items,
    ))
    .map(|(extensiblility, _start, _of, items)| Enumeration {
        extensiblity: if extensiblility.is_some() {
            Extensiblity::Extensible
        } else {
            Extensiblity::None
        },
        items,
    })
    .parse(input)
}

/// 332 underlying_type = concrete_types | constructed_types .
/// 193 concrete_types = aggregation_types | simple_types | type_ref.
/// 198 constructed_types = enumeration_type | select_type .
pub fn underlying_type(input: &str) -> ParseResult<String> {
    // FIXME
    remarked(simple_id).parse(input)
}

/// 327 type_decl = TYPE type_id `=` underlying_type `;` [ where_clause ] END_TYPE `;` .
pub fn type_decl(input: &str) -> ParseResult<Type> {
    tuple((
        tag("TYPE"),
        remarked(simple_id), // type_id
        char('='),
        underlying_type,
        char(';'),
        tag("END_TYPE"),
        char(';'),
    ))
    .map(
        |(_start, type_id, _equal, underlying_type, _semicoron1, _end, _semicoron2)| Type {
            type_id,
            underlying_type,
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
