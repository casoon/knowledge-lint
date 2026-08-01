use super::secret_scan::scan_for_secrets;
use super::{category_files, read_and_announce};
use crate::config::{CategoryConfig, TypesConfig};
use crate::reporter::Reporter;
use anyhow::Result;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct Frontmatter {
    title: Option<String>,
    #[serde(rename = "type")]
    entry_type: Option<String>,
    created: Option<String>,
    last_reviewed: Option<String>,
    status: Option<String>,
    source: Option<String>,
    #[serde(default)]
    related: Vec<String>,
    #[serde(default)]
    public: bool,
}

/// Aggregate result of linting one knowledge_entry category, so the caller can
/// print a "public" summary across the whole knowledge base.
#[derive(Default)]
pub struct CategoryStats {
    pub entries: usize,
    pub public_entries: usize,
}

pub fn lint_knowledge_category(
    dir: &Path,
    category_name: &str,
    category: &CategoryConfig,
    types: &TypesConfig,
    max_file_size: u64,
    reporter: &mut Reporter,
) -> Result<CategoryStats> {
    let matter = Matter::<YAML>::new();
    let today = chrono::Local::now().date_naive();
    let mut stats = CategoryStats::default();

    for path in category_files(dir, "_template.md") {
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let raw = read_and_announce(&path)?;
        check_file_size(&path, &raw, max_file_size, reporter);
        scan_for_secrets(&raw, reporter);

        let fm = match matter.parse_with_struct::<Frontmatter>(&raw) {
            Some(p) => p.data,
            None => {
                reporter.error("frontmatter is missing or unreadable");
                continue;
            }
        };

        stats.entries += 1;
        if fm.public {
            stats.public_entries += 1;
        }

        check_required_fields(&fm, reporter);
        check_status(&fm, types, reporter);
        check_type(&fm, category_name, category, reporter);
        check_last_reviewed(&fm, category, types, today, reporter);
        check_related_links(&fm, &path, reporter);
    }
    Ok(stats)
}

fn check_file_size(path: &Path, raw: &str, max_file_size: u64, reporter: &mut Reporter) {
    let size = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => raw.len() as u64,
    };
    if size > max_file_size {
        reporter.warn(format!(
            "file is {} KB — consider referencing large content via _attachments.yml instead of embedding it",
            size / 1000
        ));
    }
}

fn check_required_fields(fm: &Frontmatter, reporter: &mut Reporter) {
    if fm.title.as_deref().unwrap_or("").is_empty() {
        reporter.error("title is missing");
    }
    if fm.source.as_deref().unwrap_or("").is_empty() {
        reporter.error("source is missing");
    }
    if fm.created.as_deref().unwrap_or("").is_empty() {
        reporter.error("created is missing");
    }
}

fn check_status(fm: &Frontmatter, types: &TypesConfig, reporter: &mut Reporter) {
    match fm.status.as_deref() {
        None | Some("") => reporter.error("status is missing"),
        Some(s) if types.status_values.iter().any(|v| v == s) => {}
        Some(other) => reporter.error(format!(
            "status '{other}' is not one of the allowed values ({})",
            types.status_values.join(", ")
        )),
    }
}

fn check_type(
    fm: &Frontmatter,
    category_name: &str,
    category: &CategoryConfig,
    reporter: &mut Reporter,
) {
    match (&fm.entry_type, &category.entry_type) {
        (Some(t), Some(expected)) if t != expected => reporter.error(format!(
            "type '{t}' does not match category '{category_name}' (expected: '{expected}')"
        )),
        (None, Some(_)) => reporter.error("type is missing"),
        _ => {}
    }
}

fn check_last_reviewed(
    fm: &Frontmatter,
    category: &CategoryConfig,
    types: &TypesConfig,
    today: chrono::NaiveDate,
    reporter: &mut Reporter,
) {
    let Some(date_str) = &fm.last_reviewed else {
        reporter.error("last_reviewed is missing");
        return;
    };

    let date = match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            reporter.error(format!(
                "last_reviewed '{date_str}' is not a valid date (YYYY-MM-DD)"
            ));
            return;
        }
    };

    if fm.status.as_deref() != Some(types.active_status.as_str()) {
        return;
    }
    let Some(interval) = category.review_interval_days else {
        return;
    };

    let age_days = (today - date).num_days();
    if age_days > interval {
        reporter.warn(format!(
            "last_reviewed is {age_days} days old (> {interval}), but the entry is marked '{}'",
            types.active_status
        ));
    }
}

fn check_related_links(fm: &Frontmatter, path: &Path, reporter: &mut Reporter) {
    let dir = path.parent().unwrap_or(Path::new("."));
    for rel in &fm.related {
        if !dir.join(rel).exists() {
            reporter.error(format!(
                "related link '{rel}' points to a file that doesn't exist"
            ));
        }
    }
}
