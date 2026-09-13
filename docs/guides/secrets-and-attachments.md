---
title: Secrets and attachments
description: Encrypted secrets stay encrypted, plaintext keys are caught outside of them, and large originals are referenced from _attachments.yml instead of being embedded.
order: 2
---

## Encrypted secrets

A category with `kind: sops_secret` holds credentials encrypted with
[SOPS](https://github.com/getsops/sops). knowledge-lint checks every file in it structurally: the
file must parse as YAML and contain a top-level `sops:` key, which SOPS adds to everything it
encrypts.

knowledge-lint never decrypts anything and never needs a private key, so the check runs in CI
without access to secrets.

```sh
# encrypt a file for your own age key before committing it
sops -e --age <your-age-public-key> api-token.yaml > knowledge/secrets/api-token.yaml
```

A file committed without encryption is an error:

```text
Checking broken-knowledge-base/secrets/api-token.yaml
  ERROR: file in a sops_secret category has no `sops:` metadata field — not SOPS-encrypted?
```

The same category also reports files that are not a YAML mapping or not valid YAML. The check
cannot tell *for whom* a file was encrypted, only *that* it was.

## Plaintext secrets in entries

Every Markdown entry is scanned for key material that ended up outside the secrets category.

| Marker | Severity |
| --- | --- |
| `-----BEGIN PRIVATE KEY-----` | error |
| `-----BEGIN OPENSSH PRIVATE KEY-----` | error |
| `-----BEGIN RSA PRIVATE KEY-----` | error |
| `AGE-SECRET-KEY-1` | error |
| Words starting with `sk-`, `ghp_`, `gho_`, `AKIA`, `xoxb-`, `xoxp-` and more than 8 characters longer than the prefix | warning |

Key headers are unambiguous, so they fail the run. The prefixes can match ordinary identifiers,
so they only warn:

```text
Checking broken-knowledge-base/glossary/embedding.md
  WARNING: possible API key with prefix 'sk-' found — check whether this is a real secret
```

The scan is best effort. It is a safety net, not a replacement for a dedicated secret scanner.

## Large files

A Markdown entry larger than 500,000 bytes gets a warning suggesting a reference via
`_attachments.yml`. Change the threshold with `--max-file-size <bytes>`.

## `_attachments.yml`

Large originals (scans, exports, recordings) belong outside the entry text. An entry holds the
summary; `_attachments.yml` at the root of the knowledge directory points to the original. The
file is optional.

```yaml
attachments:
  - id: load-profile
    path: "./assets/load-profile.txt"
    description: "Load profile from the architecture review, kept as the original file"
    referenced_by:
      - decisions/2026-01-vector-store.md
  - id: review-recording
    path: "nas://archive/reviews/2026-01-15.mp4"
    description: "Recording of the architecture review on the office NAS (not checked from CI)"
    referenced_by:
      - decisions/2026-01-vector-store.md
```

Both `path` and `referenced_by` are relative to the knowledge directory.

| Field | Check |
| --- | --- |
| `id` | Empty is an error. |
| `description` | Empty or missing is a warning. |
| `path` starting with `http://` or `https://` | A HEAD request. Unreachable is a warning, because network conditions in CI vary. |
| `path` starting with `smb://`, `afp://` or `nas://` | Not checked; a CI runner cannot reach network shares. |
| any other `path` | Must exist on disk. Missing is an error. |
| `referenced_by` entries | Must exist on disk. Missing is an error. |

```text
Checking broken-knowledge-base/_attachments.yml
  WARNING: attachment 'contract-scan' has no description
  ERROR: attachment 'contract-scan': path './assets/contract-2026.pdf' does not exist
  ERROR: attachment 'contract-scan': referenced_by entry 'decisions/2026-03-hosting-contract.md' points to a file that doesn't exist
```

HTTP checks are the only network access knowledge-lint makes. Without `http(s)` attachments, a
run needs no network at all.
