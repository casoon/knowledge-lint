use super::{category_files, read_and_announce};
use crate::reporter::Reporter;
use anyhow::Result;
use std::path::Path;

/// Every file in a `sops_secret` category must be a SOPS-encrypted YAML document.
/// Validation is structural only (a top-level `sops:` key, which SOPS always adds
/// to its output) — this never decrypts anything and never needs a private key.
pub fn lint_secrets_category(dir: &Path, reporter: &mut Reporter) -> Result<()> {
    for path in category_files(dir, ".gitkeep") {
        let raw = read_and_announce(&path)?;

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
