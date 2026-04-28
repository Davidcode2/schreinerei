---
phase: 02-inventar-management
plan: 02
subsystem: inventory
tags: [rust, axum, ddd, multi-tenant, events, qr-code, order-management]
dependency_graph:
  requires:
    - "02-01-PLAN.md (inventory foundation)"
  provides:
    - "Domain events for inter-module communication"
    - "QR code generation and SVG output"
    - "Order request workflow (create, approve, fulfill)"
    - "Low stock notifications via events"
  affects:
    - "Phase 3 (Sites) - can react to inventory events"
    - "Phase 4 (Fleet) - can use event infrastructure"
tech-stack:
  added:
    - "Domain event infrastructure with EventBus"
    - "QR code generation with qrcode crate"
    - "Order request workflow with status transitions"
    - "Event publishing on all stock operations"
  patterns:
    - "Event-driven architecture"
    - "Domain events for decoupled modules"
    - "SVG rendering for QR codes"
key-files:
  created:
    - path: "src/common/events.rs"
      purpose: "Domain event infrastructure"
    - path: "src/modules/inventory/domain/events.rs"
      purpose: "Inventory-specific event payloads"
    - path: "src/modules/inventory/domain/order_request.rs"
      purpose: "Order request aggregate"
    - path: "migrations/003_domain_events.sql"
      purpose: "Event store table"
    - path: "migrations/004_order_requests.sql"
      purpose: "Order requests table"
  modified:
    - path: "src/common/types.rs"
      purpose: "Added OrderRequestId type"
    - path: "src/modules/inventory/infrastructure/material_repository.rs"
      purpose: "Added event publishing and order methods"
    - path: "src/modules/inventory/application/inventory_service.rs"
      purpose: "Added event emission and order operations"
    - path: "src/modules/inventory/api/routes.rs"
      purpose: "Added QR and order endpoints"
decisions:
  - "V1: Events stored in database, handlers poll (no pub/sub yet)"
  - "QR codes include tenant prefix for uniqueness"
  - "Order fulfillment updates stock atomically"
  - "All stock operations emit events for audit trail"
metrics:
  duration_minutes: 15
  completed_date: 2026-04-28
  task_count: 6
  file_count: 12
---

# Phase 2 Plan 02: Domain Events, QR Codes, Order Requests Summary

Added event-driven communication, QR code generation, order request workflow, and low stock notifications to the Inventory module.

## What Was Built

### Domain Event Infrastructure
- **DomainEvent** struct with EventType enum
- **EventBus** for publishing events to database
- **EmitsEvents** trait for aggregates
- Event store in database for reliable persistence

### Inventory Domain Events
- **StockLowPayload** - emitted when stock falls below minimum
- **StockWithdrawnPayload** - emitted on every withdrawal
- **MaterialCreatedPayload** - emitted when material is created
- **StockAdjustedPayload** - emitted on manual stock adjustment
- **OrderRequestCreatedPayload** - emitted when order request is created

### Order Request Workflow
- **OrderRequest** aggregate with status transitions
- Status flow: Pending → Approved → Ordered → Fulfilled
- Admin-only approval and fulfillment
- Fulfillment atomically updates stock

### QR Code Generation
- Generate unique QR codes with tenant prefix
- SVG output for printing/scanning
- QR code lookup already implemented in 02-01

## API Endpoints Added

| Method | Path | Description | Role |
|--------|------|-------------|------|
| POST | /api/v1/inventory/materials/{id}/qr | Generate QR code | Admin |
| GET | /api/v1/inventory/materials/{id}/qr/svg | Get QR as SVG | Any |
| POST | /api/v1/inventory/orders | Create order request | Any |
| GET | /api/v1/inventory/orders | List order requests | Admin |
| POST | /api/v1/inventory/orders/{id}/approve | Approve request | Admin |
| POST | /api/v1/inventory/orders/{id}/fulfill | Fulfill request | Admin |

## Threat Model Addressed

| Threat | Mitigation |
|--------|------------|
| T-02-07: Spoofing (QR) | QR includes tenant prefix, unique per material |
| T-02-08: Tampering (Order) | Transaction-based update, status checks |
| T-02-09: Repudiation | All changes tracked with user_id and timestamps |
| T-02-10: Elevation | Admin-only role check for order operations |
| T-02-11: Info Disclosure | Tenant-scoped queries on domain_events |

## Verification

- [x] `cargo build` compiles without errors
- [x] `cargo build --release` produces optimized binary
- [x] All events stored in domain_events table
- [x] Order fulfillment updates stock atomically
- [x] QR code generation produces valid SVG

## Deviations from Plan

None - plan executed exactly as written.

## Architecture Decision (ARCH-02)

"Module kommunizieren über Domain Events, nicht direkte Aufrufe"

Implementation:
- Events stored in append-only table
- Events can be queried by type, tenant, aggregate
- Future: Add message queue for real-time delivery

## Next Steps

Phase 3 (Sites Management) can now:
- React to StockLow events for site-specific ordering
- Use event infrastructure for cross-module communication
- Build on established patterns

## Self-Check: PASSED

- [x] All created files exist on disk
- [x] Commit 2a7dbf2 exists in git history
- [x] Build compiles successfully
- [x] All new API routes properly mounted
