use super::*;
use inflector::Inflector;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::quote;

struct Input {
    name: String,
    table: syn::Path,
    ident: syn::Ident,
    holder_ident: syn::Ident,
    holder_visitor_ident: syn::Ident,
    variants: Vec<syn::Ident>,
    variant_names: Vec<String>,
    variant_exprs: Vec<TokenStream2>,
    holder_types: Vec<syn::Type>,
    holder_exprs: Vec<TokenStream2>,
    table_fields: Vec<Option<syn::Ident>>,
}

impl Input {
    fn parse(ident: &syn::Ident, e: &syn::DataEnum, attr: &HolderAttr) -> Self {
        let name = ident.to_string().to_screaming_snake_case();
        let holder_ident = as_holder_ident(ident);
        let holder_visitor_ident = as_visitor_ident(&holder_ident);
        let variants: Vec<syn::Ident> = e.variants.iter().map(|var| var.ident.clone()).collect();
        let variant_names: Vec<_> = variants
            .iter()
            .map(|id| id.to_string().to_screaming_snake_case())
            .collect();
        let table = attr
            .table
            .clone()
            .expect_or_abort("table attribute is lacked");

        let mut holder_exprs = Vec::new();
        let mut holder_types = Vec::new();
        let mut table_fields = Vec::new();
        let mut variant_exprs = Vec::new();
        for var in &e.variants {
            let HolderAttr {
                field,
                place_holder,
                ..
            } = HolderAttr::parse(&var.attrs);
            table_fields.push(field);

            assert_eq!(var.fields.len(), 1);
            for f in &var.fields {
                let ty = FieldType::try_from(f.ty.clone()).unwrap();
                if let FieldType::Boxed(_) = ty {
                    if place_holder {
                        // ENTITY case
                        holder_types.push(as_holder_path(&f.ty));
                        holder_exprs.push(quote! { Box::new(sub.into_owned(table)?) });
                        variant_exprs.push(quote! { Box::new(owned.into()) });
                    } else {
                        abort_call_site!("Simple type should not be Boxed")
                    }
                } else {
                    variant_exprs.push(quote! { owned });
                    if place_holder {
                        // *Any case
                        holder_types.push(as_holder_path(&f.ty));
                        holder_exprs.push(quote! { sub.into_owned(table)? });
                    } else {
                        // SimpleType case
                        holder_types.push(f.ty.clone());
                        holder_exprs.push(quote! { sub });
                    }
                }
            }
        }

        Input {
            name,
            table,
            ident: ident.clone(),
            holder_ident,
            holder_visitor_ident,
            variants,
            variant_names,
            variant_exprs,
            holder_types,
            holder_exprs,
            table_fields,
        }
    }

