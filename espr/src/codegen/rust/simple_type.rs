use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use crate::ast::SimpleType::*;
        match self.0 {
            Number => tokens.append(format_ident!("f64")),
            Real => tokens.append(format_ident!("f64")),
            Integer => tokens.append(format_ident!("i64")),
            Logical => tokens.append_all(quote! { Logical }),
            Boolen => tokens.append(format_ident!("bool")),
            String_ { .. } => tokens.append(format_ident!("String")),
            Binary { .. } => unimplemented!("Binary type is not supported yet"),
        }
    }
}
