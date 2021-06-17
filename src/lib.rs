use proc_macro::TokenStream;
use proc_macro2::{Literal};
use quote::quote;
use syn::{braced, parse::Parse, parse_macro_input, token::Brace, Error, Ident, Token, Type};

use crate::internals::{Compiler, Machine};

mod internals;

#[proc_macro]
pub fn bf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Bf);

    let input_buffer = input.bf_input.input_value.to_string().into_bytes();

    let code_str = input.bf_code.code.to_string();
    let mut compiler = Compiler::new(&code_str);

    let bf_vm = Machine::new(compiler.compile(), input_buffer, Vec::<u8>::new());
    let output = String::from_utf8(bf_vm.execute()).unwrap();

    let ty = input.bf_code.ty;
    let tokens = quote! {
        #output.parse::<#ty>()
    };
    tokens.into()
}

const INPUT_TOKEN: &str = "input";

#[derive(Debug)]
struct InputToken;

impl Parse for InputToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>().and_then(|ref i| {
            if i == INPUT_TOKEN {
                Ok(InputToken)
            } else {
                Err(Error::new_spanned(
                    i,
                    format!(
                        "expected `{}`, found `{}` instead.",
                        INPUT_TOKEN,
                        i.to_string()
                    ),
                ))
            }
        })
    }
}

const CODE_TOKEN: &str = "code";

#[derive(Debug)]
struct CodeToken;

impl Parse for CodeToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>().and_then(|ref i| {
            if i == CODE_TOKEN {
                Ok(CodeToken)
            } else {
                Err(Error::new_spanned(
                    i,
                    format!(
                        "expected `{}`, found `{}` instead.",
                        CODE_TOKEN,
                        i.to_string()
                    ),
                ))
            }
        })
    }
}

#[derive(Debug)]
struct BfInput {
    input_token: InputToken,
    brace_token: Brace,
    input_value: Literal,
}

impl Parse for BfInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(BfInput {
            input_token: input.parse()?,
            brace_token: braced!(content in input),
            input_value: content.parse()?,
        })
    }
}

#[derive(Debug)]
struct BfCode {
    code_token: CodeToken,
    brace_token: Brace,
    code: Literal,
    as_token: Token![as],
    ty: Type,
}

impl Parse for BfCode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            code_token: input.parse()?,
            brace_token: braced!(content in input),
            code: content.parse()?,
            as_token: content.parse()?,
            ty: content.parse()?,
        })
    }
}

#[derive(Debug)]
struct Bf {
    bf_input: BfInput,
    bf_code: BfCode,
}

impl Parse for Bf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Bf {
            bf_input: input.parse()?,
            bf_code: input.parse()?,
        })
    }
}
