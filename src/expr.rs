use syn::punctuated::Punctuated;
use syn::{token, Attribute, Ident, Member, Path};

ast_enum_of_structs! {
    /// A pattern in a local binding, function signature, match expression, or
    /// various other places.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Pat {
        /// A pattern that matches any value: `_`.
        pub Wild(PatWild {
            pub underscore_token: Token![_],
        }),

        /// A pattern that binds a new variable: `ref mut binding`.
        pub Ident(PatIdent {
            pub by_ref: Option<Token![ref]>,
            pub mutability: Option<Token![mut]>,
            pub ident: Ident,
        }),

        /// A struct or struct variant pattern: `Variant { x, y, .. }`.
        pub Struct(PatStruct {
            pub path: Path,
            pub brace_token: token::Brace,
            pub fields: Punctuated<FieldPat, Token![,]>,
            pub dot2_token: Option<Token![..]>,
        }),

        /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
        pub TupleStruct(PatTupleStruct {
            pub path: Path,
            pub pat: PatTuple,
        }),

        /// A path pattern like `Color::Red`.
        pub Path(PatPath {
            pub path: Path,
        }),

        /// A tuple pattern: `(a, b)`.
        pub Tuple(PatTuple {
            pub paren_token: token::Paren,
            pub front: Punctuated<Pat, Token![,]>,
            pub dot2_token: Option<Token![..]>,
            pub comma_token: Option<Token![,]>,
            pub back: Punctuated<Pat, Token![,]>,
        }),

        /// A reference pattern: `&mut (first, second)`.
        pub Ref(PatRef {
            pub and_token: Token![&],
            pub mutability: Option<Token![mut]>,
            pub pat: Box<Pat>,
        }),

        /// A dynamically sized slice pattern: `[a, b, i.., y, z]`.
        pub Slice(PatSlice {
            pub bracket_token: token::Bracket,
            pub front: Punctuated<Pat, Token![,]>,
            pub middle: Option<Box<Pat>>,
            pub dot2_token: Option<Token![..]>,
            pub comma_token: Option<Token![,]>,
            pub back: Punctuated<Pat, Token![,]>,
        }),
    }
}

ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    pub struct FieldPat {
        pub attrs: Vec<Attribute>,
        pub member: Member,
        pub colon_token: Option<Token![:]>,
        pub pat: Box<Pat>,
    }
}

mod parsing {
    use syn::ext::IdentExt;
    use syn::parse::{Parse, ParseStream, Result};
    use syn::{token, Ident, Member, Path};

    use path;

    use super::*;

