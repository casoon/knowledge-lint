# knowledge-lint

A single Rust binary that validates and maintains a Markdown + YAML-frontmatter
knowledge base — the kind of `knowledge/` directory tree used by
[ai-knowledge-template](https://github.com/casoon/ai-knowledge-template), but
not tied to it. Categories, `type` values, and `status` vocabulary are all
declared in one config file, not hardcoded.

Full schema reference: [SPEC.md](SPEC.md).

## Why

Bash + `grep`/`cut` gets fragile fast once frontmatter has quoted strings,
nested punctuation, or non-ASCII text — it validates silently wrong instead
of failing loudly. A YAML/Python-based validator works, but adds a runtime
dependency. This is a static binary: no interpreter, no package manager,
same behavior in CI and locally.

## Install

```bash
cargo install knowledge-lint
```

Or build from source:

```bash
git clone https://github.com/casoon/knowledge-lint
cd knowledge-lint
cargo build --release
```

## Usage

```bash
# validate
knowledge-lint lint knowledge

# strip example/demo content back to an empty scaffold
knowledge-lint clean knowledge
```

`lint` exits `1` on any error (warnings alone exit `0`), so it composes
directly into CI:

```yaml
# .github/workflows/lint.yml
- uses: dtolnay/rust-toolchain@stable
- run: cargo install knowledge-lint
- run: knowledge-lint lint knowledge
```

## What it checks

- Every subdirectory of the knowledge directory is a declared category —
  undeclared categories are an error, not silently ignored.
- Frontmatter completeness (`title`, `type`, `created`, `last_reviewed`,
  `status`, `source`) and that `type` matches the entry's category.
- `status` is one of the values declared in `_types.yml` (fully
  configurable, not hardcoded to any language or vocabulary).
- `last_reviewed` is a valid date, and entries marked as still-active that
  are older than their category's review interval get a warning.
- `related` links resolve to files that actually exist.
- Every file in a `sops_secret` category is actually
  [SOPS](https://github.com/getsops/sops)-encrypted — checked structurally
  (a `sops:` metadata key), never by decrypting.
- A best-effort scan for plaintext secrets accidentally committed outside
  a `sops_secret` category (private-key headers, common API-key prefixes).
- Large files get a warning to reference them via `_attachments.yml`
  instead of embedding them.
- Every entry in `_attachments.yml` still resolves — local paths are
  checked for existence, `http(s)://` URLs with a HEAD request (a miss is
  a warning, since CI network conditions vary), network-share schemes are
  skipped as unreachable from a CI runner by construction. `referenced_by`
  entries (which knowledge entries mention the attachment) are checked the
  same way, and a missing `description` is a warning.
- A summary line reports how many `knowledge_entry` files are marked
  `public: true`, out of the total.

See [SPEC.md](SPEC.md) for the full config schema and the explicit
non-goals (frontmatter field *names* are fixed by design, not remappable).

## Origin

Extracted from [casoon/ai-knowledge-template](https://github.com/casoon/ai-knowledge-template),
where it started as an embedded tool before being generalized (config-driven
status vocabulary, English default messages) into its own project.

## License

[MIT](LICENSE).
