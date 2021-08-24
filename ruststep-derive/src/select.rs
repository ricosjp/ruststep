use super::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn decompose_box_ty(ty: &syn::Type) -> &syn::Type {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        assert_eq!(path.segments.len(), 1);
        let box_path = &path.segments[0];
        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args,
            ..
        }) = &box_path.arguments
        {
            assert_eq!(args.len(), 1);
            if let syn::GenericArgument::Type(ty) = &args[0] {
                return ty;
            }
        }
    }
    unreachable!("Not Box<T>")
}

pub fn derive_holder(ident: &syn::Ident, e: &syn::DataEnum) -> TokenStream2 {
    let holder_ident = as_holder_ident(ident);
    let variants: Vec<&proc_macro2::Ident> = e.variants.iter().map(|var| &var.ident).collect();
    let ty: Vec<&syn::Type> = e
        .variants
        .iter()
        .map(|var| {
            assert_eq!(var.fields.len(), 1);
            var.fields.iter().map(|f| decompose_box_ty(&f.ty))
        })
        .flatten()
        .collect();
    let holder_types: Vec<_> = ty.iter().map(|t| as_holder_path(t)).collect();

    dbg!(holder_ident);
    dbg!(variants);
    dbg!(ty);
    dbg!(holder_types);

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        enum BaseAnyHolder {
            Sub1(Box<Sub1Holder>),
            Sub2(Box<Sub2Holder>),
        }

        impl Holder for BaseAnyHolder {
            type Owned = BaseAny;
            type Table = Table;
            fn into_owned(self, table: &Table) -> Result<Self::Owned> {
                Ok(match self {
                    BaseAnyHolder::Sub1(sub) => BaseAny::Sub1(Box::new(sub.into_owned(table)?)),
                    BaseAnyHolder::Sub2(sub) => BaseAny::Sub2(Box::new(sub.into_owned(table)?)),
                })
            }
            fn name() -> &'static str {
                "BaseAny"
            }
            fn attr_len() -> usize {
                0
            }
        }

        impl<'de> de::Deserialize<'de> for BaseAnyHolder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("A", 2, BaseAnyHolderVisitor {})
            }
        }

        struct BaseAnyHolderVisitor;

        impl<'de> ::serde::de::Visitor<'de> for BaseAnyHolderVisitor {
            type Value = BaseAnyHolder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "BaseAny")
            }

            // Entry point for Record or Parameter::Typed
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
                match key.as_str() {
                    "SUB1" => {
                        let value: Sub1Holder = map.next_value()?; // send to Self::visit_seq
                        return Ok(BaseAnyHolder::Sub1(Box::new(value)));
                    }
                    "SUB2" => {
                        let value: Sub2Holder = map.next_value()?; // send to Self::visit_seq
                        return Ok(BaseAnyHolder::Sub2(Box::new(value)));
                    }
                    _ => {
                        use ::serde::de::{Error, Unexpected};
                        return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                    }
                }
            }
        }

        impl WithVisitor for BaseAnyHolder {
            type Visitor = BaseAnyHolderVisitor;
            fn visitor_new() -> Self::Visitor {
                BaseAnyHolderVisitor {}
            }
        }
    } // quote!
}

pub fn derive_deserialize(_ident: &syn::Ident, _e: &syn::DataEnum) -> TokenStream2 {
    todo!()
}
