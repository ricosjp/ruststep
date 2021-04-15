//! Parser for exchange structure

mod anchor;
mod data;
mod header;
mod parameter;
mod reference;

pub use anchor::*;
pub use data::*;
pub use header::*;
pub use parameter::*;
pub use reference::*;

use crate::{
    ast::*,
    parser::{combinator::*, token::*},
};
use nom::Parser;

/// exchange_file = `ISO-10303-21;`
///                 [header_section]
///              \[ [anchor_section] \]
///              \[ [reference_section] \]
///               { [data_section] }
///                 `END-ISO-10303-21;`
///               { signature_section } .
pub fn exchange_file(input: &str) -> ParseResult<Exchange> {
    tuple_((
        tag_("ISO-10303-21;"),
        header_section,
        opt_(anchor_section),
        opt_(reference_section),
        many0_(data_section),
        tag_("END-ISO-10303-21;"),
        many0_(signature_section),
    ))
    .map(
        |(_start, header, anchor, reference, data, _end, signature)| Exchange {
            header,
            anchor: anchor.unwrap_or_default(),
            reference: reference.unwrap_or_default(),
            data,
            signature,
        },
    )
    .parse(input)
}

/// signature_section  = `SIGNATURE` signature_content `ENDSEC;`.
pub fn signature_section(input: &str) -> ParseResult<String> {
    tuple_((tag_("SIGNATURE"), signature_content, tag_("ENDSEC;")))
        .map(|(_start, sig, _end)| sig)
        .parse(input)
}
