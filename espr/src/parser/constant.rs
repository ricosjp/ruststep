use crate::{
    ast::constant::*,
    parser::{combinator::*, identifier::*, types::*, expression::*},
};

/// 195 constant_decl = CONSTANT constant_body { constant_body } END_CONSTANT ’;’ .
pub fn constant_decl(input: &str) -> ParseResult<Constant> {
        tuple((tag("CONSTANT"), many1(constant_body), tag("END_CONSTANT"), char(';')))
        .map(|(_constant, bodies, _end_constant, _semicolon)| Constant {
            bodies: bodies
        })
        .parse(input)
}

/// 194 constant_body = constant_id ’:’ instantiable_type ’:=’ expression ’;’ .
pub fn constant_body(input: &str) -> ParseResult<ConstantBody> {
    tuple((
       constant_id, char(':'), instantiable_type, tag(":="), expression, char(';')
    ))
    .map(|(constant_id, _colon, ty, _assignment, expression, _semicolon)| ConstantBody {
        name: constant_id,
        ty: ty,
        expression: expression
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::*;
    use crate::ast::expression::*;
    use nom::Finish;
    #[test]
    fn constant_decl() {
        let (residual, (constant, _remark)) = super::constant_decl(
            r#"
            CONSTANT
                t1 : NUMBER := 5;
            END_CONSTANT;
            "#.trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(constant, Constant {
            bodies: vec!(
                ConstantBody { 
                    name: "t1".to_string(), 
                    ty: UnderlyingType::Simple(SimpleType::Number), 
                    expression: Expression::Literal(Literal::Real(5f64))
                })
        });
    }
}
