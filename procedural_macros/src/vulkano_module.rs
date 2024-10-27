use ahash::{AHashMap, AHashSet};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{braced, parse::{Parse, ParseStream}, punctuated::Punctuated, Error, Ident, Result, Token, Type};

use crate::{data::{Data, DataKeyword, DataManufacturer}, system::{System, SystemKeyword}};

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
            Ok(Self::System(input.parse()?))
        } else {
            Err(ahead.error())
        }
    }
}

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(Vertices);
    custom_keyword!(often);
}

struct TokenStreamMap {
    map: AHashMap<String, Vec<&'static str>>,
}

impl TokenStreamMap {
    fn new() -> Self {
        Self {
            map: AHashMap::new(),
        }
    }

    fn insert(&mut self, token_stream: TokenStream, new_config: &[&'static str]) {
        let token_stream = token_stream.to_string();
        if let Some(config) = self.map.get_mut(&token_stream) {
            config.extend_from_slice(new_config);
        } else {
            self.map.insert(
                token_stream.to_string(),
                new_config.iter().map(|s| *s).collect(),
            );
        }
    }

    fn to_token_stream(self) -> TokenStream {
        todo!()
    }
}

impl ToTokens for TokenStreamMap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (token_stream, config) in &self.map {
            match token_stream {
                _ => tokens.extend(token_stream.parse::<TokenStream>()),
            }
        }
    }
}

#[derive(Default)]
pub struct VulkanoModuleManufacturer {
    pub imports: AHashSet<&'static str>,
    pub items: AHashMap<Ident, ItemManufacturer>,
}

pub enum ItemManufacturer {
    Immutable(TokenStream),
    Data(DataManufacturer),
}

pub fn expand(input: VulkanoModule) -> Result<TokenStream> {
    let VulkanoModule { name, items } = input;
    let mut vulkano_module = VulkanoModuleManufacturer::default();

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

    let imports = vulkano_module.imports.iter();

    let items = vulkano_module.items.into_iter().map(|(_ident, item)| {
        match item {
            ItemManufacturer::Immutable(stream) => stream,
        }
    });

    Ok(quote! {
        mod #name {
            use bevy::prelude::*;
            // use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
            #(#imports)*

            #(#items)*

            // #[derive(Resource)]
            // pub struct RenderData {
            //     #fields
            // }

            // pub struct NonSendRenderData {
            //     #non_send_fields
            // }

            // pub fn render(command_buffer: NonSendMut<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>, render_data: ResMut<RenderData>, non_send_render_data: NonSendMut<NonSendRenderData>) {
            //     let command_buffer = command_buffer.into_inner();

            //     #function
            // }
        }
    })
}
