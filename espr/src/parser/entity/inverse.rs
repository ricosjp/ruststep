use super::attribute::*;
use crate::{
    ast::entity::*,
    parser::{combinator::*, identifier::*, types::*},
};

/// 249 inverse_clause = INVERSE [inverse_attr] { [inverse_attr] } .
pub fn inverse_clause(input: &str) -> ParseResult<InverseClause> {
    tuple((tag("INVERSE"), many1(inverse_attr)))
        .map(|(_inverse, attributes)| InverseClause { attributes })
        .parse(input)
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
        |(name, _comma, dest_aggregation, dest, _for, attribute_prefix, attribute, _semicolon)| {
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

#[cfg(test)]
mod tests {
    use super::AggregationOption;
    use crate::ast::{expression::*, types::*};
    use nom::Finish;

    #[test]
    fn inverse() {
        let (residual, (inv, _remarks)) = super::inverse_attr("opens : door FOR handle;")
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(inv.name, "opens");
        assert_eq!(inv.dest, "door");
        assert_eq!(inv.attribute, "handle");
        assert_eq!(inv.dest_aggregation, AggregationOption::None);
        assert_eq!(inv.attribute_prefix, None);
    }

    #[test]
    fn inverse_agg() {
        let (residual, (inv, _remarks)) =
            super::inverse_attr("opens : SET [0:1] OF door FOR handle;")
                .finish()
                .unwrap();
        assert_eq!(residual, "");
        assert_eq!(inv.name, "opens");
        assert_eq!(inv.dest, "door");
        assert_eq!(inv.attribute, "handle");
        assert_eq!(
            inv.dest_aggregation,
            AggregationOption::Set {
                bound: Some(Bound {
                    upper: Expression::real(1.0),
                    lower: Expression::real(0.0)
                })
            }
        );
        assert_eq!(inv.attribute_prefix, None);
    }
}
