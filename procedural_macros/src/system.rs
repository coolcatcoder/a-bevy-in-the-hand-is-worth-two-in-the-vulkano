use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{braced, parse::{Parse, ParseStream}, punctuated::Punctuated, Error, Ident, Result, Token, Type};

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(System);
    custom_keyword!(Vertices);
    custom_keyword!(often);
}
pub use keyword::System as SystemKeyword;

use crate::VulkanoModuleManufacturer;

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

        Ok(Self{name, bindings,})
    }
}

impl System {
    pub fn affect(self, vulkano_module: &mut VulkanoModuleManufacturer) -> Result<()> {
        let function = quote!{};

        for binding in self.bindings {
            match binding {
                Binding::Vertices {
                    data_identifier,
                    field_identifier,
                } => {
                    vulkano_module.imports.insert("use super::#vertex_type;");
    
                    if often {
                        vulkano_module.imports.insert("use vulkano::buffer::allocator::SubbufferAllocator;");
    
                        fields.insert(quote! {pub #identifier: Vec<#vertex_type>,}, &[]);
    
                        non_send_fields.insert(quote! {subbuffer_allocator: SubbufferAllocator,}, &[]);
    
                        function.extend(quote!{
                            let #identifier = non_send_render_data.subbuffer_allocator.allocate_slice(render_data.#identifier.len().try_into().unwrap()).unwrap();
                            #identifier.write().unwrap().clone_from_slice(&render_data.#identifier);
                        });
                    } else {
                        todo!("Vertices not often, not done.");
                    }
                }
            }
        }

        Ok(())
    }
}