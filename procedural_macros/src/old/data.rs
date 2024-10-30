use std::str::FromStr;

use ahash::AHashMap;
use heck::AsUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::{quote, IdentFragment};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Result, Token, Type,
};

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(Data);
    custom_keyword!(Resource);
    custom_keyword!(External);
    custom_keyword!(Vertices);
    custom_keyword!(Often);
}
pub use keyword::Data as DataKeyword;

use crate::vulkano_module::{ItemManufacturer, VulkanoModuleManufacturer};

enum Command {
    Resource,
    External(Span),
    Vertices {
        name: Ident,
        often: bool,
        vertex_type: Type,
    },
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ahead = input.lookahead1();

        if ahead.peek(keyword::Resource) {
            input.parse::<keyword::Resource>()?;

            Ok(Command::Resource)
        } else if ahead.peek(keyword::External) {
            Ok(Command::External(input.parse::<keyword::External>()?.span))
        } else if ahead.peek(keyword::Vertices) {
            input.parse::<keyword::Vertices>()?;

            let often = input.parse::<Option<keyword::Often>>()?.is_some();
            let name = input.parse()?;
            let vertex_type = input.parse()?;

            Ok(Command::Vertices {
                name,
                often,
                vertex_type,
            })
        } else {
            Err(ahead.error())
        }
    }
}

pub enum DataManufacturer {
    External,
    Internal { derives: Vec<String>, fields: AHashMap<Ident, Field> },
}

impl DataManufacturer {
    pub fn to_tokens(self, ident: Ident) -> TokenStream {
        match self {
            DataManufacturer::External => quote! {},
            DataManufacturer::Internal { fields } => {
                let mut collected_fields = quote! {};

                fields.into_iter().for_each(|(name, field)| match field {
                    Field::Vertices { often, vertex_type } => {
                        if often {
                            collected_fields.extend(quote! {
                                pub #name: Vec<#vertex_type>,
                            });
                        } else {
                            todo!("Not Often DataManufacturer");
                        }
                    }
                });

                // I hate this.
                let ident_camel_case =
                    TokenStream::from_str(&format!("{}", AsUpperCamelCase(ident.to_string())))
                        .unwrap();

                quote! {
                    pub struct #ident_camel_case {
                        #collected_fields
                    }
                }
            }
        }
    }
}

pub enum Field {
    Vertices { often: bool, vertex_type: Type },
}

pub struct Data {
    name: Ident,
    commands: Punctuated<Command, Token![,]>,
}

impl Parse for Data {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<DataKeyword>()?;
        let name = input.parse()?;
        let braced;
        braced!(braced in input);
        let commands = Punctuated::parse_terminated(&braced)?;

        Ok(Self { name, commands })
    }
}

impl Data {
    pub fn affect(self, vulkano_module: &mut VulkanoModuleManufacturer) -> Result<()> {
        if self.commands.len() == 1 {
            if matches!(self.commands[0], Command::External(..)) {
                vulkano_module.items.insert(
                    self.name,
                    ItemManufacturer::Data(DataManufacturer::External),
                );
                return Ok(());
            }
        }

        let mut fields = AHashMap::default();

        for command in self.commands {
            match command {
                Command::Resource => {
                    todo!()
                }
                Command::External(span) => {
                    return Err(Error::new(
                        span,
                        "In Data, the command External must be used alone.",
                    ));
                }
                Command::Vertices {
                    name,
                    often,
                    vertex_type,
                } => {
                    fields.insert(name, Field::Vertices { often, vertex_type });
                }
            }
        }

        vulkano_module.items.insert(
            self.name,
            ItemManufacturer::Data(DataManufacturer::Internal { fields }),
        );

        Ok(())
    }
}
