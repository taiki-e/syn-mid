use syn::ext::IdentExt;
use syn::parse::{ParseStream, Result};
use syn::{Path, PathArguments, PathSegment};

use super::*;

trait ParseHelper: Sized {
    fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self>;
}

impl ParseHelper for PathSegment {
    fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self> {
        if input.peek(Token![super])
            || input.peek(Token![self])
            || input.peek(Token![Self])
            || input.peek(Token![crate])
            || input.peek(Token![extern])
        {
            let ident = input.call(Ident::parse_any)?;
            return Ok(PathSegment::from(ident));
        }

        let ident = input.parse()?;
        if !expr_style && input.peek(Token![<]) && !input.peek(Token![<=])
            || input.peek(Token![::]) && input.peek3(Token![<])
        {
            Ok(PathSegment {
                ident: ident,
                arguments: PathArguments::AngleBracketed(input.parse()?),
            })
        } else {
            Ok(PathSegment::from(ident))
        }
    }
}

impl ParseHelper for Path {
    fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self> {
        if input.peek(Token![dyn]) {
            return Err(input.error("expected path"));
        }

        Ok(Path {
            leading_colon: input.parse()?,
            segments: {
                let mut segments = Punctuated::new();
                let value = PathSegment::parse_helper(input, expr_style)?;
                segments.push_value(value);
                while input.peek(Token![::]) {
                    let punct: Token![::] = input.parse()?;
                    segments.push_punct(punct);
                    let value = PathSegment::parse_helper(input, expr_style)?;
                    segments.push_value(value);
                }
                segments
            },
        })
    }
}

pub fn path(input: ParseStream, expr_style: bool) -> Result<Path> {
    Path::parse_helper(input, expr_style)
}
