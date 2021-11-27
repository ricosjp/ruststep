use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for Simple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, ::serde::Serialize, ::serde::Deserialize)]
            pub struct #id(pub #ty);
        });
    }
}
