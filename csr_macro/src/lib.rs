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

use syn::punctuated::{self, Punctuated};

// use proc_macro::TokenStream;
use syn::parse::Parser;
// use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, parse_quote, token, Expr, Field, Ident, Result, Stmt, Token};
use syn::{Attribute, PathSegment};

// CSRRW
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRW(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    // let ts: proc_macro::TokenStream = input.clone();

    let punctuated = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(input)
        .unwrap();

    eprintln!("punct:\n {:?}", punctuated);

    let mut p = punctuated.iter();

    let arg1 = p.next().unwrap();
    eprintln!("arg1 {:?}", arg1);

    let arg2 = p.next().unwrap();
    eprintln!("arg2 {:?}", arg2);

    let instr_str = format!("csrrw {{0}}, {}, x0", quote!(#arg1).to_string());
    eprintln!("instr_str {:?}", instr_str);

    let instr_str = format!(
        "core::arch::asm!({:?}, out(reg) r_out, in(reg) r_in);",
        instr_str
    );

    eprintln!("instr_str {:?}", instr_str);

    let instr_str = syn::parse_str::<Stmt>(&instr_str).unwrap();

    eprintln!("expr: instr_str {:?}", instr_str);

    // let instr_str = quote!(core::arch::asm!("csrrs {0}, 0x0, x0", out(reg) r_out, in(reg) r_in););

    quote!({
        // comment
        let r_out: usize;
        let r_in: usize = 0;
        // core::arch::asm!("csrrs {0}, 0x0, x0", out(reg) r_out, in(reg) r_in);
        // core::arch::asm!(#instr_str, out(reg) r_out, in(reg) r_in);
        #instr_str
        r_out
    })
    .into()
}

// CSRRS
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRS(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}

// CSRRC
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRC(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}

// CSRRWI
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRWI(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}

// CSRRSI
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRSI(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}

// CSRRCI
#[proc_macro]
#[allow(non_snake_case)]
pub fn CSRRCI(input: TokenStream) -> TokenStream {
    eprintln!("input:\n {:?}", input);
    quote!(
        // comment
        42
    )
    .into()
}
