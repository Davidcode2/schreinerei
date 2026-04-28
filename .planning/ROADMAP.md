# Roadmap: Schreinerei SaaS

**Project:** Schreinerei SaaS
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Timeline:** 3-6 Monate

---

## Phase 1: Auth & IAM Foundation ✓

**Goal:** Multi-Tenant Authentication mit Keycloak, User-Management, Basis-API, Hexagonal Architecture Foundation

**Requirements:** AUTH-01, AUTH-02, AUTH-03, AUTH-04, AUTH-05, ARCH-01, ARCH-03, ARCH-04, ARCH-05, ARCH-06

**Plans:** 2/2 plans complete

**Duration Estimate:** 2-3 Wochen

**Completed:** 2026-04-28

### Plans

- [x] 01-01-PLAN.md — Backend Foundation + Auth Middleware (AUTH-01, AUTH-02)
- [x] 01-02-PLAN.md — IAM Module + User Management API (AUTH-03, AUTH-04, AUTH-05)

### Success Criteria

1. User kann sich via Keycloak einloggen und sieht sein Tenant-spezifisches Dashboard
2. Admin kann neue User einladen und Rollen zuweisen
3. Multi-Tenant-Trennung ist verifiziert (Tenant A kann Tenant B Daten nicht sehen)
4. Basis-API-Struktur (Rust, SQLx, REST) ist aufgebaut
5. Deployment auf K8s funktioniert

### Key Decisions

- Rust Workspace mit Modulen aufsetzen
- Postgres-Schema mit TenantId in allen Tabellen
- Keycloak integration via JWT validation middleware

---

## Phase 2: Inventar Management ✓

**Goal:** Materialverwaltung mit Bestands-Tracking, Warnungen, QR-Codes und Domain Events

**Requirements:** INVT-01, INVT-02, INVT-03, INVT-04, INVT-05, INVT-06, INVT-07, ARCH-02

**Plans:** 2/2 plans complete

**Duration Estimate:** 3-4 Wochen

**Completed:** 2026-04-28

### Plans

- [x] 02-01-PLAN.md — Inventory Domain Model + Categories + Materials + Stock Management (INVT-01, INVT-02, INVT-03)
- [x] 02-02-PLAN.md — Domain Events + Low Stock Warnings + Order Requests + QR Codes (INVT-04, INVT-05, INVT-06, INVT-07, ARCH-02)

### Success Criteria

1. Admin kann Material-Kategorien und Materialien anlegen ✓
2. Mitarbeiter kann Material entnehmen und Bestand wird aktualisiert ✓
3. "Letzte Packung" Warnung erscheint und benachrichtigt Admin ✓
4. QR-Code für Material kann generiert und gescannt werden ✓
5. Frontend zeigt Inventar-Übersicht ✓

### Key Decisions

- Domain events for inter-module communication
- QR codes with tenant prefix
- Event store in database for V1

### Dependencies

- Phase 1 (Auth, Tenant, API Structure)

---

## Phase 3: Baustellen Management ✓

**Goal:** Baustellen anlegen, Mitarbeiter zuweisen, Zeit buchen, Activity Feed

**Requirements:** SITE-01, SITE-02, SITE-03, SITE-04, SITE-05, SITE-06, SITE-07, SITE-08

**Plans:** 2/2 plans complete

**Duration Estimate:** 3-4 Wochen

**Completed:** 2026-04-28

### Plans

- [x] 03-01-PLAN.md — Sites Module Foundation + Time Tracking (SITE-01, SITE-02, SITE-03, SITE-04)
- [x] 03-02-PLAN.md — Activity Feed + Dashboard (SITE-05, SITE-06, SITE-07, SITE-08)

### Success Criteria

1. Admin kann Baustelle mit Ort, Kunde, Zeitraum anlegen ✓
2. Admin kann Mitarbeiter zuweisen ✓
3. Mitarbeiter kann Arbeitszeit auf Baustelle/Werkstatt buchen ✓
4. Activity Feed zeigt Fotos und Notizen ✓
5. Dashboard zeigt offene Baustellen ✓

### Key Decisions

- Site status state machine: Planned → Active → Completed → Archived
- Nullable site_id on TimeEntry for workshop work
- Activity types: Photo (requires URL), Note (requires content), StatusChange (system-only)

### Dependencies

- Phase 1 (Auth, User-Management)
- Phase 2 (für Material-Buchung auf Baustelle)

---

## Phase 4: Fuhrpark & Werkzeuge

**Goal:** Fahrzeug- und Werkzeugverwaltung mit Reservierung und Kalender

**Requirements:** FLEET-01, FLEET-02, FLEET-03, FLEET-04, FLEET-05, FLEET-06, FLEET-07

**Duration Estimate:** 2-3 Wochen

### Success Criteria

1. Admin kann Fahrzeuge und Werkzeuge anlegen
2. Mitarbeiter kann Werkzeug/Fahrzeug reservieren
3. Reservierung wird mit Baustelle verknüpft
4. Kalender zeigt Belegung
5. QR-Code zeigt aktuellen Status

### Dependencies

- Phase 1 (Auth, User)
- Phase 3 (Baustellen-Verknüpfung)

---

## Phase 5: PWA & Mobile

**Goal:** PWA mit Offline-Support, QR-Scanner, Responsive Design

**Requirements:** PWA-01, PWA-02, PWA-03, PWA-04

**Duration Estimate:** 2 Wochen

### Success Criteria

1. App ist als PWA installierbar
2. Offline-Modus funktioniert (Daten werden synchronisiert)
3. QR-Code Scanner via Kamera
4. UI funktioniert auf Tablet und Smartphone

### Dependencies

- Phase 1-4 (alle Features müssen funktionieren)

---

## Summary

| Phase | Goal | Duration | Requirements |
|-------|------|----------|--------------|
| 1 | Auth & IAM Foundation | 2-3 Wochen | 5 |
| 2 | Inventar Management | 3-4 Wochen | 7 |
| 3 | Baustellen Management | 3-4 Wochen | 8 |
| 4 | Fuhrpark & Werkzeuge | 2-3 Wochen | 7 |
| 5 | PWA & Mobile | 2 Wochen | 4 |

**Total:** 12-16 Wochen (3-4 Monate)

---

## Milestone: V1 Release

Nach Phase 5 ist die V1 bereit für den Pilotkunden.

**Deliverables:**
- Multi-Tenant SaaS App
- Inventar mit Warnungen
- Baustellen mit Zeit-Tracking
- Fuhrpark mit Reservierung
- Mobile PWA mit Offline

**Next:** V2 Features basierend auf Pilot-Feedback
