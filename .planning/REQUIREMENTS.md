# Requirements: Schreinerei SaaS

**Defined:** 2026-04-28
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## v1 Requirements

### Architecture & Quality

- [x] **ARCH-01**: Hexagonal Architecture mit Domain/Application/Infrastructure Layern
- [x] **ARCH-02**: Module kommunizieren über Domain Events, nicht direkte Aufrufe
- [x] **ARCH-03**: Rust Traits als Ports für Infrastructure-Abstraktion
- [x] **ARCH-04**: Unit Tests können Domain/Application ohne Datenbank testen (Mock-Ports)
- [x] **ARCH-05**: TenantId wird automatisch aus JWT extrahiert (Request Context Pattern)
- [x] **ARCH-06**: State-Based Persistence mit Audit Log (nicht Event Sourcing)

### Authentication & IAM

- [x] **AUTH-01**: User kann sich via Keycloak einloggen
- [x] **AUTH-02**: Multi-Tenant-Trennung ist gewährleistet (TenantId in jeder Query)
- [x] **AUTH-03**: Admin kann neue User einladen
- [x] **AUTH-04**: Rollen: Admin, Mitarbeiter mit unterschiedlichen Berechtigungen
- [x] **AUTH-05**: User kann sein Profil bearbeiten

### Inventar

- [x] **INVT-01**: Admin kann Material-Kategorien anlegen (Platten, Kanten, Lacke, Schrauben, etc.)
- [x] **INVT-02**: Admin kann Materialien mit Bestand anlegen
- [x] **INVT-03**: Mitarbeiter kann Material entnehmen und Bestand reduzieren
- [x] **INVT-04**: System warnt bei "letzte Packung" und benachrichtigt Admin
- [x] **INVT-05**: Admin sieht Benachrichtigungen über niedrige Bestände
- [x] **INVT-06**: Admin kann Bestellanforderungen erstellen
- [x] **INVT-07**: Material kann mit QR-Code getaggt und gescannt werden

### Baustellen

- [x] **SITE-01**: Admin kann Baustelle anlegen (Ort, Kunde, Zeitraum)
- [x] **SITE-02**: Admin kann Mitarbeiter auf Baustelle zuweisen
- [x] **SITE-03**: Mitarbeiter kann Arbeitszeit auf Baustelle buchen
- [x] **SITE-04**: Mitarbeiter kann Zeit in Werkstatt buchen (Vorbereitung, CNC)
- [x] **SITE-05**: Mitarbeiter kann Foto/Notiz zu Baustelle hinzufügen (Activity Feed)
- [x] **SITE-06**: Baustelle hat Timeline mit allen Aktivitäten
- [x] **SITE-07**: Admin kann geschätzte Arbeitstage hinterlegen
- [x] **SITE-08**: Admin sieht Dashboard offener Baustellen

### Fuhrpark & Werkzeuge

- [x] **FLEET-01**: Admin kann Fahrzeuge anlegen
- [x] **FLEET-02**: Admin kann Werkzeuge/Geräte anlegen
- [x] **FLEET-03**: Mitarbeiter kann Werkzeug für Zeitraum reservieren
- [x] **FLEET-04**: Reservierung wird mit Baustelle verknüpft (automatisch Zeitraum übernehmen)
- [x] **FLEET-05**: Mitarbeiter sieht Belegung von Fahrzeugen/Werkzeugen im Kalender
- [x] **FLEET-06**: Mitarbeiter kann Fahrzeug reservieren
- [x] **FLEET-07**: QR-Code an Werkzeug zeigt aktuellen Status/Reservierung

### PWA & Mobile

- [x] **PWA-01**: App ist als PWA installierbar
- [x] **PWA-02**: Offline-Modus für Grundfunktionen (Daten werden synchronisiert wenn wieder online)
- [x] **PWA-03**: QR-Code Scanner funktioniert über Kamera
- [x] **PWA-04**: Responsive Design für Tablet und Smartphone

## v2 Requirements

### Erweiterte Features

