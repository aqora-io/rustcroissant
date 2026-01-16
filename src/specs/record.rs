use schematic::Config;
use serde::{Deserialize, Serialize};

use crate::specs::{
    common::{Id, Iri, JsonObject, NonEmptyString, Ref},
    data_type::DataType,
    serde::one_or_many,
    source::Source,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct RecordSet {
    #[serde(rename = "@type")]
    pub r#type: RecordSetType,

    #[serde(rename = "@id")]
    pub id: Id,

    pub name: Option<NonEmptyString>,

    pub description: Option<NonEmptyString>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub data_type: Vec<DataType>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub key: Vec<Ref>,

    #[serde(default)]
    #[setting(validate = schematic::validate::min_length(1))]
    pub field: Vec<Field>,

    #[serde(default)]
    pub data: Option<Vec<JsonObject>>,

    #[serde(default)]
    pub examples: Option<RecordSetExamples>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Config)]
pub enum RecordSetType {
    #[serde(rename = "cr:RecordSet", alias = "RecordSet")]
    RecordSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum RecordSetExamples {
    Inline(Vec<JsonObject>),
    Source(Source),
    #[setting(validate = schematic::validate::url)]
    Url(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "@type")]
    pub r#type: FieldType,

    #[serde(rename = "@id")]
    pub id: Id,

    pub name: Option<NonEmptyString>,

    pub description: Option<NonEmptyString>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub data_type: Vec<DataType>,

    #[serde(default)]
    pub source: Option<Source>,

    pub repeated: Option<bool>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub equivalent_property: Vec<Iri>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub references: Vec<FieldRef>,

    #[serde(default)]
    pub sub_field: Vec<Field>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub parent_field: Vec<ParentField>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct FieldRef {
    pub field: Ref,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Config)]
pub enum FieldType {
    #[serde(rename = "cr:Field", alias = "Field")]
    Field,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum ParentField {
    Ref(Ref),
    Inline(FieldLike),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct FieldLike {
    #[serde(rename = "@type")]
    pub r#type: Option<FieldType>,

    #[serde(rename = "@id")]
    pub id: Option<Id>,

    #[serde(default)]
    pub source: Option<Source>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub references: Vec<Ref>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}
