use crate::parser::combinator::*;

/// ANCHOR_SECTION = `ANCHOR;` ANCHOR_LIST `ENDSEC;` .
pub fn anchor_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// ANCHOR_LIST = { ANCHOR } .
pub fn anchor_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// ANCHOR = ANCHOR_NAME `=` ANCHOR_ITEM { ANCHOR_TAG } `;` .
pub fn anchor(input: &str) -> ParseResult<()> {
    todo!()
}

/// ANCHOR_ITEM = `$` | INTEGER | REAL | STRING | ENUMERATION | BINARY | RHS_OCCURRENCE_NAME | RESOURCE | ANCHOR_ITEM_LIST .
pub fn anchor_item(input: &str) -> ParseResult<()> {
    todo!()
}

/// ANCHOR_ITEM_LIST = `(` \[ ANCHOR_ITEM { `,` ANCHOR_ITEM } \] `)` .
pub fn anchor_item_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// ANCHOR_TAG = `{` TAG_NAME `:` ANCHOR_ITEM `}` .
pub fn anchor_tag(input: &str) -> ParseResult<()> {
    todo!()
}
