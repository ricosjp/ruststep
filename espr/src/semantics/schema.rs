use super::*;
use proc_macro2::TokenStream;
use quote::*;

/// Corresponding to `SCHEMA` in EXPRESS
///
/// Here, we consinder following simple schema definition in EXPRESS language
///
/// ```text
/// SCHEMA ONE;
///
/// ENTITY first;
/// m_ref : second;
/// fattr : STRING;
/// END_ENTITY;
///
/// ENTITY second;
/// sattr : STRING;
/// END_ENTITY;
///
/// END_SCHEMA;
/// ```
///
/// EXPRESS's schema consists of `ENTITY`es,
/// which will be translated into Rust struct definitions.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub types: Vec<Type>,
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        tokens.append_all(quote! {
            mod #name {
                #(#types)*
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_defined_definition() {
        let defined = Type::Defined {
            name: "test_defined_type".into(),
            underlying: "FormerType".into(),
        };
        println!("{}", defined.to_token_stream());
    }
}
