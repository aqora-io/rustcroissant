#![cfg(feature = "arrow")]
use crate::specs::{
    common::NonEmptyString,
    data_type::DataType,
    record::{Field, FieldType, RecordSet, RecordSetType},
};
use arrow_schema::{DataType as ArrowType, Field as ArrowField, Schema};

impl From<&ArrowType> for DataType {
    fn from(dt: &ArrowType) -> Self {
        match dt {
            ArrowType::Boolean => DataType::Boolean,
            ArrowType::Int8
            | ArrowType::Int16
            | ArrowType::Int32
            | ArrowType::Int64
            | ArrowType::UInt8
            | ArrowType::UInt16
            | ArrowType::UInt32
            | ArrowType::UInt64 => DataType::Integer,
            ArrowType::Float16 | ArrowType::Float32 | ArrowType::Float64 => DataType::Float,
            ArrowType::Utf8 | ArrowType::LargeUtf8 => DataType::Text,
            ArrowType::Binary | ArrowType::LargeBinary => DataType::Text,
            ArrowType::Date32 | ArrowType::Date64 => DataType::Date,
            ArrowType::Timestamp(_, _) => DataType::DateTime,
            ArrowType::Struct(_) | ArrowType::List(_) => DataType::Object,
            _ => DataType::Text,
        }
    }
}

impl From<&ArrowField> for Field {
    fn from(field: &ArrowField) -> Self {
        let mut out = Field {
            r#type: FieldType::Field,
            id: NonEmptyString(format!("#field-{}", field.name()).to_string()),
            name: Some(NonEmptyString(field.name().to_owned())),
            description: None,
            data_type: vec![DataType::from(field.data_type())],
            source: None,
            repeated: Some(matches!(field.data_type(), ArrowType::List(_))),
            equivalent_property: vec![],
            references: vec![],
            sub_field: vec![],
            parent_field: vec![],
            extra: Default::default(),
        };

        if let ArrowType::Struct(fields) = field.data_type() {
            out.sub_field = fields
                .iter()
                .map(|field| Field::from(field.as_ref()))
                .collect();
        }

        out
    }
}

impl From<(&Schema, &str)> for RecordSet {
    fn from((schema, name): (&Schema, &str)) -> Self {
        RecordSet {
            r#type: RecordSetType::RecordSet,
            id: NonEmptyString("#recordset-main".to_string()),
            name: Some(NonEmptyString(name.to_owned())),
            description: None,
            data_type: vec![],
            key: vec![],
            field: schema
                .fields()
                .iter()
                .map(|field| Field::from(field.as_ref()))
                .collect(),
            data: None,
            examples: None,
            extra: Default::default(),
        }
    }
}
