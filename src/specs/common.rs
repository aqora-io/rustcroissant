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
    pub struct NonEmptyString(#[setting(validate = schematic::validate::not_empty)] pub String);
);

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
        #[setting(validate = schematic::validate::url)]
        Url(String),
        Path(RelativePathBuf),
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
