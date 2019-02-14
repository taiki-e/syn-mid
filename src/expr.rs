#[cfg(not(feature = "full"))]
use proc_macro2::TokenStream;
#[cfg(not(feature = "full"))]
use syn::punctuated::Punctuated;
#[cfg(not(feature = "full"))]
use syn::{token, Attribute, Expr, Ident, Macro, Member, Path, QSelf};

#[cfg(all(not(feature = "full"), feature = "extra-traits"))]
use std::hash::{Hash, Hasher};
#[cfg(all(not(feature = "full"), feature = "extra-traits"))]
use tt::TokenStreamHelper;

#[cfg(feature = "full")]
pub use syn::{
    Pat, PatIdent, PatLit, PatMacro, PatPath, PatRange, PatRef, PatSlice, PatStruct, PatTuple,
    PatTupleStruct, PatVerbatim, PatWild,
};
#[cfg(not(feature = "full"))]
ast_enum_of_structs! {
    /// A pattern in a local binding, function signature, match expression, or
    /// various other places.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Pat {
        /// A pattern that matches any value: `_`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Wild(PatWild {
            pub underscore_token: Token![_],
        }),

        /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Ident(PatIdent {
            pub by_ref: Option<Token![ref]>,
            pub mutability: Option<Token![mut]>,
            pub ident: Ident,
            pub subpat: Option<(Token![@], Box<Pat>)>,
        }),

        /// A struct or struct variant pattern: `Variant { x, y, .. }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Struct(PatStruct {
            pub path: Path,
            pub brace_token: token::Brace,
            pub fields: Punctuated<FieldPat, Token![,]>,
            pub dot2_token: Option<Token![..]>,
        }),

        /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub TupleStruct(PatTupleStruct {
            pub path: Path,
            pub pat: PatTuple,
        }),

        /// A path pattern like `Color::Red`, optionally qualified with a
        /// self-type.
        ///
        /// Unqualified path patterns can legally refer to variants, structs,
        /// constants or associated constants. Qualified path patterns like
        /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
        /// associated constants.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Path(PatPath {
            pub qself: Option<QSelf>,
            pub path: Path,
        }),

        /// A tuple pattern: `(a, b)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Tuple(PatTuple {
            pub paren_token: token::Paren,
            pub front: Punctuated<Pat, Token![,]>,
            pub dot2_token: Option<Token![..]>,
            pub comma_token: Option<Token![,]>,
            pub back: Punctuated<Pat, Token![,]>,
        }),

        /// A box pattern: `box v`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Box(PatBox {
            pub box_token: Token![box],
            pub pat: Box<Pat>,
        }),

        /// A reference pattern: `&mut (first, second)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Ref(PatRef {
            pub and_token: Token![&],
            pub mutability: Option<Token![mut]>,
            pub pat: Box<Pat>,
        }),

        /// A literal pattern: `0`.
        ///
        /// This holds an `Expr` rather than a `Lit` because negative numbers
        /// are represented as an `Expr::Unary`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Lit(PatLit {
            pub expr: Box<Expr>,
        }),

        /// A range pattern: `1..=2`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Range(PatRange {
            pub lo: Box<Expr>,
            pub limits: RangeLimits,
            pub hi: Box<Expr>,
        }),

        /// A dynamically sized slice pattern: `[a, b, i.., y, z]`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Slice(PatSlice {
            pub bracket_token: token::Bracket,
            pub front: Punctuated<Pat, Token![,]>,
            pub middle: Option<Box<Pat>>,
            pub dot2_token: Option<Token![..]>,
            pub comma_token: Option<Token![,]>,
            pub back: Punctuated<Pat, Token![,]>,
        }),

        /// A macro in expression position.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(PatMacro {
            pub mac: Macro,
        }),

        /// Tokens in pattern position not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(PatVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(all(not(feature = "full"), feature = "extra-traits"))]
impl Eq for PatVerbatim {}

#[cfg(all(not(feature = "full"), feature = "extra-traits"))]
impl PartialEq for PatVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(all(not(feature = "full"), feature = "extra-traits"))]
impl Hash for PatVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

#[cfg(feature = "full")]
pub use syn::RangeLimits;
#[cfg(not(feature = "full"))]
ast_enum! {
    /// Limit types of a range, inclusive or exclusive.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum RangeLimits {
        /// Inclusive at the beginning, exclusive at the end.
        HalfOpen(Token![..]),
        /// Inclusive at the beginning and end.
        Closed(Token![..=]),
    }
}

#[cfg(feature = "full")]
pub use syn::FieldPat;
#[cfg(not(feature = "full"))]
ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FieldPat {
        pub attrs: Vec<Attribute>,
        pub member: Member,
        pub colon_token: Option<Token![:]>,
        pub pat: Box<Pat>,
    }
}

