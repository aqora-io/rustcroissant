use schematic::Config;

crate::config_enum!(
    #[derive(Config)]
    pub enum CrTerm {
        #[serde(rename = "cr:annotation", alias = "annotation")]
        Annotation,

        #[serde(rename = "cr:arrayShape", alias = "arrayShape")]
        ArrayShape,

        #[serde(rename = "cr:citeAs", alias = "citeAs")]
        CiteAs,

        #[serde(rename = "cr:column", alias = "column")]
        Column,

        #[serde(rename = "cr:containedIn", alias = "containedIn")]
        ContainedIn,

        #[serde(rename = "cr:data", alias = "data")]
        Data,

        #[serde(rename = "cr:dataBiases", alias = "dataBiases")]
        DataBiases,

        #[serde(rename = "cr:dataCollection", alias = "dataCollection")]
        DataCollection,

        #[serde(rename = "cr:dataType", alias = "dataType")]
        DataType,

        #[serde(rename = "cr:equivalentProperty", alias = "equivalentProperty")]
        EquivalentProperty,

        #[serde(rename = "cr:examples", alias = "examples")]
        Examples,

        #[serde(rename = "cr:excludes", alias = "excludes")]
        Excludes,

        #[serde(rename = "cr:extract", alias = "extract")]
        Extract,

        #[serde(rename = "cr:field", alias = "field")]
        Field,

        #[serde(rename = "cr:fileObject", alias = "fileObject")]
        FileObject,

        #[serde(rename = "cr:fileProperty", alias = "fileProperty")]
        FileProperty,

        #[serde(rename = "cr:fileSet", alias = "fileSet")]
        FileSet,

        #[serde(rename = "cr:format", alias = "format")]
        Format,

        #[serde(rename = "cr:includes", alias = "includes")]
        Includes,

        #[serde(rename = "cr:isArray", alias = "isArray")]
        IsArray,

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

        #[serde(rename = "cr:readLines", alias = "readLines")]
        ReadLines,

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

        #[serde(rename = "cr:sdVersion", alias = "sdVersion")]
        SdVersion,

        #[serde(rename = "cr:separator", alias = "separator")]
        Separator,

        #[serde(rename = "cr:source", alias = "source")]
        Source,

        #[serde(rename = "cr:subField", alias = "subField")]
        SubField,

        #[serde(rename = "cr:transform", alias = "transform")]
        Transform,

        #[serde(rename = "cr:unArchive", alias = "unArchive")]
        UnArchive,

        #[serde(rename = "cr:value", alias = "value")]
        Value,
    }
);

crate::config_struct!(
    #[derive(Config)]
    pub struct ContextObject {
        #[serde(rename = "@id")]
        pub id: CrTerm,

        #[serde(rename = "@type")]
        pub r#type: ContextType,
    }
);

impl ContextObject {
    pub const fn new(id: CrTerm, r#type: ContextType) -> Self {
        Self { id, r#type }
    }
}

crate::config_enum!(
    #[derive(Config)]
    pub enum ContextType {
        #[serde(rename = "@json", alias = "json")]
        Json,
        #[serde(rename = "@vocab", alias = "vocab")]
        Vocab,
    }
);

crate::config_struct!(
    #[derive(Config)]
    pub struct Context {
        pub annotation: Option<CrTerm>,
        pub array_shape: Option<CrTerm>,
        pub cite_as: CrTerm,
        pub column: CrTerm,

        #[setting(default = "dct:conformsTo")]
        pub conforms_to: String,

        pub contained_in: Option<CrTerm>,

        #[setting(default = "http://mlcommons.org/croissant/")]
        pub cr: Option<url::Url>,

        pub data_biases: Option<CrTerm>,
        pub data_collection: Option<CrTerm>,

        #[setting(default = ContextObject::new(CrTerm::Data, ContextType::Json))]
        pub data: ContextObject,

        #[setting(default = ContextObject::new(CrTerm::DataType, ContextType::Vocab))]
        pub data_type: ContextObject,

        #[setting(default = "http://purl.org/dc/terms/")]
        pub dct: Option<url::Url>,

        pub equivalent_property: Option<CrTerm>,

        #[setting(default = ContextObject::new(CrTerm::Examples, ContextType::Json))]
        pub examples: Option<ContextObject>,

        pub excludes: Option<CrTerm>,
        pub extract: CrTerm,
        pub field: CrTerm,
        pub file_object: CrTerm,
        pub file_property: CrTerm,
        pub file_set: Option<CrTerm>,
        pub format: Option<CrTerm>,
        pub includes: Option<CrTerm>,
        pub is_array: Option<CrTerm>,
        pub is_live_dataset: Option<CrTerm>,
        pub json_path: Option<CrTerm>,
        pub key: Option<CrTerm>,

        #[serde(rename = "@language")]
        pub language: String,

        pub md5: Option<CrTerm>,
        pub parent_field: Option<CrTerm>,
        pub path: Option<CrTerm>,
        pub personal_sensitive_information: Option<CrTerm>,

        #[setting(default = "http://mlcommons.org/croissant/RAI/")]
        pub rai: Option<url::Url>,

        pub read_lines: Option<CrTerm>,
        pub record_set: Option<CrTerm>,
        pub references: Option<CrTerm>,
        pub regex: Option<CrTerm>,
        pub repeated: Option<CrTerm>,
        pub replace: Option<CrTerm>,

        #[setting(default = "https://schema.org/")]
        pub sc: Option<url::Url>,

        pub sd_version: Option<CrTerm>,
        pub separator: Option<CrTerm>,
        pub source: CrTerm,
        pub sub_field: Option<CrTerm>,
        pub transform: Option<CrTerm>,
        pub un_archive: Option<CrTerm>,
        pub value: Option<CrTerm>,

        #[serde(rename = "@vocab")]
        pub vocab: String,

        pub wd: Option<url::Url>,
        pub wdt: Option<url::Url>,
    }
);
