use crate::specs::common::{Id, NonEmptyString};
use schematic::Config;

crate::config_struct!(
    #[derive(Config)]
    pub struct Agent {
        #[serde(rename = "@type")]
        pub r#type: Option<NonEmptyString>,

        #[serde(rename = "@id")]
        pub id: Option<Id>,

        pub name: Option<NonEmptyString>,

        pub url: Option<url::Url>,
    }
);
