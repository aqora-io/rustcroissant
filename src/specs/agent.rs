use crate::specs::common::{Id, NonEmptyString};
use schematic::Config;

crate::config_struct!(
    #[derive(Config)]
    pub struct Agent {
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<NonEmptyString>,

        #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
        pub id: Option<Id>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<NonEmptyString>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<url::Url>,
    }
);
