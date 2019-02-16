//! Providing the features between "full" and "derive" of syn.
//!
//! This crate provides the following two unique data structures.
//!
//! * [`syn_mid::ItemFn`] -- A function whose body is not parsed.
//!
//!   ```text
//!   fn process(n: usize) -> Result<()> { ... }
//!   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^     ^
//!   ```
//!
//! * [`syn_mid::Block`] -- A block whose body is not parsed.
//!
//!   ```text
//!   { ... }
//!   ^     ^
//!   ```
//!
//! Other data structures are the same as data structures of [syn]. These are defined in this crate
//! because they cannot be used in [syn] without "full" feature.
//!
//! ## Optional features
//!
//! syn-mid in the default features aims to provide the features between "full"
//! and "derive" of [syn].
//!
//! * **`clone-impls`** *(enabled by default)* — Clone impls for all syntax tree
//!   types.
//!
//! [`syn_mid::ItemFn`]: struct.ItemFn.html
//! [`syn_mid::Block`]: struct.Block.html
//! [syn]: https://github.com/dtolnay/syn
//!

#![doc(html_root_url = "https://docs.rs/syn-mid/0.2.0")]
#![deny(unsafe_code)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        renamed_and_removed_lints,
        redundant_field_names, // Rust 1.17+ => remove
        const_static_lifetime, // Rust 1.17+ => remove
        deprecated_cfg_attr, // Rust 1.30+ => remove
        map_clone,
        large_enum_variant
    )
)]

// Many of the code contained in this crate are copies from https://github.com/dtolnay/syn.

extern crate proc_macro2;
extern crate quote;
#[allow(unused_imports)]
#[macro_use]
extern crate syn;

#[macro_use]
mod macros;

mod arg;
mod pat;
mod path;
mod print;

pub use self::arg::*;
pub use self::pat::*;

use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{token, Abi, Attribute, Generics, Ident, ReturnType, Visibility};

ast_struct! {
    /// A braced block containing Rust statements.
    pub struct Block {
        pub brace_token: token::Brace,
        /// Statements in a block
        pub stmts: TokenStream,
    }
}

ast_struct! {
    /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
    /// }`.
    pub struct ItemFn {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub constness: Option<Token![const]>,
        pub unsafety: Option<Token![unsafe]>,
        pub asyncness: Option<Token![async]>,
        pub abi: Option<Abi>,
        pub ident: Ident,
        pub fn_token: Token![fn],
        pub generics: Generics,
        pub paren_token: token::Paren,
        pub inputs: Punctuated<FnArg, Token![,]>,
        pub variadic: Option<Token![...]>,
        pub output: ReturnType,
        pub block: Block,
    }
}

mod parsing {
    use syn::parse::{Parse, ParseStream, Result};
    use syn::{Abi, Attribute, Generics, Ident, ReturnType, Visibility, WhereClause};

    use super::{Block, FnArg, ItemFn};

    impl Parse for ItemFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(FnArg::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let stmts = content.parse()?;

            Ok(ItemFn {
                attrs: outer_attrs,
                vis: vis,
                constness: constness,
                unsafety: unsafety,
                asyncness: asyncness,
                abi: abi,
                ident: ident,
                fn_token: fn_token,
                paren_token: paren_token,
                inputs: inputs,
                output: output,
                variadic: None,
                generics: Generics {
                    where_clause: where_clause,
                    ..generics
                },
                block: Block {
                    brace_token: brace_token,
                    stmts: stmts,
                },
            })
        }
    }
}

mod printing {
    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};

    use super::*;

    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.stmts.clone());
            });
        }
    }

    impl ToTokens for ItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(&self.attrs);
            self.vis.to_tokens(tokens);
            self.constness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            self.fn_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.inputs.to_tokens(tokens);
                if self.variadic.is_some() && !self.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.variadic.to_tokens(tokens);
            });
            self.output.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.block.stmts.clone());
            });
        }
    }
}
