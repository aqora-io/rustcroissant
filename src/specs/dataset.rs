use crate::{
    config_enum, config_struct,
    specs::{
        agent::Agent,
        common::{DatasetType, Iri, JsonObject, NonEmptyString},
        record::RecordSet,
        resource::Resource,
    },
};
use chrono::NaiveDate;
use schematic::{Config, Schema, SchemaBuilder, Schematic, schema::UnionType};

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
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub license: crate::specs::OneOrMany<StringOrUrl>,

        /// The name of the dataset.
        pub name: NonEmptyString,

        /// The URL of the dataset. This generally corresponds to the Web page
        /// for the dataset.
        #[setting(required)]
        pub url: Option<url::Url>,

        /// The creator(s) of the dataset.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub creator: crate::specs::OneOrMany<Agent>,

        /// The date the dataset was published.
        pub date_published: NaiveDate,

        /// A set of keywords associated with the dataset, either as free text,
        /// or a DefinedTerm with a formal definition.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub keywords: crate::specs::OneOrMany<Keyword>,

        /// The publisher of the dataset, which may be distinct from its creator.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub publisher: crate::specs::OneOrMany<Agent>,

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
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub same_as: crate::specs::OneOrMany<Iri>,

        /// A license document that applies to this structured data, typically indicated
        /// by URL.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub sd_license: crate::specs::OneOrMany<StringOrUrl>,

        /// The language(s) of the content of the dataset.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub in_language: crate::specs::OneOrMany<Language>,

        /// By contrast with [schema.org/Dataset](http://schema.org/Dataset),
        /// Croissant requires the distribution property to have values of type
        /// [FileObject](https://github.com/mlcommons/croissant/blob/main/docs/
        /// croissant-spec-1.1.md#fileobject) or
        /// [FileSet](https://github.com/mlcommons/croissant/blob/main/docs/
        /// croissant-spec-1.1.md#fileset). These are subclasses of
        /// [DataDownload](http://schema.org/DataDownload), so this definition
        /// is compatible with the original definition of the distribution
        /// property in schema.org.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        #[setting(validate = crate::specs::validate::validate_distribution)]
        pub distribution: crate::specs::OneOrMany<Resource>,

        /// Whether the dataset is a live dataset.
        pub is_live_dataset: Option<bool>,

        /// A citation to the dataset itself, or a citation for a publication
        /// that describes the dataset. Ideally, citations should be expressed
        /// using the [BibTeX](https://www.bibtex.org/) format. Note that this is
        /// different from schema.org/citation, which is used to make a
        /// citation to another publication from this dataset.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[setting(validate = crate::specs::validate::validate_bibtex)]
        pub cite_as: Option<NonEmptyString>,

        /// The version of the dataset metadata, which may be distinct from the
        /// version of the dataset content. This property is modeled after
        /// [schema.org/sdLicense](https://schema.org/sdLicense) and
        /// [schema.org/sdPublisher](https://schema.org/sdPublisher), and may
        /// move to schema.org in the future.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sd_version: Option<semver::Version>,

        #[serde(
            default,
            skip_serializing_if = "crate::specs::OneOrMany::is_empty",
            rename = "recordSet"
        )]
        #[setting(validate = crate::specs::validate::validate_record_sets)]
        pub record_sets: crate::specs::OneOrMany<RecordSet>,

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
    #[serde(untagged)]
    pub enum Keyword {
        Url(url::Url),
        Text(NonEmptyString),
        DefinedTerm(DefinedTerm),
    }
);

impl Default for Keyword {
    fn default() -> Self {
        Self::Text(NonEmptyString::default())
    }
}

impl Schematic for Keyword {
    fn build_schema(mut schema: SchemaBuilder) -> Schema {
        schema.union(UnionType::new_one([
            schema.infer::<url::Url>(),
            schema.infer::<NonEmptyString>(),
            schema.infer::<DefinedTerm>(),
        ]))
    }
}

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum Language {
        Text(NonEmptyString),
        Object(JsonObject),
    }
);

