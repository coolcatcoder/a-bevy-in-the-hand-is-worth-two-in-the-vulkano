use std::{ops::Deref, str::FromStr};

use ahash::{AHashMap, AHashSet};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Result, Token, Type,
};

use crate::{
    data::{Data, DataKeyword, DataManufacturer}, helpers::HashTokenStream, module::Module, system::{System, SystemKeyword}
};

pub struct VulkanoModule {
    name: Ident,
    items: Punctuated<Item, Token![,]>,
}

impl Parse for VulkanoModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

enum Item {
    System(System),
    Data(Data),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ahead = input.lookahead1();

        if ahead.peek(SystemKeyword) {
            Ok(Self::System(input.parse()?))
        } else if ahead.peek(DataKeyword) {
            Ok(Self::Data(input.parse()?))
        } else {
            Err(ahead.error())
        }
    }
}

#[derive(Default)]
pub struct VulkanoModuleManufacturer {
    pub imports: AHashSet<HashTokenStream>,
    pub items: AHashMap<Ident, ItemManufacturer>,
}

pub enum ItemManufacturer {
    Immutable(TokenStream),
    Data(DataManufacturer),
}

pub fn expand(input: VulkanoModule) -> Result<TokenStream> {
    let VulkanoModule { name, items } = input;
    let mut module = Module::default();

    for item in items {
        match item {
            Item::System(system) => {
                system.affect(&mut vulkano_module)?;
            }
            Item::Data(data) => {
                data.affect(&mut vulkano_module)?;
            }
        }
    }

    Ok(module.to_tokens())

    // let imports = vulkano_module
    //     .imports
    //     .into_iter()
    //     .map(|import| import.0);

    // let items = vulkano_module
    //     .items
    //     .into_iter()
    //     .map(|(ident, item)| match item {
    //         ItemManufacturer::Immutable(stream) => stream,
    //         ItemManufacturer::Data(data) => data.to_tokens(ident),
    //     });

    // Ok(quote! {
    //     mod #name {
    //         use bevy::prelude::*;
    //         // use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
    //         #(#imports)*

    //         #(#items)*

    //         // #[derive(Resource)]
    //         // pub struct RenderData {
    //         //     #fields
    //         // }

    //         // pub struct NonSendRenderData {
    //         //     #non_send_fields
    //         // }

    //         // pub fn render(command_buffer: NonSendMut<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>, render_data: ResMut<RenderData>, non_send_render_data: NonSendMut<NonSendRenderData>) {
    //         //     let command_buffer = command_buffer.into_inner();

    //         //     #function
    //         // }
    //     }
    // })
}
