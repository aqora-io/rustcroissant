use crate::specs::common::{Id, JsonObject, NonEmptyString};
use schematic::Config;

crate::config_struct!(
    #[derive(Config)]
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
);
