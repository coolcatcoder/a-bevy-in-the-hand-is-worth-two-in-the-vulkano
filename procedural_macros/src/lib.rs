use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, Error, Ident, Token};

mod data;
mod system;

#[proc_macro]
pub fn nothing(input: StdTokenStream) -> StdTokenStream {
    StdTokenStream::new()
}

#[proc_macro]
pub fn vulkano_module(input: StdTokenStream) -> StdTokenStream {
    let input = parse_macro_input!(input as ModuleInput);
    expand_vulkano_module(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

struct ModuleInput {
    name: Ident,
    items: Punctuated<ItemInput, Token![,]>,
}

impl Parse for ModuleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

enum ItemInput {
    //System(SystemInput),
    Data(DataInput),
}

impl Parse for ItemInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ahead = input.lookahead1();

        // if ahead.peek(SystemKeyword) {
        //     Ok(Self::System(input.parse()?))
        // } else
        if ahead.peek(DataKeyword) {
            Ok(Self::Data(input.parse()?))
        } else {
            Err(ahead.error())
        }
    }
}

fn expand_vulkano_module(input: ModuleInput) -> Result<TokenStream> {

}