    fn def_holder(&self) -> TokenStream2 {
        let Input {
            holder_ident,
            variants,
            holder_types,
            ..
        } = self;
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub enum #holder_ident {
                #(#variants(#holder_types)),*
            }
        } // quote!
    }

    fn impl_holder(&self) -> TokenStream2 {
        let Input {
            name,
            ident,
            holder_ident,
            variants,
            table,
            holder_exprs,
            ..
        } = self;
        let ruststep = ruststep_crate();

        quote! {
            impl #ruststep::tables::Holder for #holder_ident {
                type Owned = #ident;
                type Table = #table;
                fn into_owned(self, table: &Self::Table) -> #ruststep::error::Result<Self::Owned> {
                    Ok(match self {
                        #(#holder_ident::#variants(sub) => #ident::#variants(#holder_exprs)),*
                    })
                }
                fn name() -> &'static str {
                    #name
                }
                fn attr_len() -> usize {
                    0
                }
            }
        } // quote!
    }

    fn impl_deserialize(&self) -> TokenStream2 {
        let Input {
            name,
            holder_ident,
            holder_visitor_ident,
            ..
        } = self;
        quote! {
            impl<'de> ::serde::de::Deserialize<'de> for #holder_ident {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_tuple_struct(#name, 0, #holder_visitor_ident {})
                }
            }
        } // quote!
    }

    fn def_visitor(&self) -> TokenStream2 {
        let Input {
            holder_ident,
            holder_visitor_ident,
            name,
            variants,
            variant_names,
            variant_exprs,
            ..
        } = self;
        let ruststep = ruststep_crate();

        quote! {
            pub struct #holder_visitor_ident;

            impl<'de> ::serde::de::Visitor<'de> for #holder_visitor_ident {
                type Value = #holder_ident;
                fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(formatter, #name)
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
                        #(
                        #variant_names => {
                            let owned = map.next_value()?;
                            return Ok(#holder_ident::#variants(#variant_exprs));
                        }
                        )*
                        _ => {
                            use ::serde::de::{Error, Unexpected};
                            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                        }
                    }
                }
            }

            impl #ruststep::tables::WithVisitor for #holder_ident {
                type Visitor = #holder_visitor_ident;
                fn visitor_new() -> Self::Visitor {
                    #holder_visitor_ident {}
                }
            }
        } // quote!
    }

    fn impl_entity_table(&self) -> TokenStream2 {
        let Input {
            ident,
            holder_ident,
            variants,
            table_fields,
            table,
            variant_exprs,
            ..
        } = self;
        let ruststep = ruststep_crate();
        let mut vars = Vec::new();
        let mut fields = Vec::new();
        let mut exprs = Vec::new();
        for ((var, field), expr) in variants
            .iter()
            .zip(table_fields.iter())
            .zip(variant_exprs.iter())
        {
            if let Some(field) = field {
                vars.push(var);
                fields.push(field);
                exprs.push(expr);
            }
        }

        quote! {
            impl #ruststep::tables::EntityTable<#holder_ident> for #table {
                fn get_owned(&self, entity_id: u64) -> #ruststep::error::Result<#ident> {
                    #(
                    if let Ok(owned) = #ruststep::tables::get_owned(self, &self.#fields, entity_id) {
                        return Ok(#ident::#vars(#exprs));
                    }
                    )*
                    Err(#ruststep::error::Error::UnknownEntity(entity_id))
                }
                fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = #ruststep::error::Result<#ident>> + 'table> {
                    Box::new(::itertools::chain![
                        #(
                        #ruststep::tables::owned_iter(self, &self.#fields)
                            .map(|owned| owned.map(|owned| #ident::#vars(#exprs)))
                        ),*
                    ])
                }
            }
        } // quote!
    }
}

pub fn derive_holder(ident: &syn::Ident, e: &syn::DataEnum, attr: &HolderAttr) -> TokenStream2 {
    let input = Input::parse(ident, e, attr);
    let def_holder_tt = input.def_holder();
    let impl_holder_tt = input.impl_holder();

    if attr.generate_deserialize {
        let impl_deserialize_tt = input.impl_deserialize();
        let def_visitor_tt = input.def_visitor();
        let impl_entity_table_tt = input.impl_entity_table();
        quote! {
            #def_holder_tt
            #impl_holder_tt
            #impl_deserialize_tt
            #def_visitor_tt
            #impl_entity_table_tt
        } // quote!
    } else {
        quote! {
            #def_holder_tt
            #impl_holder_tt
        } // quote!
    }
}