- **EXT-01**: Wartungsintervalle für Großmaschinen mit automatischen Erinnerungen
- **EXT-02**: Wartungsintervalle für Fahrzeuge
- **EXT-03**: Hackschnitzel-Füllstand überwachen
- **EXT-04**: Mitarbeiterplanung mit Urlaubskalender
- **EXT-05**: Statistiken über Verbrauch und Arbeitszeit je Baustelle
- **EXT-06**: Kosten-Tracking pro Baustelle
- **EXT-07**: Rechnung aus Baustelle generieren (PDF)
- **EXT-08**: Budget-Tracking mit Status-Leiste
- **EXT-09**: Packlisten-Generator bei "Montage"-Status
- **EXT-10**: Voice-to-Text für schnelle Dokumentation

### RFID Integration

- **RFID-01**: RFID-Chips an Werkzeugen
- **RFID-02**: Sensoren in Fahrzeugen
- **RFID-03**: Automatisches Tracking welches Werkzeug in welchem Fahrzeug

### CAD/CNC Integration

- **CAD-01**: DXF-Dateien hochladen und anzeigen
- **CAD-02**: Bsolid-Integration für CNC-Programme
- **CAD-03**: Stückliste aus CAD extrahieren
- **CAD-04**: Nesting für Plattenzuschnitt

### Public Website

- **WEB-01**: Landing Page mit Features und Pricing
- **WEB-02**: Self-Service Registrierung
- **WEB-03**: Stripe-Integration für Payments

## Out of Scope

| Feature | Reason |
|---------|--------|
| RFID Hardware | Hardware nicht vorhanden, später ergänzen |
| CAD/CNC Integration | Nicht kritisch für Pilot, später implementieren |
| Native Mobile App | PWA first, später React Native/Capacitor möglich |
| Öffentliche Website | Fokus auf App, Website später |
| Datenmigration | Pilot startet bei Null |
| Mehrsprachigkeit | Deutsch first, später i18n |
| AI-Suche | Nice-to-have, nicht in V1 |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| ARCH-01 | Phase 1 | Complete |
| ARCH-02 | Phase 2+ | Complete |
| ARCH-03 | Phase 1 | Complete |
| ARCH-04 | Phase 1 | Complete |
| ARCH-05 | Phase 1 | Complete |
| ARCH-06 | Phase 1 | Complete |
| AUTH-01 | Phase 1 | Complete |
| AUTH-02 | Phase 1 | Complete |
| AUTH-03 | Phase 1 | Complete |
| AUTH-04 | Phase 1 | Complete |
| AUTH-05 | Phase 1 | Complete |
| INVT-01 | Phase 2 | Complete |
| INVT-02 | Phase 2 | Complete |
| INVT-03 | Phase 2 | Complete |
| INVT-04 | Phase 2 | Complete |
| INVT-05 | Phase 2 | Complete |
| INVT-06 | Phase 2 | Complete |
| INVT-07 | Phase 2 | Complete |
| SITE-01 | Phase 3 | Complete |
| SITE-02 | Phase 3 | Complete |
| SITE-03 | Phase 3 | Complete |
| SITE-04 | Phase 3 | Complete |
| SITE-05 | Phase 3 | Complete |
| SITE-06 | Phase 3 | Complete |
| SITE-07 | Phase 3 | Complete |
| SITE-08 | Phase 3 | Complete |
| FLEET-01 | Phase 4 | Complete |
| FLEET-02 | Phase 4 | Complete |
| FLEET-03 | Phase 4 | Complete |
| FLEET-04 | Phase 4 | Complete |
| FLEET-05 | Phase 4 | Complete |
| FLEET-06 | Phase 4 | Complete |
| FLEET-07 | Phase 4 | Complete |
| PWA-01 | Phase 5 | Complete |
| PWA-02 | Phase 5 | Complete |
| PWA-03 | Phase 5 | Complete |
| PWA-04 | Phase 5 | Complete |

**Coverage:**
- v1 requirements: 37 total
- Complete: 37
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-28*
*Last updated: 2026-04-28 after adding architecture requirements*
