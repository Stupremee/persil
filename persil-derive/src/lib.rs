//! Proc macros for `persil`.
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, ItemFn, LitStr, Signature, Token,
};

/// Two strigns that form a category and an event name.
///
/// #[trace("parse", "expression")]
///         ^^^^^^^  ^^^^^^^^^^^^
///         category    event
struct Args {
    category: LitStr,
    event: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let vars = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

        if vars.len() != 2 {
            return Err(Error::new_spanned(
                vars,
                "`trace` macro requires two strings as arguments",
            ));
        }

        let category = vars[0].clone();
        let event = vars[1].clone();
        Ok(Args { category, event })
    }
}

/// Attribute that can be put onto a function and will
/// trace the whole function call with the given category
/// and event name.
///
/// `async` and `const` functions are not traceable and will
/// throw a compile error.
///
/// # Example
///
/// ```
/// use persil_derive::trace;
///
/// /// Will start tracing the function in the `calculations`
/// /// categroy and the `factorial` name.
/// #[trace("calculations", "factorial")]
/// fn factorial(mut n: u64) -> u64 {
///     let mut p = 1;
///     while n > 1 {
///         p *= n;
///         n -= 1;
///     }
///     p
/// }
/// ```
#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    generate(args, input)
}

fn generate(args: Args, fun: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        block,
        sig,
    } = fun;

    let Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    if let Some(tok) = asyncness {
        return (quote_spanned! {
            tok.span =>
            compile_error!("tracing async functions is not supported");
        })
        .into();
    }

    if let Some(tok) = constness {
        return (quote_spanned! {
            tok.span =>
            compile_error!("tracing const functions is not supported");
        })
        .into();
    }

    let Args { category, event } = args;
    let body = quote! {
        let __profiler = persil::trace(#category, #event);
        #block
    };

    (quote! {
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            #body
        }
    })
    .into()
}
