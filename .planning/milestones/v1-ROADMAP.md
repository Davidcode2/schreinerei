# Milestone v1: V1 Release

**Status:** ✅ SHIPPED 2026-04-29
**Phases:** 1-5
**Total Plans:** 12

## Overview

Complete SaaS application for Schreinereien (carpentry workshops) with multi-tenant authentication, inventory management, construction site tracking, fleet management, and mobile-first PWA with offline support.

## Phases

### Phase 1: Auth & IAM Foundation

**Goal**: Multi-Tenant Authentication mit Keycloak, User-Management, Basis-API, Hexagonal Architecture Foundation

**Requirements:** AUTH-01, AUTH-02, AUTH-03, AUTH-04, AUTH-05, ARCH-01, ARCH-03, ARCH-04, ARCH-05, ARCH-06

**Plans:** 2 plans

Plans:

- [x] 01-01: Backend Foundation + Auth Middleware
- [x] 01-02: IAM Module + User Management API

**Details:**
- Rust backend with Axum 0.8, SQLx 0.8
- JWT validation with JWKS caching
- Multi-tenant isolation via TenantId in all tables
- Keycloak integration via PKCE flow

**Completed:** 2026-04-28

---

### Phase 2: Inventar Management

**Goal**: Materialverwaltung mit Bestands-Tracking, Warnungen, QR-Codes und Domain Events

**Requirements:** INVT-01, INVT-02, INVT-03, INVT-04, INVT-05, INVT-06, INVT-07, ARCH-02

**Plans:** 2 plans

Plans:

- [x] 02-01: Inventory Module Foundation + Stock Management
- [x] 02-02: Domain Events + Low Stock Warnings + QR Codes

**Details:**
- Category and Material aggregates with stock tracking
- QR code generation and lookup
- Domain events for inter-module communication
- Low stock detection with notifications

**Completed:** 2026-04-28

---

### Phase 3: Baustellen Management

**Goal**: Baustellen anlegen, Mitarbeiter zuweisen, Zeit buchen, Activity Feed

**Requirements:** SITE-01, SITE-02, SITE-03, SITE-04, SITE-05, SITE-06, SITE-07, SITE-08

**Plans:** 2 plans

Plans:

- [x] 03-01: Sites Module Foundation + Time Tracking
- [x] 03-02: Activity Feed + Dashboard

**Details:**
- Site aggregate with status state machine (Planned → Active → Completed → Archived)
- TimeEntry with work types (SiteWork, Workshop, CNC, Delivery)
- Activity Feed with photos and notes
- Dashboard showing open sites

**Completed:** 2026-04-28

---

### Phase 4: Fuhrpark & Werkzeuge

**Goal**: Fahrzeug- und Werkzeugverwaltung mit Reservierung und Kalender

**Requirements:** FLEET-01, FLEET-02, FLEET-03, FLEET-04, FLEET-05, FLEET-06, FLEET-07

**Plans:** 2 plans

Plans:

- [x] 04-01: Fleet Module Foundation + Vehicles & Tools CRUD
- [x] 04-02: Reservations + Calendar + QR Status

**Details:**
- Unified reservations table for vehicles and tools
- ResourceType enum for polymorphic resource_id
- Availability check with overlap detection
- Calendar view of reservations
- QR code shows current status + upcoming reservations

**Completed:** 2026-04-28

---

### Phase 5: PWA & Mobile

**Goal**: PWA mit Offline-Support, QR-Scanner, Responsive Design

**Requirements:** PWA-01, PWA-02, PWA-03, PWA-04

**Plans:** 4 plans

Plans:

- [x] 05-01: Frontend Foundation + PWA Setup
- [x] 05-02: Auth + API Client (OAuth2 PKCE)
- [x] 05-03: Core Feature Components
- [x] 05-04: Offline + QR Scanner

**Details:**
- Vite + React 18 + TypeScript frontend
- OAuth2 PKCE authentication with Keycloak
- React Query + Zustand for state management
- IndexedDB (Dexie.js) for offline data storage
- html5-qrcode for camera QR scanning
- Mobile-first responsive design with Tailwind + shadcn/ui

**Completed:** 2026-04-29

---

## Milestone Summary

**Key Decisions:**
- Rust Workspace mit Modulen aufsetzen
- Postgres-Schema mit TenantId in allen Tabellen
- Keycloak integration via JWT validation middleware
- DDD layering: domain, application, infrastructure, api
- SQLx runtime queries (no compile-time macros)
- Domain events for inter-module communication
- QR codes with tenant prefix
- Vite + React 18 for frontend
- OAuth2 PKCE for SPA auth
- Dexie.js for IndexedDB abstraction

**Issues Resolved:**
- Migration syntax error fixed (partial unique index)
- Frontend API endpoint mismatches corrected
- All 8 E2E flows verified working

**Technical Debt Incurred:**
- No rate limiting (infrastructure level)
- Event polling vs pub/sub (acceptable for V1)
- No conflict resolution for offline edits
- Offline queue retry limit: 3 attempts

**Deliverables:**
- ✓ Multi-Tenant SaaS App
- ✓ Inventar mit Warnungen
- ✓ Baustellen mit Zeit-Tracking
- ✓ Fuhrpark mit Reservierung
- ✓ Mobile PWA mit Offline

---

_For current project status, see .planning/ROADMAP.md_
