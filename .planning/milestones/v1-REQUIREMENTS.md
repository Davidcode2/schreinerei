# Requirements Archive: V1 Release

**Milestone:** v1
**Archived:** 2026-04-29
**Total Requirements:** 37
**Status:** All Complete ✓

---

## Architecture & Quality

- [x] **ARCH-01**: Hexagonal Architecture mit Domain/Application/Infrastructure Layern — Complete
- [x] **ARCH-02**: Module kommunizieren über Domain Events, nicht direkte Aufrufe — Complete
- [x] **ARCH-03**: Rust Traits als Ports für Infrastructure-Abstraktion — Complete
- [x] **ARCH-04**: Unit Tests können Domain/Application ohne Datenbank testen (Mock-Ports) — Complete
- [x] **ARCH-05**: TenantId wird automatisch aus JWT extrahiert (Request Context Pattern) — Complete
- [x] **ARCH-06**: State-Based Persistence mit Audit Log (nicht Event Sourcing) — Complete

## Authentication & IAM

- [x] **AUTH-01**: User kann sich via Keycloak einloggen — Complete (PKCE flow)
- [x] **AUTH-02**: Multi-Tenant-Trennung ist gewährleistet (TenantId in jeder Query) — Complete
- [x] **AUTH-03**: Admin kann neue User einladen — Complete
- [x] **AUTH-04**: Rollen: Admin, Mitarbeiter mit unterschiedlichen Berechtigungen — Complete
- [x] **AUTH-05**: User kann sein Profil bearbeiten — Complete

## Inventar

- [x] **INVT-01**: Admin kann Material-Kategorien anlegen (Platten, Kanten, Lacke, Schrauben, etc.) — Complete
- [x] **INVT-02**: Admin kann Materialien mit Bestand anlegen — Complete
- [x] **INVT-03**: Mitarbeiter kann Material entnehmen und Bestand reduzieren — Complete
- [x] **INVT-04**: System warnt bei "letzte Packung" und benachrichtigt Admin — Complete
- [x] **INVT-05**: Admin sieht Benachrichtigungen über niedrige Bestände — Complete
- [x] **INVT-06**: Admin kann Bestellanforderungen erstellen — Complete
- [x] **INVT-07**: Material kann mit QR-Code getaggt und gescannt werden — Complete

## Baustellen

- [x] **SITE-01**: Admin kann Baustelle anlegen (Ort, Kunde, Zeitraum) — Complete
- [x] **SITE-02**: Admin kann Mitarbeiter auf Baustelle zuweisen — Complete
- [x] **SITE-03**: Mitarbeiter kann Arbeitszeit auf Baustelle buchen — Complete
- [x] **SITE-04**: Mitarbeiter kann Zeit in Werkstatt buchen (Vorbereitung, CNC) — Complete
- [x] **SITE-05**: Mitarbeiter kann Foto/Notiz zu Baustelle hinzufügen (Activity Feed) — Complete
- [x] **SITE-06**: Baustelle hat Timeline mit allen Aktivitäten — Complete
- [x] **SITE-07**: Admin kann geschätzte Arbeitstage hinterlegen — Complete
- [x] **SITE-08**: Admin sieht Dashboard offener Baustellen — Complete

## Fuhrpark & Werkzeuge

- [x] **FLEET-01**: Admin kann Fahrzeuge anlegen — Complete
- [x] **FLEET-02**: Admin kann Werkzeuge/Geräte anlegen — Complete
- [x] **FLEET-03**: Mitarbeiter kann Werkzeug für Zeitraum reservieren — Complete
- [x] **FLEET-04**: Reservierung wird mit Baustelle verknüpft (automatisch Zeitraum übernehmen) — Complete
- [x] **FLEET-05**: Mitarbeiter sieht Belegung von Fahrzeugen/Werkzeugen im Kalender — Complete
- [x] **FLEET-06**: Mitarbeiter kann Fahrzeug reservieren — Complete
- [x] **FLEET-07**: QR-Code an Werkzeug zeigt aktuellen Status/Reservierung — Complete

## PWA & Mobile

- [x] **PWA-01**: App ist als PWA installierbar — Complete
- [x] **PWA-02**: Offline-Modus für Grundfunktionen (Daten werden synchronisiert wenn wieder online) — Complete
- [x] **PWA-03**: QR-Code Scanner funktioniert über Kamera — Complete
- [x] **PWA-04**: Responsive Design für Tablet und Smartphone — Complete

---

## Traceability Table

| Requirement | Phase | Status | Outcome |
|-------------|-------|--------|---------|
| ARCH-01 | Phase 1 | Complete | DDD layering implemented |
| ARCH-02 | Phase 2+ | Complete | Domain events working |
| ARCH-03 | Phase 1 | Complete | Traits as ports pattern |
| ARCH-04 | Phase 1 | Complete | Mock-friendly architecture |
| ARCH-05 | Phase 1 | Complete | TenantContext from JWT |
| ARCH-06 | Phase 1 | Complete | Audit logs in all tables |
| AUTH-01 | Phase 1 | Complete | PKCE flow with Keycloak |
| AUTH-02 | Phase 1 | Complete | TenantId in all queries |
| AUTH-03 | Phase 1 | Complete | User invite system |
| AUTH-04 | Phase 1 | Complete | Admin/Employee roles |
| AUTH-05 | Phase 1 | Complete | Profile editing |
| INVT-01 | Phase 2 | Complete | Category CRUD |
| INVT-02 | Phase 2 | Complete | Material with stock |
| INVT-03 | Phase 2 | Complete | Stock withdrawal |
| INVT-04 | Phase 2 | Complete | Low stock warnings |
| INVT-05 | Phase 2 | Complete | Notification system |
| INVT-06 | Phase 2 | Complete | Order requests |
| INVT-07 | Phase 2 | Complete | QR generation + scan |
| SITE-01 | Phase 3 | Complete | Site CRUD |
| SITE-02 | Phase 3 | Complete | User assignment |
| SITE-03 | Phase 3 | Complete | Time tracking on site |
| SITE-04 | Phase 3 | Complete | Workshop time entries |
| SITE-05 | Phase 3 | Complete | Activity feed |
| SITE-06 | Phase 3 | Complete | Site timeline |
| SITE-07 | Phase 3 | Complete | Estimated days |
| SITE-08 | Phase 3 | Complete | Dashboard |
| FLEET-01 | Phase 4 | Complete | Vehicle CRUD |
| FLEET-02 | Phase 4 | Complete | Tool CRUD |
| FLEET-03 | Phase 4 | Complete | Tool reservation |
| FLEET-04 | Phase 4 | Complete | Site-linked reservations |
| FLEET-05 | Phase 4 | Complete | Calendar view |
| FLEET-06 | Phase 4 | Complete | Vehicle reservation |
| FLEET-07 | Phase 4 | Complete | QR status display |
| PWA-01 | Phase 5 | Complete | PWA manifest + icons |
| PWA-02 | Phase 5 | Complete | Offline sync |
| PWA-03 | Phase 5 | Complete | QR scanner |
| PWA-04 | Phase 5 | Complete | Responsive design |

---

## Coverage Summary

- **v1 requirements:** 37 total
- **Complete:** 37
- **Unmapped:** 0 ✓
- **Coverage:** 100%

---

*Requirements archived: 2026-04-29*
*Originally defined: 2026-04-28*
