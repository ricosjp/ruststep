use super::{remark::*, schema::*, combinator::*};
use nom::Finish;

/// Entire syntax tree parsed from EXPRESS Language string
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub schemas: Vec<Schema>,
    pub remarks: Vec<Remark>,
}

impl SyntaxTree {
    pub fn parse(input: &str) -> Result<Self, nom::error::VerboseError<&str>> {
        let (residual, (schemas, remarks)) = tuple((spaces, space_separated(schema_decl), spaces))
            .map(|(_start_space, schemas, _end_space)| schemas)
            .parse(input)
            .finish()?;
        assert!(residual.is_empty());
        Ok(SyntaxTree { schemas, remarks })
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

#[cfg(test)]
mod tests {

    #[test]
    fn parse_remarks() {
        let st = super::SyntaxTree::parse(
            r#"
            SCHEMA one;
              ENTITY first;
                m_ref : second;
                fattr : STRING;
              END_ENTITY; -- first
              ENTITY second;
                sattr : STRING;
              END_ENTITY; -- second
              (* this is the example! *)
            END_SCHEMA;

            (* Hey! *)

            SCHEMA geometry0;
              ENTITY point;
                x, (* y, *) z: REAL; -- skip y
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();
        dbg!(&st);
        assert_eq!(st.remarks.len(), 6);
    }
}
