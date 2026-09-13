---
title: Frontmatter
description: The fixed set of frontmatter fields every knowledge_entry file carries, and how each one is checked.
order: 1
---

```yaml
---
title: "RAG (retrieval-augmented generation)"
type: glossary
created: 2026-01-10
last_reviewed: 2026-01-10
status: current
source: intern
related: ["./chunking.md", "../decisions/2026-01-vector-store.md"]
public: true
---
```

| Field | Required | Check |
| --- | --- | --- |
| `title` | yes | Non-empty. |
| `type` | yes | Must equal the category's `type` in `_types.yml`. |
| `created` | yes | Non-empty. The format is not enforced. |
| `last_reviewed` | yes | A date in `YYYY-MM-DD` format. Drives the review-interval warning. |
| `status` | yes | One of `status_values` from `_types.yml`. |
| `source` | yes | Non-empty. Where the knowledge comes from: a meeting, a document, `intern`. |
| `related` | no | List of paths relative to the entry's own folder. Each must point to an existing file. |
| `public` | no | Boolean. Not validated; counted in the summary line so a publishing layer can filter on it. |

Other fields are allowed and ignored, so a knowledge base can add its own metadata (for example
`sphere`, `classification` or `document_id`) without breaking the lint.

A file without a readable frontmatter block is reported as
`frontmatter is missing or unreadable`, and no further field checks run for it.

## `related` links

Paths resolve against the folder of the entry that contains them, not against the knowledge
directory:

```yaml
# in knowledge/glossary/rag.md
related: ["./chunking.md", "../decisions/2026-01-vector-store.md"]
```

A link to a file that does not exist is an error. Links are checked for existence only; the
target does not have to be a knowledge entry.

## The `public` summary

After all checks, lint reports how many entries are marked public:

```text
3 of 4 knowledge entries are marked public.
```

knowledge-lint itself does not publish anything. The count is there so a site generator that
publishes only `public: true` entries has a visible counterpart in every run.
