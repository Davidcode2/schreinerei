# Requirements: Schreinerei SaaS

**Defined:** 2026-05-01
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.

## v1.9 Requirements

### Category Settings (CATS)

- [x] **CATS-01**: User can edit category name via inventory settings page
- [x] **CATS-02**: User can delete category (blocked if materials reference it, with clear error)
- [x] **CATS-03**: User can navigate to inventory settings page via settings wheel icon on inventory page

### Material Editing (EDIT)

- [x] **EDIT-01**: User can edit inventory item location via edit icon in details section
- [x] **EDIT-02**: User can edit minimum quantity via modal (same modal as location edit)
- [x] **EDIT-03**: User can set available quantity to an arbitrary number (stock correction)

### Stock-In (STOCK)

- [x] **STOCK-01**: User can record material stock-in ("Material einlagern") with amount and notes
- [x] **STOCK-02**: Stock-in events appear in inventory history with "MaterialAdded" entry type

### Enriched History (HIST)

- [x] **HIST-01**: Inventory history shows color-coded event types (green for stock-in, red for withdrawal, blue for location/quantity changes)
- [x] **HIST-02**: History entries display the user who performed the action
- [x] **HIST-03**: Baustelle name in withdrawal history is clickable and navigates to baustelle detail page

### Overview Display (VIEW)

- [x] **VIEW-01**: Inventory overview shows category name on each material entry

## v2 Requirements

Deferred to future release.

## Product Backlog from 2026-05 Note

This section captures the expanded product vision from the long-form note and translates it into structured backlog requirements. These are not yet committed to a milestone and should be pulled into future milestone plans in coherent slices.

### Architecture and Platform

- [x] **ARCH-01**: The system remains a modular monolith with bounded contexts for IAM, Projects, Inventory, Fleet/Assets, Manufacturing, and Billing. Delivered in v1.12 through the `projects` boundary alias.
- [x] **ARCH-02**: Every repository query and domain mutation remains tenant-scoped via request context, not manual function threading. Delivered in v1.12 at the API extraction boundary.
- **ARCH-03**: State changes are stored in relational current-state tables with audit/history entries for each change.
- [x] **ARCH-04**: The app remains mobile-first for tablet and phone before any dedicated native or machine-terminal clients. Delivered in v1.12 as an explicit engineering checklist grounded in the current runtime baseline.
- **ARCH-05**: File storage supports images and documents in S3-compatible object storage or compatible cluster-local storage.

### Inventory and Procurement

- **INV-10**: The inventory domain supports extensible inventory item types, including materials, edges, paints, screws, drill bits, cutters, large machines, small machines, loan PCs, and fleet-adjacent assets.
- **INV-11**: Inventory categories can define whether expiry-date tracking (`Haltbarkeitsdatum`) is required.
- **INV-12**: Expiry-managed stock is recorded per batch or per dated lot, including quantity for each expiry date.
- **INV-13**: The system alerts users when expiry-managed stock has passed its expiry date.
- **INV-14**: A worker can explicitly mark that the last package of an item was taken.
- **INV-15**: A manager receives a replenishment signal or notification when the last package was taken or minimum stock is breached.
- **INV-16**: Goods receipt / stock-in flows support recording quantities, supplier-facing notes, and batch metadata where required.
- **INV-17**: Order and goods-receipt management can be linked to inventory replenishment workflows.
- **INV-18**: Consumables and reusable cutting assets can enter external-service states such as `beim_schaerfen`.

### Assets, Fleet, and Maintenance

- **AST-10**: Vehicles, mobile tools, and large workshop machines are managed as assets with type-specific metadata.
- **AST-11**: Assets can have maintenance intervals and due reminders.
- **AST-12**: Maintenance reminders remain visible to workers until resolved, without disappearing after a single notice.
- **AST-13**: A user can reserve a tool or vehicle for a date range and see who currently has it.
- **AST-14**: Reservations can be linked directly to a project so the date range and purpose can default from the project context.
- **AST-15**: The weekly fleet booking view defaults to vehicle-first layout with mobile-tappable day cells and bottom-sheet confirmation.
- **AST-16**: Each reservable vehicle has a stable unique display color used for booking visualization.
- **AST-17**: The system can later accept location signals for vehicles and mobile machines without changing the core reservation model.
- **AST-18**: RFID-based tool-in-vehicle detection is a future adapter path and should not be required for the baseline workflow.

