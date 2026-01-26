use schematic::{Config, ConfigEnum};

crate::config_enum!(
    #[serde(untagged)]
    #[derive(Config)]
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
        #[serde(rename = "sc:Object")]
        Object,
        #[serde(rename = "sc:ImageObject")]
        ImageObject,
        #[serde(rename = "cr:BoundingBox")]
        BoundingBox(BoundingBoxFormat),
        #[serde(rename = "cr:Split")]
        Split,
        #[serde(rename = "cr:Label")]
        Label,
        Custom(String),
    }
);

crate::config_enum!(
    #[derive(ConfigEnum)]
    pub enum BoundingBoxFormat {
        CenterXywh,
        Xywh,
        Xyxy,
    }
);
