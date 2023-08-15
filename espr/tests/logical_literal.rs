use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA IFC4X3_DEV_6a23ae8;
ENTITY IfcGeometricRepresentationContext;
	TrueNorth : OPTIONAL BOOLEAN;
 WHERE
	North2D : NOT(EXISTS(TrueNorth)) OR (HIINDEX(TrueNorth.DirectionRatios) = 2);
END_ENTITY;

END_SCHEMA;
"#;

#[test]
fn logical_literal() {
    let st = SyntaxTree::parse(EXPRESS).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    insta::assert_snapshot!(tt, @r###"
    pub mod IFC4X3_DEV_6a23ae8 {
        use ruststep::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            IfcGeometricRepresentationContext:
                HashMap<u64, as_holder!(IfcGeometricRepresentationContext)>,
        }
        impl Tables {
            pub fn IfcGeometricRepresentationContext_holders(
                &self,
            ) -> &HashMap<u64, as_holder!(IfcGeometricRepresentationContext)> {
                &self.IfcGeometricRepresentationContext
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = IfcGeometricRepresentationContext)]
        #[holder(generate_deserialize)]
        pub struct IfcGeometricRepresentationContext {
            pub TrueNorth: Option<bool>,
        }
    }
    "###);
}
