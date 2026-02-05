use relative_path::RelativePathBuf;
use schematic::{Config, ConfigEnum};
use std::collections::BTreeMap;

crate::config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum JsonValue {
        Null,
        Bool(bool),
        I64(i64),
        U64(u64),
        F64(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(BTreeMap<String, JsonValue>),
    }
);

pub type JsonObject = BTreeMap<String, JsonValue>;

crate::config_struct!(
    #[serde(transparent)]
    #[derive(Config, Hash)]
    pub struct NonEmptyString(#[setting(validate = schematic::validate::not_empty)] String);
);

impl NonEmptyString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl std::ops::Deref for NonEmptyString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for NonEmptyString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for NonEmptyString {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

pub type Id = NonEmptyString;
pub type Iri = NonEmptyString;

crate::config_struct!(
    #[derive(Config)]
    pub struct Ref {
        #[serde(rename = "@id")]
        pub id: Id,
    }
);

crate::config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum IriOrObject {
        Iri(Iri),
        Object(JsonObject),
    }
);

crate::config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum UrlOrRelativePath {
        Url(Option<url::Url>),
        Path(Option<RelativePathBuf>),
    }
);

crate::config_unit_enum!(
    #[derive(ConfigEnum)]
    pub enum DatasetType {
        #[serde(rename = "sc:Dataset", alias = "Dataset")]
        #[default]
        Dataset,
    }
);
