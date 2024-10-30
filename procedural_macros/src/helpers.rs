use std::hash::Hash;

use derive_more::derive::{Deref, DerefMut};
use proc_macro2::TokenStream;

#[derive(Deref, DerefMut)]
pub struct HashTokenStream(pub TokenStream);

impl Eq for HashTokenStream {

}

impl PartialEq for HashTokenStream {
    fn eq(&self, other: &Self) -> bool {
        // 2 allocations I wish I could avoid.
        self.0.to_string() == other.0.to_string()
    }
}

impl Hash for HashTokenStream {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl From<TokenStream> for HashTokenStream {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}