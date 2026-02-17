use crate::specs::{RecordSet, RecordSetType};

pub struct Converter {
    pub split_name: String,
    pub source_id: String,
    pub suffix: String,
}

impl Converter {
    pub fn new(
        split_name: impl Into<String>,
        source_id: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Self {
        Self {
            split_name: split_name.into(),
            source_id: source_id.into(),
            suffix: suffix.into(),
        }
    }

    pub(crate) fn build(&self, fields: Vec<crate::specs::record::Field>) -> RecordSet {
        RecordSet {
            r#type: RecordSetType::RecordSet,
            id: self.split_name.clone().into(),
            name: Some(self.split_name.clone().into()),
            field: fields,
            description: Some(format!("{} subset", self.split_name).into()),
            ..Default::default()
        }
    }
}
