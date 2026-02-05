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
        /// A set of JSON-LD context definitions that make the rest of the
        /// Croissant description less verbose.
        ///
        /// For the recommended JSON-LD context, see
        /// [Appendix 1](https://github.com/mlcommons/croissant#appendix-1).
        #[serde(rename = "@context")]
        pub context: crate::specs::context::Context,

        /// The type of a croissant dataset must be
        /// [schema.org/Dataset](http://schema.org/Dataset).
        #[serde(rename = "@type")]
        #[setting(default)]
        pub r#type: DatasetType,

        /// Croissant datasets must declare that they conform to the versioned
        /// schema, e.g. http://mlcommons.org/croissant/1.1. In case a dataset
        /// conforms to multiple specifications, those can be added in the form
        /// of a list.
        #[setting(default = "http://mlcommons.org/croissant/1.1")]
        pub conforms_to: Option<url::Url>,

        /// Description of the dataset.
        pub description: NonEmptyString,

        /// The license of the dataset. Croissant recommends using the URL of a
        /// known license, e.g., one of the licenses listed at
        /// https://spdx.org/licenses/.
        #[serde(deserialize_with = "one_or_many")]
        pub license: Vec<StringOrUrl>,

        /// The name of the dataset.
        pub name: NonEmptyString,

        /// The URL of the dataset. This generally corresponds to the Web page
        /// for the dataset.
        #[setting(required)]
        pub url: Option<url::Url>,

        /// The creator(s) of the dataset.
        #[serde(default, deserialize_with = "one_or_many")]
        pub creator: Vec<Agent>,

        /// The date the dataset was published.
        pub date_published: NaiveDate,

        /// A set of keywords associated with the dataset, either as free text,
        /// or a DefinedTerm with a formal definition.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub keywords: Vec<Keyword>,

        /// The publisher of the dataset, which may be distinct from its creator.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub publisher: Vec<Agent>,

        /// The version of the dataset following the requirements below.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<semver::Version>,

        /// The date the dataset was initially created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_created: Option<NaiveDate>,

        /// The date the dataset was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_modified: Option<NaiveDate>,

        /// The URL of another Web resource that represents the same dataset as
        /// this one.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub same_as: Vec<Iri>,

        /// A license document that applies to this structured data, typically indicated
        /// by URL.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub sd_license: Vec<StringOrUrl>,

        /// The language(s) of the content of the dataset.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub in_language: Vec<Language>,

        /// By contrast with [schema.org/Dataset](http://schema.org/Dataset),
        /// Croissant requires the distribution property to have values of type
        /// [FileObject](https://github.com/mlcommons/croissant/blob/main/docs/
        /// croissant-spec-1.1.md#fileobject) or
        /// [FileSet](https://github.com/mlcommons/croissant/blob/main/docs/
        /// croissant-spec-1.1.md#fileset). These are subclasses of
        /// [DataDownload](http://schema.org/DataDownload), so this definition
        /// is compatible with the original definition of the distribution
        /// property in schema.org.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            deserialize_with = "one_or_many"
        )]
        #[setting(validate = crate::specs::validate::validate_distribution)]
        pub distribution: Vec<Resource>,

        /// Whether the dataset is a live dataset.
        pub is_live_dataset: Option<bool>,

        /// A citation to the dataset itself, or a citation for a publication
        /// that describes the dataset. Ideally, citations should be expressed
        /// using the [BibTeX](https://www.bibtex.org/) format. Note that this is
        /// different from schema.org/citation, which is used to make a
        /// citation to another publication from this dataset.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cite_as: Option<NonEmptyString>,

        /// The version of the dataset metadata, which may be distinct from the
        /// version of the dataset content. This property is modeled after
        /// [schema.org/sdLicense](https://schema.org/sdLicense) and
        /// [schema.org/sdPublisher](https://schema.org/sdPublisher), and may
        /// move to schema.org in the future.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sd_version: Option<semver::Version>,

        #[serde(
            skip_serializing_if = "Vec::is_empty",
            rename = "recordSet",
            default,
            deserialize_with = "one_or_many"
        )]
        #[setting(validate = crate::specs::validate::validate_record_sets)]
        pub record_sets: Vec<RecordSet>,

        #[serde(default)]
        pub alternate_name: Vec<String>,
    }
);

config_struct!(
    #[derive(Config)]
    pub struct DefinedTerm {
        pub name: Option<NonEmptyString>,

        pub description: Option<NonEmptyString>,

        pub url: Option<url::Url>,
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum Keyword {
        DefinedTerm(DefinedTerm),
        Text(NonEmptyString),
        Url(url::Url),
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
        Url(url::Url),
    }
);
