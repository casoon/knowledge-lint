use crate::config::{load_types, CategoryKind};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Removes every file in every declared category except the kind-specific survivor
/// (`_template.md` for knowledge_entry, `.gitkeep` for sops_secret/assets), and
/// resets `_attachments.yml` to an empty list if it exists.
pub fn run_clean(knowledge_dir: &Path) -> Result<()> {
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
