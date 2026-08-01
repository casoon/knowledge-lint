# knowledge-lint — Specification

`knowledge-lint` validates and maintains a Markdown-based knowledge base: a
directory tree of category folders, each holding either frontmatter-driven
Markdown entries, SOPS-encrypted secrets, or plain referenced assets. This
document is the normative reference for the config and content it expects —
independent of any single project that uses it (it was extracted from
[casoon/ai-knowledge-template](https://github.com/casoon/ai-knowledge-template)
but has no dependency on it).

## Design goals

- **Config-driven categories.** Adding a category must never require a code
  change — only an edit to `_types.yml`.
- **Language-independent.** No field names, status values, or messages are
  hardcoded to a specific natural language. Defaults exist, everything else
  is declared in config.
- **Fixed frontmatter schema, flexible everything else.** The set of
  frontmatter *keys* (`title`, `type`, `created`, ...) is fixed by this
  spec — see [Non-goals](#non-goals) for why. Category names, `type` values,
  `status` values, and review intervals are all configurable.
- **A single static binary.** No runtime dependency beyond the compiled
  binary — no interpreter, no package manager, works identically in CI and
  locally.
- **Fail loudly, not silently.** Every check either passes, warns, or hard
  errors with a non-zero exit code. There is no "best effort" mode that
  swallows a malformed entry.

## `_types.yml`

Lives at the root of the knowledge directory (default path: `knowledge/`,
overridable via CLI argument).

```yaml
status_values: [current, deprecated]   # optional, default shown
active_status: current                 # optional, default shown

categories:
  <category-name>:
    kind: knowledge_entry | sops_secret | assets
    type: <string>                     # required for kind: knowledge_entry
    review_interval_days: <int>        # optional, only used for kind: knowledge_entry
```

| Field | Scope | Required | Meaning |
|---|---|---|---|
| `status_values` | top-level | no (default: `[current, deprecated]`) | Every allowed value for a `knowledge_entry`'s `status` frontmatter field. Must contain at least two values conceptually, though the tool does not enforce a minimum. |
| `active_status` | top-level | no (default: `current`) | Which value in `status_values` means "still maintained" — only entries with this status are checked against `review_interval_days`. |
| `categories.<name>.kind` | per category | yes | One of three category kinds, see below. |
| `categories.<name>.type` | per category | required for `knowledge_entry` | The expected `type` value in that category's frontmatter. A mismatch is an error. |
| `categories.<name>.review_interval_days` | per category | no | If set, entries with `status == active_status` whose `last_reviewed` is older than this many days produce a warning. |

Any subdirectory of the knowledge directory that is **not** declared in
`_types.yml` is an error — categories must be explicit.

### Category kinds

**`knowledge_entry`** — a normal Markdown file with YAML frontmatter (see
[Frontmatter schema](#frontmatter-schema-knowledge_entry)). The reference
implementation validates every `.md` file in the category directory except
one named `_template.md`, which is treated as a scaffold and skipped.

**`sops_secret`** — every file in the category must be a
[SOPS](https://github.com/getsops/sops)-encrypted YAML document. Validation
is structural only: the file must parse as YAML and contain a top-level
`sops:` key (which SOPS always adds to encrypted output). `knowledge-lint`
does not decrypt anything and never needs access to a private key. A file
named `.gitkeep` is ignored.

**`assets`** — a plain storage location for files referenced from
`_attachments.yml`. No content validation is performed. A file named
`.gitkeep` is ignored.

## Frontmatter schema (`knowledge_entry`)

```yaml
---
title: <string>            # required, non-empty
type: <string>              # required, must equal the category's `type`
created: <string>           # required, non-empty (format not enforced)
last_reviewed: <string>     # required, must be YYYY-MM-DD
status: <string>            # required, must be one of `status_values`
source: <string>            # required, non-empty
related: [<string>, ...]    # optional, paths relative to the file's own directory
public: <bool>               # optional, not validated for a specific value
---
```

`related` entries are resolved relative to the directory the referencing
file lives in and must point to a file that exists. `public` is parsed but
not semantically checked by `knowledge-lint` itself — it exists so that a
presentation layer (e.g. a docs site) can filter on it; see
[casoon/ai-knowledge-template](https://github.com/casoon/ai-knowledge-template)
for a worked example with Astro Starlight.

Every `knowledge_entry` file is also scanned for plaintext secret patterns
(private-key PEM headers, an `age` secret-key prefix, and common API-key
prefixes like `sk-`, `ghp_`, `AKIA`) — a hit is an error for the
unambiguous patterns and a warning for the prefix heuristics, since those
can false-positive.

## `_attachments.yml`

Optional. If present, lives at the root of the knowledge directory.

```yaml
attachments:
  - id: <string>              # required, non-empty
    path: <string>             # required — see resolution rules below
    description: <string>      # optional, free text
    referenced_by: [<string>]  # optional, free text, not validated
```

Path resolution:

- `http://` / `https://` — checked with a HEAD request; unreachable is a
  **warning**, not an error (network conditions in CI are not assumed to be
  reliable).
- `smb://`, `afp://`, `nas://` — not checked (unreachable from a typical CI
  runner by construction), field presence only.
- Anything else — treated as a filesystem path, resolved relative to the
  knowledge directory if not absolute, and must exist on disk. Missing is
  an **error**.

## CLI

```
knowledge-lint lint [KNOWLEDGE_DIR] [--max-file-size <bytes>]
knowledge-lint clean [KNOWLEDGE_DIR]
```

`KNOWLEDGE_DIR` defaults to `knowledge`. `lint` exits `1` if any error was
found (warnings alone exit `0`) — this is the contract CI should rely on.
`clean` removes every file in every category except the kind-specific
survivor (`_template.md` for `knowledge_entry`, `.gitkeep` for
`sops_secret`/`assets`), and resets `_attachments.yml` to an empty list if
it exists. `clean` is meant for stripping a freshly generated instance's
demo content — it is destructive and does not ask for confirmation.

## Non-goals

- **Frontmatter field names are not configurable.** Making every key
  (`title`, `created`, ...) remappable would turn this into a generic
  schema-validation engine rather than an opinionated convention with
  tooling — closer to what
  [OKF](https://github.com/GoogleCloudPlatform/knowledge-catalog) or
  [MADR](https://github.com/adr/madr) are. If your project needs a
  different field vocabulary, this is deliberately not the right tool
  without forking it.
- **No decryption, no key management.** `knowledge-lint` verifies that a
  `sops_secret` file *looks* encrypted; it never touches a private key.
- **No content-quality or prose checks.** This validates structure and
  metadata, not whether an entry is well-written or factually correct.
