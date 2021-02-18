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

/// list = `(` \[ [parameter] { `,` [parameter] } \] `)` .
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

/// parameter = [typed_parameter] | [untyped_parameter] | [omitted_parameter] .
pub fn parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// typed_parameter = [keyword] `(` [parameter] `)` .
pub fn typed_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// untyped_parameter = `$` | [integer] | [real] | [string] | [rhs_occurence_name] | [enumeration] | [binary] | [list] .
pub fn untyped_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// omitted_parameter = `*` .
pub fn omitted_parameter(input: &str) -> ParseResult<Parameter> {
    todo!()
}

/// parameter_list = [parameter] { `,` [parameter] } .
pub fn parameter_list(input: &str) -> ParseResult<Vec<Parameter>> {
    todo!()
}
