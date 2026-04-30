# Summary: 11-01 — Fix Material Withdrawal FK

**Status:** Complete
**Duration:** ~10 min
**Requirement:** CORE-01

## Changes Made

### src/modules/iam/infrastructure/user_repository.rs
- Added `find_or_create_by_keycloak_id()` method to find or create user from Keycloak ID
- Added `get_local_user_id()` helper method

### src/modules/iam/application/user_service.rs
- Added `email` field to `TenantContext` struct
- Updated `from_auth()` to include email from AuthenticatedUser

### src/modules/inventory/infrastructure/material_repository.rs
- Added `pool()` getter method to expose the database pool

### src/modules/inventory/application/inventory_service.rs
- Added `pool: PgPool` field to `InventoryService`
- Added `resolve_local_user_id()` helper method
- Updated `withdraw_material()` to resolve Keycloak ID to local user ID
- Updated `adjust_stock()` to resolve Keycloak ID to local user ID
- Updated `create_order_request()` to resolve Keycloak ID to local user ID
- Updated `approve_order_request()` to resolve Keycloak ID to local user ID
- Updated all event payloads to use `local_user_id` instead of `ctx.user_id`

## Root Cause

The `AuthenticatedUser.user_id` contains the Keycloak UUID, but database FK constraints reference `users.id` (local UUID). The backend was passing Keycloak IDs directly to FK columns.

## Solution

1. Resolve Keycloak user ID to local `users.id` before any FK insert
2. Use `find_or_create_by_keycloak_id()` to auto-provision users if needed
3. Store resolved local user ID in `stock_entries.user_id`

## Verification

- `cargo check` passes
- `cargo build --release` succeeds
