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
//! * **`extra-traits`** — Debug, Eq, PartialEq, Hash impls for all syntax tree
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

mod expr;
mod item;
mod path;
mod print;
#[cfg(feature = "extra-traits")]
mod tt;

pub use self::expr::*;
pub use self::item::*;

use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{token, Abi, AttrStyle, Attribute, Ident, Visibility};

#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "extra-traits")]
use tt::TokenStreamHelper;

ast_struct! {
    /// A braced block containing Rust statements.
    pub struct Block #manual_extra_traits {
        pub brace_token: token::Brace,
        /// Statements in a block
        pub stmts: TokenStream,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for Block {}

#[cfg(feature = "extra-traits")]
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.brace_token == other.brace_token
            && TokenStreamHelper(&self.stmts) == TokenStreamHelper(&other.stmts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for Block {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.brace_token.hash(state);
        TokenStreamHelper(&self.stmts).hash(state);
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
        pub decl: Box<FnDecl>,
        pub block: Block,
    }
}

mod parsing {
    use syn::parse::{Parse, ParseStream, Result};
    use syn::{Abi, Attribute, Generics, Ident, ReturnType, Visibility, WhereClause};

    use super::{Block, FnArg, FnDecl, ItemFn};

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
                decl: Box::new(FnDecl {
                    fn_token: fn_token,
                    paren_token: paren_token,
                    inputs: inputs,
                    output: output,
                    variadic: None,
                    generics: Generics {
                        where_clause: where_clause,
                        ..generics
                    },
                }),
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
    use std::iter;

    use super::*;

    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.stmts.clone());
            });
        }
    }

    trait FilterAttrs<'a> {
        type Ret: Iterator<Item = &'a Attribute>;

        fn outer(self) -> Self::Ret;
        fn inner(self) -> Self::Ret;
    }

    impl<'a, T> FilterAttrs<'a> for T
    where
        T: IntoIterator<Item = &'a Attribute>,
    {
        type Ret = iter::Filter<T::IntoIter, fn(&&Attribute) -> bool>;

        fn outer(self) -> Self::Ret {
            #[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
            fn is_outer(attr: &&Attribute) -> bool {
                match attr.style {
                    AttrStyle::Outer => true,
                    _ => false,
                }
            }
            self.into_iter().filter(is_outer)
        }

        fn inner(self) -> Self::Ret {
            #[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
            fn is_inner(attr: &&Attribute) -> bool {
                match attr.style {
                    AttrStyle::Inner(_) => true,
                    _ => false,
                }
            }
            self.into_iter().filter(is_inner)
        }
    }

    impl ToTokens for ItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.constness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            NamedDecl(&self.decl, &self.ident).to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(self.block.stmts.clone());
            });
        }
    }

    struct NamedDecl<'a>(&'a FnDecl, &'a Ident);

    impl<'a> ToTokens for NamedDecl<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.fn_token.to_tokens(tokens);
            self.1.to_tokens(tokens);
            self.0.generics.to_tokens(tokens);
            self.0.paren_token.surround(tokens, |tokens| {
                self.0.inputs.to_tokens(tokens);
                if self.0.variadic.is_some() && !self.0.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.0.variadic.to_tokens(tokens);
            });
            self.0.output.to_tokens(tokens);
            self.0.generics.where_clause.to_tokens(tokens);
        }
    }
}
