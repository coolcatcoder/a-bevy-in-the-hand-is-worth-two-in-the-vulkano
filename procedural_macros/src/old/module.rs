use ahash::{AHashMap, AHashSet};
use proc_macro2::TokenStream;
use smart_default::SmartDefault;
use syn::{Ident, Result};

use crate::helpers::HashTokenStream;

#[derive(SmartDefault)]
pub struct Module {
    imports: AHashSet<HashTokenStream>,
    
    items: AHashMap<Ident, Item>,
}

impl Module {
    /// Returns whether the value was newly inserted.
    pub fn import(&mut self, tokens: TokenStream) -> bool {
        self.imports.insert(tokens.into())
    }

    pub fn insert_item(&mut self, identifier: Ident, item: Item) -> Result<()> {
        todo!()
    }

    pub fn register_item(&mut self, identifier: Ident) -> Result<()> {
        todo!()
    }
}

pub enum Item {
    Function(Option<Function>),
}

#[derive(SmartDefault)]
pub struct Function {
    parameters: Vec<TokenStream>,
    unique_parameters: AHashSet<HashTokenStream>,
}

impl Function {
    pub fn add_parameter(&mut self, tokens: TokenStream) {
        self.parameters.push(tokens);
    }

    /// Returns whether the value was newly inserted.
    pub fn add_unique_parameter(&mut self, tokens: TokenStream) -> bool {
        self.unique_parameters.insert(tokens.into())
    }
}

impl Module {
    pub fn to_tokens(self) -> TokenStream {
        todo!()
    }
}