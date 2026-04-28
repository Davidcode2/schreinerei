# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Requirements

### Validated

- ✓ Multi-Tenant Auth via Keycloak — Phase 1
- ✓ User-Management mit Rollen (Admin, Mitarbeiter) — Phase 1

### Active

- [ ] Material-Inventar (Platten, Kanten, Lacke, Schrauben, etc.)
- [ ] "Letzte Packung" Warnung mit Benachrichtigung an Chef
- [ ] Baustellen anlegen und verwalten
- [ ] Mitarbeiter auf Baustellen zuweisen
- [ ] Arbeitszeit auf Baustellen buchen
- [ ] Baustellen-Fotos und Notizen (Activity Feed)
- [ ] Fahrzeug-Inventar und Reservierung
- [ ] Werkzeug-Reservierung mit Zeitraum und Baustellen-Verknüpfung
- [ ] Mobile-first PWA mit Offline-Support
- [ ] QR-Code Scanner für Werkzeug/Material

### Out of Scope

- RFID Hardware-Integration — Hardware nicht vorhanden, später ergänzen
- CAD/CNC Integration (DXF, Bsolid) — Nicht kritisch für Pilot, später implementieren
- Native Mobile App — PWA first, später React Native/Capacitor möglich
- Öffentliche Website/Landing Page — Fokus auf App, Website später
- Datenmigration — Pilot startet bei Null

## Context

**Pilot-Kunde:** Eine Schreinerei wird als erster Kunde die Software testen. Ziel: Frühes Feedback für iterative Verbesserung.

**Problem-Szenarien aus der Praxis:**
1. **"Haben wir alles?"** — Um 7:00 Uhr am Bulli fehlt das Spezial-Werkzeug. 1 Stunde Fahrt verloren.
2. **WhatsApp-Chaos** — Infos zwischen Urlaubsfotos, Kollege findet die Maße nicht.
3. **"Wo gehört das hin?"** — 150 weiße Bauteile, welches ist für welchen Schrank?
4. **Reklamationen** — Kunde behauptet, Kratzer waren schon vorher da. Keine Dokumentation.

**Lösungen:**
1. Packlisten-Generator — Automatische Checkliste bei "Montage"-Status
2. Projekt-Timeline — Fotos und Notizen direkt im Projekt, nicht in WhatsApp
3. Visual Part-Finder — Label scannen, 3D-Explosionszeichnung zeigt Position
4. Voice-to-Documentation — 5 Sekunden Sprachnotiz, KI wandelt in Text um

## Constraints

- **Timeline**: 3-6 Monate für V1
- **Tech Stack**: Rust Backend (SQLx, REST), Vite+React Frontend (PWA), Keycloak Auth, PostgreSQL, Kubernetes
- **Architecture**: Hexagonal Architecture (Ports & Adapters) mit Modular Monolith und DDD Bounded Contexts
- **Multi-Tenancy**: Von Anfang an implementiert — TenantId in jeder Query
- **Deployment**: Bestehender Kubernetes-Cluster
- **Offline**: Wichtig — Service Worker, IndexedDB für Baustellen ohne Empfang
- **Longevity**: 10+ Jahre Wartbarkeit ohne komplette Neuentwicklung

## Architecture

### Hexagonal Architecture (Ports & Adapters)

Jedes Modul hat drei Schichten:

- **Domain**: Reine Business-Logik, Entities, Value Objects, Domain Events. Keine Dependencies auf Frameworks oder Externes.
- **Application**: Use Cases / Services. Orchestriert Domain-Objekte. Definiert Ports (Traits) für Infrastructure.
- **Infrastructure**: Adapter für Datenbank, External APIs, Message Bus. Implementiert Ports aus Application.

### Modular Monolith mit DDD Bounded Contexts

Module sind vertikale Slices mit eigener Hexagon-Struktur:

```
src/
├── common/           # Shared Kernel (TenantId, Money, Error Types)
├── auth/             # Keycloak integration & JWT verification
└── modules/
    ├── iam/          # Identity & Access Management
    ├── inventory/    # Material management
    ├── sites/        # Construction site management
    └── fleet/        # Vehicle & tool management
```

Jedes Modul:
```
modules/inventory/
├── domain/           # Material, StockLevel, Events
├── application/      # OrderMaterialUseCase, Ports (traits)
└── infrastructure/   # PostgresRepo, EmailClient (implementations)
```

### Inter-Module Communication

Module kommunizieren über **Domain Events**, nicht direkte Aufrufe:

- ProjectManagement publiziert `ProjectCompleted` Event
- Billing hört auf Event und erstellt Draft Invoice
- Vorteil: Module können später in separate Microservices extrahiert werden

### Multi-Tenancy

TenantId wird nie manuell übergeben. Stattdessen:

1. API Layer extrahiert TenantId aus Keycloak JWT
2. TenantId wird in Request Context / Thread Local gespeichert
3. Repositories lesen TenantId automatisch aus Context
4. Verhindert Data Leaks zwischen Tenants

### Rust Traits als Ports

```rust
// domain/port.rs
pub trait MaterialRepository {
    async fn find_by_id(&self, id: MaterialId) -> Result<Option<Material>>;
    async fn save(&self, material: &Material) -> Result<()>;
}

// infrastructure/postgres.rs
pub struct PostgresMaterialRepository { /* ... */ }
impl MaterialRepository for PostgresMaterialRepository { /* ... */ }
```

Vorteil: Unit Tests mit Mock-Implementierungen ohne Datenbank.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust Backend | Sicherheit, Performance, 10+ Jahre Wartbarkeit | ✓ Axum 0.8, SQLx 0.8 working |
| Vite + React PWA | Einfach, schnell, Offline-Support möglich, später native App mit Capacitor | — Pending |
| Keycloak (bestehend) | Bereits im Cluster, SSO, Multi-Tenant-ready | ✓ JWT validation with JWKS caching working |
| Modularer Monolith | Klare Trennung, später extrahierbar, keine Microservice-Komplexität | ✓ DDD structure established in modules/iam |
| Multi-Tenant ab Tag 1 | Architektur mitdenken, nicht nachrüsten | ✓ TenantId in all tables, enforced in repository layer |
| Kein RFID in V1 | Hardware nicht vorhanden, manuelle Erfassung first | — Pending |
| SQLx runtime queries | No database during build | ✓ Working, type safety reduced but functional |
| DDD layering | Clean architecture, testability | ✓ domain/application/infrastructure/api structure |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-28 after Phase 1*
