use ahash::AHashSet;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Result, Token, Type,
};

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(System);
    custom_keyword!(Vertices);
    custom_keyword!(often);
}
pub use keyword::System as SystemKeyword;

use crate::{
    data::{DataManufacturer, Field}, helpers::HashTokenStream, module::Module, vulkano_module::{ItemManufacturer, VulkanoModuleManufacturer}
};

enum Binding {
    Vertices {
        data_identifier: Ident,
        field_identifier: Ident,
    },
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(keyword::Vertices) {
            input.parse::<keyword::Vertices>()?;
            let data_identifier = input.parse()?;
            input.parse::<Token![.]>()?;
            let field_identifier = input.parse()?;

            Ok(Binding::Vertices {
                data_identifier,
                field_identifier,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct System {
    name: Ident,
    bindings: Punctuated<Binding, Token![,]>,
}

impl Parse for System {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<SystemKeyword>()?;
        let name = input.parse()?;
        let braced;
        braced!(braced in input);
        let bindings = Punctuated::parse_terminated(&braced)?;

        Ok(Self { name, bindings })
    }
}

impl System {
    pub fn affect(self, module: Module) -> Result<()> {
        let mut parameters = AHashSet::<HashTokenStream>::new();
        let mut body_no_duplicates = AHashSet::<HashTokenStream>::new();
        let mut body = quote! {};

        for binding in self.bindings {
            match binding {
                Binding::Vertices {
                    data_identifier,
                    field_identifier,
                } => {
                    let (often, vertex_type) = {
                        let Some(data) = vulkano_module.items.get(&data_identifier) else {
                            return Err(Error::new(
                                data_identifier.span(),
                                "This Data does not exist!",
                            ));
                        };
                        let ItemManufacturer::Data(data) = data else {
                            return Err(Error::new(data_identifier.span(), "This is not Data!"));
                        };
                        let DataManufacturer::Internal { fields } = data else {
                            return Err(Error::new(
                                data_identifier.span(),
                                "External Data is not yet supported!",
                            ));
                        };
                        let Some(field) = fields.get(&field_identifier) else {
                            return Err(Error::new(
                                data_identifier.span(),
                                "This field does not exist in the Data!",
                            ));
                        };
                        let Field::Vertices { often, vertex_type } = field else {
                            return Err(Error::new(
                                data_identifier.span(),
                                "This field is not Vertices!",
                            ));
                        };

                        (*often, vertex_type)
                    };

                    if often {
                        let buffer_identifier = format_ident!("{}_{}_buffer", data_identifier, field_identifier);

                        vulkano_module.imports.insert(quote!{
                            use super::#vertex_type;
                            // TODO: Do we really want this here, and not somewhere else?
                            use vulkano::buffer::allocator::SubbufferAllocator;
                        }.into());

                        parameters.insert(quote!{
                            todo!()
                        }.into());

                        body_no_duplicates.insert(quote!{
                            let #buffer_identifier = subbuffer_allocator.allocate_slice(#data_identifier.#field_identifier.len().try_into().unwrap()).unwrap();
                            #buffer_identifier.write().unwrap().clone_from_slice(#data_identifier.#field_identifier);
                        }.into());
                    } else {
                        todo!("Vertices not often, not done.");
                    }
                }
            }
        }

        let name = self.name;
        vulkano_module.items.insert(
            name.clone(),
            ItemManufacturer::Immutable(quote! {
                pub fn #name() {
                    #body
                }
            }),
        );

        Ok(())
    }
}
