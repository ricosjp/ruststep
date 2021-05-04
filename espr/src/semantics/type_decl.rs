use super::*;
use crate::ast;
use inflector::Inflector;

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

impl ToTokens for Rename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            pub type #id = #ty;
        });
    }
}

/// Enumeration of values,
/// e.g. `TYPE text_path = ENUMERATION OF (up, right, down, left); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    id: String,
    items: Vec<String>,
}

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let items: Vec<_> = self
            .items
            .iter()
            .map(|i| format_ident!("{}", i.to_pascal_case()))
            .collect();
        tokens.append_all(quote! {
            #[derive(Debug, Clone)]
            pub enum #id {
                #( #items ),*
            }
        });
    }
}

/// Select of user defined types,
/// e.g. `TYPE geometric_set_select = SELECT (point, curve); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    id: String,
    types: Vec<TypeRef>,
}

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let mut entries = Vec::new();
        let mut entry_types = Vec::new();
        for ty in &self.types {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDecl {
    Simple(Simple),
    Rename(Rename),
    Enumeration(Enumeration),
    Select(Select),
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        use ast::types::UnderlyingType;
        let id = type_decl.type_id.clone();
        Ok(match &type_decl.underlying_type {
            UnderlyingType::Simple(ty) => TypeDecl::Simple(Simple {
                id,
                ty: SimpleType(*ty),
            }),
            UnderlyingType::Reference(name) => {
                let ty = ns.lookup_type(scope, name)?;
                TypeDecl::Rename(Rename { id, ty })
            }
            UnderlyingType::Enumeration {
                items,
                extensibility: _,
            } => TypeDecl::Enumeration(Enumeration {
                id,
                items: items.clone(),
            }),
            UnderlyingType::Select {
                types,
                extensibility: _,
            } => {
                let types = types
                    .iter()
                    .map(|ty| ns.lookup_type(scope, ty))
                    .collect::<Result<Vec<_>, _>>()?;
                TypeDecl::Select(Select { id, types })
            }
            _ => panic!(),
        })
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDecl::Simple(simple) => simple.to_tokens(tokens),
            TypeDecl::Rename(rename) => rename.to_tokens(tokens),
            TypeDecl::Enumeration(e) => e.to_tokens(tokens),
            TypeDecl::Select(select) => select.to_tokens(tokens),
        }
    }
}