#[cfg(not(feature = "full"))]
mod parsing {
    use proc_macro2::TokenStream;
    use proc_macro2::{Delimiter, TokenTree};
    use syn::ext::IdentExt;
    use syn::parse::{Parse, ParseStream, Result};
    use syn::token::{Brace, Bracket, Paren};
    use syn::*;

    use path;

    use super::*;

    impl Parse for RangeLimits {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![..=]) {
                input.parse().map(RangeLimits::Closed)
            } else if lookahead.peek(Token![...]) {
                let dot3: Token![...] = input.parse()?;
                Ok(RangeLimits::Closed(Token![..=](dot3.spans)))
            } else if lookahead.peek(Token![..]) {
                input.parse().map(RangeLimits::HalfOpen)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for Pat {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![_]) {
                input.call(pat_wild).map(Pat::Wild)
            } else if lookahead.peek(Token![box]) {
                input.call(pat_box).map(Pat::Box)
            } else if lookahead.peek(Token![-]) || lookahead.peek(Lit) {
                pat_lit_or_range(input)
            } else if input.peek(Ident)
                && ({
                    input.peek2(Token![::])
                        || input.peek2(Token![!])
                        || input.peek2(token::Brace)
                        || input.peek2(token::Paren)
                        || input.peek2(Token![..])
                            && !{
                                let ahead = input.fork();
                                ahead.parse::<Ident>()?;
                                ahead.parse::<RangeLimits>()?;
                                ahead.is_empty() || ahead.peek(Token![,])
                            }
                })
                || input.peek(Token![self]) && input.peek2(Token![::])
                || input.peek(Token![::])
                || input.peek(Token![<])
                || input.peek(Token![Self])
                || input.peek(Token![super])
                || input.peek(Token![extern])
                || input.peek(Token![crate])
            {
                pat_path_or_macro_or_struct_or_range(input)
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

    fn parse_delimiter(input: ParseStream) -> Result<(MacroDelimiter, TokenStream)> {
        input.step(|cursor| {
            if let Some((TokenTree::Group(g), rest)) = cursor.token_tree() {
                let span = g.span();
                let delimiter = match g.delimiter() {
                    Delimiter::Parenthesis => MacroDelimiter::Paren(Paren(span)),
                    Delimiter::Brace => MacroDelimiter::Brace(Brace(span)),
                    Delimiter::Bracket => MacroDelimiter::Bracket(Bracket(span)),
                    Delimiter::None => {
                        return Err(cursor.error("expected delimiter"));
                    }
                };
                Ok(((delimiter, g.stream().clone()), rest))
            } else {
                Err(cursor.error("expected delimiter"))
            }
        })
    }

    fn pat_path_or_macro_or_struct_or_range(input: ParseStream) -> Result<Pat> {
        let (qself, path) = path::qpath(input, true)?;

        if input.peek(Token![..]) {
            return pat_range(input, qself, path).map(Pat::Range);
        }

        if qself.is_some() {
            return Ok(Pat::Path(PatPath {
                qself: qself,
                path: path,
            }));
        }

        if input.peek(Token![!]) && !input.peek(Token![!=]) {
            let mut contains_arguments = false;
            for segment in &path.segments {
                match segment.arguments {
                    PathArguments::None => {}
                    PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                        contains_arguments = true;
                    }
                }
            }

            if !contains_arguments {
                let bang_token: Token![!] = input.parse()?;
                let (delimiter, tts) = parse_delimiter(input)?;
                return Ok(Pat::Macro(PatMacro {
                    mac: Macro {
                        path: path,
                        bang_token: bang_token,
                        delimiter: delimiter,
                        tts: tts,
                    },
                }));
            }
        }

        if input.peek(token::Brace) {
            pat_struct(input, path).map(Pat::Struct)
        } else if input.peek(token::Paren) {
            pat_tuple_struct(input, path).map(Pat::TupleStruct)
        } else if input.peek(Token![..]) {
            pat_range(input, qself, path).map(Pat::Range)
        } else {
            Ok(Pat::Path(PatPath {
                qself: qself,
                path: path,
            }))
        }
    }

    fn pat_wild(input: ParseStream) -> Result<PatWild> {
        Ok(PatWild {
            underscore_token: input.parse()?,
        })
    }

    fn pat_box(input: ParseStream) -> Result<PatBox> {
        Ok(PatBox {
            box_token: input.parse()?,
            pat: input.parse()?,
        })
    }

    fn pat_ident(input: ParseStream) -> Result<PatIdent> {
        Ok(PatIdent {
            by_ref: input.parse()?,
            mutability: input.parse()?,
            ident: input.call(Ident::parse_any)?,
            subpat: {
                if input.peek(Token![@]) {
                    let at_token: Token![@] = input.parse()?;
                    let subpat: Pat = input.parse()?;
                    Some((at_token, Box::new(subpat)))
                } else {
                    None
                }
            },
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

        let mut pat = Pat::Ident(PatIdent {
            by_ref: by_ref,
            mutability: mutability,
            ident: ident.clone(),
            subpat: None,
        });

        if let Some(boxed) = boxed {
            pat = Pat::Box(PatBox {
                pat: Box::new(pat),
                box_token: boxed,
            });
        }

        Ok(FieldPat {
            member: Member::Named(ident),
            pat: Box::new(pat),
            attrs: Vec::new(),
            colon_token: None,
        })
    }

    fn pat_range(input: ParseStream, qself: Option<QSelf>, path: Path) -> Result<PatRange> {
        Ok(PatRange {
            lo: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: qself,
                path: path,
            })),
            limits: input.parse()?,
            hi: input.call(pat_lit_expr)?,
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

    fn pat_lit_or_range(input: ParseStream) -> Result<Pat> {
        let lo = input.call(pat_lit_expr)?;
        if input.peek(Token![..]) {
            Ok(Pat::Range(PatRange {
                lo: lo,
                limits: input.parse()?,
                hi: input.call(pat_lit_expr)?,
            }))
        } else {
            Ok(Pat::Lit(PatLit { expr: lo }))
        }
    }

    fn expr_lit(input: ParseStream) -> Result<ExprLit> {
        Ok(ExprLit {
            attrs: Vec::new(),
            lit: input.parse()?,
        })
    }

    fn pat_lit_expr(input: ParseStream) -> Result<Box<Expr>> {
        let neg: Option<Token![-]> = input.parse()?;

        let lookahead = input.lookahead1();
        let expr = if lookahead.peek(Lit) {
            Expr::Lit(input.call(expr_lit)?)
        } else if lookahead.peek(Ident)
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![<])
            || lookahead.peek(Token![self])
            || lookahead.peek(Token![Self])
            || lookahead.peek(Token![super])
            || lookahead.peek(Token![extern])
            || lookahead.peek(Token![crate])
        {
            Expr::Path(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        Ok(Box::new(if let Some(neg) = neg {
            Expr::Unary(ExprUnary {
                attrs: Vec::new(),
                op: UnOp::Neg(neg),
                expr: Box::new(expr),
            })
        } else {
            expr
        }))
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

#[cfg(not(feature = "full"))]
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
            if let Some((ref at_token, ref subpat)) = self.subpat {
                at_token.to_tokens(tokens);
                subpat.to_tokens(tokens);
            }
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

    fn print_path(tokens: &mut TokenStream, qself: &Option<QSelf>, path: &Path) {
        let qself = match *qself {
            Some(ref qself) => qself,
            None => {
                path.to_tokens(tokens);
                return;
            }
        };
        qself.lt_token.to_tokens(tokens);
        qself.ty.to_tokens(tokens);

        let pos = if qself.position > 0 && qself.position >= path.segments.len() {
            path.segments.len() - 1
        } else {
            qself.position
        };
        let mut segments = path.segments.pairs();
        if pos > 0 {
            TokensOrDefault(&qself.as_token).to_tokens(tokens);
            path.leading_colon.to_tokens(tokens);
            for (i, segment) in segments.by_ref().take(pos).enumerate() {
                if i + 1 == pos {
                    segment.value().to_tokens(tokens);
                    qself.gt_token.to_tokens(tokens);
                    segment.punct().to_tokens(tokens);
                } else {
                    segment.to_tokens(tokens);
                }
            }
        } else {
            qself.gt_token.to_tokens(tokens);
            path.leading_colon.to_tokens(tokens);
        }
        for segment in segments {
            segment.to_tokens(tokens);
        }
    }

    impl ToTokens for PatPath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            print_path(tokens, &self.qself, &self.path);
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

    impl ToTokens for PatBox {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.box_token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatRef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatLit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.expr.to_tokens(tokens);
        }
    }

    impl ToTokens for PatRange {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.lo.to_tokens(tokens);
            match self.limits {
                RangeLimits::HalfOpen(ref t) => t.to_tokens(tokens),
                RangeLimits::Closed(ref t) => Token![...](t.spans).to_tokens(tokens),
            }
            self.hi.to_tokens(tokens);
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

    impl ToTokens for PatMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mac.to_tokens(tokens);
        }
    }

    impl ToTokens for PatVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
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
