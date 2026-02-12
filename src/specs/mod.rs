mod agent;
pub(crate) mod common;
mod context;
pub(crate) mod data_type;
mod dataset;
#[macro_use]
mod macros;
pub mod record;
pub(crate) mod resource;
mod serde;
mod source;
mod validate;

use ::serde::{Deserialize, Serialize};
pub use agent::*;
pub use common::*;
pub use data_type::*;
pub use dataset::*;
pub use record::*;
pub use resource::*;
use schematic::{Config, Schema, SchemaBuilder, Schematic, schema::UnionType};
pub use source::*;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Default for OneOrMany<T> {
    fn default() -> Self {
        OneOrMany::Many(Vec::new())
    }
}

impl<T> OneOrMany<T> {
    pub fn is_empty(&self) -> bool {
        matches!(self, OneOrMany::Many(v) if v.is_empty())
    }

    pub fn into_vec(self) -> Vec<T> {
        match self {
            OneOrMany::One(v) => vec![v],
            OneOrMany::Many(v) => v,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            OneOrMany::One(v) => std::slice::from_ref(v),
            OneOrMany::Many(v) => v.as_slice(),
        }
    }
}

impl<T> Schematic for OneOrMany<T>
where
    T: Schematic,
{
    fn build_schema(mut schema: SchemaBuilder) -> Schema {
        schema.union(UnionType::new_one([
            schema.infer::<T>(),
            schema.infer::<Vec<T>>(),
        ]))
    }
}

impl<T> Deref for OneOrMany<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            OneOrMany::One(v) => std::slice::from_ref(v),
            OneOrMany::Many(v) => v,
        }
    }
}