### Projects, Baustellen, and Operations

- **PROJ-10**: Projects represent both external Baustellen and internal workshop jobs unless a real domain split becomes necessary later.
- **PROJ-11**: A project stores location, planned start, scheduling context, assigned workers, and operational notes.
- **PROJ-12**: Projects appear on dashboards regardless of `planned` or `active` state unless a view explicitly filters them.
- **PROJ-13**: Every material consumption booking is linked to a project to support billing and analytics.
- **PROJ-14**: Workers can book time to a project for both on-site work and workshop preparation / CNC work.
- **PROJ-15**: Managers can assign workers and vehicles to projects and see the planning context in calendar form.
- **PROJ-16**: Projects support estimated duration, budget, optional quote attachment, and budget-consumption tracking.
- **PROJ-17**: Completed project data can be used to generate invoices and downstream finance artifacts.
- **PROJ-18**: A project feed acts as the canonical context channel for notes, images, documents, field observations, and follow-up work.
- **PROJ-19**: The document action on a project / Baustelle supports taking a photo, uploading one or more photos, uploading one or more documents, and attaching a text note in one entry flow.
- **PROJ-20**: Feed entries show creation timestamp and previews for attached images and documents.
- **PROJ-21**: Project creation should eventually support customer-driven or email-driven intake flows, but only after the core project workflow is stable.

### Workshop Execution and Logistics

- **OPS-10**: When a project enters a montage-ready state, the system can generate a packlist / loading checklist.
- **OPS-11**: Packlists combine manufactured parts, mounting consumables, fittings, and special site-specific materials.
- **OPS-12**: Workers can confirm loading quickly on mobile without heavy manual data entry.
- **OPS-13**: A workshop location map can show where tools or stock areas belong.
- **OPS-14**: The workshop map can later surface status indicators such as stock state or charging state where hardware/data exists.

### Staff Planning and Capacity

- **STAFF-10**: The system stores employee working time models relevant for planning.
- **STAFF-11**: Managers can track holidays / absences and estimate remaining staffing capacity.
- **STAFF-12**: Capacity planning is linked to assigned project durations and project schedules.

### Reporting and Search

- **RPT-10**: Managers can filter past projects by customer, period, type, worker, cost, and duration.
- **RPT-11**: The system provides dashboards for open/completed projects, material consumption, labor time, and related operational KPIs.
- **RPT-12**: The system supports consumption statistics per project and time statistics per project.
- **RPT-13**: AI-assisted search can help users find relevant projects, materials, documents, or context faster.

### Manufacturing, Measurement, and CAD/CNC Glue

- **MFG-10**: Projects can store and organize manufacturing-relevant files such as DXF, BSolid exports, and related documents.
- **MFG-11**: The system can link part lists and nesting-related artifacts to projects and manufacturing workflows.
- **MFG-12**: The system acts as glue between measurement capture, project context, and downstream manufacturing software rather than replacing specialist CAD/CAM tools outright.
- **MEAS-10**: The app supports digital measurement capture with structured notes, images, and exportable records.
- **MEAS-11**: Measurement capture should integrate Leica DISTO Bluetooth input for critical dimensions when feasible.
- **MEAS-12**: The system should accept DXF upload as an input path for room-layout creation.
- **MEAS-13**: The system can link point-cloud or external scan artifacts to the project as reference material.
- **MEAS-14**: Measurement workflows should include plausibility checks such as total-vs-part dimension validation and missing-data warnings.
- **MEAS-15**: The long-term AI target is photo/sketch-assisted measurement extraction and material-list generation, but only on top of a reliable structured capture flow.

### Communication, Voice, and Feedback

