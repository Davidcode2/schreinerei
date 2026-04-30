# Phase 15: ts-rs Type Generation - Context

**Gathered:** 2026-04-30
**Status:** Ready for planning
**Source:** Requirements ROADMAP.md + codebase analysis

<domain>
## Phase Boundary

This phase adds ts-rs derive macros to all backend DTOs and generates TypeScript types to prevent frontend-backend type drift. The generated types will replace the currently manual type definitions in `frontend/src/types/*.ts`.

**What's in scope:**
- Add ts-rs crate dependency to Cargo.toml
- Add `#[derive(TS)]` and `#[ts(export)]` to all request/response DTOs in API routes
- Configure ts-rs output directory (`frontend/src/types/generated.ts`)
- Run type generation via `cargo test --features ts-rs/export`
- Update frontend imports to use generated types
- Add CI check to fail if generated types differ from committed

**What's NOT in scope:**
- Changing API contracts or DTO field names
- Adding new DTOs (only adding ts-rs to existing)
- Frontend business logic changes
- E2E test changes
</domain>

<decisions>
## Implementation Decisions

### D-01: ts-rs library selection
**Decision:** Use ts-rs crate for type generation
**Rationale:** Already documented in AGENTS.md as the chosen solution. Zero-config, derive-macro based, generates directly to TypeScript.

### D-02: Output location
**Decision:** Generate to `frontend/src/types/generated.ts`
**Rationale:** Single file for all generated types, separate from manually maintained types (like `KeycloakTokenPayload` which comes from auth, not backend DTOs).

### D-03: Type generation command
**Decision:** Use `cargo test --features ts-rs/export`
**Rationale:** ts-rs exports types during test compilation. This is the standard approach per ts-rs documentation.

### D-04: CI integration
**Decision:** Add GitHub Actions step that regenerates types and fails if git shows uncommitted changes
**Rationale:** Prevents type drift - any DTO change that affects types must include regenerated output.

### D-05: DTO scope - all API route DTOs
**Decision:** Add ts-rs to ALL request and response DTOs in:
- `src/modules/inventory/api/routes.rs`
- `src/modules/sites/api/routes.rs`
- `src/modules/fleet/api/routes.rs`
- `src/modules/iam/api/routes.rs`

**Rationale:** Complete coverage prevents partial type safety gaps.

### D-06: Enum handling
**Decision:** Add ts-rs to enum types used in DTOs (status types, etc.)
**Rationale:** Enums are part of the API contract and need TypeScript equivalents.

### the agent's Discretion
- Exact file organization within `generated.ts` (ts-rs handles this automatically)
- Whether to keep or remove existing manual type files after migration
- Test file naming and location

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### ts-rs Documentation
- ts-rs crate: https://docs.rs/ts-rs/latest/ts_rs/
- Export feature: `#[ts(export)]` macro generates file
- Output directory: configurable via `#[ts(export, export_to = "path/")]`

### Existing Type Files (to be replaced)
- `frontend/src/types/inventory.ts` — Manual inventory types
- `frontend/src/types/sites.ts` — Manual sites types
- `frontend/src/types/fleet.ts` — Manual fleet types
- `frontend/src/types/user.ts` — Manual user types (partial replacement)
- `frontend/src/types/api.ts` — Generic types (keep)

### Backend DTO Files
- `src/modules/inventory/api/routes.rs` — ~20 DTOs
- `src/modules/sites/api/routes.rs` — ~15 DTOs
- `src/modules/fleet/api/routes.rs` — ~25 DTOs
- `src/modules/iam/api/routes.rs` — ~5 DTOs

</canonical_refs>

<specifics>
## Specific Requirements

### TEST-07: ts-rs derive macros added to all backend DTOs
- Add `ts-rs` dependency to `Cargo.toml` with `export` feature in dev-dependencies
- Add `use ts_rs::TS;` import to each routes.rs file
- Add `#[derive(Serialize, TS)]` and `#[ts(export, export_to = "frontend/src/types/generated.ts")]` to each DTO struct
- Handle enum types with `#[derive(TS)]`

### TEST-08: TypeScript types auto-generated from Rust structs
- Run `cargo test --features ts-rs/export` to generate types
- Verify `frontend/src/types/generated.ts` is created with all types
- Types must match current manual definitions exactly (field names, types, nullability)

### TEST-09: CI fails if generated types differ from committed
- Add GitHub Actions workflow step
- Step: regenerate types, check git diff, fail if uncommitted changes
- Command: `cargo test --features ts-rs/export && git diff --exit-code frontend/src/types/generated.ts`

</specifics>

<deferred>
## Deferred Ideas

None — Phase scope is well-defined by requirements.

</deferred>

---

*Phase: 15-ts-rs-type-generation*
*Context gathered: 2026-04-30*
