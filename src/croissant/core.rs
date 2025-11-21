use chrono::DateTime;
use derive_builder::{self, Builder};
use serde;
use serde::{Deserialize, Serialize};

const CR_PREFIX: &str = "cr:";
const SC_PREFIX: &str = "sc:";

/// Field represents a field in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct Field {
    #[serde(rename = "@id")]
    #[garde(length(min = 1))]
    pub id: String,
    #[serde(rename = "@type")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub r#type: String,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub description: String,
    #[serde(rename = "dataType")]
    #[garde(length(min = 1))]
    pub data_type: String,
    #[garde(dive)]
    pub source: FieldSource,
}

impl Field {
    pub fn builder() -> FieldBuilder {
        FieldBuilder::default()
    }
}

/// FieldSource represents the source information for a field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct FieldSource {
    #[garde(dive)]
    pub extract: Extract,
    #[serde(rename = "fileObject")]
    #[garde(dive)]
    pub file_object: FileObject,
}

impl FieldSource {
    pub fn builder() -> FieldSourceBuilder {
        FieldSourceBuilder::default()
    }
}

/// Extract represents the extraction information for a field source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct Extract {
    #[garde(length(min = 1))]
    pub column: String,
}

impl Extract {
    pub fn builder() -> ExtractBuilder {
        ExtractBuilder::default()
    }
}

/// FileObject represents a file object reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct FileObject {
    #[serde(rename = "@id")]
    #[garde(length(min = 1))]
    pub id: String,
}

impl FileObject {
    pub fn builder() -> FileObjectBuilder {
        FileObjectBuilder::default()
    }
}

/// Distribution represents a file in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct Distribution {
    #[serde(rename = "@id")]
    #[garde(length(min = 1))]
    pub id: String,
    #[serde(rename = "@type")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub r#type: String,
    #[garde(length(min = 1))]
    pub name: String,
    #[serde(rename = "contentSize")]
    #[garde(length(min = 1))] // TODO: validate number
    pub content_size: String,
    #[serde(rename = "contentUrl")]
    #[garde(url)]
    pub content_url: String,
    #[serde(rename = "encodingFormat")]
    #[garde(length(min = 1))]
    pub encoding_format: String,
    #[garde(length(min = 1))]
    pub sha256: String,
}

impl Distribution {
    pub fn builder() -> DistributionBuilder {
        DistributionBuilder::default()
    }
}

/// RecordSet represents a record set in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct RecordSet {
    #[serde(rename = "@id")]
    #[garde(length(min = 1))]
    pub id: String,
    #[serde(rename = "@type")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub r#type: String,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub description: String,
    #[garde(length(min = 0), dive)]
    pub field: Vec<Field>,
}

impl RecordSet {
    pub fn builder() -> RecordSetBuilder {
        RecordSetBuilder::default()
    }
}

/// Context represents the JSON-LD context in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct Context {
    #[serde(rename = "@language")]
    #[garde(length(min = 1))]
    pub language: String,
    #[serde(rename = "@vocab")]
    #[garde(url)]
    pub vocab: String,
    #[serde(rename = "citeAs")]
    #[garde(skip)] // TODO: validate BibTeX
    pub cite_as: String,
    #[garde(skip)]
    pub column: String,
    #[serde(rename = "conformsTo")]
    #[garde(url)]
    pub conforms_to: String,
    #[garde(url)]
    pub cr: String,
    #[garde(url)]
    pub dct: String,
    #[garde(dive)]
    pub data: DataContext,
    #[serde(rename = "dataType")]
    #[garde(dive)]
    pub data_type: DataTypeContext,
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub extract: String,
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub field: String,
    #[serde(rename = "fileObject")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub file_object: String,
    #[serde(rename = "fileProperty")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub file_property: String,
    #[garde(url)]
    pub sc: String,
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub source: String,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }
}

/// DataContext represents the data field in the context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct DataContext {
    #[serde(rename = "@id")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub id: String,
    #[serde(rename = "@type")]
    #[garde(prefix("@"), length(min = 1))]
    pub r#type: String,
}

impl DataContext {
    pub fn builder() -> DataContextBuilder {
        DataContextBuilder::default()
    }
}

/// DataTypeContext represents the dataType field in the context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct DataTypeContext {
    #[serde(rename = "@id")]
    #[garde(prefix(CR_PREFIX), length(min = 1))]
    pub id: String,
    #[serde(rename = "@type")]
    #[garde(prefix("@"), length(min = 1))]
    pub r#type: String,
}

impl DataTypeContext {
    pub fn builder() -> DataTypeContextBuilder {
        DataTypeContextBuilder::default()
    }
}

/// Metadata represents the complete Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, garde::Validate, Builder)]
pub struct Metadata {
    #[serde(rename = "@context")]
    #[garde(dive)]
    pub context: Context,
    #[serde(rename = "@type")]
    #[garde(prefix(SC_PREFIX), length(min = 1))]
    pub r#type: String,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub description: String,
    #[serde(rename = "conformsTo")]
    #[garde(url)]
    pub conforms_to: String,
    #[serde(rename = "datePublished")]
    #[garde(skip)] // TODO: validate date
    pub date_published: String,
    #[garde(skip)] // TODO: validate SemVer
    pub version: String,
    #[garde(length(min = 0), dive)]
    pub distribution: Vec<Distribution>,
    #[serde(rename = "recordSet")]
    #[garde(length(min = 0), dive)]
    pub record_set: Vec<RecordSet>,
}

impl Metadata {
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::default()
    }
}

// ============================================================================
// Data Type Inference
// ============================================================================

/// Supported data types for Croissant fields
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Date,
    Boolean,
}

impl DataType {
    /// Convert to schema.org data type string
    pub fn to_schema_org(&self) -> &'static str {
        match self {
            DataType::Integer => "sc:Integer",
            DataType::Float => "sc:Float",
            DataType::Text => "sc:Text",
            DataType::Date => "sc:Date",
            DataType::Boolean => "sc:Boolean",
        }
    }
}

/// Infer the data type from a value string
pub fn infer_data_type(value: &str) -> DataType {
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

// ============================================================================
// Context Creation
// ============================================================================

/// Create the default context for Croissant metadata
pub fn create_default_context() -> Context {
    Context::builder()
        .language("en".to_string())
        .vocab("https://schema.org/".to_string())
        .cite_as("cr:citeAs".to_string())
        .column("cr:column".to_string())
        .conforms_to("dct:conforms_to".to_string())
        .cr("http://purl.org/dc/terms/".to_string())
        .data(
            DataContext::builder()
                .id("cr:data".to_string())
                .r#type("@json".to_string())
                .build()
                .unwrap(), // TODO: error,
        )
        .data_type(
            DataTypeContext::builder()
                .id("cr:DataType".to_string())
                .r#type("@vocal".to_string())
                .build()
                .unwrap(), // TODO:
                           // error handling
        )
        .extract("cr:extract".to_string())
        .field("cr:field".to_string())
        .file_object("cr:fileObject".to_string())
        .file_property("cr:fileProperty".to_string())
        .sc("https://schema.org/".to_string())
        .source("cr:source".to_string())
        .build()
        .unwrap() // TODO: real error handling
}
