#![warn(rust_2018_idioms)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;
use syn_mid::ItemFn;

// These codes copied from https://github.com/taiki-e/const_fn/blob/master/src/lib.rs

/// An attribute for easy generation of a const function with conditional compilations.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    assert!(!args.is_empty(), "requires an argument");

    let mut function: ItemFn = parse_macro_input!(function);
    let mut const_function = function.clone();

    if function.constness.is_some() {
        function.constness = None;
    } else {
        const_function.constness = Some(Default::default());
    }

    let args = TokenStream2::from(args);
    TokenStream::from(quote! {
        #[cfg(not(#args))]
        #function
        #[cfg(#args)]
        #const_function
    })
}
