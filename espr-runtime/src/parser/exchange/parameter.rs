use crate::parser::combinator::*;

#[derive(Debug, Clone, PartialEq)]
pub enum UntypedParameter {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    List(Vec<Parameter>),
    // FIXME Add Binary, $
}

/// LIST = `(` \[ PARAMETER { `,` PARAMETER } \] `)` .
pub fn list(input: &str) -> ParseResult<UntypedParameter> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Typed {
        name: String,
        ty: Box<Parameter>,
    },
    Untyped(UntypedParameter),
    /// `*`
    Omitted,
}

/// PARAMETER = TYPED_PARAMETER  | UNTYPED_PARAMETER | OMITTED_PARAMETER  .
pub fn parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// TYPED_PARAMETER = KEYWORD `(` PARAMETER `)` .
pub fn typed_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// UNTYPED_PARAMETER = `$` | INTEGER | REAL | STRING | RHS_OCCURENCE_NAME | ENUMERATION | BINARY | LIST .
pub fn untyped_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// OMITTED_PARAMETER = `*` .
pub fn omitted_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// PARAMETER_LIST = PARAMETER { `,` PARAMETER } .
pub fn parameter_list(input: &str) -> ParseResult<Vec<Parameter>> {
    todo!()
}
