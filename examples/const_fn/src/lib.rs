// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Error;
use syn_mid::ItemFn;

/// An attribute for easy generation of a const function with conditional compilations.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    if args.is_empty() {
        return Error::new(Span::call_site(), "#[const_fn] attribute requires an argument")
            .to_compile_error()
            .into();
    }

    let const_function: ItemFn = syn::parse_macro_input!(function);

    if const_function.sig.constness.is_none() {
        return Error::new_spanned(
            const_function.sig.fn_token,
            "#[const_fn] attribute may only be used on const functions",
        )
        .to_compile_error()
        .into();
    }

    let mut function = const_function.clone();
    function.sig.constness = None;

    let args = TokenStream2::from(args);
    TokenStream::from(quote! {
        #[cfg(not(#args))]
        #function
        #[cfg(#args)]
        #const_function
    })
}
