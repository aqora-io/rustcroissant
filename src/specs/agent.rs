use schematic::Config;
use serde::{Deserialize, Serialize};

use crate::specs::common::{Id, JsonObject, NonEmptyString};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Config)]
pub struct Agent {
    #[serde(rename = "@type")]
    pub r#type: Option<NonEmptyString>,

    #[serde(rename = "@id")]
    pub id: Option<Id>,

    pub name: Option<NonEmptyString>,

    #[setting(validate = schematic::validate::url)]
    pub url: Option<String>,

    #[serde(flatten, default)]
    pub extra: JsonObject,
}
