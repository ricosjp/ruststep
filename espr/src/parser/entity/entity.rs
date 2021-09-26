use super::{attribute::*, derive::*, domain::*, inverse::*, unique::*};
use crate::{
    ast::*,
    parser::{combinator::*, identifier::*, subsuper::*, types::*},
};

/// 215 explicit_attr = [attribute_decl] { `,` [attribute_decl] } `:` \[ OPTIONAL \] [parameter_type] `;` .
pub fn explicit_attr(input: &str) -> ParseResult<Vec<EntityAttribute>> {
    tuple((
        comma_separated(attribute_decl),
        char(':'),
        opt(tag("OPTIONAL")),
        parameter_type,
        char(';'),
    ))
    .map(|(attrs, _colon, optional, ty, _semicolon)| {
        attrs
            .into_iter()
            .map(|name| EntityAttribute {
                name,
                ty: ty.clone(),
                optional: optional.is_some(),
            })
            .collect()
    })
    .parse(input)
}

/// 207 entity_head = ENTITY [entity_id] [subsuper] `;` .
pub fn entity_head(input: &str) -> ParseResult<(String, Option<Constraint>, Option<SubTypeDecl>)> {
    tuple((
        tag("ENTITY"), // parse with trailing space
        entity_id,
        subsuper,
        char(';'),
    ))
    .map(|(_start, id, (constraint, subtype), _semicolon)| (id, constraint, subtype))
    .parse(input)
}

/// 204 entity_body = { [explicit_attr] } \[ [derive_clause] \] \[ [inverse_clause] \] \[ [unique_clause] \] \[ [where_clause] \] .
pub fn entity_body(input: &str) -> ParseResult<EntityBody> {
    tuple((
        many0(explicit_attr),
        opt(derive_clause),
        opt(inverse_clause),
        opt(unique_clause),
        opt(where_clause),
    ))
    .map(
        |(attributes, derive_clause, inverse_clause, unique_clause, where_clause)| EntityBody {
            attributes: attributes.into_iter().flatten().collect(),
            derive_clause,
            inverse_clause,
            unique_clause,
            where_clause,
        },
    )
    .parse(input)
}

