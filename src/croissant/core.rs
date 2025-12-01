use chrono::DateTime;
use derive_builder::Builder;
use garde::Validate;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{borrow::Cow, fmt};

use crate::croissant::{self, errors::Error};

fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    if value.is_array() {
        Vec::<T>::deserialize(value).map_err(de::Error::custom)
    } else {
        Ok(vec![T::deserialize(value).map_err(de::Error::custom)?])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
#[garde(context(MetadataContext))]
pub struct Text(#[garde(length(min = 1))] pub Cow<'static, str>);

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self(Cow::Owned(text.to_string()))
    }
}

pub type Id = Text;

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(context(MetadataContext))]
pub enum CroissantType {
    #[serde(rename = "sc:Dataset")]
    Dataset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(context(MetadataContext))]
pub enum CrType {
    #[serde(rename = "cr:RecordSet")]
    RecordSet,
    #[serde(rename = "cr:Field")]
    Field,
    #[serde(rename = "cr:FileObject")]
    FileObject,
    #[serde(rename = "cr:FileSet")]
    FileSet,
}

impl fmt::Display for CrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            CrType::RecordSet => "cr:RecordSet",
            CrType::Field => "cr:Field",
            CrType::FileObject => "cr:FileObject",
            CrType::FileSet => "cr:FileSet",
        };
        write!(f, "{}", key)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(untagged)]
#[garde(context(MetadataContext))]
pub enum DataType {
    #[serde(rename = "sc:Enumeration")]
    Enumeration,
    #[serde(rename = "sc:Boolean")]
    Boolean,
    #[serde(rename = "sc:Integer")]
    Integer,
    #[serde(rename = "sc:Float")]
    Float,
    #[serde(rename = "sc:Text")]
    Text,
    #[serde(rename = "sc:Date")]
    Date,
    #[serde(rename = "sc:DateTime")]
    DateTime,
    #[serde(rename = "sc:URL")]
    Url,
    #[serde(rename = "sc:ImageObject")]
    ImageObject,
    #[serde(rename = "cr:BoundingBox")]
    BoundingBox(#[garde(dive)] BoundingBoxFormat),
    #[serde(rename = "cr:Split")]
    Split,
    #[serde(rename = "cr:Label")]
    Label,
    CustomIri(#[garde(dive)] Text),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            DataType::Enumeration => "sc:Enumeration",
            DataType::Boolean => "sc:Boolean",
            DataType::Integer => "sc:Integer",
            DataType::Float => "sc:Float",
            DataType::Text => "sc:Text",
            DataType::Date => "sc:Date",
            DataType::DateTime => "sc:Datetime",
            DataType::Url => "sc:Url",
            DataType::ImageObject => "sc:ImageObject",
            DataType::BoundingBox(bbox) => &format!("{}", bbox),
            DataType::Split => "cr:Split",
            DataType::Label => "cr:Label",
            DataType::CustomIri(text) => &format!("{}", text),
        };
        write!(f, "{}", key)
    }
}

impl From<&String> for DataType {
    fn from(value: &String) -> Self {
        let trimmed = value.trim();

        // Try to parse as integer
        if trimmed.parse::<i64>().is_ok() {
            return DataType::Integer;
        }

        // Try to parse as float
        if trimmed.parse::<f64>().is_ok() {
            return DataType::Float;
        }

        // Try to parse as boolean
        if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
            return DataType::Boolean;
        }

        // Try to parse as date (YYYY-MM-DD)
        if chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").is_ok() {
            return DataType::Date;
        }

        // Try to parse as ISO 8601 datetime
        if DateTime::parse_from_rfc3339(trimmed).is_ok() {
            return DataType::Date;
        }

        // Default to Text
        DataType::Text
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[garde(context(MetadataContext))]
pub enum BoundingBoxFormat {
    CenterXywh,
    Xywh,
    Xyxy,
}

impl fmt::Display for BoundingBoxFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            Self::CenterXywh => "CenterXYWH",
            Self::Xywh => "XYWH",
            Self::Xyxy => "XYXY",
        };
        write!(f, "{}", key)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(tag = "@type")]
