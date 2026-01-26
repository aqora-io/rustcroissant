use crate::specs::{
    common::{NonEmptyString, Ref},
    serde::one_or_many,
};
use schematic::{Config, ConfigEnum, RegexSetting};

config_enum!(
    #[derive(Config)]
    #[serde(untagged)]
    pub enum Source {
        Ref(Ref),
        DataSource(DataSource),
    }
);

config_struct!(
    #[derive(Config)]
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
);

config_enum!(
    #[derive(Config)]
    pub enum SourceRef {
        FileObject(Ref),
        FileSet(Ref),
        RecordSet(Ref),
    }
);

config_enum!(
    #[derive(Config)]
    pub enum Extract {
        Column(NonEmptyString),
        FileProperty(FileProperty),
        JsonPath(NonEmptyString),
        Regex(NonEmptyString),
    }
);

config_enum!(
    #[derive(ConfigEnum)]
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
);

config_enum!(
    #[derive(Config)]
    pub enum Transform {
        Delimiter(char),
        Regex(RegexSetting),
        #[serde(alias = "jsonQuery")]
        JsonPath(NonEmptyString),
    }
);
