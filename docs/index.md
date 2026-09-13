---
title: Overview
description: What knowledge-lint checks, where it stops, and how this documentation is organised.
order: 0
---

knowledge-lint validates a knowledge base made of Markdown files with YAML frontmatter: a
directory of category folders, declared in one `_types.yml`. It is a single Rust binary, so it
behaves the same locally and in CI, without an interpreter or package manager.

It was extracted from [ai-knowledge-template](https://github.com/casoon/ai-knowledge-template),
but works with any knowledge directory that follows the same conventions.

## What it checks

- Every folder is a declared category, and every entry's `type` matches its category.
- Required frontmatter fields, allowed `status` values and valid `last_reviewed` dates.
- Overdue reviews for entries that are still marked active.
- `related` links and `_attachments.yml` references point to files that exist.
- Secrets are SOPS-encrypted, and entries contain no plaintext keys.

## Where it stops

knowledge-lint checks structure and metadata, not prose. It does not decrypt secrets, does not
judge whether an entry is correct, and does not let you rename the frontmatter fields.

## How the docs are organised

- **Getting started**: [install](getting-started/installation/) the binary and
  [lint a first knowledge base](getting-started/quickstart/).
- **Guides**: [configuring `_types.yml`](guides/configuration/),
  [secrets and attachments](guides/secrets-and-attachments/), [CI usage](guides/ci/).
- **Reference**: [frontmatter](reference/frontmatter/), every [rule](reference/rules/) and the
  [CLI](reference/cli/). The normative schema is in
  [SPEC.md](https://github.com/casoon/knowledge-lint/blob/master/SPEC.md).
