# Summary: 11-02 — Fix Fleet Reservation FK

**Status:** Complete
**Duration:** ~10 min
**Requirement:** CORE-02

## Changes Made

### src/modules/fleet/infrastructure/fleet_repository.rs
- Added `pool()` getter method to expose the database pool

### src/modules/fleet/application/fleet_service.rs
- Added `pool: PgPool` field to `FleetService`
- Added `resolve_local_user_id()` helper method
- Updated `create_reservation()` to resolve Keycloak ID to local user ID
- Updated `update_reservation()` to use resolved user ID for ownership check
- Updated `cancel_reservation()` to use resolved user ID for ownership check
- Updated `list_reservations()` to resolve user ID for filtering
- Updated `list_my_reservations()` to use resolved local user ID

## Root Cause

Same as 11-01: Keycloak user IDs were being passed to FK columns expecting local `users.id` values.

## Additional Issue Found

The ownership checks in `update_reservation()` and `cancel_reservation()` compared `current.user_id` (local UUID) with `ctx.user_id` (Keycloak UUID), which would always fail, preventing users from managing their own reservations.

## Solution

1. Resolve Keycloak user ID before creating reservation
2. Use resolved local user ID for ownership comparisons
3. Use resolved local user ID for filtering reservations

## Verification

- `cargo check` passes
- `cargo build --release` succeeds
