#![cfg(feature = "arrow")]
use crate::specs::{
    data_type::DataType,
    record::{Field, FieldType, RecordSet, RecordSetType},
};
use arrow_schema::{DataType as ArrowType, Field as ArrowField, Schema};
use parquet::arrow::arrow_reader::ArrowReaderMetadata;

impl From<&ArrowType> for DataType {
    fn from(data_type: &ArrowType) -> Self {
        match data_type {
            ArrowType::Boolean => DataType::Boolean,
            ArrowType::Int8 => DataType::Int8,
            ArrowType::Int16 => DataType::Int16,
            ArrowType::Int32 => DataType::Int32,
            ArrowType::Int64 => DataType::Int64,
            ArrowType::UInt8 => DataType::UInt8,
            ArrowType::UInt16 => DataType::UInt16,
            ArrowType::UInt32 => DataType::UInt32,
            ArrowType::UInt64 => DataType::UInt64,
            ArrowType::Float16 => DataType::Float16,
            ArrowType::Float32 => DataType::Float32,
            ArrowType::Float64 => DataType::Float64,
            ArrowType::Utf8 | ArrowType::LargeUtf8 => DataType::Text,
            ArrowType::Binary | ArrowType::LargeBinary => DataType::Text,
            ArrowType::Date32 | ArrowType::Date64 => DataType::Date,
            ArrowType::Time32(_) | ArrowType::Time64(_) => DataType::Time,
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
            id: field.name().to_owned().into(),
            name: Some(field.name().to_owned().into()),
            data_type: crate::specs::OneOrMany::One(DataType::from(field.data_type())),
            repeated: Some(matches!(
                field.data_type(),
                ArrowType::List(_)
                    | ArrowType::ListView(_)
                    | ArrowType::FixedSizeList(_, _)
                    | ArrowType::LargeList(_)
                    | ArrowType::LargeListView(_)
            )),
            ..Default::default()
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
            id: "#recordset-main".into(),
            name: Some(name.into()),

            field: schema
                .fields()
                .iter()
                .map(|field| Field::from(field.as_ref()))
                .collect(),
            ..Default::default()
        }
    }
}

pub fn to_record_set(metadata: &ArrowReaderMetadata, name: &str) -> RecordSet {
    RecordSet::from((metadata.schema().as_ref(), name))
}
