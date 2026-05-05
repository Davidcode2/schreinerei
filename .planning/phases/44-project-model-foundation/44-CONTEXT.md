# Phase 44: Project Model Foundation - Context

**Gathered:** 2026-05-05
**Status:** Ready for planning
**Source:** Milestone v1.13 definition and requirement review synthesis

## Phase Boundary

This phase broadens the current `sites` aggregate into a clearer project execution model without creating a second competing runtime entity. The goal is to support both external Baustellen and internal workshop work while preserving the existing working flows and data model as much as possible.

This phase is foundational. It should not attempt to solve the full timeline unification or project-linked material/time enforcement yet, but it must make those later phases possible.

## Implementation Decisions

### Locked

- Keep one aggregate lineage: evolve `sites` into `projects`; do not create a separate parallel project table unless forced by evidence.
- Preserve runtime continuity: existing routes, persistence, and shipped flows must keep working during the transition.
- Introduce internal workshop projects so future productive workshop work does not depend on null project/site linkage.
- Strengthen planning fields on the existing aggregate instead of inventing a second planning surface.

### OpenCode's Discretion

- Exact naming and migration strategy between `site` and `project` in API and frontend copy.
- Whether Phase 44 should introduce alias types, DTO fields, or route copy changes first.
- How much of worker assignment UX belongs in this phase versus a later planning-focused phase.

## Canonical References

### Milestone docs
- `.planning/PROJECT.md` — current milestone goal and active scope
- `.planning/REQUIREMENTS.md` — `PROJ-10`, `PROJ-11`
- `.planning/ROADMAP.md` — Phase 44 goal, requirement mapping, success criteria
- `.planning/REQUIREMENT-REVIEW-SUMMARY.md` — product-direction decisions and priority calls

### Detailed requirement reviews
- `.planning/requirement-reviews/projects/PROJ-10.md` — unify Baustelle and project model
- `.planning/requirement-reviews/projects/PROJ-11.md` — planning context and editable project fields
- `.planning/requirement-reviews/architecture/ARCH-01.md` — modular boundary evolution toward `projects`

### Existing implementation baseline
- `src/modules/projects` via `src/modules.rs` alias — architectural boundary introduced in v1.12
- `src/modules/sites/domain/site.rs` — current aggregate shape
- `src/modules/sites/api/routes.rs` — current route contract
- `frontend/src/pages/sites/*` — current user-facing project/baustelle flows

## Specific Ideas

- Start with terminology and model broadening before deep data migration.
- Keep the change low-risk and compatible with shipped activity feed and time booking flows.
- Internal workshop work should become a first-class context, not an afterthought.

## Deferred Ideas

- Unified project timeline composer belongs to Phase 45.
- Required project linkage for material/time capture belongs to Phase 46.
- Dashboard visibility changes belong to Phase 47.

---

*Phase: 44-project-model-foundation*
*Context gathered: 2026-05-05 via milestone definition*
