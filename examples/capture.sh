#!/usr/bin/env sh
# Regenerates the captured CLI output in examples/output/, shown on the website
# (site/src/showcase.ts and site/src/pages/index.astro).
#
#   cargo build --release && examples/capture.sh
#
# Staleness warnings depend on the day of the capture, so the day counts in the
# output change when this is re-run.
set -u

examples=$(cd "$(dirname "$0")" && pwd)
bin=${KNOWLEDGE_LINT:-$examples/../target/release/knowledge-lint}
bin=$(cd "$(dirname "$bin")" && pwd)/$(basename "$bin")

cd "$examples"
mkdir -p output

"$bin" lint knowledge-base > output/lint-knowledge-base.txt
echo "lint knowledge-base: exit $?"

"$bin" lint broken-knowledge-base > output/lint-broken-knowledge-base.txt
echo "lint broken-knowledge-base: exit $?"

# clean is destructive, so it runs on a throwaway copy.
tmp=$(mktemp -d)
cp -R knowledge-base "$tmp/"
(cd "$tmp" && "$bin" clean knowledge-base) > output/clean-knowledge-base.txt
echo "clean knowledge-base (copy): exit $?"
rm -rf "$tmp"
