#[cfg(not(feature = "full"))]
use syn::punctuated::Punctuated;
#[cfg(not(feature = "full"))]
use syn::{token, Generics, Lifetime, ReturnType, Type};

#[cfg(not(feature = "full"))]
use super::*;

#[cfg(feature = "full")]
pub use syn::FnDecl;
#[cfg(not(feature = "full"))]
ast_struct! {
    /// Header of a function declaration, without including the body.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FnDecl {
        pub fn_token: Token![fn],
        pub generics: Generics,
        pub paren_token: token::Paren,
        pub inputs: Punctuated<FnArg, Token![,]>,
        pub variadic: Option<Token![...]>,
        pub output: ReturnType,
    }
}

#[cfg(feature = "full")]
pub use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg};
#[cfg(not(feature = "full"))]
ast_enum_of_structs! {
    /// An argument in a function signature: the `n: usize` in `fn f(n: usize)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum FnArg {
        /// Self captured by reference in a function signature: `&self` or `&mut
        /// self`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub SelfRef(ArgSelfRef {
            pub and_token: Token![&],
            pub lifetime: Option<Lifetime>,
            pub mutability: Option<Token![mut]>,
            pub self_token: Token![self],
        }),

        /// Self captured by value in a function signature: `self` or `mut
        /// self`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub SelfValue(ArgSelf {
            pub mutability: Option<Token![mut]>,
            pub self_token: Token![self],
        }),

        /// An explicitly typed pattern captured by a function signature.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Captured(ArgCaptured {
            pub pat: Pat,
            pub colon_token: Token![:],
            pub ty: Type,
        }),

        /// A pattern whose type is inferred captured by a function signature.
        pub Inferred(Pat),
        /// A type not bound to any pattern in a function signature.
        pub Ignored(Type),
    }
}

#[cfg(not(feature = "full"))]
mod parsing {
    use syn::parse::{Parse, ParseStream, Result};

    use super::*;

    impl Parse for FnArg {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(Token![&]) {
                let ahead = input.fork();
                if ahead.call(arg_self_ref).is_ok() && !ahead.peek(Token![:]) {
                    return input.call(arg_self_ref).map(FnArg::SelfRef);
                }
            }

            if input.peek(Token![mut]) || input.peek(Token![self]) {
                let ahead = input.fork();
                if ahead.call(arg_self).is_ok() && !ahead.peek(Token![:]) {
                    return input.call(arg_self).map(FnArg::SelfValue);
                }
            }

            let ahead = input.fork();
            let err = match ahead.call(arg_captured) {
                Ok(_) => return input.call(arg_captured).map(FnArg::Captured),
                Err(err) => err,
            };

            let ahead = input.fork();
            if ahead.parse::<Type>().is_ok() {
                return input.parse().map(FnArg::Ignored);
            }

            Err(err)
        }
    }

    fn arg_self_ref(input: ParseStream) -> Result<ArgSelfRef> {
        Ok(ArgSelfRef {
            and_token: input.parse()?,
            lifetime: input.parse()?,
            mutability: input.parse()?,
            self_token: input.parse()?,
        })
    }

    fn arg_self(input: ParseStream) -> Result<ArgSelf> {
        Ok(ArgSelf {
            mutability: input.parse()?,
            self_token: input.parse()?,
        })
    }

    fn arg_captured(input: ParseStream) -> Result<ArgCaptured> {
        Ok(ArgCaptured {
            pat: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[cfg(not(feature = "full"))]
mod printing {
    use proc_macro2::TokenStream;
    use quote::ToTokens;

    use super::*;

    impl ToTokens for ArgSelfRef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.lifetime.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.self_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ArgSelf {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mutability.to_tokens(tokens);
            self.self_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ArgCaptured {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.pat.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
        }
    }
}
