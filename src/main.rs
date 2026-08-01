use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(about = "Validates and maintains a Markdown knowledge base against a _types.yml config")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check frontmatter, status values, related-links, staleness, secret encryption, attachments
    Lint {
        /// Path to the knowledge directory
        #[arg(default_value = "knowledge")]
        knowledge_dir: PathBuf,

        /// Files above this size (bytes) get a warning suggesting _attachments.yml instead
        #[arg(long, default_value_t = 500_000)]
        max_file_size: u64,
    },
    /// Remove example content from every category, keep structure and _template.md
    Clean {
        /// Path to the knowledge directory
        #[arg(default_value = "knowledge")]
        knowledge_dir: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct TypesConfig {
    categories: HashMap<String, CategoryConfig>,

    /// Allowed values for the `status` field. Freely chosen (e.g. `[current, deprecated]`
    /// or `[draft, stable, deprecated]`), must contain at least two values.
    #[serde(default = "default_status_values")]
    status_values: Vec<String>,

    /// Which value in `status_values` means "still active, needs upkeep". Only entries
    /// with this status are checked for an overdue `last_reviewed` date.
    #[serde(default = "default_active_status")]
    active_status: String,
}

fn default_status_values() -> Vec<String> {
    vec!["current".to_string(), "deprecated".to_string()]
}

fn default_active_status() -> String {
    "current".to_string()
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum CategoryKind {
    KnowledgeEntry,
    SopsSecret,
    /// Plain storage for files referenced from _attachments.yml — no frontmatter
    /// schema, no content validation.
    Assets,
}

#[derive(Debug, Deserialize)]
struct CategoryConfig {
    kind: CategoryKind,
    #[serde(rename = "type")]
    entry_type: Option<String>,
    review_interval_days: Option<i64>,
}

#[derive(Debug, Default, Deserialize)]
struct Frontmatter {
    title: Option<String>,
    #[serde(rename = "type")]
    entry_type: Option<String>,
    created: Option<String>,
    last_reviewed: Option<String>,
    status: Option<String>,
    source: Option<String>,
    #[serde(default)]
    related: Vec<String>,
    #[allow(dead_code)]
    public: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct AttachmentsFile {
    #[serde(default)]
    attachments: Vec<AttachmentEntry>,
}

#[derive(Debug, Deserialize)]
struct AttachmentEntry {
    id: String,
    path: String,
    #[allow(dead_code)]
    description: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    referenced_by: Vec<String>,
}

struct Reporter {
    errors: usize,
    warnings: usize,
}

impl Reporter {
    fn new() -> Self {
        Self {
            errors: 0,
            warnings: 0,
        }
    }
    fn error(&mut self, msg: impl AsRef<str>) {
        println!("  ERROR: {}", msg.as_ref());
        self.errors += 1;
    }
    fn warn(&mut self, msg: impl AsRef<str>) {
        println!("  WARNING: {}", msg.as_ref());
        self.warnings += 1;
    }
}

const SECRET_PATTERNS: &[&str] = &[
    "-----BEGIN PRIVATE KEY-----",
    "-----BEGIN OPENSSH PRIVATE KEY-----",
    "-----BEGIN RSA PRIVATE KEY-----",
    "AGE-SECRET-KEY-1",
];

const SECRET_PREFIXES: &[&str] = &["sk-", "ghp_", "gho_", "AKIA", "xoxb-", "xoxp-"];

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Lint {
            knowledge_dir,
            max_file_size,
        } => run_lint(&knowledge_dir, max_file_size),
        Command::Clean { knowledge_dir } => run_clean(&knowledge_dir),
    }
}

fn load_types(knowledge_dir: &Path) -> Result<TypesConfig> {
    let types_path = knowledge_dir.join("_types.yml");
    let types_raw = fs::read_to_string(&types_path)
        .with_context(|| format!("Could not read {}", types_path.display()))?;
    serde_yaml::from_str(&types_raw)
        .with_context(|| format!("Could not parse {} as YAML", types_path.display()))
}

fn run_lint(knowledge_dir: &Path, max_file_size: u64) -> Result<()> {
    let mut reporter = Reporter::new();
    let types = load_types(knowledge_dir)?;

    lint_categories(knowledge_dir, &types, max_file_size, &mut reporter)?;
    lint_attachments(knowledge_dir, &mut reporter);

    println!();
    if reporter.errors > 0 {
        println!(
            "{} error(s), {} warning(s) found.",
            reporter.errors, reporter.warnings
        );
        std::process::exit(1);
    }
    println!("All checks passed ({} warning(s)).", reporter.warnings);
    Ok(())
}

fn run_clean(knowledge_dir: &Path) -> Result<()> {
    let types = load_types(knowledge_dir)?;

    for (name, category) in &types.categories {
        let dir = knowledge_dir.join(name);
        fs::create_dir_all(&dir)?;

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            let keep = match category.kind {
                CategoryKind::KnowledgeEntry => file_name == "_template.md",
                CategoryKind::SopsSecret | CategoryKind::Assets => file_name == ".gitkeep",
            };
            if !keep {
                fs::remove_file(&path)
                    .with_context(|| format!("Could not delete {}", path.display()))?;
            }
        }

        fs::write(dir.join(".gitkeep"), "")?;
        println!("Cleaned: {}/", dir.display());
    }

    let attachments_path = knowledge_dir.join("_attachments.yml");
    if attachments_path.exists() {
        fs::write(
            &attachments_path,
            "# Central registry for references to large source files, see README.md.\n\nattachments: []\n",
        )?;
        println!("Reset: {}", attachments_path.display());
    }

    println!("Done. _template.md remains as the scaffold for new entries.");
    Ok(())
}

