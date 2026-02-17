use schematic::{Config, ConfigEnum};

crate::config_enum!(
    /// The data type of values expected for a `Field` in a `RecordSet`.
    ///
    /// Specifying data types is crucial for data validation and downstream
    /// processing, for example to enable ML frameworks to automatically
    /// populate the correct data structures when loading datasets.
    ///
    /// Croissant supports two kinds of data types:
    /// - Atomic data types, such as integers and strings
    /// - Semantic data types, which convey additional meaning and may be
    ///   structured
    ///
    /// Data types can be specified:
    /// - On individual `Field`s, to constrain each field value
    /// - On an entire `RecordSet`, to constrain each record and define
    ///   mandatory fields
    ///
    /// This enum is inspired by the `Datatype` class in
    /// [CSVW](https://csvw.org/).
    #[serde(untagged)]
    #[derive(Config)]
    pub enum DataType {
        /// Describes a boolean value.
        #[serde(rename = "sc:Boolean")]
        Boolean,

        /// Describes a date.
        #[serde(rename = "sc:Date")]
        Date,

        /// Describes a combination of date and time of day.
        #[serde(rename = "sc:DateTime")]
        DateTime,

        /// Describes a floating-point number.
        #[serde(rename = "sc:Float")]
        Float,

        /// A 16-bit floating-point number
        #[serde(rename = "sc:Float16")]
        Float16,

        /// A 32-bit floating-point number
        #[serde(rename = "sc:Float32")]
        Float32,

        /// A 64-bit floating-point number
        #[serde(rename = "sc:Float64")]
        Float64,

        /// Describes an integer.
        #[serde(rename = "sc:Integer")]
        Integer,

        /// Int number
        #[serde(rename = "sc:Int")]
        Int,

        /// A 8-bit int number
        #[serde(rename = "sc:Int8")]
        Int8,

        /// A 16-bit int number
        #[serde(rename = "sc:Int16")]
        Int16,

        /// A 32-bit int number
        #[serde(rename = "sc:Int32")]
        Int32,

        /// A 64-bit int number
        #[serde(rename = "sc:Int64")]
        Int64,

        /// A 8-bit int unsigned  number
        #[serde(rename = "sc:UInt8")]
        UInt8,

        /// A 16-bit int unsigned  number
        #[serde(rename = "sc:UInt16")]
        UInt16,

        /// A 32-bit int unsigned  number
        #[serde(rename = "sc:UInt32")]
        UInt32,

        /// A 64-bit int unsigned  number
        #[serde(rename = "sc:UInt64")]
        UInt64,

        /// Describes a string value.
        #[serde(rename = "sc:Text")]
        Text,

        /// Describes a time value.
        #[serde(rename = "sc:Time")]
        Time,

        /// Describes the content of an image (pixels).
        #[serde(rename = "sc:ImageObject")]
        ImageObject,

        /// Describes the coordinates of a bounding box represented as a
        /// four-number array.
        #[serde(rename = "cr:BoundingBox")]
        BoundingBox(BoundingBoxFormat),

        /// Describes a field containing the content of a video file.
        #[serde(rename = "sc:VideoObject")]
        VideoObject,

        /// Describes a `RecordSet` used to split data according to
        /// intended usage (e.g., training, validation, testing).
        #[serde(rename = "cr:Split")]
        Split,

        /// Describes a categorical label, commonly used in ML datasets.
        #[serde(rename = "cr:Label")]
        Label,

        #[serde(rename = "sc:Enumeration")]
        Enumeration,

        #[serde(rename = "sc:URL")]
        Url,

        #[serde(rename = "sc:Object")]
        Object,

        /// A custom data type identified by a URI or string.
        Custom(String),
    }
);

crate::config_enum!(
    /// Supported formats for `cr:BoundingBox` values.
    ///
    /// Bounding boxes are represented as arrays of four numbers, whose
    /// interpretation depends on the selected format.
    #[derive(ConfigEnum)]
    pub enum BoundingBoxFormat {
        /// Center-based format: (center_x, center_y, width, height).    
        CenterXywh,

        /// Top-left based format: (x, y, width, height).
        Xywh,

        /// Corner-based format: (x_min, y_min, x_max, y_max).
        Xyxy,
    }
);
