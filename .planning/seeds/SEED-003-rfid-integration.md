# SEED-003: RFID Hardware Integration

## WHY

RFID tracking would automate tool location tracking, eliminating manual check-in/out. Currently out of scope because hardware isn't available.

## Surface When

- Hardware is purchased (RFID chips, vehicle sensors)
- Customer requests automated tracking
- Manual tool tracking becomes too cumbersome
- Planning V2 features

## Details

### Requirements (from original spec)

- **RFID-01**: RFID-Chips an Werkzeugen
- **RFID-02**: Sensoren in Fahrzeugen
- **RFID-03**: Automatisches Tracking welches Werkzeug in welchem Fahrzeug

### The Problem It Solves

Current manual flow:
1. MA lädt Werkzeug morgens in Fahrzeug
2. MA nutzt es über den Tag an der Baustelle
3. MA bringt es nach Feierabend zurück in Werkstatt
4. ODER: MA belässt Werkzeug im Fahrzeug

Problem: Manual entry for every tool is too much bureaucracy.

### RFID Solution

1. RFID chip attached to each tool
2. RFID sensor in each vehicle
3. System automatically detects:
   - Which tools are in which vehicle
   - When tools enter/leave workshop
   - When tools return with different vehicle

### Edge Cases

- Tool in wrong vehicle (belongs to Shop A, detected in Shop B's vehicle)
- Tool missing for X days (alert)
- Sensor offline (fallback to manual mode)
- Multiple tools entering simultaneously

### Technical Considerations

**Hardware Options:**
- UHF RFID readers (range 3-10m)
- Passive RFID tags (cheap, no battery)
- Active RFID tags (more expensive, longer range)

**Integration Points:**
- MQTT or HTTP webhook from RFID readers
- Real-time updates to frontend
- Event sourcing might make sense here (audit trail of all movements)

**Infrastructure:**
```
modules/fleet/
├── infrastructure/
│   ├── rfid_reader_adapter.rs  # Handles sensor data
│   └── location_tracker.rs     # Updates tool locations
```

### Privacy/GDPR

- Tracking tools, not people
- But tool location + vehicle assignment = indirect person tracking
- Need clear consent and data retention policy

## Breadcrumbs

- REQUIREMENTS.md Out of Scope table
- PROJECT.md Context section mentions RFID
- FLEET requirements include QR-code tracking (manual alternative)

## Related Seeds

- SEED-001: Module Extraction Pattern
- SEED-002: CAD/CNC Integration

---
*Planted: 2026-04-28*