- **COM-10**: Workers can capture project observations quickly with photos, notes, and voice-backed documentation.
- **COM-11**: Voice notes can be transcribed into structured project documentation with timestamps and media linkage.
- **COM-12**: The system can highlight claims-relevant field observations for later dispute protection.
- **AI-10**: Voice-enabled interactions should be designed as low-latency, low-token workflows with local-first or compressed transcription where practical.
- **AI-11**: AI workflows should ask clarifying questions when ambiguity would otherwise create costly workshop errors.
- **FB-10**: Users can submit product feedback directly from the app.
- **FB-11**: Voice or LLM-assisted workflows can detect complaints or improvement suggestions and route them to a structured feedback store together with relevant usage context.

### Billing, Compliance, and Localization

- **FIN-10**: Projects can generate invoice-ready data, including PDF invoice generation.
- **FIN-11**: The product roadmap must include German e-invoicing readiness.
- **FIN-12**: DATEV-compatible export or integration is a target capability for finance workflows.
- **LOC-10**: The UI architecture supports multilingual operation.
- **PRIV-10**: GPS-based employee or vehicle tracking requires explicit GDPR review before commitment to implementation.

### Suggested Delivery Order

Recommended near-term sequence based on current shipped state and dependency weight:

1. **PROJ-12, PROJ-18 to PROJ-20** — finish the project feed and dashboard behavior around already shipped Baustelle workflows
2. **INV-11 to INV-16** — expiry-aware stock and replenishment alerts build naturally on current inventory history and stock-in flows
3. **AST-10 to AST-16** — maintenance and stronger asset/fleet modeling extend the current reservation module
4. **PROJ-13 to PROJ-17, RPT-10 to RPT-12** — project-linked material/time analytics, budgeting, and invoice preparation
5. **LOC-10, FB-10, FB-11, AI-10 to AI-11** — cross-cutting UX and feedback foundations
6. **MEAS-* and MFG-* backlog** — measurement and manufacturing glue as a later dedicated program

### Integration Tests

- **INT-01**: Integration tests with real PostgreSQL for inventory module
- **INT-02**: Integration tests for sites module
- **INT-03**: Integration tests for fleet module
- **INT-04**: Multi-tenant isolation tests for all modules

### Self-Service Registration

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email

### Offline Enhancements

- **OFFL-01**: Offline stock-in queue with reconnect sync
- **OFFL-02**: Conflict resolution for concurrent edits

## Out of Scope

| Feature | Reason |
|---------|--------|
| Bulk stock-in / batch import | Single-item actions sufficient for v1.9, batch later |
| Category hierarchy (nested) | Flat categories match carpentry domain |
| Real-time WebSocket history | Polling sufficient for 5-20 users per org |
| Soft-delete for categories | Block-with-error is simpler, FK constraint prevents accidental delete |
| Separate audit table for categories | domain_events already captures all mutations |
| RFID Hardware Integration | Hardware not available, later |
| CAD/CNC Integration | Not critical for pilot, later |
| Barcode scanning | v2+ concern |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CATS-01 | Phase 31 | Complete |
| CATS-02 | Phase 31 | Complete |
| CATS-03 | Phase 31 | Complete |
| EDIT-01 | Phase 31 | Complete |
| EDIT-02 | Phase 31 | Complete |
| EDIT-03 | Phase 31 | Complete |
| STOCK-01 | Phase 31 | Complete |
| STOCK-02 | Phase 32 | Complete |
| HIST-01 | Phase 32 | Complete |
| HIST-02 | Phase 32 | Complete |
| HIST-03 | Phase 32 | Complete |
| VIEW-01 | Phase 31 | Complete |

**Coverage:**
- v1.9 requirements: 12 total
- Mapped to phases: 12/12 ✓
- Unmapped: 0

**Mapping rationale:**
- **Phase 30** (Backend API Foundation): Enabler phase — builds API endpoints and migration that Phase 31-32 depend on. No v1 requirements mapped directly.
- **Phase 31** (Settings, Editing & Stock-In): Frontend surfaces for category management, material editing, stock-in recording, and overview display. 8 requirements become observable here.
- **Phase 32** (Enriched History): Visual history enhancements — color-coding, user attribution, clickable links. 4 requirements become observable here. STOCK-02 maps here because stock-in entries are only visible in the enriched history feed.
- **Phase 33** (Type Safety & Coverage): Quality gate — ts-rs type generation and test coverage. No v1 requirements mapped directly (validates Phase 30-32).

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-04 — note-derived backlog and delivery guidance added*
