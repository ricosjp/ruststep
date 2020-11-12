use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiableFactor {}

/// 274 qualifiable_factor = attribute_ref | constant_factor | function_call | general_ref | population .
pub fn qualifiable_factor(_input: &str) -> IResult<&str, QualifiableFactor> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedAttribute {}

/// 275 qualified_attribute = SELF group_qualifier attribute_qualifier .
pub fn qualified_attribute(_input: &str) -> IResult<&str, QualifiedAttribute> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Qualifier {}

/// 276 qualifier = attribute_qualifier | group_qualifier | index_qualifier .
pub fn qualifier(_input: &str) -> IResult<&str, Qualifier> {
    todo!()
}
