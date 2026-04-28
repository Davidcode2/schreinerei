# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Multi-Tenant Auth via Keycloak
- [ ] User-Management mit Rollen (Admin, Mitarbeiter)
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
- **Architecture**: Modularer Monolith mit DDD Bounded Contexts
- **Multi-Tenancy**: Von Anfang an implementiert — TenantId in jeder Query
- **Deployment**: Bestehender Kubernetes-Cluster
- **Offline**: Wichtig — Service Worker, IndexedDB für Baustellen ohne Empfang

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust Backend | Sicherheit, Performance, 10+ Jahre Wartbarkeit | — Pending |
| Vite + React PWA | Einfach, schnell, Offline-Support möglich, später native App mit Capacitor | — Pending |
| Keycloak (bestehend) | Bereits im Cluster, SSO, Multi-Tenant-ready | — Pending |
| Modularer Monolith | Klare Trennung, später extrahierbar, keine Microservice-Komplexität | — Pending |
| Multi-Tenant ab Tag 1 | Architektur mitdenken, nicht nachrüsten | — Pending |
| Kein RFID in V1 | Hardware nicht vorhanden, manuelle Erfassung first | — Pending |

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
*Last updated: 2026-04-28 after initialization*
