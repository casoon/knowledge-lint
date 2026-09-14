---
title: Quickstart
description: Declare one category, add one entry, run the lint, and read what it reports.
order: 2
---

## Declare a category

knowledge-lint looks for a `knowledge/` directory by default. Its `_types.yml` declares the
categories; each category is a folder of the same name.

```yaml
# knowledge/_types.yml
categories:
  glossary:
    kind: knowledge_entry
    type: glossary
    review_interval_days: 365
```

Without `status_values`, the allowed statuses are `current` and `deprecated`.

## Add an entry

```markdown
---
title: "Frontmatter"
type: glossary
created: 2026-09-14
last_reviewed: 2026-09-14
status: current
source: intern
public: true
---

A YAML block at the top of a Markdown file that holds its metadata.
```

Save it as `knowledge/glossary/frontmatter.md`.

## Run the lint

```sh
knowledge-lint lint
```

```text
Checking knowledge/glossary/frontmatter.md

1 of 1 knowledge entries are marked public.
All checks passed (0 warning(s)).
```

The exit code is `0`.

## Break something

Add a folder `knowledge/notes/` without declaring it, and set `status: draft` in the entry:

```text
Checking knowledge/notes/
  ERROR: category 'notes' is not declared in _types.yml
Checking knowledge/glossary/frontmatter.md
  ERROR: status 'draft' is not one of the allowed values (current, deprecated)

1 of 1 knowledge entries are marked public.
2 error(s), 0 warning(s) found.
```

The exit code is `1`. Either declare the folder and add `draft` to `status_values`, or remove
the folder and go back to `status: current`.

## Next steps

- [Configuring `_types.yml`](../../guides/configuration/) for every option.
- [CI usage](../../guides/ci/) to run the check on every push.
- The [showcase](../../../showcase/) for the full output on a larger sample knowledge base.
