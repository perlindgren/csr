extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn read_write(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}
