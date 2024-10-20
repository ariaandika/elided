use proc_macro::TokenStream as Token1;
use proc_macro2::TokenStream;
use syn::{*, parse::{Parse, ParseStream}};
use quote::{quote, format_ident, ToTokens};

mod from_unit;

#[proc_macro_derive(FromUnit)]
pub fn from_unit(input: Token1) -> Token1 {
    syn::parse_macro_input!(input as from_unit::FromUnit)
        .into_token_stream()
        .into()
}