fn lint_categories(
    knowledge_dir: &Path,
    types: &TypesConfig,
    max_file_size: u64,
    reporter: &mut Reporter,
) -> Result<()> {
    for entry in fs::read_dir(knowledge_dir)
        .with_context(|| format!("Could not read {}", knowledge_dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(category) = types.categories.get(&name) else {
            println!("Checking {}/", entry.path().display());
            reporter.error(format!(
                "category '{name}' is not declared in _types.yml"
            ));
            continue;
        };

        match category.kind {
            CategoryKind::KnowledgeEntry => lint_knowledge_category(
                &entry.path(),
                &name,
                category,
                types,
                max_file_size,
                reporter,
            )?,
            CategoryKind::SopsSecret => lint_secrets_category(&entry.path(), reporter)?,
            CategoryKind::Assets => {}
        }
    }
    Ok(())
}

fn lint_knowledge_category(
    dir: &Path,
    category_name: &str,
    category: &CategoryConfig,
    types: &TypesConfig,
    max_file_size: u64,
    reporter: &mut Reporter,
) -> Result<()> {
    let matter = Matter::<YAML>::new();
    let today = chrono::Local::now().date_naive();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("_template.md") {
            continue;
        }

        println!("Checking {}", path.display());

        let size = fs::metadata(path)?.len();
        if size > max_file_size {
            reporter.warn(format!(
                "file is {} KB — consider referencing large content via _attachments.yml instead of embedding it",
                size / 1000
            ));
        }

        let raw = fs::read_to_string(path)
            .with_context(|| format!("Could not read {}", path.display()))?;
        scan_for_secrets(&raw, reporter);

        let fm = match matter.parse_with_struct::<Frontmatter>(&raw) {
            Some(p) => p.data,
            None => {
                reporter.error("frontmatter is missing or unreadable");
                continue;
            }
        };

        if fm.title.as_deref().unwrap_or("").is_empty() {
            reporter.error("title is missing");
        }
        if fm.source.as_deref().unwrap_or("").is_empty() {
            reporter.error("source is missing");
        }
        if fm.created.as_deref().unwrap_or("").is_empty() {
            reporter.error("created is missing");
        }

        match fm.status.as_deref() {
            None | Some("") => reporter.error("status is missing"),
            Some(s) if types.status_values.iter().any(|v| v == s) => {}
            Some(other) => reporter.error(format!(
                "status '{other}' is not one of the allowed values ({})",
                types.status_values.join(", ")
            )),
        }

        match (&fm.entry_type, &category.entry_type) {
            (Some(t), Some(expected)) if t != expected => reporter.error(format!(
                "type '{t}' does not match category '{category_name}' (expected: '{expected}')"
            )),
            (None, Some(_)) => reporter.error("type is missing"),
            _ => {}
        }

        match &fm.last_reviewed {
            None => reporter.error("last_reviewed is missing"),
            Some(date_str) => match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Ok(date) => {
                    let age_days = (today - date).num_days();
                    if fm.status.as_deref() == Some(types.active_status.as_str()) {
                        if let Some(interval) = category.review_interval_days {
                            if age_days > interval {
                                reporter.warn(format!(
                                    "last_reviewed is {age_days} days old (> {interval}), but the entry is marked '{}'",
                                    types.active_status
                                ));
                            }
                        }
                    }
                }
                Err(_) => reporter.error(format!(
                    "last_reviewed '{date_str}' is not a valid date (YYYY-MM-DD)"
                )),
            },
        }

        let dir = path.parent().unwrap_or(Path::new("."));
        for rel in &fm.related {
            let target = dir.join(rel);
            if !target.exists() {
                reporter.error(format!(
                    "related link '{rel}' points to a file that doesn't exist"
                ));
            }
        }
    }
    Ok(())
}

