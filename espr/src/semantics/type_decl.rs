use super::*;
use crate::ast;
use inflector::Inflector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnderlyingType {
    Simple(TypeRef),
    Reference(TypeRef),
    Select(Vec<TypeRef>),
    // FIXME
    Unsupported,
}

impl Legalize for UnderlyingType {
    type Input = ast::types::UnderlyingType;
    fn legalize(ns: &Namespace, scope: &Scope, input: &Self::Input) -> Result<Self, SemanticError> {
        let underlying_type = match input {
            ast::types::UnderlyingType::Simple(simple) => {
                UnderlyingType::Simple(TypeRef::SimpleType(*simple))
            }
            ast::types::UnderlyingType::Reference(name) => {
                UnderlyingType::Reference(ns.lookup_type(scope, name)?)
            }
            ast::types::UnderlyingType::Select { types, .. } => {
                let refs: Result<Vec<TypeRef>, _> =
                    types.iter().map(|ty| ns.lookup_type(scope, ty)).collect();
                UnderlyingType::Select(refs?)
            }
            _ => UnderlyingType::Unsupported,
        };
        Ok(underlying_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    type_id: String,
    underlying_type: UnderlyingType,
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        Ok(TypeDecl {
            type_id: type_decl.type_id.clone(),
            underlying_type: UnderlyingType::legalize(ns, scope, &type_decl.underlying_type)?,
        })
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.type_id.to_pascal_case());
        match &self.underlying_type {
            UnderlyingType::Simple(type_ref) | UnderlyingType::Reference(type_ref) => tokens
                .append_all(quote! {
                    #[derive(Debug, Clone, PartialEq)]
                    pub struct #id(pub #type_ref);
                }),
            UnderlyingType::Select(types) => tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub enum #id {
                    #(#types(Box<#types>)),*
                }
            }),
            _ => tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct #id {}
            }),
        }
    }
}
