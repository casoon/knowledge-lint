# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-08-01

### Added

- `lint` command: validates a knowledge directory against its `_types.yml` and exits 1 on any
  error, 0 when only warnings remain.
- Config-driven categories with the kinds `knowledge_entry`, `sops_secret` and `assets`, and a
  configurable status vocabulary (`status_values`, `active_status`).
- Frontmatter checks for required fields, `type` per category, allowed `status` values,
  `last_reviewed` dates, review intervals and `related` links.
- Structural SOPS check for secrets and a best-effort scan for plaintext keys in entries.
- `_attachments.yml` checks for local paths, `http(s)` URLs and `referenced_by` entries.
- `clean` command: removes demo content from every category and resets `_attachments.yml`.
