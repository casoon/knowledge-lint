---
title: "Decision: vector store for the internal RAG system"
type: decision
created: 2026-01-15
last_reviewed: 2026-07-01
status: current
source: "Architecture review, 2026-01-15"
related: ["../glossary/rag.md"]
---

## Context

The internal RAG system needs a vector store that fits the existing Postgres
setup without adding another service to operate.

## Decision

`pgvector` as a Postgres extension instead of a separate vector database.

## Consequences

If the data volume grows well beyond one million entries, this decision is due
for review. The load profile is referenced as `load-profile` in
[`_attachments.yml`](../_attachments.yml).
