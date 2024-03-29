use std::str::FromStr;

use lang_util::{SmolStr, TextSize};

use glsl_lang_types::ast;

use super::{LexerPosition, LexicalError, PreprocessorToken, Token};

pub fn parse_int(lex: &mut logos::Lexer<Token>, radix: u32) -> Result<i32, LexicalError> {
    let mut slice = lex.slice();
    let fb = slice.bytes().next();
    let sgn = if fb == Some(b'-') {
        slice = &slice[1..];
        0xFFFFFFFFu32
    } else if fb == Some(b'+') {
        slice = &slice[1..];
        1u32
    } else {
        1u32
    };

    Ok(sgn.wrapping_mul(
        u32::from_str_radix(
            &slice[match radix {
                16 => 2,
                _ => 0,
            }..],
            radix,
        )
        .map_err(|source| LexicalError::InvalidIntLiteral {
            location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
            source,
            length: TextSize::of(slice),
        })?,
    ) as i32)
}

pub fn parse_uint(lex: &mut logos::Lexer<Token>, radix: u32) -> Result<u32, LexicalError> {
    let mut slice = lex.slice();
    let fb = slice.bytes().next();
    let sgn = if fb == Some(b'-') {
        slice = &slice[1..];
        0xFFFFFFFFu32
    } else if fb == Some(b'+') {
        slice = &slice[1..];
        1u32
    } else {
        1u32
    };

    Ok(sgn.wrapping_mul(
        u32::from_str_radix(
            &slice[match radix {
                16 => 2,
                _ => 0,
            }..slice.len() - 1],
            radix,
        )
        .map_err(|source| LexicalError::InvalidIntLiteral {
            location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
            source,
            length: TextSize::of(slice),
        })?,
    ))
}

pub fn parse_f32(lex: &mut logos::Lexer<Token>) -> Result<f32, LexicalError> {
    let s = lex.slice();
    f32::from_str(s.strip_suffix(|c| c == 'f' || c == 'F').unwrap_or(s)).map_err(|source| {
        LexicalError::InvalidFloatLiteral {
            location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
            source,
            length: TextSize::of(s),
        }
    })
}

pub fn parse_f64(lex: &mut logos::Lexer<Token>) -> Result<f64, LexicalError> {
    let s = lex.slice();
    f64::from_str(
        s.strip_suffix(|c| c == 'f' || c == 'F')
            .and_then(|s| s.strip_suffix(|c| c == 'l' || c == 'L'))
            .unwrap_or(s),
    )
    .map_err(|source| LexicalError::InvalidFloatLiteral {
        location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
        source,
        length: TextSize::of(s),
    })
}

pub fn parse_pp_int<'i>(
    lex: &mut logos::Lexer<'i, PreprocessorToken<'i>>,
) -> Result<i32, LexicalError> {
    let s = lex.slice();
    i32::from_str(s).map_err(|source| LexicalError::InvalidIntLiteral {
        location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
        source,
        length: TextSize::of(s),
    })
}

pub fn parse_pp_path<'i>(lex: &mut logos::Lexer<'i, PreprocessorToken<'i>>) -> &'i str {
    &lex.slice()[1..lex.slice().len() - 1]
}

pub fn parse_ident(lex: &mut logos::Lexer<Token>) -> Result<SmolStr, LexicalError> {
    Ok(lex.slice().into())
}

pub fn parse_rs_ident(lex: &mut logos::Lexer<Token>) -> Result<SmolStr, LexicalError> {
    let s = lex.slice();
    if lex.extras.1.allow_rs_ident {
        Ok(s.into())
    } else {
        Err(LexicalError::ForbiddenRsQuote {
            location: LexerPosition::new_raw(lex.extras.1.source_id, lex.span().start),
            length: TextSize::of(s),
        })
    }
}

fn parse_cmt_int(
    extras: &(super::ParseContext, super::ParseOptions),
    slice: &str,
    span: std::ops::Range<usize>,
    is_single: bool,
) {
    use lang_util::NodeContent;

    if extras.0.has_comments() {
        let source_id = extras.1.source_id;

        let comment = if is_single {
            ast::CommentData::Single(slice[2..].to_owned())
        } else {
            ast::CommentData::Multi(slice[2..slice.len() - 2].to_owned())
        }
        .spanned(
            LexerPosition::new_raw(source_id, span.start),
            LexerPosition::new_raw(source_id, span.end),
        );

        extras.0.add_comment(comment);
    }
}

pub fn parse_cmt(lex: &mut logos::Lexer<Token>, is_single: bool) {
    parse_cmt_int(&lex.extras, lex.slice(), lex.span(), is_single)
}

pub fn parse_pp_cmt<'i>(lex: &mut logos::Lexer<'i, PreprocessorToken<'i>>, is_single: bool) {
    parse_cmt_int(&lex.extras, lex.slice(), lex.span(), is_single)
}
