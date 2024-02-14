//! `μfmt` macros

#![deny(warnings)]

extern crate proc_macro;

use core::mem;
use proc_macro::TokenStream;
use std::borrow::Cow;
use std::cmp::Ordering;

use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    Data, DeriveInput, Expr, Fields, GenericParam, Ident, LitStr, Token,
};

/// Automatically derive the `uDebug` trait for a `struct` or `enum`
///
/// Supported items
///
/// - all kind of `struct`-s
/// - all kind of `enum`-s
///
/// `union`-s are not supported
#[proc_macro_derive(uDebug)]
pub fn debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut generics = input.generics;

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(ufmt::uDebug));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ident = &input.ident;
    let ts = match input.data {
        Data::Struct(data) => {
            let ident_s = ident.to_string();

            let body = match data.fields {
                Fields::Named(fields) => {
                    let fields = fields
                        .named
                        .iter()
                        .map(|field| {
                            let ident = field.ident.as_ref().expect("UNREACHABLE");
                            let name = ident.to_string();

                            quote!(field(#name, &self.#ident)?)
                        })
                        .collect::<Vec<_>>();

                    quote!(f.debug_struct(#ident_s)?#(.#fields)*.finish())
                }

                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len())
                        .map(|i| {
                            let i = Literal::u64_unsuffixed(i as u64);

                            quote!(field(&self.#i)?)
                        })
                        .collect::<Vec<_>>();

                    quote!(f.debug_tuple(#ident_s)?#(.#fields)*.finish())
                }

                Fields::Unit => quote!(f.write_str(#ident_s)),
            };

            quote!(
                impl #impl_generics ufmt::uDebug for #ident #ty_generics #where_clause {
                    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> core::result::Result<(), W::Error>
                    where
                        W: ufmt::uWrite + ?Sized,
                    {
                        #body
                    }
                }

            )
        }

        Data::Enum(data) => {
            let arms = data
                .variants
                .iter()
                .map(|var| {
                    let variant = &var.ident;
                    let variant_s = variant.to_string();

                    match &var.fields {
                        Fields::Named(fields) => {
                            let mut pats = Vec::with_capacity(fields.named.len());
                            let mut methods = Vec::with_capacity(fields.named.len());
                            for field in &fields.named {
                                let ident = field.ident.as_ref().unwrap();
                                let ident_s = ident.to_string();

                                pats.push(quote!(#ident));
                                methods.push(quote!(field(#ident_s, #ident)?));
                            }

                            quote!(
                                #ident::#variant { #(#pats),* } => {
                                    f.debug_struct(#variant_s)?#(.#methods)*.finish()
                                }
                            )
                        }

                        Fields::Unnamed(fields) => {
                            let pats = &(0..fields.unnamed.len())
                                .map(|i| Ident::new(&format!("_{}", i), Span::call_site()))
                                .collect::<Vec<_>>();

                            quote!(
                                #ident::#variant(#(#pats),*) => {
                                    f.debug_tuple(#variant_s)?#(.field(#pats)?)*.finish()
                                }
                            )
                        }

                        Fields::Unit => quote!(
                            #ident::#variant => {
                                f.write_str(#variant_s)
                            }
                        ),
                    }
                })
                .collect::<Vec<_>>();

            let body = if arms.is_empty() {
                // Debug's implementation uses `::core::intrinsics::unreachable()`
                quote!(unsafe { core::unreachable!() })
            } else {
                quote!(
                    match self {
                        #(#arms),*
                    }
                )
            };

            quote!(
                impl #impl_generics ufmt::uDebug for #ident #ty_generics #where_clause {
                    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> core::result::Result<(), W::Error>
                        where
                        W: ufmt::uWrite + ?Sized,
                    {
                        #body
                    }
                }
            )
        }

        Data::Union(..) => {
            return parse::Error::new(Span::call_site(), "this trait cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    ts.into()
}

#[proc_macro]
pub fn uwrite(input: TokenStream) -> TokenStream {
    write(input, false)
}

#[proc_macro]
pub fn uwriteln(input: TokenStream) -> TokenStream {
    write(input, true)
}

fn write(input: TokenStream, newline: bool) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let formatter = &input.formatter;
    let literal = input.literal;

    let mut format = literal.value();
    if newline {
        format.push('\n');
    }
    let pieces = match parse(&format, literal.span()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(pieces) => pieces,
    };

    let required_args = pieces.iter().filter(|piece| !piece.is_str()).count();
    let supplied_args = input.args.len();
    match supplied_args.cmp(&required_args) {
        Ordering::Less => {
            return parse::Error::new(
                literal.span(),
                &format!(
                    "format string requires {} arguments but {} {} supplied",
                    required_args,
                    supplied_args,
                    if supplied_args == 1 { "was" } else { "were" }
                ),
            )
            .to_compile_error()
            .into();
        }
        Ordering::Greater => {
            return parse::Error::new(
                input.args[required_args].span(),
                "argument never used".to_string(),
            )
            .to_compile_error()
            .into();
        }
        Ordering::Equal => {}
    }

    let mut args = vec![];
    let mut pats = vec![];
    let mut exprs = vec![];
    let mut i = 0;
    for piece in pieces {
        if let Piece::Str(s) = piece {
            exprs.push(quote!(f.write_str(#s)?;))
        } else {
            let pat = mk_ident(i);
            let arg = &input.args[i];
            i += 1;

            args.push(quote!(&(#arg)));
            pats.push(quote!(#pat));

            match piece {
                Piece::Display => {
                    exprs.push(quote!(ufmt::uDisplay::fmt(#pat, f)?;));
                }

                Piece::Debug { pretty } => {
                    exprs.push(if pretty {
                        quote!(f.pretty(|f| ufmt::uDebug::fmt(#pat, f))?;)
                    } else {
                        quote!(ufmt::uDebug::fmt(#pat, f)?;)
                    });
                }
                Piece::Hex {
                    upper_case,
                    pad_char,
                    pad_length,
                    prefix,
                } => {
                    exprs.push(quote!(ufmt::uDisplayHex::fmt_hex(#pat, f, ufmt::HexOptions{
                        upper_case:#upper_case,
                        pad_char: #pad_char,
                        pad_length: #pad_length,
                        ox_prefix: #prefix})?;));
                }
                Piece::Str(_) => unreachable!(),
                Piece::Float { decimal_places, pad_length } => {
                    exprs.push(quote!(ufmt::uDisplayFloat::fmt_float(
                        #pat, 
                        f, 
                        #decimal_places, 
                        #pad_length
                    )?;))
                }
            }
        }
    }

    quote!(match (#(#args),*) {
        (#(#pats),*) => {
            use ufmt::UnstableDoAsFormatter as _;

            (#formatter).do_as_formatter(|f| {
                #(#exprs)*
                core::result::Result::Ok(())
            })
        }
    })
    .into()
}

struct Input {
    formatter: Expr,
    _comma: Token![,],
    literal: LitStr,
    _comma2: Option<Token![,]>,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let formatter = input.parse()?;
        let _comma = input.parse()?;
        let literal = input.parse()?;

        if input.is_empty() {
            Ok(Input {
                formatter,
                _comma,
                literal,
                _comma2: None,
                args: Punctuated::new(),
            })
        } else {
            Ok(Input {
                formatter,
                _comma,
                literal,
                _comma2: input.parse()?,
                args: Punctuated::parse_terminated(input)?,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
enum Piece<'a> {
    Debug {
        pretty: bool,
    },
    Display,
    Str(Cow<'a, str>),
    Hex {
        upper_case: bool,
        pad_char: u8,
        pad_length: usize,
        prefix: bool,
    },
    Float {
        decimal_places: usize,
        pad_length: usize,
    },
}

impl Piece<'_> {
    fn is_str(&self) -> bool {
        matches!(self, Piece::Str(_))
    }
}

fn mk_ident(i: usize) -> Ident {
    Ident::new(&format!("__{}", i), Span::call_site())
}

// `}}` -> `}`
fn unescape(mut literal: &str, span: Span) -> parse::Result<Cow<str>> {
    if literal.contains('}') {
        let mut buf = String::new();

        while literal.contains('}') {
            const ERR: &str = "format string contains an unmatched right brace";
            let mut parts = literal.splitn(2, '}');

            match (parts.next(), parts.next()) {
                (Some(left), Some(right)) => {
                    const ESCAPED_BRACE: &str = "}";

                    if let Some(tail) = right.strip_prefix(ESCAPED_BRACE) {
                        buf.push_str(left);
                        buf.push('}');

                        literal = tail;
                    } else {
                        return Err(parse::Error::new(span, ERR));
                    }
                }

                _ => unreachable!(),
            }
        }

        buf.push_str(literal);

        Ok(buf.into())
    } else {
        Ok(Cow::Borrowed(literal))
    }
}

fn parse(mut literal: &str, span: Span) -> parse::Result<Vec<Piece>> {
    let mut pieces = vec![];

    let mut buf = String::new();
    loop {
        let mut parts = literal.splitn(2, '{');
        match (parts.next(), parts.next()) {
            // empty string literal
            (None, None) => break,

            // end of the string literal
            (Some(s), None) => {
                if buf.is_empty() {
                    if !s.is_empty() {
                        pieces.push(Piece::Str(unescape(s, span)?));
                    }
                } else {
                    buf.push_str(&unescape(s, span)?);

                    pieces.push(Piece::Str(Cow::Owned(buf)));
                }

                break;
            }

            (head, Some(tail)) => {
                const DEBUG: &str = ":?}";
                const DEBUG_PRETTY: &str = ":#?}";
                const DISPLAY: &str = "}";
                const ESCAPED_BRACE: &str = "{";

                let head = head.unwrap_or("");
                if tail.starts_with(DEBUG)
                    || tail.starts_with(DEBUG_PRETTY)
                    || tail.starts_with(DISPLAY)
                    || tail.starts_with(':')
                {
                    if buf.is_empty() {
                        if !head.is_empty() {
                            pieces.push(Piece::Str(unescape(head, span)?));
                        }
                    } else {
                        buf.push_str(&unescape(head, span)?);

                        pieces.push(Piece::Str(Cow::Owned(mem::take(&mut buf))));
                    }

                    if let Some(tail_tail) = tail.strip_prefix(DEBUG) {
                        pieces.push(Piece::Debug { pretty: false });

                        literal = tail_tail;
                    } else if let Some(tail_tail) = tail.strip_prefix(DEBUG_PRETTY) {
                        pieces.push(Piece::Debug { pretty: true });

                        literal = tail_tail;
                    } else if let Some(tail2) = tail.strip_prefix(':') {
                        let (piece, remainder) = parse_colon(tail2, span)?;
                        pieces.push(piece);
                        literal = remainder;
                    } else {
                        pieces.push(Piece::Display);

                        literal = &tail[DISPLAY.len()..];
                    }
                } else if let Some(tail_tail) = tail.strip_prefix(ESCAPED_BRACE) {
                    buf.push_str(&unescape(head, span)?);
                    buf.push('{');

                    literal = tail_tail;
                } else {
                    return Err(parse::Error::new(
                        span,
                        INVALID_FORMAT_STR,
                    ));
                }
            }
        }
    }
    Ok(pieces)
}

const INVALID_FORMAT_STR: &str = "invalid format string: expected `{{`, `{}`, `{:?}`, `{:#?}`, '{:x}' or '{:.<0..6>}'";

/// parses the stuff after a `{:` into a [Piece] and the trailing `&str` (what comes after the `}`)
fn parse_colon(format: &str, span: Span) -> parse::Result<(Piece, &str)> {
    let err_piece = || -> syn::Error {
        parse::Error::new(
            span,
            INVALID_FORMAT_STR,
        )
    };

    let mut chars = format.chars();
    let ch = chars.next().ok_or(err_piece())?;

    let (ch, prefix) = if ch == '#' {
        let ch = chars.next().ok_or(err_piece())?;
        (ch, true)
    } else {
        (ch, false)
    };
    let (mut ch, pad_char) = if ch == '0' {
        let ch = chars.next().ok_or(err_piece())?;
        (ch, b'0')
    } else {
        (ch, b' ')
    };

    let mut pad_length = 0_usize;
    while ch.is_ascii_digit() {
        pad_length = pad_length * 10 + ch.to_digit(10).unwrap() as usize;
        ch = chars.next().ok_or(err_piece())?;
    }

    let cmd = match ch {
        '.' => Some('.'),
        'x' => Some('x'),
        'X' => Some('X'),
        _ => None,
    };

    if cmd.is_some() {
        ch = chars.next().ok_or(err_piece())?;
    }

    let mut decimal_places = 0_usize;
    while ch.is_ascii_digit() {
        decimal_places = decimal_places * 10 + ch.to_digit(10).unwrap() as usize;
        ch = chars.next().ok_or(err_piece())?;
    }

    if ch != '}' {
        return Err(err_piece());
    }

    match cmd {
        Some(cmd) => match cmd {
            'x' => Ok((
                Piece::Hex {
                    upper_case: false,
                    pad_char,
                    pad_length,
                    prefix,
                },
                chars.as_str(),
            )),
            'X' => Ok((
                Piece::Hex {
                    upper_case: true,
                    pad_char,
                    pad_length,
                    prefix,
                },
                chars.as_str(),
            )),
            '.' => if pad_char == b' ' && prefix == false && decimal_places < 7{
                Ok((
                    Piece::Float {
                        decimal_places,
                        pad_length,
                    },
                    chars.as_str(),
                ))   
            } else {
                Err(err_piece())
            }
            _ => Err(err_piece()),
        }
        None => Err(err_piece()),
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::Span;

    use crate::Piece;

    #[test]
    fn pieces() {
        let span = Span::call_site();

        // string interpolation
        assert_eq!(
            super::parse("The answer is {}", span).ok(),
            Some(vec![
                Piece::Str(Cow::Borrowed("The answer is ")),
                Piece::Display
            ]),
        );

        assert_eq!(
            super::parse("{:?}", span).ok(),
            Some(vec![Piece::Debug { pretty: false }]),
        );

        assert_eq!(
            super::parse("{:#?}", span).ok(),
            Some(vec![Piece::Debug { pretty: true }]),
        );

        assert_eq!(
            super::parse("{:x}", span).ok(),
            Some(vec![Piece::Hex {
                upper_case: false,
                pad_char: b' ',
                pad_length: 0,
                prefix: false
            }]),
        );

        assert_eq!(
            super::parse("{:9x}", span).ok(),
            Some(vec![Piece::Hex {
                upper_case: false,
                pad_char: b' ',
                pad_length: 9,
                prefix: false
            }]),
        );

        assert_eq!(
            super::parse("{:9X}", span).ok(),
            Some(vec![Piece::Hex {
                upper_case: true,
                pad_char: b' ',
                pad_length: 9,
                prefix: false
            }]),
        );

        assert_eq!(
            super::parse("{:#X}", span).ok(),
            Some(vec![Piece::Hex {
                upper_case: true,
                pad_char: b' ',
                pad_length: 0,
                prefix: true
            }]),
        );

        assert_eq!(
            super::parse("{:.0}", span).ok(),
            Some(vec![Piece::Float {
                decimal_places: 0,
                pad_length: 0,
            }]),
        );

        assert_eq!(
            super::parse("{:.6}", span).ok(),
            Some(vec![Piece::Float {
                decimal_places: 6,
                pad_length: 0, 
            }]),
        );

        assert_eq!(
            super::parse("{:17.6}", span).ok(),
            Some(vec![Piece::Float {
                decimal_places: 6,
                pad_length: 17, 
            }]),
        );

        // escaped braces
        assert_eq!(
            super::parse("{{}} is not an argument", span).ok(),
            Some(vec![Piece::Str(Cow::Borrowed("{} is not an argument"))]),
        );

        // left brace & junk
        assert!(super::parse("{", span).is_err());
        assert!(super::parse(" {", span).is_err());
        assert!(super::parse("{ ", span).is_err());
        assert!(super::parse("{ {", span).is_err());
        assert!(super::parse("{:q}", span).is_err());
    }

    #[test]
    fn unescape() {
        let span = Span::call_site();

        // no right brace
        assert_eq!(super::unescape("", span).ok(), Some(Cow::Borrowed("")));
        assert_eq!(
            super::unescape("Hello", span).ok(),
            Some(Cow::Borrowed("Hello"))
        );

        // unmatched right brace
        assert!(super::unescape(" }", span).is_err());
        assert!(super::unescape("} ", span).is_err());
        assert!(super::unescape("}", span).is_err());

        // escaped right brace
        assert_eq!(super::unescape("}}", span).ok(), Some(Cow::Borrowed("}")));
        assert_eq!(super::unescape("}} ", span).ok(), Some(Cow::Borrowed("} ")));
    }
}
