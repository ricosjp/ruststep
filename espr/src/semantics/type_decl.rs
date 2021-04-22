use super::*;
use crate::ast;
use inflector::Inflector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnderlyingType {
    Reference(TypeRef),
    // FIXME
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    type_id: String,
    underlying_type: UnderlyingType,
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;

    fn legalize(
        _ns: &Namespace,
        _scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        dbg!(&type_decl);
        let underlying_type = match type_decl.underlying_type {
            ast::types::UnderlyingType::Simple(simple) => {
                UnderlyingType::Reference(TypeRef::SimpleType(simple))
            }
            _ => UnderlyingType::Unsupported,
        };
        Ok(TypeDecl {
            type_id: type_decl.type_id.clone(),
            underlying_type,
        })
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.type_id.to_pascal_case());
        match &self.underlying_type {
            UnderlyingType::Reference(type_ref) => tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct #id(pub #type_ref);
            }),
            _ => tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct #id {}
            }),
        }
    }
}
