---
phase: 28-material-history-tab
plan: 01
subsystem: api
tags: [rust, sqlx, inventory, history]
requires: []
provides:
  - Site-scoped inventory history endpoint with category and extractor fields
affects: [frontend-history-hook, activity-feed-material-tab]
tech-stack:
  added: []
  patterns: [tenant-scoped joined query]
key-files:
  created: []
  modified:
    - src/modules/inventory/domain/stock_entry.rs
    - src/modules/inventory/infrastructure/material_repository.rs
    - src/modules/inventory/api/routes.rs
key-decisions:
  - "Use COALESCE(u.name, u.email, se.user_id::text) to guarantee extracted_by display value"
requirements-completed: [HIST-03, HIST-04, HIST-05]
duration: 35min
completed: 2026-05-01
---

# Phase 28 Plan 01: Backend Site Material History Summary

**Tenant-scoped site history endpoint now returns eager-joined material/category/site and extractor context in one query.**

## Task Commits
1. `c1331f8` — backend contracts + endpoint + query

## Deviations from Plan
- **[Rule 3 - Blocking]** `cargo test --features ts-rs/export` is invalid for current ts-rs version; used `cargo test export_bindings --lib` to regenerate/export DTO bindings.

## Self-Check: PASSED
