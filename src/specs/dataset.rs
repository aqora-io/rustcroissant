use chrono::NaiveDate;
use schematic::Config;
use serde::{Deserialize, Serialize};

use crate::specs::{
    agent::Agent,
    common::{DatasetType, Iri, IriOrObject, JsonObject, NonEmptyString},
    record::RecordSet,
    resource::Resource,
    serde::one_or_many,
};

#[derive(Default)]
pub struct Context;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[config(context = Context)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    #[serde(rename = "@context")]
    pub context: crate::specs::context::Context,

    #[serde(rename = "@type")]
    pub r#type: DatasetType,

    #[setting(validate = schematic::validate::url, default = "http://mlcommons.org/croissant/1.0")]
    pub conforms_to: String,

    pub name: NonEmptyString,

    pub description: NonEmptyString,

    #[serde(deserialize_with = "one_or_many")]
    #[setting(validate = schematic::validate::min_length(1))]
    pub license: Vec<IriOrObject>,

    #[setting(validate = schematic::validate::url)]
    pub url: String,

    #[serde(deserialize_with = "one_or_many")]
    pub creator: Vec<Agent>,

    pub date_published: NaiveDate,

    #[serde(deserialize_with = "one_or_many")]
    #[setting(validate = crate::specs::validate::validate_distribution)]
    pub distribution: Vec<Resource>,

    #[serde(rename = "recordSet", default, deserialize_with = "one_or_many")]
    #[setting(validate = crate::specs::validate::validate_record_sets)]
    pub record_sets: Vec<RecordSet>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub keywords: Vec<Keyword>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub publisher: Vec<Agent>,

    pub version: Option<NonEmptyString>,

    pub date_created: Option<NaiveDate>,

    pub date_modified: Option<NaiveDate>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub same_as: Vec<Iri>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub sd_license: Vec<IriOrObject>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub in_language: Vec<Language>,

    pub is_live_dataset: Option<bool>,

    pub cite_as: Option<NonEmptyString>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
pub struct DefinedTerm {
    pub name: Option<NonEmptyString>,

    pub description: Option<NonEmptyString>,

    #[setting(validate = schematic::validate::url)]
    pub url: Option<String>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum Keyword {
    DefinedTerm(DefinedTerm),
    Text(NonEmptyString),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum Language {
    Text(NonEmptyString),
    Object(JsonObject),
}