config_enum!(
    #[serde(untagged)]
    pub enum StringOrUrl {
        Url(url::Url),
        String(NonEmptyString),
    }
);

impl Default for StringOrUrl {
    fn default() -> Self {
        StringOrUrl::String(NonEmptyString::default())
    }
}

impl Schematic for StringOrUrl {
    fn build_schema(mut schema: SchemaBuilder) -> Schema {
        schema.union(UnionType::new_one([
            schema.infer::<url::Url>(),
            schema.infer::<NonEmptyString>(),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specs::context;
    use chrono::Datelike;
    use proptest::prelude::*;
    use serde_json::{Value, json};

    struct DatasetBuilder {
        extra: serde_json::Map<String, Value>,
    }

    impl DatasetBuilder {
        fn new() -> Self {
            Self {
                extra: Default::default(),
            }
        }

        fn field(mut self, key: &str, value: impl Into<Value>) -> Self {
            self.extra.insert(key.to_string(), value.into());
            self
        }

        fn base_json(&self) -> Value {
            let mut base = json!({
                "@context": context::Context::default(),
                "@type": "sc:Dataset",
                "name": "Dataset",
                "description": "Description",
                "url": "https://example.com",
                "datePublished": "2024-01-01"
            });

            base.as_object_mut().unwrap().extend(self.extra.clone());

            base
        }

        fn build(self) -> Dataset {
            serde_json::from_value(self.base_json()).expect("dataset should deserialize")
        }
    }

    macro_rules! assert_variant {
        ($value:expr, $pat:pat) => {
            assert!(
                matches!($value, $pat),
                "expected {}, got {:?}",
                stringify!($pat),
                $value
            );
        };
    }

    #[test]
    fn license_string_variant() {
        let dataset = DatasetBuilder::new().field("license", "MIT").build();
        assert_variant!(&dataset.license[0], StringOrUrl::String(NonEmptyString(_)));
    }

    #[test]
    fn license_url_variant() {
        let dataset = DatasetBuilder::new()
            .field("license", "https://spdx.org/licenses/MIT.html")
            .build();

        assert_variant!(&dataset.license[0], StringOrUrl::Url(_));
    }

    #[test]
    fn string_or_url_array() {
        let dataset = DatasetBuilder::new()
            .field("license", json!(["MIT", "https://example.com"]))
            .build();

        assert_eq!(dataset.license.len(), 2);
    }

    #[test]
    fn keyword_variants_and_shape() {
        let dataset = DatasetBuilder::new()
            .field(
                "keywords",
                json!([
                    "text",
                    { "name": "Defined", "description": "desc" },
                    "https://example.com/keyword"
                ]),
            )
            .build();

        match &dataset.keywords[..] {
            [
                Keyword::Text(_),
                Keyword::DefinedTerm(term),
                Keyword::Url(_),
            ] => {
                assert!(term.name.is_some());
                assert!(term.description.is_some());
            }
            keywords => panic!("unexpected keyword variants: {keywords:?}"),
        }
    }

    #[test]
    fn keyword_single_value_coerces() {
        let dataset = DatasetBuilder::new().field("keywords", "ml").build();

        assert_eq!(dataset.keywords.len(), 1);
        assert_variant!(dataset.keywords[0], Keyword::Text(_));
    }

    #[test]
    fn language_text_and_object() {
        let dataset = DatasetBuilder::new()
            .field(
                "inLanguage",
                json!([
                    "en",
                    { "id": "http://id.loc.gov/vocabulary/iso639-1/en" }
                ]),
            )
            .build();

        assert_eq!(dataset.in_language.len(), 2);
    }

    #[test]
    fn optional_dates_and_versions_parse() {
        let dataset = DatasetBuilder::new()
            .field("version", "1.2.3")
            .field("sdVersion", "0.1.0")
            .field("dateCreated", "2020-01-01")
            .field("dateModified", "2021-01-01")
            .build();

        assert_eq!(dataset.version.unwrap().to_string(), "1.2.3");
        assert!(dataset.date_created.is_some());
        assert!(dataset.date_modified.is_some());
    }

    #[test]
    fn same_as_array() {
        let dataset = DatasetBuilder::new()
            .field(
                "sameAs",
                json!(["https://example.com/a", "https://example.com/b"]),
            )
            .build();

        assert_eq!(dataset.same_as.len(), 2);
    }

    #[test]
    fn creator_and_publisher() {
        let dataset = DatasetBuilder::new()
            .field(
                "creator",
                json!({
                    "@type": "Person",
                    "name": "Alice"
                }),
            )
            .field(
                "publisher",
                json!([{
                    "@type": "Organization",
                    "name": "Org"
                }]),
            )
            .build();

        assert_eq!(dataset.creator.len(), 1);
        assert_eq!(dataset.publisher.len(), 1);
    }

    #[test]
    fn defaults_apply_on_deserialize() {
        let dataset = DatasetBuilder::new().build();

        assert_eq!(dataset.r#type, DatasetType::Dataset);
        assert!(dataset.creator.is_empty());
        assert!(dataset.publisher.is_empty());
        assert!(dataset.keywords.is_empty());
    }

    #[test]
    fn serde_roundtrip_preserves_shape() {
        let dataset = DatasetBuilder::new()
            .field("keywords", json!(["a", "b"]))
            .field("inLanguage", "en")
            .build();

        let json = serde_json::to_value(&dataset).unwrap();
        let back: Dataset = serde_json::from_value(json).unwrap();

        assert_eq!(back.keywords.len(), 2);
        assert_eq!(back.in_language.len(), 1);
    }

    proptest! {
        #[test]
        fn prop_string_or_url_classification(license in ".{1,100}") {
            let dataset = DatasetBuilder::new()
                .field("license", license.clone())
                .build();

            match &dataset.license[0] {
                StringOrUrl::String(v) => prop_assert_eq!(v,  &NonEmptyString::new( license)),
                StringOrUrl::Url(u) => {
                    let reparsed = url::Url::parse(&license)
                        .expect("string classified as Url must parse as Url");

                    prop_assert_eq!(u.as_str(), reparsed.as_str());
                },
            }
        }

        #[test]
        fn prop_keyword_text_roundtrip(keyword in ".{1,50}") {
            let dataset = DatasetBuilder::new()
                .field("keywords", keyword.clone())
                .build();

            prop_assert_eq!(dataset.keywords.len(), 1);

            match &dataset.keywords[0] {
                Keyword::Text(text) =>  prop_assert_eq!(text,  &NonEmptyString::new(keyword)),
                Keyword::DefinedTerm(term) => prop_assert_eq!(term,  &DefinedTerm::default()),
                Keyword::Url(url) => {
                    let reparsed = url::Url::parse(&keyword)
                        .expect("string classified as Url must parse as Url");

                    prop_assert_eq!(url.as_str(), reparsed.as_str());

                },
            }
        }

        #[test]
        fn prop_language_text(lang in ".{1,10}") {
            let dataset = DatasetBuilder::new()
                .field("inLanguage", lang)
                .build();

            prop_assert_eq!(dataset.in_language.len(), 1);
        }

        #[test]
        fn prop_semver_roundtrip(
            major in 0u64..10,
            minor in 0u64..10,
            patch in 0u64..10
        ) {
            let version = format!("{major}.{minor}.{patch}");

            let dataset = DatasetBuilder::new()
                .field("version", version.clone())
                .build();

            prop_assert_eq!(dataset.version.unwrap().to_string(), version);
        }

        #[test]
        fn prop_dates_valid(year in 1970i32..2100, month in 1u32..12, day in 1u32..28) {
            let date = format!("{year:04}-{month:02}-{day:02}");

            let dataset = DatasetBuilder::new()
                .field("datePublished", date)
                .build();

            prop_assert!(dataset.date_published.year() >= 1970);
        }
    }
}
