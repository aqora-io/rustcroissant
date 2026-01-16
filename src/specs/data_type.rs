use schematic::{Config, ConfigEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Config)]
#[serde(untagged)]
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
    BoundingBox(BoundingBoxFormat),
    #[serde(rename = "cr:Split")]
    Split,
    #[serde(rename = "cr:Label")]
    Label,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ConfigEnum)]
pub enum BoundingBoxFormat {
    CenterXywh,
    Xywh,
    Xyxy,
}
