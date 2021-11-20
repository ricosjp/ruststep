use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};
use proc_macro::TokenStream;

/// Compile inline EXPRESS into Rust code, and expand it on the call site.
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
#[proc_macro]
pub fn inline_express(input: TokenStream) -> TokenStream {
    // FIXME Use proc-macro-error
    //
    // espr::Result does not match its requirement currently. We have to fix it.
    //
    let input: syn::LitStr =
        syn::parse(input).expect("inline_express! argument must be string literal");
    let st = SyntaxTree::parse(&input.value()).expect("Tokenize failed");
    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    ir.to_token_stream(CratePrefix::External).into()
}