    impl Parse for Pat {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![_]) {
                input.call(pat_wild).map(Pat::Wild)
            } else if input.peek(Ident)
                && ({
                    input.peek2(Token![::])
                        || input.peek2(Token![!])
                        || input.peek2(token::Brace)
                        || input.peek2(token::Paren)
                })
                || input.peek(Token![self]) && input.peek2(Token![::])
                || input.peek(Token![::])
                || input.peek(Token![<])
                || input.peek(Token![Self])
                || input.peek(Token![super])
                || input.peek(Token![extern])
                || input.peek(Token![crate])
            {
                pat_path_or_struct(input)
            } else if input.peek(Token![ref])
                || input.peek(Token![mut])
                || input.peek(Token![self])
                || input.peek(Ident)
            {
                input.call(pat_ident).map(Pat::Ident)
            } else if lookahead.peek(token::Paren) {
                input.call(pat_tuple).map(Pat::Tuple)
            } else if lookahead.peek(Token![&]) {
                input.call(pat_ref).map(Pat::Ref)
            } else if lookahead.peek(token::Bracket) {
                input.call(pat_slice).map(Pat::Slice)
            } else {
                Err(lookahead.error())
            }
        }
    }

    fn pat_path_or_struct(input: ParseStream) -> Result<Pat> {
        let path = path::path(input, true)?;

        if input.peek(token::Brace) {
            pat_struct(input, path).map(Pat::Struct)
        } else if input.peek(token::Paren) {
            pat_tuple_struct(input, path).map(Pat::TupleStruct)
        } else {
            Ok(Pat::Path(PatPath { path: path }))
        }
    }

    fn pat_wild(input: ParseStream) -> Result<PatWild> {
        Ok(PatWild {
            underscore_token: input.parse()?,
        })
    }

    fn pat_ident(input: ParseStream) -> Result<PatIdent> {
        Ok(PatIdent {
            by_ref: input.parse()?,
            mutability: input.parse()?,
            ident: input.call(Ident::parse_any)?,
        })
    }

    fn pat_tuple_struct(input: ParseStream, path: Path) -> Result<PatTupleStruct> {
        Ok(PatTupleStruct {
            path: path,
            pat: input.call(pat_tuple)?,
        })
    }

    fn pat_struct(input: ParseStream, path: Path) -> Result<PatStruct> {
        let content;
        let brace_token = braced!(content in input);

        let mut fields = Punctuated::new();
        while !content.is_empty() && !content.peek(Token![..]) {
            let value = content.call(field_pat)?;
            fields.push_value(value);
            if !content.peek(Token![,]) {
                break;
            }
            let punct: Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        let dot2_token = if fields.empty_or_trailing() && content.peek(Token![..]) {
            Some(content.parse()?)
        } else {
            None
        };

        Ok(PatStruct {
            path: path,
            brace_token: brace_token,
            fields: fields,
            dot2_token: dot2_token,
        })
    }

    fn field_pat(input: ParseStream) -> Result<FieldPat> {
        let boxed: Option<Token![box]> = input.parse()?;
        let by_ref: Option<Token![ref]> = input.parse()?;
        let mutability: Option<Token![mut]> = input.parse()?;
        let member: Member = input.parse()?;

        if boxed.is_none() && by_ref.is_none() && mutability.is_none() && input.peek(Token![:])
            || is_unnamed(&member)
        {
            return Ok(FieldPat {
                attrs: Vec::new(),
                member: member,
                colon_token: input.parse()?,
                pat: input.parse()?,
            });
        }

        let ident = match member {
            Member::Named(ident) => ident,
            Member::Unnamed(_) => unreachable!(),
        };

        let pat = Pat::Ident(PatIdent {
            by_ref: by_ref,
            mutability: mutability,
            ident: ident.clone(),
        });

        Ok(FieldPat {
            member: Member::Named(ident),
            pat: Box::new(pat),
            attrs: Vec::new(),
            colon_token: None,
        })
    }

    fn pat_tuple(input: ParseStream) -> Result<PatTuple> {
        let content;
        let paren_token = parenthesized!(content in input);

        let mut front = Punctuated::new();
        let mut dot2_token = None::<Token![..]>;
        let mut comma_token = None::<Token![,]>;
        loop {
            if content.is_empty() {
                break;
            }
            if content.peek(Token![..]) {
                dot2_token = Some(content.parse()?);
                comma_token = content.parse()?;
                break;
            }
            let value: Pat = content.parse()?;
            front.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            front.push_punct(punct);
        }

        let mut back = Punctuated::new();
        while !content.is_empty() {
            let value: Pat = content.parse()?;
            back.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            back.push_punct(punct);
        }

        Ok(PatTuple {
            paren_token: paren_token,
            front: front,
            dot2_token: dot2_token,
            comma_token: comma_token,
            back: back,
        })
    }

    fn pat_ref(input: ParseStream) -> Result<PatRef> {
        Ok(PatRef {
            and_token: input.parse()?,
            mutability: input.parse()?,
            pat: input.parse()?,
        })
    }

    fn pat_slice(input: ParseStream) -> Result<PatSlice> {
        let content;
        let bracket_token = bracketed!(content in input);

        let mut front = Punctuated::new();
        let mut middle = None;
        loop {
            if content.is_empty() || content.peek(Token![..]) {
                break;
            }
            let value: Pat = content.parse()?;
            if content.peek(Token![..]) {
                middle = Some(Box::new(value));
                break;
            }
            front.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            front.push_punct(punct);
        }

        let dot2_token: Option<Token![..]> = content.parse()?;
        let mut comma_token = None::<Token![,]>;
        let mut back = Punctuated::new();
        if dot2_token.is_some() {
            comma_token = content.parse()?;
            if comma_token.is_some() {
                loop {
                    if content.is_empty() {
                        break;
                    }
                    let value: Pat = content.parse()?;
                    back.push_value(value);
                    if content.is_empty() {
                        break;
                    }
                    let punct = content.parse()?;
                    back.push_punct(punct);
                }
            }
        }

        Ok(PatSlice {
            bracket_token: bracket_token,
            front: front,
            middle: middle,
            dot2_token: dot2_token,
            comma_token: comma_token,
            back: back,
        })
    }

    fn is_unnamed(member: &Member) -> bool {
        match *member {
            Member::Named(_) => false,
            Member::Unnamed(_) => true,
        }
    }

}

mod printing {
    use proc_macro2::TokenStream;
    use quote::ToTokens;

    use print::TokensOrDefault;

    use super::*;

    impl ToTokens for PatWild {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.underscore_token.to_tokens(tokens);
        }
    }

    impl ToTokens for PatIdent {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.by_ref.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
        }
    }

    impl ToTokens for PatStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.path.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                self.fields.to_tokens(tokens);
                // NOTE: We need a comma before the dot2 token if it is present.
                if !self.fields.empty_or_trailing() && self.dot2_token.is_some() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.dot2_token.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for PatTupleStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.path.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatPath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.path.to_tokens(tokens)
        }
    }

    impl ToTokens for PatTuple {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.paren_token.surround(tokens, |tokens| {
                self.front.to_tokens(tokens);
                if let Some(ref dot2_token) = self.dot2_token {
                    if !self.front.empty_or_trailing() {
                        // Ensure there is a comma before the .. token.
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    dot2_token.to_tokens(tokens);
                    self.comma_token.to_tokens(tokens);
                    if self.comma_token.is_none() && !self.back.is_empty() {
                        // Ensure there is a comma after the .. token.
                        <Token![,]>::default().to_tokens(tokens);
                    }
                }
                self.back.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for PatRef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatSlice {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.bracket_token.surround(tokens, |tokens| {
                self.front.to_tokens(tokens);

                // If we need a comma before the middle or standalone .. token,
                // then make sure it's present.
                if !self.front.empty_or_trailing()
                    && (self.middle.is_some() || self.dot2_token.is_some())
                {
                    <Token![,]>::default().to_tokens(tokens);
                }

                // If we have an identifier, we always need a .. token.
                if self.middle.is_some() {
                    self.middle.to_tokens(tokens);
                    TokensOrDefault(&self.dot2_token).to_tokens(tokens);
                } else if self.dot2_token.is_some() {
                    self.dot2_token.to_tokens(tokens);
                }

                // Make sure we have a comma before the back half.
                if !self.back.is_empty() {
                    TokensOrDefault(&self.comma_token).to_tokens(tokens);
                    self.back.to_tokens(tokens);
                } else {
                    self.comma_token.to_tokens(tokens);
                }
            })
        }
    }

    impl ToTokens for FieldPat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if let Some(ref colon_token) = self.colon_token {
                self.member.to_tokens(tokens);
                colon_token.to_tokens(tokens);
            }
            self.pat.to_tokens(tokens);
        }
    }
}
