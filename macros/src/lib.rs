use proc_macro::TokenStream as Token1;
use proc_macro2::TokenStream;
use syn::{*, parse::{Parse, ParseStream}};
use quote::{quote, format_ident, ToTokens};

mod from_unit;
mod manual_dispatch;
mod std_error;

#[proc_macro_derive(FromUnit)]
pub fn from_unit(input: Token1) -> Token1 {
    syn::parse_macro_input!(input as from_unit::FromUnit)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn dispatch(input: Token1) -> Token1 {
    syn::parse_macro_input!(input as manual_dispatch::ManualDispatch)
        .into_token_stream()
        .into()
}

#[proc_macro_derive(StdError)]
pub fn std_error(input: Token1) -> Token1 {
    syn::parse_macro_input!(input as std_error::StdError)
        .into_token_stream()
        .into()
}

