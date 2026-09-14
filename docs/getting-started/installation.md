---
title: Installation
description: knowledge-lint is a single Rust binary. Install it from crates.io, straight from the repository, or build it from source.
order: 1
---

All three ways need a Rust toolchain with `cargo`. No prebuilt binaries are published.

## From crates.io

```sh
cargo install knowledge-lint
```

This installs the latest published release, currently 0.1.0, into `~/.cargo/bin`.

## From the repository

```sh
cargo install --git https://github.com/casoon/knowledge-lint
```

Add `--rev <commit>` to pin a commit. For CI, pinning plus a cached binary is the recommended
setup, see [CI usage](../../guides/ci/).

## From source

```sh
git clone https://github.com/casoon/knowledge-lint
cd knowledge-lint
cargo build --release
```

The binary is `target/release/knowledge-lint`.

## Verify the install

```sh
knowledge-lint --help
```

```text
Validates and maintains a Markdown + YAML-frontmatter knowledge base against a declarative _types.yml config

Usage: knowledge-lint <COMMAND>

Commands:
  lint   Check frontmatter, status values, related-links, staleness, secret encryption, attachments
  clean  Remove example content from every category, keep structure and _template.md
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Next steps

Continue with the [Quickstart](../quickstart/) to lint a first knowledge base.
