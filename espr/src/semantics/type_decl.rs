use super::*;
use crate::ast;
use inflector::Inflector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    type_id: String,
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;

    fn legalize(
        _ns: &Namespace,
        _scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        dbg!(&type_decl);
        Ok(TypeDecl {
            type_id: type_decl.type_id.clone(),
        })
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.type_id.to_pascal_case());
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct #id {}
        })
    }
}