/// 206 entity_decl = [entity_head] [entity_body] END_ENTITY `;` .
pub fn entity_decl(input: &str) -> ParseResult<Entity> {
    tuple((entity_head, entity_body, tag("END_ENTITY"), char(';')))
        .map(
            |(
                (name, constraint, subtype_of),
                EntityBody {
                    attributes,
                    derive_clause,
                    inverse_clause,
                    unique_clause,
                    where_clause,
                },
                _end,
                _semicolon,
            )| Entity {
                name,
                attributes,
                constraint,
                subtype_of,
                derive_clause,
                inverse_clause,
                unique_clause,
                where_clause,
            },
        )
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn entity_head() {
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(constraint, None);
        assert_eq!(subtype, None);
        assert_eq!(residual, "");
    }

    #[test]
    fn subtype_of() {
        // example from ISO 10303-11:2004(E) p.50
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY odd_number SUBTYPE OF (integer_number);")
                .finish()
                .unwrap();
        assert_eq!(name, "odd_number");
        assert_eq!(constraint, None);
        assert_eq!(
            subtype,
            Some(SubTypeDecl {
                entity_references: vec!["integer_number".to_string()]
            })
        );
        assert_eq!(residual, "");
    }

    #[test]
    fn abstract_entity() {
        // example from ISO 10303-11:2004(E) p.52
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY line ABSTRACT;")
                .finish()
                .unwrap();
        assert_eq!(name, "line");
        assert_eq!(constraint, Some(Constraint::AbstractEntity));
        assert_eq!(subtype, None);
        assert_eq!(residual, "");
    }

    #[test]
    fn one_of() {
        // example from ISO 10303-11:2004(E) p.57 with some modification
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY pet ABSTRACT SUPERTYPE OF (ONEOF(cat, rabbit, dog));")
                .finish()
                .unwrap();
        assert_eq!(name, "pet");
        assert_eq!(
            constraint,
            Some(Constraint::AbstractSuperType(Some(
                SuperTypeExpression::OneOf {
                    exprs: vec![
                        SuperTypeExpression::Reference("cat".to_string()),
                        SuperTypeExpression::Reference("rabbit".to_string()),
                        SuperTypeExpression::Reference("dog".to_string())
                    ]
                }
            )))
        );
        assert_eq!(subtype, None);
        assert_eq!(residual, "");
    }

    #[test]
    fn explicit_attr() {
        let (residual, (attrs, _remark)) = super::explicit_attr("x : REAL;").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(attrs.len(), 1);
        let attr = &attrs[0];
        assert_eq!(attr.name, "x");
        assert!(matches!(attr.ty, Type::Simple(SimpleType::Real)));
    }

    #[test]
    fn explicit_attr2() {
        let (residual, (attrs, _remark)) = super::explicit_attr("x, y : REAL;").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(attrs.len(), 2);
        let attr = &attrs[0];
        assert_eq!(attr.name, "x");
        assert!(matches!(attr.ty, Type::Simple(SimpleType::Real)));
        let attr = &attrs[1];
        assert_eq!(attr.name, "y");
        assert!(matches!(attr.ty, Type::Simple(SimpleType::Real)));
    }

    #[test]
    fn explicit_attr_optional() {
        let (residual, (attrs, _remark)) =
            super::explicit_attr("x: OPTIONAL REAL;").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(attrs.len(), 1);
        let attr = &attrs[0];
        assert_eq!(attr.name, "x");
        assert!(matches!(attr.ty, Type::Simple(SimpleType::Real)));
        assert!(attr.optional);
    }

    #[test]
    fn entity_decl() {
        let exp_str = r#"
        ENTITY fiRst;
          m_Ref : second;
          fattr : REAL;
        END_ENTITY;
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
        assert_eq!(entity.name, "fiRst");

        assert_eq!(entity.attributes.len(), 2);
        // check `m_ref`
        assert_eq!(entity.attributes[0].name, "m_Ref");
        assert!(matches!(entity.attributes[0].ty, Type::Named(_)));
        // check `fattr`
        assert_eq!(entity.attributes[1].name, "fattr");
        assert!(matches!(
            entity.attributes[1].ty,
            Type::Simple(SimpleType::Real)
        ));

        assert_eq!(residual, "");
    }

    #[test]
    fn entity_subtype() {
        let exp_str = r#"
        ENTITY camera_model_d2 SUBTYPE OF (camera_model);
            view_window          : planar_box;
            view_window_clipping : BOOLEAN;
          WHERE
            wr1: SELF\geometric_representation_item.dim = 2;
        END_ENTITY;
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
        dbg!(&entity);
        assert_eq!(residual, "");

        assert_eq!(entity.name, "camera_model_d2");
        assert_eq!(entity.attributes.len(), 2);
        assert!(entity.where_clause.is_some());
    }

    #[test]
    fn entity_unique() {
        let exp_str = r#"
        ENTITY drawing_revision SUBTYPE OF (presentation_set);
          revision_identifier : identifier;
          drawing_identifier  : drawing_definition;
          intended_scale      : OPTIONAL text;
        UNIQUE
          ur1 : revision_identifier, drawing_identifier;
        END_ENTITY;
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
        dbg!(&entity);
        assert_eq!(residual, "");

        assert_eq!(entity.name, "drawing_revision");
        assert_eq!(entity.attributes.len(), 3);
        assert!(entity.unique_clause.is_some());
    }

    #[test]
    fn entity_derive() {
        let exp_str = r#"
        ENTITY si_unit SUBTYPE OF (named_unit);
          prefix : OPTIONAL si_prefix;
          name   : si_unit_name;
        DERIVE
          SELF\named_unit.dimensions : dimensional_exponents := dimensions_for_si_unit(SELF.name);
        END_ENTITY;
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
        dbg!(&entity);
        assert_eq!(residual, "");

        assert_eq!(entity.name, "si_unit");
        assert_eq!(entity.attributes.len(), 2);
        assert!(entity.derive_clause.is_some());
    }

    #[test]
    fn entity_ap203_axis2_placement_3d() {
        // From AP203
        let exp_str = r#"
        ENTITY axis2_placement_3d SUBTYPE OF (placement);
            axis          : OPTIONAL direction;
            ref_direction : OPTIONAL direction;
        DERIVE
            p : LIST [3:3] OF direction := build_axes(axis,ref_direction);
        WHERE
            wr1: (SELF\placement.location.dim = 3);
            wr2: ((NOT EXISTS(axis)) OR (axis.dim = 3));
            wr3: ((NOT EXISTS(ref_direction)) OR (ref_direction.dim = 3));
            wr4: ((NOT EXISTS(axis)) OR (NOT EXISTS(ref_direction)) OR (cross_product(axis,ref_direction).magnitude > 0));
        END_ENTITY;
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
        dbg!(&entity);
        assert_eq!(residual, "");
    }

    #[test]
    fn bspline_curve() {
        // from AP201
        let input = r#"
        ENTITY b_spline_curve
          SUPERTYPE OF (ONEOF (uniform_curve,b_spline_curve_with_knots,
              quasi_uniform_curve,bezier_curve) ANDOR rational_b_spline_curve)
          SUBTYPE OF (bounded_curve);
            degree              : INTEGER;
            control_points_list : LIST [2:?] OF cartesian_point;
            curve_form          : b_spline_curve_form;
            closed_curve        : LOGICAL;
            self_intersect      : LOGICAL;
          DERIVE
            upper_index_on_control_points : INTEGER := SIZEOF(
                                               control_points_list) - 1;
            control_points                : ARRAY [0:
                                               upper_index_on_control_points] OF
                                                cartesian_point := list_to_array(
                                               control_points_list,0,
                                               upper_index_on_control_points);
          WHERE
            wr1: ('EXPLICIT_DRAUGHTING.UNIFORM_CURVE' IN TYPEOF(SELF)) OR (
                     'EXPLICIT_DRAUGHTING.QUASI_UNIFORM_CURVE' IN TYPEOF(SELF)) 
                     OR ('EXPLICIT_DRAUGHTING.BEZIER_CURVE' IN TYPEOF(SELF)) OR (
                     'EXPLICIT_DRAUGHTING.B_SPLINE_CURVE_WITH_KNOTS' IN TYPEOF(
                     SELF));
        END_ENTITY; -- b_spline_curve
        "#
        .trim();

        let (residual, (entity, _remark)) = super::entity_decl(input).finish().unwrap();
        dbg!(&entity);
        assert_eq!(residual, "");
    }
}
