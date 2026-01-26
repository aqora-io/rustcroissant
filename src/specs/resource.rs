use crate::specs::{
    common::{Id, Iri, JsonObject, NonEmptyString, Ref, UrlOrRelativePath},
    serde::one_or_many,
};
use schematic::Config;

config_enum!(
    #[derive(Config)]
    #[serde(tag = "@type")]
    pub enum Resource {
        #[serde(rename = "cr:FileObject")]
        FileObject(FileObject),

        #[serde(rename = "cr:FileSet")]
        FileSet(FileSet),
    }
);

impl Resource {
    pub fn id(&self) -> &Id {
        match self {
            Resource::FileObject(o) => &o.id,
            Resource::FileSet(s) => &s.id,
        }
    }
}

config_struct!(
    #[derive(Config)]
    pub struct FileObject {
        #[serde(rename = "@id")]
        pub id: Id,

        pub name: Option<NonEmptyString>,

        pub description: Option<NonEmptyString>,

        pub content_url: UrlOrRelativePath,

        pub encoding_format: Iri,

        #[setting(validate = schematic::validate::regex("^[0-9]+ B"))]
        pub content_size: Option<String>,

        #[serde(default, deserialize_with = "one_or_many")]
        pub same_as: Vec<Iri>,

        pub sha256: Option<NonEmptyString>,

        pub md5: Option<NonEmptyString>,

        #[serde(default, deserialize_with = "one_or_many")]
        pub contained_in: Vec<Ref>,

        #[serde(flatten, default)]
        pub extra: JsonObject,
    }
);

config_struct!(
    #[derive(Config)]
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
);