pub fn derive_deserialize(_ident: &syn::Ident, _e: &syn::DataEnum) -> TokenStream2 {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            #[holder(table = Table)]
            #[holder(generate_deserialize)]
            pub enum S1 {
                #[holder(field = a)]
                #[holder(use_place_holder)]
                A(Box<A>),
                #[holder(field = b)]
                #[holder(use_place_holder)]
                B(Box<B>),
            }
            "#,
        )
        .unwrap();

        let tt = crate::derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[derive(Clone, Debug, PartialEq)]
        pub enum S1Holder {
            A(Box<AHolder>),
            B(Box<BHolder>),
        }
        impl ::ruststep::tables::Holder for S1Holder {
            type Owned = S1;
            type Table = Table;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                Ok(match self {
                    S1Holder::A(sub) => S1::A(Box::new(sub.into_owned(table)?)),
                    S1Holder::B(sub) => S1::B(Box::new(sub.into_owned(table)?)),
                })
            }
            fn name() -> &'static str {
                "S1"
            }
            fn attr_len() -> usize {
                0
            }
        }
        impl<'de> ::serde::de::Deserialize<'de> for S1Holder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("S1", 0, S1HolderVisitor {})
            }
        }
        pub struct S1HolderVisitor;
        impl<'de> ::serde::de::Visitor<'de> for S1HolderVisitor {
            type Value = S1Holder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "S1")
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                match key.as_str() {
                    "A" => {
                        let owned = map.next_value()?;
                        return Ok(S1Holder::A(Box::new(owned.into())));
                    }
                    "B" => {
                        let owned = map.next_value()?;
                        return Ok(S1Holder::B(Box::new(owned.into())));
                    }
                    _ => {
                        use serde::de::{Error, Unexpected};
                        return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                    }
                }
            }
        }
        impl ::ruststep::tables::WithVisitor for S1Holder {
            type Visitor = S1HolderVisitor;
            fn visitor_new() -> Self::Visitor {
                S1HolderVisitor {}
            }
        }
        impl ::ruststep::tables::EntityTable<S1Holder> for Table {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<S1> {
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.a, entity_id) {
                    return Ok(S1::A(Box::new(owned.into())));
                }
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.b, entity_id) {
                    return Ok(S1::B(Box::new(owned.into())));
                }
                Err(::ruststep::error::Error::UnknownEntity(entity_id))
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<S1>> + 'table> {
                Box::new(::itertools::chain![
                    ::ruststep::tables::owned_iter(self, &self.a)
                        .map(|owned| owned.map(|owned| S1::A(Box::new(owned.into())))),
                    ::ruststep::tables::owned_iter(self, &self.b)
                        .map(|owned| owned.map(|owned| S1::B(Box::new(owned.into()))))
                ])
            }
        }
        "###);
    }

    #[test]
    fn base_any() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            # [holder (table = Tables)]
            #[holder(generate_deserialize)]
            pub enum BaseAny {
                #[holder(use_place_holder)]
                # [holder (field = base)]
                Base(Box<Base>),
                #[holder(use_place_holder)]
                # [holder (field = sub)]
                Sub(Box<SubAny>),
            }
            "#,
        )
        .unwrap();

        let tt = crate::derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[derive(Clone, Debug, PartialEq)]
        pub enum BaseAnyHolder {
            Base(Box<BaseHolder>),
            Sub(Box<SubAnyHolder>),
        }
        impl ::ruststep::tables::Holder for BaseAnyHolder {
            type Owned = BaseAny;
            type Table = Tables;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                Ok(match self {
                    BaseAnyHolder::Base(sub) => BaseAny::Base(Box::new(sub.into_owned(table)?)),
                    BaseAnyHolder::Sub(sub) => BaseAny::Sub(Box::new(sub.into_owned(table)?)),
                })
            }
            fn name() -> &'static str {
                "BASE_ANY"
            }
            fn attr_len() -> usize {
                0
            }
        }
        impl<'de> ::serde::de::Deserialize<'de> for BaseAnyHolder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("BASE_ANY", 0, BaseAnyHolderVisitor {})
            }
        }
        pub struct BaseAnyHolderVisitor;
        impl<'de> ::serde::de::Visitor<'de> for BaseAnyHolderVisitor {
            type Value = BaseAnyHolder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "BASE_ANY")
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                match key.as_str() {
                    "BASE" => {
                        let owned = map.next_value()?;
                        return Ok(BaseAnyHolder::Base(Box::new(owned.into())));
                    }
                    "SUB" => {
                        let owned = map.next_value()?;
                        return Ok(BaseAnyHolder::Sub(Box::new(owned.into())));
                    }
                    _ => {
                        use serde::de::{Error, Unexpected};
                        return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                    }
                }
            }
        }
        impl ::ruststep::tables::WithVisitor for BaseAnyHolder {
            type Visitor = BaseAnyHolderVisitor;
            fn visitor_new() -> Self::Visitor {
                BaseAnyHolderVisitor {}
            }
        }
        impl ::ruststep::tables::EntityTable<BaseAnyHolder> for Tables {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<BaseAny> {
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.base, entity_id) {
                    return Ok(BaseAny::Base(Box::new(owned.into())));
                }
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.sub, entity_id) {
                    return Ok(BaseAny::Sub(Box::new(owned.into())));
                }
                Err(::ruststep::error::Error::UnknownEntity(entity_id))
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<BaseAny>> + 'table> {
                Box::new(::itertools::chain![
                    ::ruststep::tables::owned_iter(self, &self.base)
                        .map(|owned| owned.map(|owned| BaseAny::Base(Box::new(owned.into())))),
                    ::ruststep::tables::owned_iter(self, &self.sub)
                        .map(|owned| owned.map(|owned| BaseAny::Sub(Box::new(owned.into()))))
                ])
            }
        }
        "###);
    }

    #[test]
    fn other_attributes() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            #[derive(
                Debug, Clone, PartialEq, AsRef, AsMut, Deref, DerefMut, :: derive_new :: new, Holder,
            )]
            # [holder (table = Tables)]
            # [holder (field = sub1)]
            #[holder(generate_deserialize)]
            pub struct Sub1 {
                #[as_ref]
                #[as_mut]
                #[deref]
                #[deref_mut]
                #[holder(use_place_holder)]
                pub base: Base,
                pub y1: f64,
            }
            "#,
        )
        .unwrap();

        let tt = crate::derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[doc = r" Auto-generated by `#[derive(Holder)]`"]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Sub1Holder {
            pub base: ::ruststep::place_holder::PlaceHolder<BaseHolder>,
            pub y1: f64,
        }
        #[automatically_derived]
        impl ::ruststep::tables::Holder for Sub1Holder {
            type Table = Tables;
            type Owned = Sub1;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                let Sub1Holder { base, y1 } = self;
                Ok(Sub1 {
                    base: base.into_owned(table)?,
                    y1: y1,
                })
            }
            fn name() -> &'static str {
                "SUB_1"
            }
            fn attr_len() -> usize {
                2usize
            }
        }
        #[automatically_derived]
        impl ::ruststep::tables::EntityTable<Sub1Holder> for Tables {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<Sub1> {
                ::ruststep::tables::get_owned(self, &self.sub1, entity_id)
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<Sub1>> + 'table> {
                ::ruststep::tables::owned_iter(self, &self.sub1)
            }
        }
        #[doc(hidden)]
        pub struct Sub1HolderVisitor;
        #[automatically_derived]
        impl<'de> ::serde::de::Visitor<'de> for Sub1HolderVisitor {
            type Value = Sub1Holder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "SUB_1")
            }
            fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::SeqAccess<'de>,
            {
                if let Some(size) = seq.size_hint() {
                    if size != 2usize {
                        use serde::de::Error;
                        return Err(A::Error::invalid_length(size, &self));
                    }
                }
                let base = seq.next_element()?.unwrap();
                let y1 = seq.next_element()?.unwrap();
                Ok(Sub1Holder { base, y1 })
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                if key != "SUB_1" {
                    use serde::de::{Error, Unexpected};
                    return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                }
                let value = map.next_value()?;
                Ok(value)
            }
        }
        #[automatically_derived]
        impl<'de> ::serde::de::Deserialize<'de> for Sub1Holder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("SUB_1", 2usize, Sub1HolderVisitor {})
            }
        }
        #[automatically_derived]
        impl ::ruststep::tables::WithVisitor for Sub1Holder {
            type Visitor = Sub1HolderVisitor;
            fn visitor_new() -> Self::Visitor {
                Sub1HolderVisitor {}
            }
        }
        "###);
    }
}
