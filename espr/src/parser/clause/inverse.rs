use super::super::{entity::*, identifier::*, types::*, util::*};

#[derive(Debug, Clone, PartialEq)]
pub struct InverseClause {
    pub attributes: Vec<InverseAttribute>,
}

/// 249 inverse_clause = INVERSE [inverse_attr] { [inverse_attr] } .
pub fn inverse_clause(input: &str) -> ParseResult<InverseClause> {
    tuple((tag("INVERSE"), space_separated(inverse_attr)))
        .map(|(_inverse, attributes)| InverseClause { attributes })
        .parse(input)
}

/// Attribute of an inverse clause parsed by [inverse_attr]
///
/// From ISO 10303-11 document,
///
/// ```text
/// ENTITY door;
///   handle : knob;                -- inverse relationship for this attribute
///   hinges : SET [1:?] OF hinge;
/// END_ENTITY;
///
/// ENTITY knob;
/// ...
/// INVERSE
///   opens : door FOR handle;
/// (* ^      ^        ^
///    |      |        attribute name used in the parent entity
///    |      The entity which has `SELF` as attribute
///    name of this inverse relationship *)
/// END_ENTITY;
/// ```
///
/// This means
///
/// > knobs can only exist if they are used in the role of handle in one instance of a door
///
#[derive(Debug, Clone, PartialEq)]
pub struct InverseAttribute {
    /// Name of this inverse relationship
    ///
    /// `opens` in above example
    name: String,

    /// The entity name which has `SELF` as an attribute
    ///
    /// `door` in above example
    dest: String,

    /// Used if `SET` or `BAG` for parent entity specification
    dest_aggregation: AggregationOption,

    /// The attribute name used in the parent entity
    ///
    /// `handle` in above example
    attribute: String,

    /// Prefix of the attribute, used if the attribute is a sub-attribute of `dest` entity
    attribute_prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggregationOption {
    Set { bound: Option<Bound> },
    Bag { bound: Option<Bound> },
    None,
}

/// 248 inverse_attr = [attribute_decl] `:`
///                  \[ ( SET | BAG ) \[ [bound_spec] \] OF \]
///                  [entity_ref] FOR
///                  \[ [entity_ref] `.` \]
///                  [attribute_ref] `;` .
pub fn inverse_attr(input: &str) -> ParseResult<InverseAttribute> {
    let aggregation_option = opt(tuple((
        alt((tag("SET"), tag("BAG"))),
        opt(bound_spec),
        tag("OF"),
    )))
    .map(|opt| {
        if let Some((agg, bound, _of)) = opt {
            match agg {
                "SET" => AggregationOption::Set { bound },
                "BAG" => AggregationOption::Bag { bound },
                _ => unreachable!(),
            }
        } else {
            AggregationOption::None
        }
    });

    let attribute_prefix =
        opt(tuple((entity_ref, char('.')))).map(|opt| opt.map(|(prefix, _dot)| prefix));

    tuple((
        attribute_decl,
        char(':'),
        aggregation_option,
        entity_ref,
        tag("FOR"),
        attribute_prefix,
        attribute_ref,
        char(';'),
    ))
    .map(
        |(name, _comma, dest_aggregation, dest, _for, attribute_prefix, attribute, _semicoron)| {
            InverseAttribute {
                name,
                dest,
                dest_aggregation,
                attribute,
                attribute_prefix,
            }
        },
    )
    .parse(input)
}
