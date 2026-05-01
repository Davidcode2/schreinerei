# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.8 — Activity Feed & Site Status

**Shipped:** 2026-05-01
**Phases:** 4 | **Plans:** 11

### What Was Built
- Baustelle status workflow (geplant → aktiv → abgeschlossen) with modal and optimistic locking
- Tabbed activity feed with Notes/Documents and Material tabs
- Material extraction history with category, extractor, and Baustelle navigation links
- Photo upload pipeline with multipart, UUID storage, server-side thumbnails, authenticated blob fetch
- Offline photo queue with data-URL persistence and reconnect sync
- Camera-first modal entry for photo uploads

### What Worked
- Phase 28 was the smoothest — backend endpoint, frontend hook, UI tab. Three focused plans, no rework.
- Combined plan format (single PLAN.md per phase) worked well for Phases 26 and 27 which were well-scoped.
- Gap closure plans (29-05, 29-06) caught real integration issues (multipart field mismatch, duplicate activity creation).
- UAT with specific expected behavior per test made verification fast.

### What Was Inefficient
- Phase 29 required 6 plans including 2 gap-closure plans. The multipart field name mismatch (file vs photo) between online and offline paths should have been caught in the original plan.
- Offline support is only partially implemented — the queue and sync infrastructure exists, but broader offline functionality isn't in place, making runtime testing impossible.
- Returning to fix issues in phases (29-05, 29-06) cost extra execution time that upfront contract alignment could have prevented.

### Patterns Established
- Authenticated blob fetch pattern for serving protected files — should be reused for any future binary content (PDFs, documents).
- Upload-first, activity-second pattern for file attachments — decouples storage from business events.
- Gap closure plans as a formal mechanism for fixing integration issues without reopening original plans.

### Key Lessons
1. Align field names across all client paths (online, offline, tests) before executing upload plans — multipart contract mismatches are subtle and expensive to fix.
2. Nullable foreign keys (activity_id on attachments) enable decoupled upload flows — don't tie file storage to business event creation.
3. Eager-loaded queries with COALESCE for display fields (extractor name) prevent N+1 while gracefully handling missing data.

### Cost Observations
- Model mix: primarily opus for planning/verification, sonnet for execution
- 1 day for 4 phases (fast execution, tight verification cycle)
- Notable: v1.8 was planned and executed in a single day with strong verification coverage

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | 2 | 5 | Initial MVP built |
| v1.5 | 2 | 6 | Testing infrastructure established |
| v1.7 | 1 | 4 | Active context and auto-assignment |
| v1.8 | 1 | 4 | Activity feed and file attachments |

### Cumulative Quality

| Milestone | Tests | UAT Coverage | Zero-Dep Additions |
|-----------|-------|--------------|-------------------|
| v1.0 | 116+28+22 | Manual | 37 requirements |
| v1.7 | 116+28+22 | UAT introduced | 17 requirements |
| v1.8 | 116+28+22+7 | UAT + Verification | 21 requirements, 2 gap closures |

### Top Lessons (Verified Across Milestones)

1. Eager-loaded queries prevent N+1 in all list views (verified in v1.7 deduction history and v1.8 material history)
2. Keycloak user resolution via find_or_create prevents FK constraint errors across all modules
3. Upload-first, business-second pattern avoids coupling storage to domain events