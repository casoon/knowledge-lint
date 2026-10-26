#!/usr/bin/env bash
# Regenerates the terminal captures on the start page: real knowledge-lint runs against the
# fixture in fixture/. Each state is a knowledge/ directory with one glossary category:
#   broken/  embedding.md has `status: stable` (not in status_values) and a related link to a
#            file that does not exist -> 2 errors, exit 1
#   fixed/   the same entry with `status: current` and a link to rag.md -> exit 0
#
#   cargo build --release && site/captures/capture.sh
#
# Use the freshly built release binary, not an installed one that may be older.
# The entries were last reviewed on 2026-09-30 with a review interval of 365 days: a capture
# taken after 2027-09-30 also reports a review warning. Update last_reviewed in the fixture
# before re-capturing then.
#
# Rebuild the scene after a capture:
#   cd site/captures && npx -y -p @casoon/castwright@0.4.0 -p shiki castwright build lint.terminal.yaml --format svg -o .
set -euo pipefail

repo="$(cd "$(dirname "$0")/../.." && pwd)"
out="$repo/site/captures"
bin="${KNOWLEDGE_LINT:-$repo/target/release/knowledge-lint}"

for state in broken fixed; do
  status=0
  (cd "$out/fixture/$state" && "$bin" lint) > "$out/lint-$state.txt" || status=$?
  echo "$status" > "$out/exit-$state.txt"
done
(cd "$out/fixture/fixed" && grep -E 'status|related' knowledge/glossary/embedding.md) > "$out/fix.txt"
