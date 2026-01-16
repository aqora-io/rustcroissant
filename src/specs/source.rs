use schematic::{Config, ConfigEnum, RegexSetting};
use serde::{Deserialize, Serialize};

use crate::specs::{
    common::{NonEmptyString, Ref},
    serde::one_or_many,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(untagged)]
pub enum Source {
    Ref(Ref),
    DataSource(DataSource),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    #[serde(flatten)]
    pub source: SourceRef,

    #[serde(default)]
    pub extract: Option<Extract>,

    #[serde(default, deserialize_with = "one_or_many")]
    pub transform: Vec<Transform>,

    #[serde(default)]
    pub format: Option<NonEmptyString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub enum SourceRef {
    FileObject(Ref),
    FileSet(Ref),
    RecordSet(Ref),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub enum Extract {
    Column(NonEmptyString),
    FileProperty(FileProperty),
    JsonPath(NonEmptyString),
    Regex(NonEmptyString),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ConfigEnum)]
#[serde(rename_all = "camelCase")]
pub enum FileProperty {
    #[serde(alias = "fullpath")]
    FullPath,
    #[serde(alias = "filename")]
    FilenName,
    Content,
    Lines,
    #[serde(alias = "linenumbers")]
    LineNumbers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
    Delimiter(char),
    Regex(RegexSetting),
    #[serde(alias = "jsonQuery")]
    JsonPath(NonEmptyString),
}
