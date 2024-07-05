extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(CsrDerive)]
pub fn csr_derive(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        impl Csr for Gpio {
            fn read_write(&self, addr: u16, val: u8) -> u32 {
                0
            }
        }
    )
    .into()
}
