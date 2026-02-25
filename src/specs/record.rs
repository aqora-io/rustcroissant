use crate::specs::{
    common::{Id, Iri, JsonObject, NonEmptyString, Ref},
    data_type::DataType,
    source::Source,
};
use schematic::{Config, ConfigEnum};

config_struct!(
    #[derive(Config)]
    pub struct RecordSet {
        #[serde(rename = "@type")]
        pub r#type: RecordSetType,

        #[serde(rename = "@id")]
        pub id: Id,

        /// The source of data for the `FileSet`, e.g., an archive. If a
        /// `FileSet` or multiple values are provided for `containedIn`, then
        /// the union of their contents is taken (e.g., this can be used to
        /// combine files from multiple archives). A `DataSource` can also be
        /// used in case the data needs to be filtered or transformed.
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        #[setting(validate = schematic::validate::min_length(1))]
        pub field: Vec<Field>,

        /// One or more fields whose values uniquely identify each record in
        /// the `RecordSet`. (See example below.)
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub key: crate::specs::OneOrMany<Ref>,

        /// One or more records that constitute the data of the `RecordSet`.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub data: crate::specs::OneOrMany<JsonObject>,

        /// One or more records provided as example content of the `RecordSet`,
        /// or a reference to data source that contains examples.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub examples: crate::specs::OneOrMany<RecordSetExample>,

        /// One or more data-level annotations that apply to the entire record.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub annotation: crate::specs::OneOrMany<Field>,

        pub name: Option<NonEmptyString>,

        pub description: Option<NonEmptyString>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub data_type: Vec<DataType>,
    }
);

config_unit_enum!(
    #[derive(ConfigEnum)]
    pub enum RecordSetType {
        #[serde(rename = "cr:RecordSet", alias = "RecordSet")]
        #[default]
        RecordSet,
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum RecordSetExample {
        Inline(JsonObject),
        Source(Source),
        Url(url::Url),
    }
);

config_struct!(
    #[derive(Config)]
    pub struct Field {
        #[serde(rename = "@type")]
        pub r#type: FieldType,

        #[serde(rename = "@id")]
        pub id: Id,

        /// The data source of the field. This will generally reference a
        /// `FileObject` or `FileSet`'s contents (e.g., a specific column of a
        /// table).
        #[serde(default)]
        pub source: Option<Source>,

        /// The data type of the field, identified by the URI of the
        /// corresponding class. It could be either an atomic type (e.g.,
        /// `sc:Integer`) or a semantic type (e.g., `sc:GeoLocation`).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub data_type: Vec<DataType>,

        /// An optional constant value for the field. Fields with values can be
        /// used to attach key/value pairs to a RecordSet. The value of a field
        /// can be atomic, for fields with a simple dataType, or it can be
        /// structured, e.g., if the field has subfields. For the latter case, a
        /// JSON string can be used to represent the value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<JsonObject>,

        /// If true, then the Field is an array of values of type dataType. If
        /// `arrayShape` is not specified, it will default to `(-1,)`, i.e. a
        /// one-dimensional array of unknown shape.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[setting(default = false)]
        pub is_array: Option<bool>,

        /// The shape of the array as a comma-separated string. `-1` indicates
        /// dimensions of unknown/unspecified size. `(-1,)` represents a simple
        /// list. If specified, then `is_array` must be True.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub array_shape: Option<Vec<String>>,

        /// A property that is equivalent to this Field. Used in the case a
        /// dataType is specified on the RecordSet to map specific fields to
        /// specific properties associated with that dataType.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub equivalent_property: crate::specs::OneOrMany<Iri>,

        /// Another `Field` of another `RecordSet` that this field references.
        /// This is the equivalent of a foreign key reference in a
        /// relational database.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub references: crate::specs::OneOrMany<FieldRef>,

        /// Another `Field` that is nested inside this one.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub sub_field: Vec<Field>,

        /// A special case of `SubField` that should be hidden because it references
        /// a `Field` that already appears in the `RecordSet`.
        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub parent_field: crate::specs::OneOrMany<ParentField>,

        /// One or more data-level annotations that apply to the entire record.
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub annotation: Vec<Field>,

        pub name: Option<NonEmptyString>,

        pub description: Option<NonEmptyString>,

        pub repeated: Option<bool>,
    }
);

config_struct!(
    #[derive(Config)]
    pub struct FieldRef {
        pub field: Ref,
    }
);

config_unit_enum!(
    #[derive(ConfigEnum)]
    pub enum FieldType {
        #[serde(rename = "cr:Field", alias = "Field")]
        #[default]
        Field,
    }
);

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum ParentField {
        Ref(Ref),
        Inline(FieldLike),
    }
);

config_struct!(
    #[derive(Config)]
    pub struct FieldLike {
        #[serde(rename = "@type")]
        pub r#type: Option<FieldType>,

        #[serde(rename = "@id")]
        pub id: Option<Id>,

        #[serde(default)]
        pub source: Option<Source>,

        #[serde(skip_serializing_if = "crate::specs::OneOrMany::is_empty", default)]
        pub references: crate::specs::OneOrMany<Ref>,
    }
);
