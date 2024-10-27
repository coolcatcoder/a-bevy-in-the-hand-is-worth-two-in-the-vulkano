use ahash::AHashMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{braced, parse::{Parse, ParseStream}, punctuated::Punctuated, Error, Ident, Result, Token, Type};

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
            
            Ok(Command::Vertices{name, often})
        } else {
            Err(ahead.error())
        }
    }
}

pub enum DataManufacturer {
    External,
    Internal {
        fields: AHashMap<Ident, Field>
    },
}

enum Field {
    Vertices {
        often: bool,
    }
}

pub struct Data {
    name: Ident,
    commands: Punctuated<Command, Token![,]>
}

impl Parse for Data {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            commands: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Data {
    pub fn affect(self, vulkano_module: &mut VulkanoModuleManufacturer) -> Result<()> {
        if self.commands.len() == 1 {
            if matches!(self.commands[0], Command::External(..)) {
                vulkano_module.items.insert(self.name, ItemManufacturer::Data(DataManufacturer::External));
                return Ok(());
            }
        }

        let data = DataManufacturer::Internal { fields: Default::default() };

        for command in self.commands {
            match command {
                Command::Resource => {
                    todo!()
                }
                Command::External(span) => {
                    return Err(Error::new(span, "In Data, the command External must be used alone."));
                }
                Command::Vertices { name, often } => {

                }
            }
        }

        vulkano_module.items.insert(self.name, ItemManufacturer::Data(data));

        Ok(())
    }
}