use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TypesConfig {
    pub categories: HashMap<String, CategoryConfig>,

    /// Allowed values for the `status` field. Freely chosen (e.g. `[current, deprecated]`
    /// or `[draft, stable, deprecated]`), must contain at least two values.
    #[serde(default = "default_status_values")]
    pub status_values: Vec<String>,

    /// Which value in `status_values` means "still active, needs upkeep". Only entries
    /// with this status are checked for an overdue `last_reviewed` date.
    #[serde(default = "default_active_status")]
    pub active_status: String,
}

fn default_status_values() -> Vec<String> {
    vec!["current".to_string(), "deprecated".to_string()]
}

fn default_active_status() -> String {
    "current".to_string()
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CategoryKind {
    KnowledgeEntry,
    SopsSecret,
    /// Plain storage for files referenced from _attachments.yml — no frontmatter
    /// schema, no content validation.
    Assets,
}

#[derive(Debug, Deserialize)]
pub struct CategoryConfig {
    pub kind: CategoryKind,
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub review_interval_days: Option<i64>,
}

pub fn load_types(knowledge_dir: &Path) -> Result<TypesConfig> {
    let types_path = knowledge_dir.join("_types.yml");
    let types_raw = fs::read_to_string(&types_path)
        .with_context(|| format!("Could not read {}", types_path.display()))?;
    serde_yaml::from_str(&types_raw)
        .with_context(|| format!("Could not parse {} as YAML", types_path.display()))
}
