use super::{combinator::*};

pub const KEYWORDS: &'static [&'static str] = &[
    "abstract",
    "aggregate",
    "alias",
    "array",
    "as",
    "bag",
    "based",
    "on",
    "begin",
    "binary",
    "boolean",
    "by",
    "case",
    "constant",
    "derive",
    "else",
    "end",
    "end_alias",
    "end_case", 
    "end_constant", 
    "end_entity",
    "end_function", 
    "end_if", 
    "end_local", 
    "end_procedure",
    "end_repeat", 
    "end_rule", 
    "end_schema", 
    "end_subtype_constraint",
    "end_type", 
    "entity", 
    "enumeration", 
    "escape",
    "extensible", 
    "fixed", 
    "for", 
    "from",
    "function", 
    "generic", 
    "generic_entity", 
    "if",
    "integer", 
    "inverse", 
    "list", 
    "local",
    "logical", 
    "number", 
    "of", 
    "oneof",
    "optional", 
    "otherwise", 
    "procedure", 
    "query",
    "real", 
    "renamed", 
    "reference", 
    "repeat",
    "return", 
    "rule", 
    "schema", 
    "select",
    "set", 
    "skip", 
    "string", 
    "subtype",
    "constraint", 
    "supertype", 
    "then", 
    "to",
    "total_over", 
    "type", 
    "unique", 
    "until",
    "use", 
    "var", 
    "where", 
    "while",
    "with",];

pub fn is_reserved(input: &str) -> ParseResult<()> {
    let contains_keyword = !KEYWORDS.iter().fold(false, |acc, &keyword| {
        let result = nom::combinator::peek(tag(keyword))(input);
        acc || !result.is_err()
    });
    if !contains_keyword {
        Ok((input, ((), Vec::new())))
    } else {
        Err(nom::Err::Error(nom::error::VerboseError { errors: Vec::new()}))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_reserved() {
        assert_eq!(super::is_reserved("END_ENTITY"), Ok(("END_ENTITY", ((), Vec::new()))));
        assert_eq!(super::is_reserved("AbsTraCt"), Ok(("AbsTraCt", ((), Vec::new()))));
        assert_eq!(super::is_reserved("HomHom"), Err(nom::Err::Error(nom::error::VerboseError { errors: Vec::new()})))
    }
}