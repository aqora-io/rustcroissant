use schematic::{Config, ValidateError, ValidateResult};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Deref};

use crate::specs::{
    NonEmptyString, OneOrMany, StringOrUrl,
    dataset::{Context as CroissantContext, PartialDataset},
    record::{Field, ParentField, RecordSet},
    resource::Resource,
    source::{Source, SourceRef},
};

pub fn validate_distribution(
    value: &crate::specs::OneOrMany<Resource>,
    _partial: &PartialDataset,
    _context: &CroissantContext,
    _finalize: bool,
) -> ValidateResult {
    let value = value.as_slice();

    let mut ids = HashSet::<String>::new();

    for res in value {
        let id = res.id().deref();
        if !ids.insert(id.to_owned()) {
            return Err(ValidateError::new(format!(
                "duplicate distribution @id '{}'",
                id
            )));
        }
    }

    let idset: HashSet<String> = value.iter().map(|r| r.id().deref().to_owned()).collect();
    for res in value {
        match res {
            Resource::FileObject(o) => {
                for c in &o.contained_in {
                    if !idset.contains(&c.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "FileObject '{}' containedIn references missing id '{}'",
                            o.id.deref().to_owned(),
                            c.id.deref().to_owned()
                        )));
                    }
                }
            }
            Resource::FileSet(s) => {
                for c in &s.contained_in {
                    if !idset.contains(&c.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "FileSet '{}' containedIn references missing id '{}'",
                            s.id.to_owned().deref(),
                            c.id.deref().to_owned()
                        )));
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Config, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LicenseInput {
    One(StringOrUrl),
    Many(Vec<StringOrUrl>),
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
    let value = value.as_slice();

    let dist_ids: HashSet<String> = partial
        .distribution
        .as_ref()
        .map(|v| v.iter().map(|r| r.id().deref().to_owned()).collect())
        .unwrap_or_default();

    let mut rs_ids = HashSet::<String>::new();
    for rs in value {
        if !rs_ids.insert(rs.id.deref().to_owned()) {
            return Err(ValidateError::new(format!(
                "duplicate RecordSet @id '{}'",
                rs.id.deref().to_owned()
            )));
        }
    }

    let mut field_ids = HashSet::<String>::new();
    for rs in value {
        collect_field_ids(&rs.field, &mut field_ids)?;
        for f in &rs.field {
            collect_parent_field_ids(f, &mut field_ids)?;
        }
    }

    for rs in value {
        for k in &rs.key {
            if !field_ids.contains(&k.id.deref().to_owned())
                && !field_ids.contains(&format!("{}/{}", rs.id.to_owned(), k.id.to_owned()))
            {
                return Err(ValidateError::new(format!(
                    "RecordSet '{}' key references missing Field id '{}'",
                    rs.id.to_owned(),
                    k.id.to_owned()
                )));
            }
        }

        for f in &rs.field {
            validate_field(dist_ids.clone(), &rs_ids, &field_ids, &rs, f)?;
        }
    }

    Ok(())
}

fn collect_field_ids(fields: &[Field], out: &mut HashSet<String>) -> ValidateResult {
    for f in fields {
        if !out.insert(f.id.deref().to_owned()) {
            return Err(ValidateError::new(format!(
                "duplicate Field @id '{}'",
                f.id.deref().to_owned()
            )));
        }
        if !f.sub_field.is_empty() {
            collect_field_ids(&f.sub_field, out)?;
        }
    }
    Ok(())
}

fn collect_parent_field_ids(field: &Field, out: &mut HashSet<String>) -> ValidateResult {
    for pf in &field.parent_field {
        if let ParentField::Inline(inline) = pf {
            if let Some(id) = &inline.id {
                if !out.insert(id.deref().to_owned()) {
                    return Err(ValidateError::new(format!(
                        "duplicate Field @id '{}' (from parentField inline)",
                        id.deref().to_owned()
                    )));
                }
            }
        }
    }
    for sf in &field.sub_field {
        collect_parent_field_ids(sf, out)?;
    }
    Ok(())
}

fn validate_field(
    dist_ids: HashSet<String>,
    rs_ids: &HashSet<String>,
    field_ids: &HashSet<String>,
    rs: &RecordSet,
    f: &Field,
) -> ValidateResult {
    for r in &f.references {
        if !field_ids.contains(&r.field.id.deref().to_owned()) {
            return Err(ValidateError::new(format!(
                "Field '{}' in RecordSet '{}' references missing Field id '{}'",
                f.id.deref().to_owned(),
                rs.id.deref().to_owned(),
                r.field.id.deref().to_owned()
            )));
        }
    }

    for pf in &f.parent_field {
        match pf {
            ParentField::Ref(r) => {
                if !field_ids.contains(&r.id.deref().to_owned()) {
                    return Err(ValidateError::new(format!(
                        "Field '{}' parentField references missing Field id '{}'",
                        f.id.deref().to_owned(),
                        r.id.deref().to_owned()
                    )));
                }
            }
            ParentField::Inline(inline) => {
                for r in &inline.references {
                    if !field_ids.contains(&r.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "Field '{}' parentField.inline references missing Field id '{}'",
                            f.id.deref().to_owned(),
                            r.id.deref().to_owned()
                        )));
                    }
                }
                if let Some(src) = &inline.source {
                    validate_source(&dist_ids, rs_ids, src)?;
                }
            }
        }
    }

    if let Some(source) = &f.source {
        validate_source(&dist_ids, rs_ids, source)?;
    }

    for sf in &f.sub_field {
        validate_field(dist_ids.clone(), rs_ids, field_ids, rs, sf)?;
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
        Source::DataSource(ds) => {
            match &ds.source {
                SourceRef::FileObject(file_object) => {
                    if !dist_ids.contains(&file_object.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "DataSource fileObject references missing distribution id '{}'",
                            file_object.id.deref().to_owned()
                        )));
                    }
                }
                SourceRef::FileSet(file_set) => {
                    if !dist_ids.contains(&file_set.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "DataSource fileSet references missing distribution id '{}'",
                            file_set.id.deref().to_owned()
                        )));
                    }
                }
                SourceRef::RecordSet(record_set) => {
                    if !rs_ids.contains(&record_set.id.deref().to_owned()) {
                        return Err(ValidateError::new(format!(
                            "DataSource recordSet references missing RecordSet id '{}'",
                            record_set.id.deref().to_owned()
                        )));
                    }
                }
            }
            Ok(())
        }
    }
}
