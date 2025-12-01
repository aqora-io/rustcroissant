use chrono::Utc;

use crate::croissant::core::{
    DataType, Distribution, Extract, Field, FieldSource, FileObject, Id, Metadata, RecordSet, Ref,
    Text, default_context,
};
use crate::croissant::errors::{Error, Result};
use crate::croissant::utils::{calculate_sha256, get_csv_columns};
use std::path::Path;

/// Generate Croissant metadata from a CSV file
pub fn generate_metadata_from_csv(csv_path: &Path, output_path: Option<&Path>) -> Result<Metadata> {
    // Get file information
    let file_name = csv_path
        .file_name()
        .ok_or_else(|| Error::invalid_format("Invalid file path"))?
        .to_string_lossy()
        .to_string();

    let file_info = std::fs::metadata(csv_path).map_err(|_| Error::file_not_found(csv_path))?;
    let file_size = file_info.len();

    // Calculate SHA-256 hash
    let file_sha256 = calculate_sha256(csv_path)?;

    // Get column information
    let (headers, first_row) = get_csv_columns(csv_path)?;

    // Create fields based on CSV columns
    let mut fields = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let field_id = format!("main/{header}");
        let mut data_type = DataType::Url; // Default

        // Try to infer data type from first row if available
        if let Some(ref row) = first_row {
            if i < row.len() {
                data_type = DataType::from(&row[i]);
            }
        }

        let field = Field::builder()
            .id(Id::new(field_id))
            .kind(crate::croissant::core::CrType::Field)
            .name(Text::new(header))
            .description(Text::new(format!("Field for {header}")))
            .data_types(vec![data_type])
            .source(
                FieldSource::builder()
                    .extract(Some(Extract::Column {
                        name: Text::new(header),
                    }))
                    .source(crate::croissant::core::SourceRef::FileObject {
                        file_object: Ref {
                            id: Text::new(file_name.to_string()),
                        },
                    })
                    .build()
                    .map_err(|e| Error::Builder(e.to_string()))?,
            )
            .build()
            .map_err(|e| Error::Builder(e.to_string()))?;

        fields.push(field);
    }

    // Create metadata structure
    let dataset_name = csv_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let metadata = Metadata::builder()
        .context(default_context()?)
        .kind(crate::croissant::core::CroissantType::Dataset)
        .name(Text::new(format!("{dataset_name}_dataset")))
        .description(Text::new(format!("Dataset created from {file_name}")))
        .date_published(Some(Text::new(Utc::now().format("%Y-%m-%d"))))
        .version(Text::new("1.0.0"))
        .distribution(vec![
            Distribution::builder()
                .resource(crate::croissant::core::Resource::FileObject(
                    FileObject::builder()
                        .id(Id::new(file_name.to_string()))
                        .name(Text::new(file_name))
                        .content_size(Some(Text::new(format!("{file_size} B"))))
                        .encoding_format(Text::new("text/csv".to_string()))
                        .sha256(Some(file_sha256))
                        .build()
                        .map_err(|e| Error::Builder(e.to_string()))?,
                ))
                .build()
                .unwrap(), // TODO: error
        ])
        .record_sets(vec![
            RecordSet::builder()
                .id(Id::new("main"))
                .kind(crate::croissant::core::CrType::RecordSet)
                .keys(vec![Ref {
                    id: Id::new("main"),
                }])
                .fields(fields)
                .build()
                .map_err(|e| Error::Builder(e.to_string()))?,
        ])
        .build()
        .unwrap(); // TODO: error

    // Write metadata to file if output path is provided
    if let Some(output_path) = output_path {
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        std::fs::write(output_path, metadata_json)?;
    }

    Ok(metadata)
}
