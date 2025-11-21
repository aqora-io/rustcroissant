use chrono::Utc;

use crate::croissant::core::{
    DataType, Distribution, Extract, Field, FieldSource, FileObject, Metadata, RecordSet,
    create_default_context, infer_data_type,
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
        let mut data_type = DataType::Text; // Default

        // Try to infer data type from first row if available
        if let Some(ref row) = first_row {
            if i < row.len() {
                data_type = infer_data_type(&row[i]);
            }
        }

        let field = Field::builder()
            .id(field_id)
            .r#type("cr:Field".to_string())
            .name(header.clone())
            .description(format!("Field for {header}"))
            .data_type(data_type.to_schema_org().to_string())
            .source(
                FieldSource::builder()
                    .extract(Extract {
                        column: header.clone(),
                    })
                    .file_object(FileObject {
                        id: file_name.clone(),
                    })
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        fields.push(field);
    }

    // Create metadata structure
    let dataset_name = csv_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let metadata = Metadata::builder()
        .context(create_default_context())
        .r#type("sc:Dataset".to_string())
        .name(format!("{dataset_name}_dataset"))
        .description(format!("Dataset created from {file_name}"))
        .description("http://mlcommons.org/croissant/1.0".to_string())
        .date_published(Utc::now().format("%Y-%m-%d").to_string())
        .version("1.0.0".to_string())
        .distribution(vec![
            Distribution::builder()
                .id(file_name.clone())
                .r#type("cr:FileObject".to_string())
                .name(file_name.clone())
                .content_size(format!("{file_size} B"))
                .content_url(file_name)
                .encoding_format("text/csv".to_string())
                .sha256(file_sha256)
                .build()
                .unwrap(), // TODO: error
        ])
        .record_set(vec![
            RecordSet::builder()
                .id("main".to_string())
                .r#type("cr:RecordSet".to_string())
                .name("main".to_string())
                .description(format!(
                    "Records from {}",
                    csv_path.file_name().unwrap().to_string_lossy()
                ))
                .field(fields)
                .build()
                .unwrap(),
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
