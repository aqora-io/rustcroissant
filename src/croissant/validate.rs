//! Validation logic for Croissant metadata
use crate::croissant::core::Metadata;
use crate::croissant::errors::{Error, Result};
use std::path::Path;

/// Validate a Croissant metadata file
pub fn validate_file(file_path: &Path) -> Result<()> {
    let content =
        std::fs::read_to_string(file_path).map_err(|_| Error::file_not_found(file_path))?;

    let metadata: Metadata = serde_json::from_str(&content)?;
    metadata.check()?;
    Ok(())
}
