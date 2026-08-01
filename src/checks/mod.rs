pub mod attachments;
pub mod knowledge_entry;
pub mod secret_scan;
pub mod secrets;

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Files directly under `dir`'s subtree, excluding `skip_name` (e.g. `_template.md`
/// or `.gitkeep`) and anything that isn't a regular file.
pub fn category_files<'a>(dir: &'a Path, skip_name: &'a str) -> impl Iterator<Item = PathBuf> + 'a {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .filter(move |path| {
            path.is_file() && path.file_name().and_then(|n| n.to_str()) != Some(skip_name)
        })
}

/// Announces the file being checked (consistent progress output across all checks)
/// and reads it, with a uniform error message on failure.
pub fn read_and_announce(path: &Path) -> Result<String> {
    println!("Checking {}", path.display());
    fs::read_to_string(path).with_context(|| format!("Could not read {}", path.display()))
}
