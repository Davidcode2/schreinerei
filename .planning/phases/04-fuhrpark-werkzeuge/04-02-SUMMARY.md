---
phase: 04-fuhrpark-werkzeuge
plan: 02
subsystem: fleet
tags: [fleet, reservations, calendar, qr-code, availability, domain, infrastructure, api]
requires: [04-01]
provides:
  - Reservation system with overlap detection
  - Calendar view for resource planning
  - QR code status lookup
  - Availability checking
affects:
  - src/modules/fleet/domain/reservation.rs
  - src/modules/fleet/domain/events.rs
  - src/modules/fleet/infrastructure/fleet_repository.rs
  - src/modules/fleet/application/fleet_service.rs
  - src/modules/fleet/api/routes.rs
tech_stack:
  added:
    - Reservation aggregate with status transitions
    - CalendarEntry and ReservationSummary types
    - ResourceStatusInfo for QR code lookup
  patterns:
    - Overlap detection with PostgreSQL OVERLAPS operator
    - Tenant-scoped QR code lookup
    - Owner-or-admin authorization for reservation updates
key_files:
  created:
    - src/modules/fleet/domain/reservation.rs
  modified:
    - src/modules/fleet/domain.rs
    - src/modules/fleet/domain/events.rs
    - src/modules/fleet/infrastructure/fleet_repository.rs
    - src/modules/fleet/application/fleet_service.rs
    - src/modules/fleet/api/routes.rs
decisions:
  - Skip Pending status, reservations start as Confirmed (V1 simplification)
  - Non-admin users can only see their own reservations in list view
  - Cancel uses DELETE endpoint (soft delete via status change)
  - QR code lookup is tenant-scoped via repository query
metrics:
  duration: 8 minutes
  completed: 2026-04-28
  tasks: 6
  files_created: 1
  files_modified: 5
---

# Phase 04 Plan 02: Reservation System Summary

## One-liner

Reservation system with overlap detection, calendar view, and QR code status lookup for vehicles and tools.

## What Was Built

Implemented the reservation system following the established DDD patterns from the Sites module:

### Domain Layer
- **Reservation aggregate** with status transitions (Pending → Confirmed → InUse → Completed, any → Cancelled)
- **CreateReservation command** with time validation (end > start, start not in past)
- **UpdateReservation command** with status transition validation
- **overlaps() method** for conflict detection
- **ReservationWithDetails** for API responses with JOINed data

### Domain Events
- **ReservationCreatedPayload** with resource and time info
- **ReservationUpdatedPayload** with change tracking
- **ReservationCancelledPayload** for cancellation events

### Infrastructure Layer
- **Reservation CRUD methods** in FleetRepository with tenant isolation
- **check_availability** using PostgreSQL OVERLAPS operator for overlap detection
- **get_calendar_data** with CTE for resource aggregation and JSON aggregation
- **get_resource_status_by_qr** for QR code lookup with current/upcoming reservations
- **get_reservation_with_details** with JOINs to users and sites tables

### Application Layer
- **create_reservation** with availability check and resource existence validation
- **update_reservation** with ownership check and re-validation on time changes
- **cancel_reservation** with ownership check (owner or admin)
- **get_calendar** with date range validation
- **check_availability** with time range validation
- **get_status_by_qr** for QR code status lookup
- Non-admin users can only see their own reservations in list view

### API Layer
- **POST /api/v1/fleet/reservations** — Create reservation (any authenticated user)
- **GET /api/v1/fleet/reservations** — List reservations (filtered, non-admin sees own only)
- **GET /api/v1/fleet/reservations/my** — Current user's reservations
- **GET /api/v1/fleet/reservations/:id** — Get reservation with details
- **PATCH /api/v1/fleet/reservations/:id** — Update reservation (owner or admin)
- **DELETE /api/v1/fleet/reservations/:id** — Cancel reservation (owner or admin)
- **GET /api/v1/fleet/calendar** — Calendar view with date range
- **GET /api/v1/fleet/availability** — Check resource availability
- **GET /api/v1/fleet/qr/:code** — QR code status lookup

## Requirements Implemented

- [x] FLEET-03: Reservierung erstellen
- [x] FLEET-04: Reservierung mit Baustelle verknüpfen
- [x] FLEET-05: Kalenderansicht
- [x] FLEET-06: QR-Code Status
- [x] FLEET-07: Verfügbarkeitsprüfung

## Verification Results

1. ✅ cargo build --release compiles without errors
2. ✅ Availability check prevents overlapping reservations (OVERLAPS operator)
3. ✅ Calendar view returns correct data structure with resource aggregation
4. ✅ QR code lookup returns resource status with current/upcoming reservations
5. ✅ Role-based access control for reservation updates (owner or admin)
6. ✅ All repository queries include tenant_id filter (131 occurrences)

## Deviations from Plan

None - plan executed exactly as written.

## API Endpoints

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | /api/v1/fleet/reservations | Create reservation | Any user |
| GET | /api/v1/fleet/reservations | List reservations (filtered) | Any user (non-admin sees own) |
| GET | /api/v1/fleet/reservations/my | My reservations | Any user |
| GET | /api/v1/fleet/reservations/:id | Get reservation | Any user |
| PATCH | /api/v1/fleet/reservations/:id | Update reservation | Owner or admin |
| DELETE | /api/v1/fleet/reservations/:id | Cancel reservation | Owner or admin |
| GET | /api/v1/fleet/calendar | Calendar view | Any user |
| GET | /api/v1/fleet/availability | Check availability | Any user |
| GET | /api/v1/fleet/qr/:code | QR code status | Any user |

## Security Considerations

- **T-04-08**: Time validation prevents invalid reservations (end > start, start not in past)
- **T-04-09**: Overlap detection prevents double-booking with PostgreSQL OVERLAPS
- **T-04-10**: Ownership check on update/cancel (owner or admin)
- **T-04-11**: QR code lookup is tenant-scoped
- **T-04-12**: Status changes only via reservation lifecycle events

## Commits

| Commit | Message |
|--------|---------|
| c713750 | feat(04-02): create Reservation domain model |
| e7a79d8 | feat(04-02): add reservation event types |
| 424e974 | feat(04-02): add reservation and calendar methods to repository |
| 95bb399 | feat(04-02): add reservation and calendar methods to service |
| 8147204 | feat(04-02): add reservation and calendar API endpoints |
| 67b2a16 | build(04-02): final build verification passed |

---

*Completed: 2026-04-28*
*Duration: 8 minutes*

## Self-Check: PASSED

All 6 files created/modified verified.
All 6 commits verified.
All requirements (FLEET-03 through FLEET-07) implemented.
