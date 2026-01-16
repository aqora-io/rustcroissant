use std::collections::BTreeMap;

use schematic::{Config, ConfigEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
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

pub type JsonObject = BTreeMap<String, JsonValue>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Config)]
#[serde(transparent)]
pub struct NonEmptyString(#[setting(validate = schematic::validate::not_empty)] pub String);

pub type Id = NonEmptyString;
pub type Iri = NonEmptyString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Config)]
pub struct Ref {
    #[serde(rename = "@id")]
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum IriOrObject {
    Iri(Iri),
    Object(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, ConfigEnum)]
pub enum DatasetType {
    #[serde(rename = "sc:Dataset", alias = "Dataset")]
    #[default]
    Dataset,
}
