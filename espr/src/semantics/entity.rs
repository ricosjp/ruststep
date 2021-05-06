use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,
    pub subtypes: Option<Vec<TypeRef>>,
    /// FIXME This assumes that `SUPERTYPE` declaration exists for all supertypes.
    pub has_supertype_decl: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    name: String,
    ty: TypeRef,
    optional: bool,
}

impl Legalize for EntityAttribute {
    type Input = ast::entity::EntityAttribute;

    fn legalize(ns: &Namespace, scope: &Scope, attr: &Self::Input) -> Result<Self, SemanticError> {
        let ty = TypeRef::legalize(ns, scope, &attr.ty)?;
        let name = match &attr.name {
            ast::entity::AttributeDecl::Reference(name) => name.clone(),
            _ => unimplemented!(),
        };
        Ok(EntityAttribute {
            name,
            ty,
            optional: attr.optional,
        })
    }
}

impl Legalize for Entity {
    type Input = ast::entity::Entity;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        entity: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;
        let subtypes = entity
            .subtype
            .as_ref()
            .map(|subtype| {
                subtype
                    .entity_references
                    .iter()
                    .map(|name| ns.lookup_type(scope, &name))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let name = entity.name.clone();
        Ok(Entity {
            name,
            attributes,
            subtypes,
            has_supertype_decl: entity.has_supertype_decl(),
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.to_pascal_case());
        let holder_name = format_ident!("{}Holder", self.name.to_pascal_case());

        let mut attr_name = Vec::new();
        let mut attr_type = Vec::new();
        let mut holder_attr_type = Vec::new();
        let mut holder_attr_expr = Vec::new();

        for EntityAttribute { name, ty, optional } in &self.attributes {
            let name = format_ident!("{}", name);
            attr_name.push(name.clone());
            if *optional {
                attr_type.push(quote! { Option<#ty> });
                if ty.is_simple() {
                    holder_attr_type.push(quote! { Option<#ty> });
                } else {
                    holder_attr_type.push(quote! { Option<PlaceHolder<#ty>> });
                }
            } else {
                attr_type.push(quote! { #ty });
                if ty.is_simple() {
                    holder_attr_type.push(quote! { #ty });
                    holder_attr_expr.push(quote! { #name })
                } else {
                    holder_attr_type.push(quote! { PlaceHolder<#ty> });
                    holder_attr_expr.push(quote! { #name.into_owned(tables)? })
                }
            }
        }

        if let Some(subtypes) = &self.subtypes {
            for ty in subtypes {
                let (attr, ty) = match ty {
                    TypeRef::Named { name, .. } | TypeRef::Entity { name, .. } => {
                        (format_ident!("{}", name), ty)
                    }
                    _ => unreachable!(),
                };

                attr_name.push(attr.clone());
                attr_type.push(ty.to_token_stream());

                if ty.is_simple() {
                    holder_attr_type.push(quote! { #ty });
                    holder_attr_expr.push(quote! { #attr })
                } else {
                    holder_attr_type.push(quote! { PlaceHolder<#ty> });
                    holder_attr_expr.push(quote! { #attr.into_owned(tables)? })
                }

                if let TypeRef::Entity {
                    name: supertype_name,
                    has_supertype_decl,
                    ..
                } = ty
                {
                    if *has_supertype_decl {
                        let any_trait = format_ident!("{}Any", supertype_name.to_pascal_case());
                        tokens.append_all(quote! {
                            impl #any_trait for #name {}
                        });
                    }
                }
            }
        }

        assert_eq!(attr_name.len(), attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_expr.len());

        tokens.append_all(quote! {
            #[derive(Debug, Clone, derive_new::new)]
            pub struct #name {
                #(
                pub #attr_name : #attr_type,
                )*
            }

            #[derive(Clone, Debug)]
            struct #holder_name {
                #(
                #attr_name : #holder_attr_type,
                )*
            }

            impl Holder for #holder_name {
                type Table = Tables;
                type Owned = #name;
                fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
                    let #holder_name { #(#attr_name),* } = self;
                    Ok(#name {
                        #(#attr_name : #holder_attr_expr),*
                    })
                }
            }
        });

        if self.has_supertype_decl {
            let trait_name = format_ident!("{}Any", name);
            tokens.append_all(quote! {
                pub trait #trait_name:
                    ::std::any::Any
                  + ::std::fmt::Debug
                  + dyn_clone::DynClone
                {}
                dyn_clone::clone_trait_object!(#trait_name);
            });
            tokens.append_all(quote! {
                impl #trait_name for #name {}
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalize() {
        let example = SyntaxTree::example();
        let ns = Namespace::new(&example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
