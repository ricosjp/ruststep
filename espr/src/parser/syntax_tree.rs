use super::{remark::*, schema::*, util::*};
use nom::{sequence::*, Finish, Parser};

/// Entire syntax tree parsed from EXPRESS Language string
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub schemas: Vec<Schema>,
    pub remarks: Vec<Remark>,
}

impl SyntaxTree {
    pub fn parse(input: &str) -> Result<Self, nom::error::Error<&str>> {
        let (residual, st) = tuple((
            spaces_or_remarks,
            space_separated(schema),
            spaces_or_remarks,
        ))
        .map(|(mut remarks, (schemas, mut r1), mut r2)| {
            remarks.append(&mut r1);
            remarks.append(&mut r2);
            SyntaxTree { schemas, remarks }
        })
        .parse(input)
        .finish()?;
        assert!(residual.is_empty());
        Ok(st)
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
