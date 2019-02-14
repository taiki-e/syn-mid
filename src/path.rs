use syn::ext::IdentExt;
use syn::parse::{ParseStream, Result};
use syn::{Path, PathArguments, PathSegment, QSelf, Type};

use super::*;

pub trait ParseHelper: Sized {
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

pub fn qpath(input: ParseStream, expr_style: bool) -> Result<(Option<QSelf>, Path)> {
    if input.peek(Token![<]) {
        let lt_token: Token![<] = input.parse()?;
        let this: Type = input.parse()?;
        let path = if input.peek(Token![as]) {
            let as_token: Token![as] = input.parse()?;
            let path: Path = input.parse()?;
            Some((as_token, path))
        } else {
            None
        };
        let gt_token: Token![>] = input.parse()?;
        let colon2_token: Token![::] = input.parse()?;
        let mut rest = Punctuated::new();
        loop {
            let path = PathSegment::parse_helper(input, expr_style)?;
            rest.push_value(path);
            if !input.peek(Token![::]) {
                break;
            }
            let punct: Token![::] = input.parse()?;
            rest.push_punct(punct);
        }
        let (position, as_token, path) = match path {
            Some((as_token, mut path)) => {
                let pos = path.segments.len();
                path.segments.push_punct(colon2_token);
                path.segments.extend(rest.into_pairs());
                (pos, Some(as_token), path)
            }
            None => {
                let path = Path {
                    leading_colon: Some(colon2_token),
                    segments: rest,
                };
                (0, None, path)
            }
        };
        let qself = QSelf {
            lt_token: lt_token,
            ty: Box::new(this),
            position: position,
            as_token: as_token,
            gt_token: gt_token,
        };
        Ok((Some(qself), path))
    } else {
        let path = Path::parse_helper(input, expr_style)?;
        Ok((None, path))
    }
}