#[garde(context(MetadataContext))]
pub enum Resource {
    #[serde(rename = "cr:FileObject")]
    #[garde(dive)]
    FileObject(#[garde(dive)] FileObject),

    #[serde(rename = "cr:FileSet")]
    #[garde(dive)]
    FileSet(#[garde(dive)] FileSet),
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            Self::FileObject(_) => "fileObject",
            Self::FileSet(_) => "fileSet",
        };
        write!(f, "{}", key)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct FileObject {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[garde(dive)]
    pub name: Text,
    #[serde(rename = "contentUrl")]
    #[garde(dive)]
    pub content_url: Text,
    #[serde(rename = "contentSize")]
    #[garde(dive)]
    pub content_size: Option<Text>,
    #[serde(rename = "encodingFormat")]
    #[garde(dive)]
    pub encoding_format: Text,
    #[garde(inner(pattern(r"^[a-fA-F0-9]{64}$")))]
    pub sha256: Option<String>,
}

impl FileObject {
    pub fn builder() -> FileObjectBuilder {
        FileObjectBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct FileSet {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[serde(rename = "containedIn")]
    #[garde(dive)]
    pub sources: Vec<Id>,
    #[serde(rename = "encodingFormat")]
    #[garde(dive)]
    pub encoding_format: Text,
    #[garde(dive)]
    pub includes: Vec<Text>,
    #[garde(dive)]
    pub excludes: Vec<Text>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(untagged)]
#[garde(context(MetadataContext))]
pub enum Extract {
    #[garde(dive)]
    Column {
        #[serde(rename = "column")]
        #[garde(dive)]
        name: Text,
    },

    #[garde(dive)]
    FileProperty {
        #[serde(rename = "fileProperty")]
        #[garde(dive)]
        property: FileProperty,
    },

    #[garde(dive)]
    JsonPath {
        #[serde(rename = "jsonPath")]
        #[garde(dive)]
        expr: Text,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[garde(context(MetadataContext))]
pub enum FileProperty {
    #[serde(rename = "fullpath")]
    FullPath,
    #[serde(rename = "filename")]
    FileName,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "lines")]
    Lines,
    #[serde(rename = "lineNumbers")]
    LineNumbers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(untagged)]
#[garde(context(MetadataContext))]
pub enum Transform {
    #[garde(dive)]
    Regex {
        #[serde(rename = "regex")]
        #[garde(dive)]
        pattern: Text,
    },

    #[garde(skip)]
    Delimiter {
        #[serde(rename = "delimiter")]
        #[garde(skip)]
        char: char,
    },

    #[garde(dive)]
    JsonQuery {
        #[serde(rename = "jsonQuery")]
        #[garde(dive)]
        query: Text,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(tag = "kind")]
#[garde(context(MetadataContext))]
pub enum ValueFormat {
    #[serde(rename = "date")]
    #[garde(dive)]
    Date {
        #[garde(dive)]
        pattern: Text,
    },

    #[serde(rename = "number")]
    #[garde(dive)]
    Number {
        #[garde(dive)]
        pattern: Text,
    },

    #[serde(rename = "bbox")]
    #[garde(dive)]
    BoundingBox {
        #[garde(dive)]
        format: BoundingBoxFormat,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct FieldSource {
    #[serde(flatten)]
    #[garde(dive)]
    pub source: SourceRef,
    #[garde(dive)]
    pub extract: Option<Extract>,
    #[garde(dive)]
    pub transform: Option<Vec<Transform>>,
    #[garde(dive)]
    pub format: Option<ValueFormat>,
}

impl FieldSource {
    pub fn builder() -> FieldSourceBuilder {
        FieldSourceBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(untagged)]
#[garde(context(MetadataContext))]
pub enum SourceRef {
    FileObject {
        #[serde(rename = "fileObject")]
        #[garde(dive)]
        file_object: Ref,
    },
    FileSet {
        #[serde(rename = "fileSet")]
        #[garde(dive)]
        file_set: Ref,
    },
    RecordSet {
        #[serde(rename = "recordSet")]
        #[garde(dive)]
        record_set: Ref,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[garde(context(MetadataContext))]
pub struct Ref {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[garde(context(MetadataContext))]
pub struct FieldRef {
    #[garde(dive)]
    pub field: Ref,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct Field {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[serde(rename = "@type")]
    #[garde(dive)]
    pub kind: CrType, // TODO: Should be on Field
    #[garde(dive)]
    pub name: Text,
    #[garde(dive)]
    pub description: Text,
    #[serde(rename = "dataType", deserialize_with = "one_or_many")]
    #[garde(dive)]
    pub data_types: Vec<DataType>,
    #[serde(rename = "source")]
    #[garde(dive)]
    pub source: FieldSource,
    #[garde(dive)]
    #[serde(rename = "references", default, deserialize_with = "one_or_many")]
    pub references: Vec<FieldRef>,
    #[serde(rename = "subField")]
    #[garde(dive)]
    pub sub_fields: Option<Vec<Field>>,
    #[serde(rename = "parentField")]
    #[garde(dive)]
    pub parent_fields: Option<Vec<Id>>,
    #[garde(skip)]
    pub repeated: Option<bool>,
    #[garde(dive)]
    #[serde(rename = "equivalentProperty")]
    pub equivalent_properties: Option<Vec<Text>>,
}

impl Field {
    pub fn builder() -> FieldBuilder {
        FieldBuilder::default()
    }
}

pub fn validate_record_set_references(
    record_set: &RecordSet,
    ctx: &MetadataContext,
) -> garde::Result {
    for field in &record_set.fields {
        let ref_id = match &field.source.source {
            SourceRef::FileObject { file_object } => file_object.id.clone(),
            SourceRef::RecordSet { record_set } => record_set.id.clone(),
            SourceRef::FileSet { file_set } => file_set.id.clone(),
        };

        if !ref_id.0.is_empty() && !ctx.distribution_ids.contains(&ref_id) {
            return Err(garde::Error::new(format!(
                "Field '{}' references non-existent distribution id '{}'",
                field.name, ref_id
            )));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct RecordSet {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[serde(rename = "@type")]
    #[garde(dive)] // TODO: Should be only one value of CrType
    pub kind: CrType,
    #[serde(rename = "key", default, deserialize_with = "one_or_many")]
    #[garde(dive)]
    pub keys: Vec<Ref>,
    #[serde(rename = "field")]
    #[garde(dive)]
    pub fields: Vec<Field>,
    #[serde(rename = "dataType", default, deserialize_with = "one_or_many")]
    #[garde(dive)]
    pub record_types: Vec<DataType>,
}

impl RecordSet {
    pub fn builder() -> RecordSetBuilder {
        RecordSetBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct Distribution {
    #[serde(flatten)]
    #[garde(dive)]
    pub resource: Resource,
}

impl Distribution {
    pub fn builder() -> DistributionBuilder {
        DistributionBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct Context {
    #[serde(rename = "@language")]
    #[garde(dive)]
    pub language: Text,
    #[serde(rename = "@vocab")]
    #[garde(dive)]
    pub vocab: Text,
    #[serde(rename = "sc")]
    #[garde(dive)]
    pub sc: Text,
    #[serde(rename = "cr")]
    #[garde(dive)]
    pub cr: Text,
    #[serde(rename = "dct")]
    #[garde(dive)]
    pub dct: Text,
    #[serde(rename = "citeAs")]
    #[garde(dive)]
    pub cite_as: Text,
    #[serde(rename = "column")]
    #[garde(dive)]
    pub column: Text,
    #[serde(rename = "conformsTo")]
    #[garde(dive)]
    pub conforms_to: Text,
    #[serde(rename = "data")]
    #[garde(dive)]
    pub data: DataContext,
    #[serde(rename = "dataType")]
    #[garde(dive)]
    pub data_type: DataTypeContext,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }
}

pub fn default_context() -> Result<Context, croissant::errors::Error> {
    Ok(Context::builder()
        .language(Text::new("en"))
        .vocab(Text::new("https://schema.org/"))
        .cite_as(Text::new("cr:citeAs"))
        .column(Text::new("cr:column"))
        .conforms_to(Text::new("dct:conforms_to"))
        .cr(Text::new("http://purl.org/dc/terms/"))
        .data(
            DataContext::builder()
                .id(Text::new("cr:data"))
                .r#type(Text::new("@json"))
                .build()
                .map_err(|e| Error::Builder(e.to_string()))?,
        )
        .data_type(
            DataTypeContext::builder()
                .id(Text::new("cr:DataType"))
                .r#type(Text::new("@vocal"))
                .build()
                .map_err(|e| Error::Builder(e.to_string()))?,
        )
        .sc(Text::new("https://schema.org/"))
        .build()
        .map_err(|e| Error::Builder(e.to_string()))?)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct DataContext {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[serde(rename = "@type")]
    #[garde(dive)]
    pub r#type: Text,
}

impl DataContext {
    pub fn builder() -> DataContextBuilder {
        DataContextBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct DataTypeContext {
    #[serde(rename = "@id")]
    #[garde(dive)]
    pub id: Id,
    #[serde(rename = "@type")]
    #[garde(dive)]
    pub r#type: Text,
}

impl DataTypeContext {
    pub fn builder() -> DataTypeContextBuilder {
        DataTypeContextBuilder::default()
    }
}

#[derive(Clone, Default, Debug)]
pub struct MetadataContext {
    distribution_ids: HashSet<Id>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[garde(context(MetadataContext))]
pub struct Metadata {
    #[serde(rename = "@context")]
    #[garde(dive)]
    pub context: Context,
    #[serde(rename = "@type")]
    #[garde(dive)]
    pub kind: CroissantType,
    #[garde(dive)]
    pub name: Text,
    #[garde(dive)]
    pub description: Text,
    #[serde(rename = "conformsTo")]
    #[garde(dive)]
    pub conforms_to: Text,
    #[serde(rename = "datePublished")]
    #[garde(dive)]
    pub date_published: Option<Text>,
    #[garde(dive)]
    pub version: Text,
    #[garde(dive)]
    pub distribution: Vec<Distribution>,
    #[serde(rename = "recordSet")]
    #[garde(length(min = 1), inner(custom(validate_record_set_references)))]
    pub record_sets: Vec<RecordSet>,
}

impl Metadata {
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::default()
    }
}

impl Metadata {
    pub fn check(&self) -> Result<(), garde::Report> {
        let ctx = MetadataContext {
            distribution_ids: self
                .distribution
                .iter()
                .map(|d| match &d.resource {
                    Resource::FileObject(o) => o.id.clone(),
                    Resource::FileSet(s) => s.id.clone(),
                })
                .collect(),
        };

        self.validate_with(&ctx)
    }
}
