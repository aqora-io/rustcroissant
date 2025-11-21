//! Validation logic for Croissant metadata
use crate::croissant::core::CrType;
use crate::croissant::core::CroissantType;
use crate::croissant::core::Metadata;
use crate::croissant::core::RecordSet;
use crate::croissant::core::SourceRef;
use crate::croissant::errors::{Error, Result};
use std::collections::HashSet;
use std::path::Path;

/// Issue severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    Error,
    Warning,
}

/// A single validation issue
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub context: Option<String>,
}

impl ValidationIssue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Error,
            message: message.into(),
            context: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Warning,
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Collection of validation issues
#[derive(Debug, Clone)]
pub struct ValidationIssues {
    issues: Vec<ValidationIssue>,
}

impl ValidationIssues {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::error(message));
    }

    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::warning(message));
    }

    pub fn add_error_with_context(
        &mut self,
        message: impl Into<String>,
        context: impl Into<String>,
    ) {
        self.issues
            .push(ValidationIssue::error(message).with_context(context));
    }

    pub fn add_warning_with_context(
        &mut self,
        message: impl Into<String>,
        context: impl Into<String>,
    ) {
        self.issues
            .push(ValidationIssue::warning(message).with_context(context));
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == IssueSeverity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == IssueSeverity::Warning)
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Warning)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Generate a human-readable report of all issues
    pub fn report(&self) -> String {
        if self.issues.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let errors: Vec<_> = self
            .issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Error)
            .collect();
        let warnings: Vec<_> = self
            .issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Warning)
            .collect();

        if !errors.is_empty() {
            result.push_str(&format!(
                "Found the following {} error(s) during the validation:\n",
                errors.len()
            ));
            for issue in errors {
                if let Some(ref context) = issue.context {
                    result.push_str(&format!("  -  [{}] {}\n", context, issue.message));
                } else {
                    result.push_str(&format!("  -  {}\n", issue.message));
                }
            }
        }

        if !warnings.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&format!(
                "Found the following {} warning(s) during the validation:\n",
                warnings.len()
            ));
            for issue in warnings {
                if let Some(ref context) = issue.context {
                    result.push_str(&format!("  -  [{}] {}\n", context, issue.message));
                } else {
                    result.push_str(&format!("  -  {}\n", issue.message));
                }
            }
        }

        result.trim_end().to_string()
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

