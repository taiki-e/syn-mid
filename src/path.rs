// SPDX-License-Identifier: Apache-2.0 OR MIT

// Based on https://github.com/dtolnay/syn/blob/2.0.37/src/path.rs.

use syn::{
    Ident, Path, PathArguments, PathSegment, Token,
    ext::IdentExt as _,
    parse::{ParseStream, Result},
    punctuated::Punctuated,
};

fn parse_path_segment(input: ParseStream<'_>) -> Result<PathSegment> {
    if input.peek(Token![super]) || input.peek(Token![self]) || input.peek(Token![crate]) {
        let ident = input.call(Ident::parse_any)?;
        return Ok(PathSegment::from(ident));
    }

    let ident =
        if input.peek(Token![Self]) { input.call(Ident::parse_any)? } else { input.parse()? };

    if input.peek(Token![::]) && input.peek3(Token![<]) {
        Ok(PathSegment { ident, arguments: PathArguments::AngleBracketed(input.parse()?) })
    } else {
        Ok(PathSegment::from(ident))
    }
}

pub(crate) fn parse_path(input: ParseStream<'_>) -> Result<Path> {
    Ok(Path {
        leading_colon: input.parse()?,
        segments: {
            let mut segments = Punctuated::new();
            let value = parse_path_segment(input)?;
            segments.push_value(value);
            while input.peek(Token![::]) {
                let punct: Token![::] = input.parse()?;
                segments.push_punct(punct);
                let value = parse_path_segment(input)?;
                segments.push_value(value);
            }
            segments
        },
    })
}
