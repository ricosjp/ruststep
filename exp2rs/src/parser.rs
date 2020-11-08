//! Syntatic analysis of EXPRESS language
//!
//! EXPRESS language is standardized as [ISO-10303-11][ISO-10303-11].
//!
//! [ISO-10303-11]: https://www.iso.org/standard/38047.html

use nom::{bytes::complete::tag, IResult};

/// ```text
/// ENTITY first;
/// m_ref : second;
/// fattr : STRING;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq)]
struct Entity {
    /// Name of this entity type
    name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    attributes: Vec<(String, String)>,
}

fn entity(input: &str) -> IResult<&str, Entity> {
    tag("ENTITY")(input).map(|(input, _)| {
        (
            input,
            Entity {
                name: "".to_string(),
                attributes: Vec::new(),
            },
        )
    })
}

#[derive(Debug, Clone, PartialEq)]
struct Schema {
    name: String,
    entities: Vec<Entity>,
}
