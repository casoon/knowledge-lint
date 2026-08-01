use crate::checks::attachments::lint_attachments;
use crate::checks::knowledge_entry::lint_knowledge_category;
use crate::checks::secrets::lint_secrets_category;
use crate::config::{load_types, CategoryKind};
use crate::reporter::Reporter;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn run_lint(knowledge_dir: &Path, max_file_size: u64) -> Result<()> {
    let mut reporter = Reporter::new();
    let types = load_types(knowledge_dir)?;

    let mut total_entries = 0;
    let mut total_public = 0;

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
            reporter.error(format!("category '{name}' is not declared in _types.yml"));
            continue;
        };

        match category.kind {
            CategoryKind::KnowledgeEntry => {
                let stats = lint_knowledge_category(
                    &entry.path(),
                    &name,
                    category,
                    &types,
                    max_file_size,
                    &mut reporter,
                )?;
                total_entries += stats.entries;
                total_public += stats.public_entries;
            }
            CategoryKind::SopsSecret => lint_secrets_category(&entry.path(), &mut reporter)?,
            CategoryKind::Assets => {}
        }
    }

    lint_attachments(knowledge_dir, &mut reporter);

    println!();
    if total_entries > 0 {
        println!("{total_public} of {total_entries} knowledge entries are marked public.");
    }
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
