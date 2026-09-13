---
title: Configuring _types.yml
description: One file at the root of the knowledge directory declares every category, the frontmatter type it expects, the status vocabulary and how often entries must be reviewed.
order: 1
---

`_types.yml` lives at the root of the knowledge directory (`knowledge/` by default). knowledge-lint
reads it before anything else. If it is missing or not valid YAML, the run stops with an error and
exit code `1`.

```yaml
status_values: [current, deprecated]   # optional, default shown
active_status: current                 # optional, default shown

categories:
  decisions:
    kind: knowledge_entry
    type: decision
    review_interval_days: 365
  secrets:
    kind: sops_secret
  assets:
    kind: assets
```

## Fields

| Field | Scope | Required | Meaning |
| --- | --- | --- | --- |
| `status_values` | top level | no, default `[current, deprecated]` | Every allowed value of the `status` frontmatter field. |
| `active_status` | top level | no, default `current` | The status that means "still maintained". Only entries with this status are checked against `review_interval_days`. |
| `categories.<name>.kind` | category | yes | `knowledge_entry`, `sops_secret` or `assets`, see below. |
| `categories.<name>.type` | category | for `knowledge_entry` | The `type` value every entry in this category must carry. |
| `categories.<name>.review_interval_days` | category | no | Entries with `status` equal to `active_status` whose `last_reviewed` date is older than this many days get a warning. |

The category name is the name of the folder directly below the knowledge directory. Adding a
category means adding a folder and an entry here, never a code change.

## Categories must be declared

Every folder directly below the knowledge directory must appear under `categories`. An
undeclared folder is an error, and nothing inside it is checked:

```text
Checking broken-knowledge-base/meetings/
  ERROR: category 'meetings' is not declared in _types.yml
```

Files at the root of the knowledge directory (`_types.yml`, `_attachments.yml`, a
`_template.md`, a README) are not part of any category and are not validated as entries.

## Category kinds

### `knowledge_entry`

Markdown files with YAML frontmatter. Every `.md` file in the folder and its subfolders is
checked against the [frontmatter schema](../../reference/frontmatter/); other file types are
skipped. A file named `_template.md` is treated as a scaffold and skipped, wherever it sits in the
category.

Subfolders are allowed and belong to the category, for example `concepts/business/` and
`concepts/private/` under a declared `concepts` category.

Set `type` for every `knowledge_entry` category. Without it, the type check is skipped for that
category.

### `sops_secret`

Every file must be a [SOPS](https://github.com/getsops/sops)-encrypted YAML document. See
[Secrets and attachments](../secrets-and-attachments/). A file named `.gitkeep` is ignored.

### `assets`

A plain storage folder for files referenced from `_attachments.yml`. Nothing inside it is
validated.

## Your own status vocabulary

Status values are not tied to a language or a workflow. A German knowledge base can use
`[aktuell, veraltet]`, a documentation project `[draft, stable, deprecated]`:

```yaml
status_values: [draft, stable, deprecated]
active_status: stable
```

With this configuration, `status: current` is an error, and only `stable` entries are checked
for overdue reviews:

```text
Checking broken-knowledge-base/decisions/2026-02-backups.md
  ERROR: status 'current' is not one of the allowed values (draft, stable, deprecated)
```

## Review intervals

`review_interval_days` turns `last_reviewed` into a maintenance signal. An active entry that has
not been reviewed within the interval stays valid but gets a warning, so the run still exits `0`:

```text
Checking knowledge-base/glossary/chunking.md
  WARNING: last_reviewed is 469 days old (> 365), but the entry is marked 'current'
```

Entries with any other status (`deprecated`, `draft` …) are never flagged as overdue. Deprecating
an entry instead of deleting it keeps the history without producing warnings.

## What is not configurable

The frontmatter field names (`title`, `type`, `created`, `last_reviewed`, `status`, `source`,
`related`, `public`) are fixed by design. The full reasoning is in the
[specification](https://github.com/casoon/knowledge-lint/blob/master/SPEC.md#non-goals).
