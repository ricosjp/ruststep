use espr::{ast::SyntaxTree, ir::IR};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;

/// Compile and expand results inline EXPRESS
///
/// Example
/// --------
///
/// ```
/// espr_derive::inline_express!(r#"
/// SCHEMA explicit_draughting;
///   ENTITY a;
///     x: REAL;
///   END_ENTITY;
/// END_SCHEMA;
/// "#);
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn inline_express(input: TokenStream) -> TokenStream {
    let input: syn::LitStr =
        syn::parse(input).expect("inline_express! argument must be string literal");
    let st = SyntaxTree::parse(&input.value()).expect("Tokenize failed");
    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    ir.to_token_stream().into()
}
