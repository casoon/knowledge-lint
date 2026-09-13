---
title: Rules
description: Every message knowledge-lint prints, with its severity. Errors fail the run, warnings do not.
order: 2
---

Messages appear under the `Checking <path>` line of the file or folder they belong to. The text
below is the exact output; `<…>` marks the variable part.

## Categories

| Message | Severity |
| --- | --- |
| `category '<name>' is not declared in _types.yml` | error |

## Knowledge entries

Checked in this order for every `.md` file in a `knowledge_entry` category:

| Message | Severity |
| --- | --- |
| `file is <n> KB — consider referencing large content via _attachments.yml instead of embedding it` | warning |
| `possible plaintext key found (pattern: '<pattern>') — belongs in a sops_secret category, encrypted` | error |
| `possible API key with prefix '<prefix>' found — check whether this is a real secret` | warning |
| `frontmatter is missing or unreadable` | error |
| `title is missing` | error |
| `source is missing` | error |
| `created is missing` | error |
| `status is missing` | error |
| `status '<value>' is not one of the allowed values (<status_values>)` | error |
| `type '<value>' does not match category '<name>' (expected: '<type>')` | error |
| `type is missing` | error |
| `last_reviewed is missing` | error |
| `last_reviewed '<value>' is not a valid date (YYYY-MM-DD)` | error |
| `last_reviewed is <n> days old (> <interval>), but the entry is marked '<active_status>'` | warning |
| `related link '<path>' points to a file that doesn't exist` | error |

## Secrets

For every file in a `sops_secret` category except `.gitkeep`:

| Message | Severity |
| --- | --- |
| ``file in a sops_secret category has no `sops:` metadata field — not SOPS-encrypted?`` | error |
| `file in a sops_secret category is not a YAML mapping — expected a SOPS-encrypted document` | error |
| `file in a sops_secret category is not valid YAML: <details>` | error |

## Attachments

Only when `_attachments.yml` exists:

| Message | Severity |
| --- | --- |
| `Could not read <path>: <details>` | error |
| `<path> is not valid YAML: <details>` | error |
| `attachment without an id` | error |
| `attachment '<id>' has no description` | warning |
| `attachment '<id>': URL '<url>' is not reachable (<details>)` | warning |
| `attachment '<id>': path '<path>' does not exist` | error |
| `attachment '<id>': referenced_by entry '<path>' points to a file that doesn't exist` | error |

## Aborting errors

Some problems stop the run before a report is printed, with an `Error:` line and exit code
`1`: the knowledge directory or `_types.yml` cannot be read, `_types.yml` is not valid YAML
or uses an unknown `kind`, or a checked file cannot be read as UTF-8 text.

## Summary

The run ends with one or two summary lines:

```text
3 of 4 knowledge entries are marked public.
All checks passed (1 warning(s)).
```

```text
0 of 3 knowledge entries are marked public.
9 error(s), 3 warning(s) found.
```

The `public` line is omitted when the knowledge base has no entries.
