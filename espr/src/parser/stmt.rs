use super::{entity::*, expression::*, identifier::*, subsuper::*, types::*, util::*};

/// 309 stmt = [alias_stmt] | [assignment_stmt] | [case_stmt] | [compound_stmt] | [escape_stmt] | [if_stmt] | [null_stmt] | [procedure_call_stmt] | [repeat_stmt] | [return_stmt] | [skip_stmt] .
pub fn stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 174 alias_stmt = ALIAS [variable_id] FOR [general_ref] { [qualifier] } `;` [stmt] { [stmt] } END_ALIAS `;` .
pub fn alias_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 176 assignment_stmt = [general_ref] { [qualifier] } `:=` [expression] `;` .
pub fn assignment_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 191 case_stmt = CASE [selector] OF { [case_action] } \[ OTHERWISE `:` [stmt] \] END_CASE `;` .
pub fn case_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 299 selector = [expression] .
pub fn selector(input: &str) -> ParseResult<Expression> {
    expression(input)
}

/// 189 case_action = [case_label] { `,` [case_label] } `:` [stmt] .
pub fn case_action(input: &str) -> ParseResult<()> {
    todo!()
}

/// 190 case_label = [expression] .
pub fn case_label(input: &str) -> ParseResult<Expression> {
    expression(input)
}

/// 192 compound_stmt = BEGIN [stmt] { [stmt] } END `;` .
pub fn compound_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 214 escape_stmt = ESCAPE `;` .
pub fn escape_stmt(input: &str) -> ParseResult<()> {
    tuple((tag("ESCAPE"), char(';')))
        .map(|(_skip, _semicoron)| ())
        .parse(input)
}

/// 233 if_stmt = IF [logical_expression] THEN [stmt] { [stmt] } \[ ELSE [stmt] { [stmt] } \] END_IF `;` .
pub fn if_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

/// 260 null_stmt = `;` .
pub fn null_stmt(input: &str) -> ParseResult<()> {
    char(';').map(|_c| ()).parse(input)
}

/// 270 procedure_call_stmt = ( [built_in_procedure] | [procedure_ref] ) \[ [actual_parameter_list] \] `;` .
pub fn procedure_call_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInProcedure {
    Insert,
    Remove,
}

/// 188 built_in_procedure = INSERT | REMOVE .
pub fn built_in_procedure(input: &str) -> ParseResult<BuiltInProcedure> {
    alt((
        tag("INSERT").map(|_| BuiltInProcedure::Insert),
        tag("REMOVE").map(|_| BuiltInProcedure::Remove),
    ))
    .parse(input)
}

/// 286 repeat_stmt = REPEAT [repeat_control] `;` [stmt] { [stmt] } END_REPEAT `;` .
pub fn repeat_stmt(input: &str) -> ParseResult<()> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatControl {
    pub increment: Option<RepeatIncrement>,
    pub while_: Option<Expression>,
    pub until: Option<Expression>,
}

/// 285 repeat_control = \[ [increment_control] \] \[ [while_control] \] \[ [until_control] \] .
pub fn repeat_control(input: &str) -> ParseResult<RepeatControl> {
    tuple((
        opt(increment_control),
        opt(while_control),
        opt(until_control),
    ))
    .map(|(increment, while_, until)| RepeatControl {
        increment,
        while_,
        until,
    })
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatIncrement {
    pub variable: String,
    pub begin: Expression,
    pub end: Expression,
    pub increment: Option<Expression>,
}

/// 235 increment_control = [variable_id] `:=` [bound_1] TO [bound_2] \[ BY [increment] \] .
pub fn increment_control(input: &str) -> ParseResult<RepeatIncrement> {
    tuple((
        variable_id,
        tag(":="),
        bound_1,
        tag("TO"),
        bound_2,
        opt(tuple((tag("BY"), increment)).map(|(_by, inc)| inc)),
    ))
    .map(
        |(variable, _def, begin, _to, end, increment)| RepeatIncrement {
            variable,
            begin,
            end,
            increment,
        },
    )
    .parse(input)
}

/// 234 increment = [numeric_expression] .
pub fn increment(input: &str) -> ParseResult<Expression> {
    numeric_expression(input)
}

/// 339 while_control = WHILE [logical_expression] .
pub fn while_control(input: &str) -> ParseResult<Expression> {
    tuple((tag("WHILE"), logical_expression))
        .map(|(_where, expr)| expr)
        .parse(input)
}

/// 335 until_control = UNTIL [logical_expression] .
pub fn until_control(input: &str) -> ParseResult<Expression> {
    tuple((tag("UNTIL"), logical_expression))
        .map(|(_where, expr)| expr)
        .parse(input)
}

/// 290 return_stmt = RETURN \[ `(` [expression] `)` \] `;` .
pub fn return_stmt(input: &str) -> ParseResult<Option<Expression>> {
    tuple((
        tag("RETURN"),
        opt(tuple((char('('), expression, char(')'))).map(|(_open, expr, _close)| expr)),
        char(';'),
    ))
    .map(|(_return, expr, _semicoron)| expr)
    .parse(input)
}

/// 308 skip_stmt = SKIP `;` .
pub fn skip_stmt(input: &str) -> ParseResult<()> {
    tuple((tag("SKIP"), char(';')))
        .map(|(_skip, _semicoron)| ())
        .parse(input)
}
