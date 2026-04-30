# Bug Report: Phase 11 - Core Feature FK Constraint Violations

## Summary

Multiple core features fail with foreign key constraint violations because the backend passes Keycloak user IDs directly to database operations, but the FK constraints expect local `users.id` values.

## Root Cause Analysis

### The Problem

The authentication flow:
1. User authenticates with Keycloak
2. JWT contains `sub` claim = Keycloak user UUID (e.g., `a1b2c3d4-...`)
3. `AuthenticatedUser.user_id` stores this Keycloak UUID
4. Code passes this UUID to repository methods
5. Repository inserts into tables with FK to `users.id`

But the `users` table has:
- `id UUID PRIMARY KEY DEFAULT uuid_generate_v4()` - locally generated UUID
- `keycloak_user_id VARCHAR(255) NOT NULL` - the Keycloak user ID

### The Mismatch

When we try to insert into `stock_entries`:
```sql
INSERT INTO stock_entries (..., user_id, ...)
VALUES (..., <keycloak_uuid>, ...)  -- WRONG! FK expects users.id
```

The FK constraint `stock_entries_user_id_fkey` fails because no `users.id` matches the Keycloak UUID.

## Affected Features

| Feature | Error | Location |
|---------|-------|----------|
| Material Withdrawal | `stock_entries_user_id_fkey` violation | `withdraw_stock()` in material_repository.rs:296 |
| Fleet Reservation | `reservations_user_id_fkey` violation | `create_reservation()` in fleet_repository.rs |
| Time Entries | 400 validation error | Likely same issue or date format |
| Fleet Calendar | 400 validation error | Date format issue |

## Solution

### Option A: Look up local user.id before insert (Recommended)

Before any insert that references `user_id`:
1. Query `users` table to find `id` where `keycloak_user_id = <jwt_user_id>`
2. Use the local `users.id` for FK inserts

### Option B: Change FK to reference keycloak_user_id

Modify schema to have FK reference `users.keycloak_user_id` instead of `users.id`.
This requires migration and is more invasive.

### Option C: Auto-create user on first request

Ensure `get_or_create_from_auth()` is called before any operation that needs user_id.
This already exists in `UserService` but may not be called in all code paths.

## Implementation Plan

1. Create a helper to resolve Keycloak user ID to local user ID
2. Update all repositories to use local user ID for FK columns
3. Or: Ensure user sync happens on every authenticated request

## Files to Modify

- `src/common/types.rs` - Add method to resolve user ID
- `src/modules/inventory/infrastructure/material_repository.rs` - Fix `withdraw_stock()`
- `src/modules/fleet/infrastructure/fleet_repository.rs` - Fix `create_reservation()`
- `src/modules/sites/infrastructure/*.rs` - Fix time entries if affected

## Testing

After fix:
1. Test material withdrawal
2. Test fleet reservation creation  
3. Test time entry creation
4. Test calendar queries
