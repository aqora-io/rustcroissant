use schematic::Config;
use serde::{Deserialize, Serialize};

use crate::specs::{
    common::{Id, Iri, JsonObject, NonEmptyString, Ref},
    serde::one_or_many,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(tag = "@type")]
pub enum Resource {
    #[serde(rename = "cr:FileObject")]
    FileObject(FileObject),

    #[serde(rename = "cr:FileSet")]
    FileSet(FileSet),
}

impl Resource {
    pub fn id(&self) -> &Id {
        match self {
            Resource::FileObject(o) => &o.id,
            Resource::FileSet(s) => &s.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct FileObject {
    #[serde(rename = "@id")]
    pub id: Id,

    pub name: Option<NonEmptyString>,

    pub description: Option<NonEmptyString>,

    #[setting(validate = schematic::validate::url)]
    pub content_url: String,

    pub encoding_format: Iri,

    pub content_size: Option<NonEmptyString>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub same_as: Vec<Iri>,

    pub sha256: Option<NonEmptyString>,

    pub md5: Option<NonEmptyString>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub contained_in: Vec<Ref>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct FileSet {
    #[serde(rename = "@id")]
    pub id: Id,

    pub name: Option<NonEmptyString>,

    pub description: Option<NonEmptyString>,

    pub encoding_format: Option<Iri>,

    #[serde(deserialize_with = "one_or_many")]
    #[setting(validate = schematic::validate::min_length(1))]
    pub contained_in: Vec<Ref>,

    #[serde(default, deserialize_with = "one_or_many")]
    #[setting(validate = schematic::validate::min_length(1))]
    pub includes: Vec<NonEmptyString>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub excludes: Vec<NonEmptyString>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}
