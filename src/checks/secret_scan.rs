use crate::reporter::Reporter;

// These are detection markers, not embedded credentials — static analysis that
// flags "hardcoded secret" on this list is a false positive by construction.
const SECRET_PATTERNS: &[&str] = &[
    "-----BEGIN PRIVATE KEY-----",
    "-----BEGIN OPENSSH PRIVATE KEY-----",
    "-----BEGIN RSA PRIVATE KEY-----",
    "AGE-SECRET-KEY-1",
];

const SECRET_PREFIXES: &[&str] = &["sk-", "ghp_", "gho_", "AKIA", "xoxb-", "xoxp-"];

/// Best-effort scan for secrets that ended up outside a `sops_secret` category.
/// Unambiguous markers (private-key headers, an age secret-key prefix) are errors;
/// API-key-shaped prefixes are warnings only, since those can false-positive on
/// ordinary identifiers.
pub fn scan_for_secrets(content: &str, reporter: &mut Reporter) {
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
