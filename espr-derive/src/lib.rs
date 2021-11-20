use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn inline_express(input: TokenStream) -> TokenStream {
    abort_call_site!("Not implemented")
}
