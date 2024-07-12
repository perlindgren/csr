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

    let instr_str = format!("csrrw {{0}}, {}, {{1}}", quote!(#arg1).to_string());
    eprintln!("instr_str {:?}", instr_str);

    let instr_str = format!(
        "core::arch::asm!({:?}, out(reg) r_out, in(reg) {});",
        instr_str,
        quote!(#arg2).to_string()
    );

    eprintln!("instr_str {:?}", instr_str);

    let instr_str = syn::parse_str::<Stmt>(&instr_str).unwrap();

    eprintln!("expr: instr_str {:?}", instr_str);

    // let instr_str = quote!(core::arch::asm!("csrrw {0}, 0x0, {1}", out(reg) r_out, in(reg) r););

    quote!({
        // comment
        let r_out: usize;
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
