// These codes copied from https://github.com/taiki-e/const_fn/blob/master/src/lib.rs

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate syn_mid;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote};
use syn_mid::ItemFn;

fn compile_err(msg: &str) -> TokenStream {
    quote!(compile_error!(#msg);).into()
}

#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    if args.is_empty() {
        return compile_err("requires an argument");
    }

    let mut function: ItemFn = parse_macro_input!(function);

    let mut const_function = function.clone();

    if function.constness.is_some() {
        function.constness = None;
    } else {
        const_function.constness = Some(Default::default());
    }

    let args = TokenStream2::from(args);
    function.attrs.push(parse_quote!(#[cfg(not(#args))]));
    const_function.attrs.push(parse_quote!(#[cfg(#args)]));

    let mut function = function.into_token_stream();
    function.extend(const_function.into_token_stream());
    function.into()
}
