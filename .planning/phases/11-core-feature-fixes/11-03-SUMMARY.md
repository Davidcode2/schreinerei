# Summary: 11-03 — Fix Time Entries & Calendar

**Status:** Complete
**Duration:** ~10 min
**Requirements:** CORE-03, CORE-04

## Changes Made

### src/modules/sites/infrastructure/site_repository.rs
- Added `pool()` getter method to expose the database pool

### src/modules/sites/application/site_service.rs
- Added `pool: PgPool` field to `SiteService`
- Added `resolve_local_user_id()` helper method
- Updated `create_time_entry()` to resolve Keycloak ID to local user ID
- Updated `create_activity()` to resolve Keycloak ID to local user ID
- Updated `list_my_time_entries()` to use resolved local user ID for filtering

### src/modules/fleet/api/routes.rs
- Added `NaiveDate` import from chrono
- Updated `get_calendar()` to accept both RFC3339 and date-only formats
- Added `parse_date_time()` helper function for flexible date parsing

## Root Cause

1. Time entries: Same FK issue as 11-01 and 11-02
2. Calendar: Frontend sends `YYYY-MM-DD` format but backend expected RFC3339

## Solution

1. Resolve Keycloak user ID before creating time entries and activities
2. Accept both date formats in calendar endpoint:
   - RFC3339: `2024-04-30T00:00:00Z`
   - Date-only: `2024-04-30`

## Verification

- `cargo check` passes
- `cargo build --release` succeeds
