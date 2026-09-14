---
title: CI usage
description: lint exits 1 on any error and 0 when only warnings remain, so it drops into any CI system as a single step.
order: 3
---

## The exit code contract

| Result | Exit code |
| --- | --- |
| No errors (warnings allowed) | `0` |
| At least one error | `1` |
| `_types.yml` or the knowledge directory cannot be read | `1` |

Warnings (overdue reviews, missing attachment descriptions, API-key-shaped words, unreachable
URLs) never fail a run. Errors always do. There is no mode that downgrades errors.

## GitHub Actions

The shortest setup installs the published release from crates.io:

```yaml
# .github/workflows/lint.yml
name: Lint knowledge base

on:
  push:
    paths: ["knowledge/**"]
  pull_request:
    paths: ["knowledge/**"]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install knowledge-lint
      - run: knowledge-lint lint knowledge
```

Run it on direct pushes as well as on pull requests: in a small repository many entries are
committed without a pull request, and a lint that never runs on them checks nothing.

## Pin and cache the binary

`cargo install` compiles knowledge-lint on every run. Pinning a commit and caching the binary
makes the step fast and keeps a change in knowledge-lint from turning every repository red at
once. [ai-knowledge-template](https://github.com/casoon/ai-knowledge-template) uses this setup:

```yaml
jobs:
  lint:
    runs-on: ubuntu-latest
    env:
      # Update deliberately, not automatically.
      KNOWLEDGE_LINT_REV: <commit-sha>
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - id: cache
        uses: actions/cache@v4
        with:
          path: ~/.cargo/bin/knowledge-lint
          key: knowledge-lint-${{ runner.os }}-${{ env.KNOWLEDGE_LINT_REV }}

      - if: steps.cache.outputs.cache-hit != 'true'
        run: |
          cargo install --git https://github.com/casoon/knowledge-lint \
            --rev "$KNOWLEDGE_LINT_REV" --locked

      - run: knowledge-lint lint knowledge
```

## Network access

The only network request is a HEAD request per `http(s)://` path in `_attachments.yml`. A
failure is a warning, so a flaky runner does not fail the build. Network shares (`smb://`,
`afp://`, `nas://`) are skipped.

## Review dates in CI

Overdue reviews depend on the day the job runs. An entry that passes today warns once its
`review_interval_days` has passed, without any commit. This is intended: the warning shows up
in the next run, but never blocks a merge.

## Keep `clean` out of CI

`knowledge-lint clean` deletes entries without asking. It is meant for stripping demo content
once, right after creating a repository from a template, not for automation. See the
[CLI reference](../../reference/cli/).
