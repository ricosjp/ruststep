use super::*;
use crate::ast;
use inflector::Inflector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnderlyingType {
    Simple(TypeRef),
    Reference(TypeRef),
    Enumeration(Vec<String>),
    Select(Vec<TypeRef>),
}

/// Rename of primitive type,
/// e.g. `TYPE label = STRING; ENDTYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simple {
    id: String,
    ty: SimpleType,
}

impl ToTokens for Simple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            pub type #id = #ty;
        });
    }
}

/// Rename of user defined type,
/// e.g. `TYPE box_height = positive_ratio_measure; END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename {
    id: String,
    ty: TypeRef,
}

/// Enumeration of values,
/// e.g. `TYPE text_path = ENUMERATION OF (up, right, down, left); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    id: String,
    values: Vec<String>,
}

/// Select of user defined types,
/// e.g. `TYPE geometric_set_select = SELECT (point, curve); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    id: String,
    types: Vec<TypeRef>,
}

impl Legalize for UnderlyingType {
    type Input = ast::types::UnderlyingType;
    fn legalize(ns: &Namespace, scope: &Scope, input: &Self::Input) -> Result<Self, SemanticError> {
        let underlying_type = match input {
            ast::types::UnderlyingType::Simple(simple) => {
                UnderlyingType::Simple(TypeRef::SimpleType(SimpleType(*simple)))
            }
            ast::types::UnderlyingType::Reference(name) => {
                UnderlyingType::Reference(ns.lookup_type(scope, name)?)
            }
            ast::types::UnderlyingType::Enumeration { items, .. } => {
                // FIXME extensibility
                UnderlyingType::Enumeration(items.clone())
            }
            ast::types::UnderlyingType::Select { types, .. } => {
                // FIXME extensibility
                let refs: Result<Vec<TypeRef>, _> =
                    types.iter().map(|ty| ns.lookup_type(scope, ty)).collect();
                UnderlyingType::Select(refs?)
            }
            _ => unimplemented!(),
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
                    pub type #id = #type_ref;
                }),
            UnderlyingType::Enumeration(items) => {
                let items: Vec<_> = items
                    .into_iter()
                    .map(|i| format_ident!("{}", i.to_pascal_case()))
                    .collect();
                tokens.append_all(quote! {
                    #[derive(Debug, Clone)]
                    pub enum #id {
                        #( #items ),*
                    }
                });
            }
            UnderlyingType::Select(types) => {
                let mut entries = Vec::new();
                let mut entry_types = Vec::new();
                for ty in types {
                    match ty {
                        TypeRef::Entity {
                            name,
                            has_supertype_decl,
                            ..
                        } => {
                            let name = format_ident!("{}", name.to_pascal_case());
                            entries.push(quote! { #name });
                            if *has_supertype_decl {
                                // avoid Box<Box<XxxAny>>
                                entry_types.push(quote! { #ty });
                            } else {
                                entry_types.push(quote! { Box<#ty> });
                            }
                        }
                        _ => {
                            entries.push(ty.to_token_stream());
                            entry_types.push(quote! { Box<#ty> });
                        }
                    }
                }
                tokens.append_all(quote! {
                    #[derive(Debug, Clone)]
                    pub enum #id {
                        #(#entries(#entry_types)),*
                    }
                });
            }
        }
    }
}
