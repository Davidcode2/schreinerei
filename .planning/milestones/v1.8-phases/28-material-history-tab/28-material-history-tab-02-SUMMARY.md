---
phase: 28-material-history-tab
plan: 02
subsystem: ui
tags: [react-query, typescript, inventory]
requires:
  - phase: 28-01
    provides: site history API shape
provides:
  - Site material history hook and typed frontend contract
affects: [site-activity-feed]
tech-stack:
  added: []
  patterns: [site-scoped query key]
key-files:
  created: []
  modified:
    - frontend/src/types/inventory.ts
    - frontend/src/lib/api/hooks/useInventory.ts
    - frontend/src/lib/api/hooks/useInventory.test.tsx
    - frontend/src/types/generated.ts
key-decisions:
  - "Keep dedicated site-history interface separate from material-specific history DTO"
requirements-completed: [HIST-01, HIST-03, HIST-04]
duration: 20min
completed: 2026-05-01
---

# Phase 28 Plan 02: Frontend Hook and Types Summary

**React Query hook and TS contracts now fetch and type enriched material history rows for a single Baustelle.**

## Task Commits
1. `400b868` — site history hook, tests, generated type sync

## Deviations from Plan
None - plan executed with equivalent export verification command (`cargo test export_bindings --lib`).

## Self-Check: PASSED
