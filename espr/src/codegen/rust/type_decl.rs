use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

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
