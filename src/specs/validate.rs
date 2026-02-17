use schematic::{ValidateError, ValidateResult};
use std::{collections::HashSet, ops::Deref};

use crate::specs::{
    NonEmptyString, OneOrMany,
    dataset::{Context as CroissantContext, PartialDataset},
    record::{Field, ParentField, RecordSet},
    resource::Resource,
    source::{Source, SourceRef},
};

fn check_duplicate(set: &mut HashSet<String>, id: &str, what: &str) -> ValidateResult {
    if !set.insert(id.to_string()) {
        return Err(ValidateError::new(format!("duplicate {what} @id '{id}'")));
    }
    Ok(())
}

fn check_exists(set: &HashSet<String>, id: &str, message: String) -> ValidateResult {
    if !set.contains(id) {
        return Err(ValidateError::new(message));
    }
    Ok(())
}

fn validate_one_or_many<T>(
    value: &OneOrMany<T>,
    mut f: impl FnMut(&T) -> ValidateResult,
) -> ValidateResult {
    match value {
        OneOrMany::One(value) => f(value),
        OneOrMany::Many(values) => {
            for value in values {
                f(value)?;
            }
            Ok(())
        }
    }
}

pub fn validate_distribution(
    value: &OneOrMany<Resource>,
    _partial: &PartialDataset,
    _context: &CroissantContext,
    _finalize: bool,
) -> ValidateResult {
    let resources = value.as_slice();
    let mut ids = HashSet::new();

    for resource in resources {
        check_duplicate(&mut ids, resource.id().deref(), "distribution")?;
    }

    for resource in resources {
        let contained = match resource {
            Resource::FileObject(file_object) => &file_object.contained_in,
            Resource::FileSet(file_set) => &file_set.contained_in,
        };

        validate_one_or_many(contained, |contain| {
            check_exists(
                &ids,
                contain.id.deref(),
                format!(
                    "{} '{}' containedIn references missing id '{}'",
                    match resource {
                        Resource::FileObject(_) => "FileObject",
                        Resource::FileSet(_) => "FileSet",
                    },
                    resource.id().deref(),
                    contain.id.deref()
                ),
            )
        })?;
    }

    Ok(())
}

pub fn validate_bibtex(
    value: &NonEmptyString,
    _partial: &PartialDataset,
    _context: &CroissantContext,
    _finalize: bool,
) -> ValidateResult {
    match biblatex::Bibliography::parse(value.as_ref()) {
        Err(err) => Err(ValidateError::with_segments(
            err.kind.to_string(),
            vec![
                schematic::PathSegment::Index(err.span.start),
                schematic::PathSegment::Index(err.span.end),
            ],
        )),
        Ok(_) => Ok(()),
    }
}

pub fn validate_record_sets(
    value: &OneOrMany<RecordSet>,
    partial: &PartialDataset,
    _context: &CroissantContext,
    _finalize: bool,
) -> ValidateResult {
    let record_sets = value.as_slice();

    let dist_ids: HashSet<String> = partial
        .distribution
        .as_ref()
        .map(|v| v.iter().map(|r| r.id().deref().to_string()).collect())
        .unwrap_or_default();

    let mut record_set_ids = HashSet::new();
    for record_set in record_sets {
        check_duplicate(&mut record_set_ids, record_set.id.deref(), "RecordSet")?;
    }

    let mut field_ids = HashSet::new();
    for record_set in record_sets {
        collect_field_ids(&record_set.field, &mut field_ids)?;
        for f in &record_set.field {
            collect_parent_field_ids(f, &mut field_ids)?;
        }
    }

    for record_set in record_sets {
        validate_one_or_many(&record_set.key, |key| {
            let id = key.id.deref();
            if field_ids.contains(id)
                || field_ids.contains(&format!("{}/{}", record_set.id.deref(), id))
            {
                Ok(())
            } else {
                Err(ValidateError::new(format!(
                    "RecordSet '{}' key references missing Field id '{}'",
                    record_set.id.deref(),
                    id
                )))
            }
        })?;

        for field in &record_set.field {
            validate_field(&dist_ids, &record_set_ids, &field_ids, record_set, field)?;
        }
    }

    Ok(())
}

fn collect_field_ids(fields: &[Field], out: &mut HashSet<String>) -> ValidateResult {
    for field in fields {
        check_duplicate(out, field.id.deref(), "Field")?;
        collect_field_ids(&field.sub_field, out)?;
    }
    Ok(())
}

fn collect_parent_field_ids(field: &Field, out: &mut HashSet<String>) -> ValidateResult {
    validate_one_or_many(&field.parent_field, |pf| {
        if let ParentField::Inline(inline) = pf
            && let Some(id) = &inline.id
        {
            check_duplicate(out, id.deref(), "Field (from parentField inline)")?;
        }
        Ok(())
    })?;

    for sub_field in &field.sub_field {
        collect_parent_field_ids(sub_field, out)?;
    }

    Ok(())
}

fn validate_field(
    dist_ids: &HashSet<String>,
    rs_ids: &HashSet<String>,
    field_ids: &HashSet<String>,
    rs: &RecordSet,
    field: &Field,
) -> ValidateResult {
    validate_one_or_many(&field.references, |reference| {
        check_exists(
            field_ids,
            reference.field.id.deref(),
            format!(
                "Field '{}' in RecordSet '{}' references missing Field id '{}'",
                field.id.deref(),
                rs.id.deref(),
                reference.field.id.deref()
            ),
        )
    })?;

    validate_one_or_many(&field.parent_field, |parent_field| match parent_field {
        ParentField::Ref(refe) => check_exists(
            field_ids,
            refe.id.deref(),
            format!(
                "Field '{}' parentField references missing Field id '{}'",
                field.id.deref(),
                refe.id.deref()
            ),
        ),
        ParentField::Inline(inline) => {
            validate_one_or_many(&inline.references, |reference| {
                check_exists(
                    field_ids,
                    reference.id.deref(),
                    format!(
                        "Field '{}' parentField.inline references missing Field id '{}'",
                        field.id.deref(),
                        reference.id.deref()
                    ),
                )
            })?;

            if let Some(src) = &inline.source {
                validate_source(dist_ids, rs_ids, src)?;
            }

            Ok(())
        }
    })?;

    if let Some(source) = &field.source {
        validate_source(dist_ids, rs_ids, source)?;
    }

    for sub_field in &field.sub_field {
        validate_field(dist_ids, rs_ids, field_ids, rs, sub_field)?;
    }

    Ok(())
}

fn validate_source(
    dist_ids: &HashSet<String>,
    rs_ids: &HashSet<String>,
    source: &Source,
) -> ValidateResult {
    match source {
        Source::Ref(_) => Ok(()),
        Source::DataSource(data_source) => match &data_source.source {
            SourceRef::FileObject(file_object) => check_exists(
                dist_ids,
                file_object.id.deref(),
                format!(
                    "DataSource fileObject references missing distribution id '{}'",
                    file_object.id.deref()
                ),
            ),
            SourceRef::FileSet(file_set) => check_exists(
                dist_ids,
                file_set.id.deref(),
                format!(
                    "DataSource fileSet references missing distribution id '{}'",
                    file_set.id.deref()
                ),
            ),
            SourceRef::RecordSet(record_set) => check_exists(
                rs_ids,
                record_set.id.deref(),
                format!(
                    "DataSource recordSet references missing RecordSet id '{}'",
                    record_set.id.deref()
                ),
            ),
        },
    }
}
