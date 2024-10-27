use ahash::{AHashMap, AHashSet};
use data::{Data, DataKeyword};
use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use system::{System, SystemKeyword, SystemManufacturer};
use vulkano_module::VulkanoModule;
use std::{collections::HashMap, stringify};
use syn::{
    braced, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, Error, Ident, Token, Type
};

mod vulkano_module;
mod system;
mod data;

#[proc_macro]
pub fn vulkano_module(input: StdTokenStream) -> StdTokenStream {
    let input = parse_macro_input!(input as VulkanoModule);
    vulkano_module::expand(input).unwrap_or_else(Error::into_compile_error).into()
}