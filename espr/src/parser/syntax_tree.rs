use super::schema::*;
use nom::{character::complete::*, multi::*, sequence::*, Finish, Parser};

/// Entire syntax tree parsed from EXPRESS Language string
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub schemas: Vec<Schema>,
}

impl SyntaxTree {
    pub fn parse(input: &str) -> Result<Self, nom::error::Error<&str>> {
        let (_residual, schemas) = tuple((
            multispace0,
            separated_list1(multispace1, schema),
            multispace0,
        ))
        .map(|(_, schemas, _)| schemas)
        .parse(input)
        .finish()?;
        // FIXME should check residual here
        Ok(Self { schemas })
    }

    // Example syntax tree for easy testing
    //
    // FIXME Replace by e.g. proptest
    // https://github.com/AltSysrq/proptest
    #[allow(dead_code)]
    pub(crate) fn example() -> Self {
        Self::parse(
            r#"
            SCHEMA one;
              ENTITY first;
                m_ref : second;
                fattr : STRING;
              END_ENTITY;
              ENTITY second;
                sattr : STRING;
              END_ENTITY;
            END_SCHEMA;

            SCHEMA geometry0;
              ENTITY point;
                x, y, z: REAL;
              END_ENTITY;
            END_SCHEMA;
            "#
            .trim(),
        )
        .unwrap()
    }
}
