use crate::parser::combinator::*;

/// anchor_section = `ANCHOR;` [anchor_list] `ENDSEC;` .
pub fn anchor_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor_list = { [anchor] } .
pub fn anchor_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor = [anchor_name] `=` [anchor_item] { [anchor_tag] } `;` .
pub fn anchor(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor_item = `$` | [integer] | [real] | [string] | [enumeration] | [binary] | [rhs_occurrence_name] | [resource] | [anchor_item_list] .
pub fn anchor_item(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor_item_list = `(` \[ [anchor_item] { `,` [anchor_item] } \] `)` .
pub fn anchor_item_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor_tag = `{` [tag_name] `:` [anchor_item] `}` .
pub fn anchor_tag(input: &str) -> ParseResult<()> {
    todo!()
}
