#![deny(warnings)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use proc_macro2::Span;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Expr, Fields, Ident, IntSuffix, LitInt, LitStr, Token,
};

#[proc_macro_derive(uDebug)]
pub fn debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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
                            let i = LitInt::new(i as u64, IntSuffix::None, Span::call_site());

                            quote!(field(&self.#i)?)
                        })
                        .collect::<Vec<_>>();

                    quote!(f.debug_tuple(#ident_s)?#(.#fields)*.finish())
                }

                Fields::Unit => quote!(f.write(#ident_s)),
            };

            quote!(
                impl ufmt::uDebug for #ident {
                    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
                    where
                        W: ufmt::uWrite,
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
                                f.write(#variant_s)
                            }
                        ),
                    }
                })
                .collect::<Vec<_>>();

            quote!(
                impl ufmt::uDebug for #ident {
                    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
                        where
                        W: ufmt::uWrite,
                    {
                        match self {
                            #(#arms),*
                        }
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
    let input = parse_macro_input!(input as Input);

    let formatter = &input.formatter;
    let literal = input.literal.value();

    let pieces = parse(&literal);
    let mut args = input.args.iter();

    let exprs = pieces
        .iter()
        .map(|piece| match piece {
            Piece::Str(s) => quote!(ufmt::uDisplay::fmt(#s, f)?;),

            Piece::Display => {
                let arg = args.next().expect("FIXME");
                quote!(ufmt::uDisplay::fmt(&(#arg), f)?;)
            }

            Piece::Debug { pretty } => {
                let arg = args.next().expect("FIXME");

                if *pretty {
                    quote!(f.pretty(|f| ufmt::uDebug::fmt(&(#arg), f))?;)
                } else {
                    quote!(ufmt::uDebug::fmt(&(#arg), f)?;)
                }
            }
        })
        .collect::<Vec<_>>();

    quote!(ufmt::do_(#formatter, |f| {
        #(#exprs)*
        Ok(())
    }))
    .into()
}

struct Input {
    formatter: Expr,
    _comma: Token![,],
    literal: LitStr,
    _comma2: Token![,],
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(Input {
            formatter: input.parse()?,
            _comma: input.parse()?,
            literal: input.parse()?,
            _comma2: input.parse()?,
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Piece<'a> {
    Debug { pretty: bool },
    Display,
    Str(&'a str),
}

fn parse(mut literal: &str) -> Vec<Piece> {
    let mut pieces = vec![];

    loop {
        let mut parts = literal.splitn(2, '{');
        match (parts.next(), parts.next()) {
            (None, None) => break,

            (Some(s), None) => {
                if !s.is_empty() {
                    pieces.push(Piece::Str(s));
                }
                break;
            }

            (Some(head), Some(tail)) => {
                const DEBUG: &str = ":?}";
                const DEBUG_PRETTY: &str = ":#?}";
                const DISPLAY: &str = "}";

                if tail.starts_with(DEBUG) {
                    if !head.is_empty() {
                        pieces.push(Piece::Str(head));
                    }

                    pieces.push(Piece::Debug { pretty: false });

                    literal = &tail[DEBUG.len()..];
                } else if tail.starts_with(DEBUG_PRETTY) {
                    if !head.is_empty() {
                        pieces.push(Piece::Str(head));
                    }

                    pieces.push(Piece::Debug { pretty: true });

                    literal = &tail[DEBUG_PRETTY.len()..];
                } else if tail.starts_with(DISPLAY) {
                    if !head.is_empty() {
                        pieces.push(Piece::Str(head));
                    }

                    pieces.push(Piece::Display);

                    literal = &tail[DISPLAY.len()..];
                }
            }

            _ => unreachable!(),
        }
    }

    pieces
}

#[cfg(test)]
mod tests {
    use super::Piece;

    #[test]
    fn pieces() {
        assert_eq!(
            super::parse("The answer is {}"),
            vec![Piece::Str("The answer is "), Piece::Display]
        );

        assert_eq!(super::parse("{:?}"), vec![Piece::Debug { pretty: false }]);

        assert_eq!(super::parse("{:#?}"), vec![Piece::Debug { pretty: true }]);

        // FIXME
        // assert_eq!(
        //     super::parse("{{}} is not an argument"),
        //     vec![Piece::Str("{} is not an argument")]
        // );
    }
}
