use crate::{
    config_enum, config_struct,
    specs::{
        agent::Agent,
        common::{DatasetType, Iri, JsonObject, NonEmptyString},
        record::RecordSet,
        resource::Resource,
        serde::one_or_many,
    },
};
use chrono::NaiveDate;
use schematic::Config;

#[derive(Default)]
pub struct Context;

config_struct!(
    #[derive(Config)]
    #[config(context = Context)]
    pub struct Dataset {
        #[serde(rename = "@context")]
        pub context: crate::specs::context::Context,

        #[serde(rename = "@type")]
        #[setting(default)]
        pub r#type: DatasetType,

        #[setting(validate = schematic::validate::url, default = "http://mlcommons.org/croissant/1.0")]
        pub conforms_to: String,

        pub name: NonEmptyString,

        pub description: NonEmptyString,

        pub license: StringOrUrl,

        #[setting(validate = schematic::validate::url)]
        pub url: Option<String>,

        pub creator: Agent,

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

        pub version: Option<semver::Version>,

        pub date_created: Option<NaiveDate>,

        pub date_modified: Option<NaiveDate>,

        #[serde(default, deserialize_with = "one_or_many")]
        pub same_as: Vec<Iri>,

        #[serde(default, deserialize_with = "one_or_many")]
        pub sd_license: Vec<StringOrUrl>,

        #[serde(default, deserialize_with = "one_or_many")]
        pub in_language: Vec<Language>,

        pub is_live_dataset: Option<bool>,

        pub cite_as: Option<NonEmptyString>,

        #[serde(default)]
        pub alternate_name: Vec<String>,

        #[serde(flatten, default)]
        pub extra: JsonObject,
    }
);

config_struct!(
    #[derive(Config)]
    pub struct DefinedTerm {
        pub name: Option<NonEmptyString>,

        pub description: Option<NonEmptyString>,

        #[setting(validate = schematic::validate::url)]
        pub url: Option<String>,

        #[serde(flatten, default)]
        pub extra: JsonObject,
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum Keyword {
        DefinedTerm(DefinedTerm),
        Text(NonEmptyString),
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum Language {
        Text(NonEmptyString),
        Object(JsonObject),
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum StringOrUrl {
        String(NonEmptyString),
        #[setting(validate = schematic::validate::url)]
        Url(String),
    }
);