fn lint_secrets_category(dir: &Path, reporter: &mut Reporter) -> Result<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some(".gitkeep") {
            continue;
        }

        println!("Checking {}", path.display());

        let raw = fs::read_to_string(path)
            .with_context(|| format!("Could not read {}", path.display()))?;

        match serde_yaml::from_str::<serde_yaml::Value>(&raw) {
            Ok(serde_yaml::Value::Mapping(map)) => {
                let has_sops_marker = map.keys().any(|k| k.as_str() == Some("sops"));
                if !has_sops_marker {
                    reporter.error(
                        "file in a sops_secret category has no `sops:` metadata field — not SOPS-encrypted?",
                    );
                }
            }
            Ok(_) => reporter.error(
                "file in a sops_secret category is not a YAML mapping — expected a SOPS-encrypted document",
            ),
            Err(e) => reporter.error(format!("file in a sops_secret category is not valid YAML: {e}")),
        }
    }
    Ok(())
}

fn scan_for_secrets(content: &str, reporter: &mut Reporter) {
    for pattern in SECRET_PATTERNS {
        if content.contains(pattern) {
            reporter.error(format!(
                "possible plaintext key found (pattern: '{pattern}') — belongs in a sops_secret category, encrypted"
            ));
        }
    }
    for prefix in SECRET_PREFIXES {
        if content.split_whitespace().any(|word| {
            word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
                .starts_with(prefix)
                && word.len() > prefix.len() + 8
        }) {
            reporter.warn(format!(
                "possible API key with prefix '{prefix}' found — check whether this is a real secret"
            ));
        }
    }
}

fn lint_attachments(knowledge_dir: &Path, reporter: &mut Reporter) {
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
        // Paths in _attachments.yml are relative to the knowledge directory (where the file lives).
        check_attachment_path(&att.id, &att.path, knowledge_dir, reporter);
    }
}

fn check_attachment_path(id: &str, raw_path: &str, base_dir: &Path, reporter: &mut Reporter) {
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

    let candidate = if Path::new(raw_path).is_absolute() {
        PathBuf::from(raw_path)
    } else {
        base_dir.join(raw_path)
    };
    if !candidate.exists() {
        reporter.error(format!(
            "attachment '{id}': path '{raw_path}' does not exist"
        ));
    }
}
