---
phase: 19-delete-operations
plan: 01
subsystem: backend
tags: [database, soft-delete, api, delete-routes]
requires: [materials-api, sites-api, fleet-api]
provides: [soft-delete-columns, delete-material-endpoint, delete-site-endpoint]
affects: [inventory-module, sites-module, fleet-module]
tech-stack:
  added: [soft-delete-pattern, conflict-error-409]
  patterns: [soft-delete, dependency-check]
key-files:
  created:
    - migrations/010_soft_delete_columns.sql
  modified:
    - src/common/error.rs
    - src/modules/inventory/api/routes.rs
    - src/modules/inventory/application/inventory_service.rs
    - src/modules/inventory/infrastructure/material_repository.rs
    - src/modules/sites/api/routes.rs
    - src/modules/sites/application/site_service.rs
    - src/modules/sites/infrastructure/site_repository.rs
    - src/modules/fleet/infrastructure/fleet_repository.rs
decisions:
  - Use soft delete with deleted_at timestamp for offline sync compatibility
  - Return HTTP 409 Conflict when delete blocked by dependencies
  - Include count of blocking items in conflict error message
metrics:
  duration: ~15 minutes
  completed: 2026-04-30
---

# Phase 19 Plan 01: Soft Delete Migration + DELETE Routes Summary

## One-liner
Soft delete infrastructure with DELETE routes for materials, sites, vehicles, and tools, returning 409 Conflict when dependencies block deletion.

## What Changed

### Backend Changes

1. **Migration 010_soft_delete_columns.sql**
   - Added `deleted_at TIMESTAMPTZ DEFAULT NULL` to materials, sites, vehicles, tools
   - Created partial indexes for efficient non-deleted item queries

2. **AppError Enum (src/common/error.rs)**
   - Added `Conflict(String)` variant for 409 responses
   - Maps to HTTP 409 Conflict status code

3. **Inventory Module**
   - DELETE `/api/v1/inventory/materials/{id}` - soft delete materials
   - Checks for pending order requests before delete
   - Updated list/find queries to exclude soft-deleted items

4. **Sites Module**
   - DELETE `/api/v1/sites/{id}` - soft delete sites
   - Checks for active reservations before delete
   - Updated list/find queries to exclude soft-deleted items

5. **Fleet Module**
   - Updated delete_vehicle and delete_tool to use soft delete
   - Updated all list/find queries to exclude soft-deleted items
   - Updated calendar query to exclude soft-deleted resources

## Files Changed

| File | Changes |
|------|---------|
| migrations/010_soft_delete_columns.sql | New migration with deleted_at columns and partial indexes |
| src/common/error.rs | Added Conflict error variant |
| src/modules/inventory/api/routes.rs | Added delete_material handler |
| src/modules/inventory/application/inventory_service.rs | Added delete_material service method |
| src/modules/inventory/infrastructure/material_repository.rs | Added delete_material, count_pending_order_requests, updated queries |
| src/modules/sites/api/routes.rs | Added delete_site handler |
| src/modules/sites/application/site_service.rs | Added delete_site service method |
| src/modules/sites/infrastructure/site_repository.rs | Added delete_site, count_active_reservations, updated queries |
| src/modules/fleet/infrastructure/fleet_repository.rs | Updated to soft delete, updated queries |

## API Endpoints Added

| Method | Endpoint | Behavior |
|--------|----------|----------|
| DELETE | /api/v1/inventory/materials/{id} | 200 on success, 409 if pending orders, 404 if not found |
| DELETE | /api/v1/sites/{id} | 200 on success, 409 if active reservations, 404 if not found |

## Requirements Covered

- **DEL-01**: User can delete a site with confirmation dialog (soft delete) ✓
- **DEL-02**: User can delete a material with confirmation dialog (soft delete) ✓
- **DEL-05**: User sees dependency conflict message when delete is blocked ✓

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

- Migration file exists with deleted_at columns
- Conflict error variant exists and maps to 409
- DELETE routes exist for materials and sites
- All list/find queries exclude soft-deleted items
