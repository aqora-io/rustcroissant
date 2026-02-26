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

        #[serde(rename = "cr:SamplingRate", alias = "SamplingRate")]
        SamplingRate,

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

impl CrTerm {
    pub fn default(&self) -> Self {
        self.clone()
    }
}

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
        #[setting(default = CrTerm::default(&CrTerm::Annotation))]
        pub annotation: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::ArrayShape) )]
        pub array_shape: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::CiteAs))]
        pub cite_as: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Column))]
        pub column: Option<CrTerm>,

        #[setting(default = "dct:conformsTo")]
        pub conforms_to: String,

        #[setting(default = CrTerm::default(&CrTerm::ContainedIn))]
        pub contained_in: Option<CrTerm>,

        #[setting(default = "http://mlcommons.org/croissant/")]
        pub cr: Option<url::Url>,

        #[setting(default = CrTerm::default(&CrTerm::DataBiases))]
        pub data_biases: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::DataCollection))]
        pub data_collection: Option<CrTerm>,

        #[setting(default = ContextObject::new(CrTerm::Data, ContextType::Json))]
        pub data: ContextObject,

        #[setting(default = ContextObject::new(CrTerm::DataType, ContextType::Vocab))]
        pub data_type: ContextObject,

        #[setting(default = "http://purl.org/dc/terms/")]
        pub dct: Option<url::Url>,

        #[setting(default = CrTerm::default(&CrTerm::EquivalentProperty))]
        pub equivalent_property: Option<CrTerm>,

        #[setting(default = ContextObject::new(CrTerm::Examples, ContextType::Json))]
        pub examples: Option<ContextObject>,

        #[setting(default = CrTerm::default(&CrTerm::Examples))]
        pub excludes: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Extract))]
        pub extract: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Field))]
        pub field: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::FileObject))]
        pub file_object: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::FileProperty))]
        pub file_property: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::FileSet))]
        pub file_set: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Format))]
        pub format: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Includes))]
        pub includes: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::IsArray))]
        pub is_array: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::IsLiveDataset))]
        pub is_live_dataset: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::JsonPath))]
        pub json_path: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Key))]
        pub key: Option<CrTerm>,

        #[serde(rename = "@language")]
        #[setting(default = "en")]
        pub language: String,

        #[setting(default = CrTerm::default(&CrTerm::Md5))]
        pub md5: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::ParentField))]
        pub parent_field: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Path))]
        pub path: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::PersonalSensitiveInformation))]
        pub personal_sensitive_information: Option<CrTerm>,

        #[setting(default = "http://mlcommons.org/croissant/RAI/")]
        pub rai: Option<url::Url>,

        #[setting(default = CrTerm::default(&CrTerm::ReadLines))]
        pub read_lines: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::RecordSet))]
        pub record_set: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::References))]
        pub references: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Regex))]
        pub regex: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Repeated))]
        pub repeated: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Replace))]
        pub replace: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::SamplingRate))]
        pub sampling_rate: Option<CrTerm>,

        #[setting(default = "https://schema.org/")]
        pub sc: Option<url::Url>,

        #[setting(default = CrTerm::default(&CrTerm::SdVersion))]
        pub sd_version: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Separator))]
        pub separator: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Source))]
        pub source: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::SubField))]
        pub sub_field: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Transform))]
        pub transform: Option<CrTerm>,
        #[setting(default = CrTerm::default(& CrTerm::UnArchive) ) ]
        pub un_archive: Option<CrTerm>,
        #[setting(default = CrTerm::default(&CrTerm::Value))]
        pub value: Option<CrTerm>,

        #[serde(rename = "@vocab")]
        #[setting(default = "https://schema.org/")]
        pub vocab: Option<url::Url>,

        #[setting(default = "http://www.wikidata.org/entity/")]
        pub wd: Option<url::Url>,
        #[setting(default = "http://www.wikidata.org/prop/direct/")]
        pub wdt: Option<url::Url>,
    }
);
