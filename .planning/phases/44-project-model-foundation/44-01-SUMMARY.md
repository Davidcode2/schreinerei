# 44-01 Summary

## Outcome

The existing `sites` aggregate can now represent both external and internal projects without introducing a second entity or changing runtime routes.

## Changes

- Added migration `018_project_type_on_sites.sql` with non-null `project_type` and backfill to `external_site`
- Added shared `ProjectType` enum in `src/common/types.rs`
- Threaded `project_type` through site domain contracts, repository SQL, dashboard projection, and site API DTOs
- Regenerated `frontend/src/types/generated.ts` through `cargo test export_bindings --lib`
- Added targeted tests for project type parsing, internal workshop validation, and API/dashboard DTO exposure

## Verification

- `cargo test project_type_roundtrips --lib`
- `cargo test create_site_validate_allows_internal_workshop_without_location --lib`
- `cargo test site_response_includes_project_type --lib`
- `cargo test dashboard_site_includes_project_type --lib`
- `cargo test export_bindings --lib`

## Notes

- The documented `cargo test --features ts-rs/export` command is stale in this repo state; `cargo test export_bindings --lib` is the working ts-rs export path.