impl Default for ValidationIssues {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a Croissant metadata file
pub fn validate_file(file_path: &Path) -> Result<ValidationIssues> {
    let content =
        std::fs::read_to_string(file_path).map_err(|_| Error::file_not_found(file_path))?;

    let metadata: Metadata = serde_json::from_str(&content)?;
    Ok(validate_metadata(&metadata))
}

/// Validate Croissant metadata structure
pub fn validate_metadata(metadata: &Metadata) -> ValidationIssues {
    let mut issues = ValidationIssues::new();

    validate_metadata_basic(&mut issues, metadata);
    validate_distributions(&mut issues, metadata);
    validate_record_sets(&mut issues, metadata);
    validate_references(&mut issues, metadata);

    issues
}

fn validate_metadata_basic(issues: &mut ValidationIssues, metadata: &Metadata) {
    let context = format!("Metadata({})", metadata.name);

    // Validate required fields
    if metadata.name.0.is_empty() {
        issues.add_error_with_context(
            "Property \"https://schema.org/name\" is mandatory, but does not exist.",
            &context,
        );
    }

    // Validate type
    if metadata.kind != CroissantType::Dataset {
        issues.add_error_with_context(
            "The current JSON-LD doesn't extend https://schema.org/Dataset.",
            &context,
        );
    }

    // Validate conformsTo is set
    if metadata.conforms_to.0.is_empty() {
        issues.add_warning_with_context(
            "Property \"http://purl.org/dc/terms/conformsTo\" is recommended, but does not exist.",
            &context,
        );
    }

    // Validate description
    if metadata.description.0.is_empty() {
        issues.add_warning_with_context(
            "Property \"https://schema.org/description\" is recommended, but does not exist.",
            &context,
        );
    }
}

fn validate_distributions(issues: &mut ValidationIssues, metadata: &Metadata) {
    for distribution in &metadata.distribution {
        let context = format!(
            "Metadata({}) > FileObject({})",
            metadata.name, distribution.resource
        );

        match &distribution.resource {
            crate::croissant::core::Resource::FileObject(file_object) => {
                if file_object.content_url.is_none() {
                    issues.add_error_with_context(
                "Property \"https://schema.org/contentUrl\" is mandatory, but does not exist.",
                &context,
            );
                }

                // Validate encoding format
                if file_object.encoding_format.0.is_empty() {
                    issues.add_error_with_context(
                "Property \"https://schema.org/encodingFormat\" is mandatory, but does not exist.",
                &context,
            );
                }

                // Validate SHA256
                if file_object.sha256.is_none() {
                    issues.add_warning_with_context(
                "Property \"https://schema.org/sha256\" is recommended for file integrity verification.",
                &context
            );
                } else if file_object.sha256.is_some()
                    || file_object.sha256.as_ref().unwrap().len() != 64
                    || !file_object
                        .sha256
                        .as_ref()
                        .unwrap()
                        .chars()
                        .all(|c| c.is_ascii_hexdigit())
                {
                    issues.add_error_with_context(
                        "Invalid SHA256 hash format. Expected 64 hexadecimal characters.",
                        &context,
                    );
                }
            }
            crate::croissant::core::Resource::FileSet(file_set) => {
                // Validate encoding format
                if file_set.encoding_format.0.is_empty() {
                    issues.add_error_with_context(
                "Property \"https://schema.org/encodingFormat\" is mandatory, but does not exist.",
                &context,
            );
                }
            }
        }
    }
}

fn validate_record_sets(issues: &mut ValidationIssues, metadata: &Metadata) {
    match &metadata.record_sets {
        None => (),
        Some(record_sets) => {
            for record_set in record_sets {
                let context = format!(
                    "Metadata({}) > RecordSet({})",
                    metadata.name, record_set.kind
                );

                // Validate required fields
                if record_set.id.0.is_empty() {
                    issues.add_error_with_context(
                        "Property \"https://schema.org/name\" is mandatory, but does not exist.",
                        &context,
                    );
                }

                // Validate type
                if record_set.kind != CrType::RecordSet {
                    issues.add_error_with_context(
                format!(
                    "\"{}\" should have an attribute \"@type\": \"http://mlcommons.org/croissant/RecordSet\". Got {} instead.",
                    record_set.id,
                    record_set.kind,
                ),
                &context
            );
                }

                validate_fields(issues, metadata, record_set);
            }
        }
    }
}

fn validate_fields(issues: &mut ValidationIssues, metadata: &Metadata, record_set: &RecordSet) {
    for field in &record_set.fields {
        let context = format!(
            "Metadata({}) > RecordSet({}) > Field({})",
            metadata.name, record_set.kind, field.name
        );

        // Validate required fields
        if field.name.0.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/name\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate type
        if field.kind != CrType::Field {
            issues.add_error_with_context(
                format!(
                    "\"{}\" should have an attribute \"@type\": \"http://mlcommons.org/croissant/Field\". Got {} instead.",
                    field.name,
                    field.kind
                ),
                &context
            );
        }

        match &field.source {
            Some(source) =>
            // Validate source
            {
                match &source.extract {
                    Some(extract) => {
                        let is_empty = match extract {
                            crate::croissant::core::Extract::Column { name } => name.0.is_empty(),

                            _ => false,
                        };
                        let is_file_object_empty = match &source.source {
                            SourceRef::FileObject { file_object } => file_object.id.0.is_empty(),
                            _ => false,
                        };
                        if is_empty || is_file_object_empty {
                            issues.add_error_with_context(
                format!(
                    "Node \"{}\" is a field and has no source. Please, use http://mlcommons.org/croissant/source to specify the source.",
                    field.id
                ),
                &context
            )
                        }
                    }
                    None => (),
                }
            }
            None => (),
        };
    }
}

fn validate_references(issues: &mut ValidationIssues, metadata: &Metadata) {
    // Collect all distribution IDs
    let distribution_ids: HashSet<_> = metadata
        .distribution
        .iter()
        .map(|dist| match &dist.resource {
            super::core::Resource::FileObject(file_object) => file_object.id.to_owned(),
            super::core::Resource::FileSet(file_set) => file_set.id.to_owned(),
        })
        .collect();

    let record_sets = match &metadata.record_sets {
        Some(record_sets) => record_sets,
        None => return,
    };

    // Validate field references to file objects
    for record_set in record_sets {
        for field in &record_set.fields {
            match &field.source {
                Some(source) => {
                    let file_object_id = match &source.source {
                        SourceRef::FileObject { file_object } => file_object.id.to_owned(),
                        SourceRef::RecordSet { record_set } => record_set.id.to_owned(),
                        SourceRef::FileSet { file_set } => file_set.id.to_owned(),
                    };
                    if !file_object_id.0.is_empty() && !distribution_ids.contains(&file_object_id) {
                        let context = format!(
                            "Metadata({}) > RecordSet({}) > Field({})",
                            metadata.name, record_set.kind, field.name
                        );
                        issues.add_error_with_context(
                            format!("Field references non-existent file object: {file_object_id}"),
                            &context,
                        );
                    }
                }
                None => (),
            };
        }
    }
}
