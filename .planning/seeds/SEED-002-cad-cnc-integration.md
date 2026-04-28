# SEED-002: CAD/CNC Integration

## WHY

CAD/CNC integration (DXF files, Bsolid programs) is critical for the full vision but out of scope for V1. This seed captures the requirements for when this becomes a priority.

## Surface When

- Planning V2 milestone
- Customer requests CAD integration
- Starting Manufacturing module implementation
- Inventory needs to link to CAD parts

## Details

### Requirements (from original spec)

- **CAD-01**: DXF-Dateien hochladen und anzeigen
- **CAD-02**: Bsolid-Integration für CNC-Programme
- **CAD-03**: Stückliste aus CAD extrahieren
- **CAD-04**: Nesting für Plattenzuschnitt

### Use Cases

1. **Visual Part-Finder**: Scan label on part → show 3D explosion drawing with part highlighted
2. **Material Calculation**: Extract part list from CAD → calculate material needs for project
3. **Nesting**: Optimize plate usage → reduce waste

### Technical Considerations

**DXF Parsing:**
- Libraries: `dxf-rs` (Rust) or parse via Python subprocess
- Store parsed geometry in JSONB for querying
- Keep original DXF file in S3/MinIO

**Bsolid Integration:**
- Bsolid is proprietary CNC software
- Export/import via file formats, not API
- May need to run conversion jobs as async tasks

**Nesting:**
- Complex algorithm, consider existing solutions:
  - libnest (C library with bindings)
  - Deepnest (open source)
  - Commercial APIs

### Module Structure

```
modules/manufacturing/
├── domain/
│   ├── dxf_document.rs
│   ├── part.rs
│   └── nesting.rs
├── application/
│   ├── import_dxf_use_case.rs
│   ├── extract_parts_use_case.rs
│   └── generate_nesting_use_case.rs
└── infrastructure/
    ├── dxf_parser.rs
    ├── bsolid_adapter.rs
    └── nesting_service.rs
```

### Dependencies

- File storage (S3-compatible) for DXF/Bsolid files
- Job queue for long-running parsing/nesting tasks
- Possibly a CAD viewer component in frontend

## Breadcrumbs

- REQUIREMENTS.md section "CAD/CNC Integration" lists requirements
- Out of Scope table documents why deferred
- PROJECT.md Manufacturing module in architecture diagram

## Related Seeds

- SEED-001: Module Extraction Pattern
- SEED-003: RFID Hardware Integration

---
*Planted: 2026-04-28*
