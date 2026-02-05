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

        /// The extraction method from the provided source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extract: Option<Extract>,

        /// A transformation to apply on source data on top of the extracted
        /// method as specified through extract, e.g., a regular expression or
        /// a JSON path.
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            deserialize_with = "one_or_many"
        )]
        pub transform: Vec<Transform>,

        /// A format string used to parse values coming from a `DataSource`.
        ///
        /// For example, a date may be represented as the string "2022/11/10"
        /// and interpreted into the correct date via the format "yyyy/MM/dd".
        /// Formats correspond to a target data type.
        ///
        /// Commonly used formats in Croissant include:
        ///
        /// - For [sc:Date](http://schema.org/Date) and
        ///   [sc:DateTime](http://schema.org/DateTime):
        ///   [CLDR Date/Time patterns]
        ///   (https://cldr.unicode.org/translation/date-time/date-time-patterns)
        ///   (e.g., `MM/dd/yyyy`)
        ///
        /// - For [sc:Number](http://schema.org/Number),
        ///   [sc:Float](http://schema.org/Float), and
        ///   [sc:Integer](http://schema.org/Integer):
        ///   [CLDR Number and Currency patterns]
        ///   (https://cldr.unicode.org/translation/number-currency-formats/
        ///   number-and-currency-patterns)
        ///   (e.g., `0.##E0`)
        ///
        /// - For `cr:BoundingBox`:
        ///   [Keras bounding box formats]
        ///   (https://keras.io/api/keras_cv/bounding_box/formats/)
        ///   (e.g., `CENTER_XYWH`)
        ///
        /// This list is not exhaustive, and not all Croissant implementations
        /// support all possible formats.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub format: Option<NonEmptyString>,
    }
);

config_enum!(
    #[derive(Config)]
    pub enum SourceRef {
        /// The name of the referenced `FileObject` source of the data.
        FileObject(Ref),
        /// The name of the referenced `FileSet` source of the data.
        FileSet(Ref),
        /// The name of the referenced RecordSet source.
        RecordSet(Ref),
    }
);

config_enum!(
    /// Specifies how to extract a subset of data from a source.
    ///
    /// Sometimes, not all the data from the source is needed, but only a
    /// subset. The `Extract` enum describes how data should be extracted,
    /// depending on the type of the source.
    ///
    /// Supported extraction methods include:
    /// - Selecting a column from a CSV file
    /// - Extracting a property from a file or file set
    /// - Applying a JSONPath expression to JSON data
    /// - Applying a regular expression to textual data
    #[derive(Config)]
    pub enum Extract {
        /// Extracts values from a specific column of a CSV file.
        Column(NonEmptyString),

        /// Extracts a predefined property from a `FileObject` or `FileSet`,
        /// such as the filename, full path, or file contents.
        FileProperty(FileProperty),

        /// Extracts value(s) from JSON data using a JSONPath expression.
        JsonPath(NonEmptyString),

        /// Extracts value(s) from source data using a regular expression.
        Regex(NonEmptyString),
    }
);

config_enum!(
    /// Properties that can be extracted from a `FileObject` or `FileSet`.
    ///
    /// These properties describe different ways of accessing file-related
    /// data, such as the file name, full path, or contents.
    #[derive(ConfigEnum)]
    pub enum FileProperty {
        /// The full path to the file within the Croissant extraction or
        /// download folders (e.g., `data/train/metadata.csv`).
        #[serde(alias = "fullpath")]
        FullPath,

        /// The name of the file only (e.g., `metadata.csv`).
        #[serde(alias = "filename")]
        FilenName,

        /// The full byte content of the file.
        Content,

        /// The byte content of each line in the file.
        Lines,

        /// The line numbers of the file, starting from 0.
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
