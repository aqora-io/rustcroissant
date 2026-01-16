use schematic::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Config)]
pub enum CrTerm {
    #[serde(rename = "cr:citeAs", alias = "citeAs")]
    CiteAs,
    #[serde(rename = "cr:column", alias = "column")]
    Column,
    #[serde(rename = "cr:dataBiases", alias = "dataBiases")]
    DataBiases,
    #[serde(rename = "cr:dataCollection", alias = "dataCollection")]
    DataCollection,
    #[serde(rename = "cr:data", alias = "data")]
    Data,
    #[serde(rename = "cr:dataType", alias = "dataType")]
    DataType,
    #[serde(rename = "cr:examples", alias = "examples")]
    Examples,
    #[serde(rename = "cr:extract", alias = "extract")]
    Extract,
    #[serde(rename = "cr:field", alias = "field")]
    Field,
    #[serde(rename = "cr:fileProperty", alias = "fileProperty")]
    FileProperty,
    #[serde(rename = "cr:fileObject", alias = "fileObject")]
    FileObject,
    #[serde(rename = "cr:fileSet", alias = "fileSet")]
    FileSet,
    #[serde(rename = "cr:format", alias = "format")]
    Format,
    #[serde(rename = "cr:includes", alias = "includes")]
    Includes,
    #[serde(rename = "cr:isLiveDataset", alias = "isLiveDataset")]
    IsLiveDataset,
    #[serde(rename = "cr:jsonPath", alias = "jsonPath")]
    JsonPath,
    #[serde(rename = "cr:key", alias = "key")]
    Key,
    #[serde(rename = "cr:md5", alias = "md5")]
    Md5,
    #[serde(rename = "cr:parentField", alias = "parentField")]
    ParentField,
    #[serde(rename = "cr:path", alias = "path")]
    Path,
    #[serde(
        rename = "cr:personalSensitiveInformation",
        alias = "personalSensitiveInformation"
    )]
    PersonalSensitiveInformation,
    #[serde(rename = "cr:recordSet", alias = "recordSet")]
    RecordSet,
    #[serde(rename = "cr:references", alias = "references")]
    References,
    #[serde(rename = "cr:regex", alias = "regex")]
    Regex,
    #[serde(rename = "cr:repeated", alias = "repeated")]
    Repeated,
    #[serde(rename = "cr:replace", alias = "replace")]
    Replace,
    #[serde(rename = "cr:separator", alias = "separator")]
    Separator,
    #[serde(rename = "cr:source", alias = "source")]
    Source,
    #[serde(rename = "cr:subField", alias = "subField")]
    SubField,
    #[serde(rename = "cr:transform", alias = "transform")]
    Transform,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
pub struct ContextObject {
    #[serde(rename = "@id")]
    pub id: CrTerm,

    #[serde(rename = "@type")]
    pub r#type: ContextType,
}

impl ContextObject {
    pub const fn new(id: CrTerm, r#type: ContextType) -> Self {
        Self { id, r#type }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
pub enum ContextType {
    #[serde(rename = "@json", alias = "json")]
    Json,
    #[serde(rename = "@vocab", alias = "vocab")]
    Vocab,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "@language")]
    pub language: String,

    #[serde(rename = "@vocab")]
    pub vocab: String,

    pub cite_as: CrTerm,

    pub column: CrTerm,

    #[setting(default = "dct:conformsTo")]
    pub conforms_to: String,

    #[setting(validate = schematic::validate::url, default = "http://mlcommons.org/croissant/")]
    pub cr: String,
    #[setting(validate = schematic::validate::url, default = "http://mlcommons.org/croissant/RAI/")]
    pub rai: Option<String>,
    #[setting(validate = schematic::validate::url, default = "http://purl.org/dc/terms/")]
    pub dct: String,
    #[setting(validate = schematic::validate::url, default = "https://schema.org/")]
    pub sc: String,
    #[setting(validate = schematic::validate::url, default = "http://www.wikidata.org/wiki/")]
    pub wd: Option<String>,

    pub data_biases: Option<CrTerm>,
    pub data_collection: Option<CrTerm>,
    #[setting(default = ContextObject::new(CrTerm::Data, ContextType::Json))]
    pub data: ContextObject,
    #[setting(default = ContextObject::new(CrTerm::DataType, ContextType::Vocab))]
    pub data_type: ContextObject,
    #[setting(default = ContextObject::new(CrTerm::Examples, ContextType::Json))]
    pub examples: Option<ContextObject>,

    pub extract: CrTerm,
    pub field: CrTerm,
    pub file_property: CrTerm,
    pub file_object: CrTerm,
    pub file_set: Option<CrTerm>,
    pub format: Option<CrTerm>,
    pub includes: Option<CrTerm>,
    pub is_live_dataset: Option<CrTerm>,
    pub json_path: Option<CrTerm>,
    pub key: Option<CrTerm>,
    pub md5: Option<CrTerm>,
    pub parent_field: Option<CrTerm>,
    pub path: Option<CrTerm>,
    pub personal_sensitive_information: Option<CrTerm>,
    pub record_set: Option<CrTerm>,
    pub references: Option<CrTerm>,
    pub regex: Option<CrTerm>,
    pub repeated: Option<CrTerm>,
    pub replace: Option<CrTerm>,
    pub separator: Option<CrTerm>,
    pub source: CrTerm,
    pub sub_field: Option<CrTerm>,
    pub transform: Option<CrTerm>,
}
