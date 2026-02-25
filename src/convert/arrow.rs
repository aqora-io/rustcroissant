use crate::{
    convert::converter::Converter,
    specs::{
        OneOrMany, RecordSet,
        data_type::DataType,
        record::{Field, FieldType},
    },
};
use arrow_schema::{DataType as ArrowType, Field as ArrowField};
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

pub struct ParquetTransformer<'a> {
    pub metadata: &'a ArrowReaderMetadata,
}

impl<'a> ParquetTransformer<'a> {
    pub fn new(metadata: &'a ArrowReaderMetadata) -> Self {
        Self { metadata }
    }
}

impl<'a> super::transformer::TransformerTrait for ParquetTransformer<'a> {
    fn transform(&self, convert: &Converter) -> RecordSet {
        let schema = self.metadata.schema();

        let fields = schema
            .fields()
            .iter()
            .map(|field| Self::to_croissant_field(field, convert))
            .collect();

        convert.build(fields)
    }
}

impl<'a> ParquetTransformer<'a> {
    fn to_croissant_field(field: &ArrowField, convert: &Converter) -> Field {
        let sub_field = match field.data_type() {
            ArrowType::Struct(fields) => fields
                .iter()
                .map(|field| Self::to_croissant_field(field, convert))
                .collect(),
            _ => Vec::new(),
        };

        let field_name = format!("{}/{}", convert.source_id, field.name().to_owned());
        Field {
            r#type: FieldType::Field,
            id: field_name.clone().into(),
            name: Some(field_name.into()),
            data_type: vec![DataType::from(field.data_type())],
            description: Some(format!("Column '{}' {}", field.name(), convert.suffix).into()),
            repeated: Some(matches!(
                field.data_type(),
                ArrowType::List(_)
                    | ArrowType::ListView(_)
                    | ArrowType::FixedSizeList(_, _)
                    | ArrowType::LargeList(_)
                    | ArrowType::LargeListView(_)
            )),
            source: Some(crate::specs::Source::DataSource(crate::specs::DataSource {
                source: crate::specs::SourceRef::FileObject(crate::specs::Ref {
                    id: convert.source_id.clone().into(),
                }),
                extract: Some(crate::specs::Extract::Column(
                    field.name().to_owned().into(),
                )),
                ..Default::default()
            })),
            sub_field,
            ..Default::default()
        }
    }
}
