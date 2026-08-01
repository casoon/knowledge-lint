use crate::reporter::Reporter;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct AttachmentsFile {
    #[serde(default)]
    attachments: Vec<AttachmentEntry>,
}

#[derive(Debug, Deserialize)]
struct AttachmentEntry {
    id: String,
    path: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    referenced_by: Vec<String>,
}

pub fn lint_attachments(knowledge_dir: &Path, reporter: &mut Reporter) {
    let path = knowledge_dir.join("_attachments.yml");
    if !path.exists() {
        return;
    }

    println!("Checking {}", path.display());

    let raw = match fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            reporter.error(format!("Could not read {}: {e}", path.display()));
            return;
        }
    };

    let parsed: AttachmentsFile = match serde_yaml::from_str(&raw) {
        Ok(p) => p,
        Err(e) => {
            reporter.error(format!("{} is not valid YAML: {e}", path.display()));
            return;
        }
    };

    for att in &parsed.attachments {
        if att.id.is_empty() {
            reporter.error("attachment without an id");
        }
        if att.description.trim().is_empty() {
            reporter.warn(format!("attachment '{}' has no description", att.id));
        }
        check_source_path(&att.id, &att.path, knowledge_dir, reporter);
        check_referenced_by(&att.id, &att.referenced_by, knowledge_dir, reporter);
    }
}

/// Paths in `_attachments.yml` are relative to the knowledge directory (where the file lives).
fn check_source_path(id: &str, raw_path: &str, knowledge_dir: &Path, reporter: &mut Reporter) {
    if raw_path.starts_with("http://") || raw_path.starts_with("https://") {
        match ureq::head(raw_path).call() {
            Ok(_) => {}
            Err(e) => reporter.warn(format!(
                "attachment '{id}': URL '{raw_path}' is not reachable ({e})"
            )),
        }
        return;
    }

    // Network-share schemes can't be checked from here.
    for scheme in ["smb://", "afp://", "nas://"] {
        if raw_path.starts_with(scheme) {
            return;
        }
    }

    if !resolve(raw_path, knowledge_dir).exists() {
        reporter.error(format!(
            "attachment '{id}': path '{raw_path}' does not exist"
        ));
    }
}

/// `referenced_by` entries name the knowledge entries that mention this attachment —
/// always paths inside the knowledge base, relative to the knowledge directory.
fn check_referenced_by(
    id: &str,
    referenced_by: &[String],
    knowledge_dir: &Path,
    reporter: &mut Reporter,
) {
    for rel in referenced_by {
        if !resolve(rel, knowledge_dir).exists() {
            reporter.error(format!(
                "attachment '{id}': referenced_by entry '{rel}' points to a file that doesn't exist"
            ));
        }
    }
}

fn resolve(raw_path: &str, base_dir: &Path) -> PathBuf {
    if Path::new(raw_path).is_absolute() {
        PathBuf::from(raw_path)
    } else {
        base_dir.join(raw_path)
    }
}
