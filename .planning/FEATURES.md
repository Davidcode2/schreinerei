# Feature Inventory and Product Gaps

**Last updated:** 2026-05-04

## Purpose

This document compares the currently shipped product surface with the broader product vision captured in the April/May 2026 note. It is the planning bridge between today's implemented app and the next milestone definitions.

## Current Shipped Feature Set

### Platform and Architecture

- Multi-tenant SaaS with organization-based tenancy
- Multi-user application with Keycloak authentication
- Mobile-first web app / PWA for tablet and phone
- Offline support for field workflows
- Rust backend, React frontend, PostgreSQL persistence
- Kubernetes and GitHub Actions deployment foundation

### Inventory

- Inventory CRUD in-app
- Category management
- Stock tracking and stock corrections
- Stock-in flow
- Inventory history with user attribution
- Baustelle-linked material withdrawals
- Category visible on overview entries

### Baustellen / Project Execution

- Baustelle CRUD
- Baustelle status workflow (`geplant`, `aktiv`, `abgeschlossen`)
- Activity feed with notes
- Activity feed entries with camera uploads, documents, and mixed attachments
- Fullscreen media viewer with deep links, sharing, download, and metadata
- Material activity visible in Baustelle context

### Fleet and Reservations

- Vehicle and tool reservations
- `/fleet` as primary fleet reservation surface
- Embedded fleet calendar
- Two-tap date-range selection
- Bottom confirmation sheet for mobile booking flow
- Stable per-resource color accents for calendar visibility

### Quality and Delivery

- Full CRUD across core entities already shipped in v1.x
- E2E coverage exists for current delivered flows
- Modular-monolith direction already documented in planning

## Comparison Against Note

### Already Covered or Largely Covered

- Multi tenant
- Multi user
- Inventory foundation
- Geraetemanagement / reservation basics
- Baustellenmanagement basics
- Mobile-first tablet/phone app
- Material withdrawals linked to Baustellen
- Photo and document capture on Baustellen
- Calendar-based fleet reservations

### Partially Covered

- Vehicle booking UX: current fleet calendar exists, but the product note adds stricter defaults and interaction details for the weekly vehicle-first view
- Project communication: current Baustelle activity feed exists, but it is not yet positioned as the central replacement for WhatsApp-style project context
- Maintenance: reservations and inventory exist, but maintenance intervals and service workflows are not implemented
- Material control: current stock levels exist, but low-stock purchase triggers, expiry-date handling, and richer replenishment workflows are missing
- Central overview: basic entity views exist, but not the operational cross-module dashboard described in the note

### Not Yet Covered

- Expanded inventory domains: plates, edges, paints, screws, drill bits, cutters, large machines, loan PCs, fleet inventory, and arbitrary future inventory types
- Per-batch / per-expiry inventory handling for items with `Haltbarkeitsdatum`
- "Last package taken" notification and ordering workflow
- Machine maintenance planning and persistent reminder surfacing
- Tool and vehicle location tracking
- RFID-assisted automatic tool-in-vehicle detection
- Workshop location map with tool placement and stock status indicators
- Multi-language UI
- AI-powered search
- Employee planning, absence, and capacity planning
- Order intake and goods receipt management
- Sharpening workflow for saw blades and cutters
- Consumption statistics and labor statistics per Baustelle / project
- Project budgeting, quote tracking, and invoice generation
- E-invoicing and DATEV readiness
- Packlist / loading checklist generation for montage
- Rich project timeline as the main project coordination channel
- Visual part finder for manufactured parts
- Voice-to-documentation workflow
- Digital measurement (`Aufmass`) workflows and hardware integrations
- CAD/CNC file handling, DXF/BSolid workflows, nesting, and manufacturing glue code
- In-app user feedback capture with contextual improvement routing

## Recommended Product Structure

To keep the system extensible over a ten-plus-year horizon, the note should be captured as domain modules rather than as one flat feature list.

### Module 1: IAM and Tenant Context

Scope:
- Keycloak bridge
- internal user profile mapping
- tenant scoping on every request
- role and permission model

Planning stance:
- Already foundationally present
- Continue to enforce request-context-based `TenantId` propagation

### Module 2: Inventory and Procurement

Scope:
- materials and consumables
- inventory categories and extensible inventory item types
- stock levels, replenishment thresholds, and low-stock alerts
- expiry-aware inventory for paints, glues, or similar consumables
- order intake, goods receipt, and purchasing workflows
- sharpening / external servicing state for reusable consumables and tools

Key design ideas from the note:
- Inventory must be extensible beyond simple materials
- Some categories need per-batch attributes such as expiry date
- Stock movements should remain easy for workers, with minimal manual input
- "Last package taken" should trigger a manager notification and purchasing action

Recommended approach:
- Keep a common inventory item base model plus category-specific attributes
- Use a state-based relational model for current stock plus audit/history tables for every mutation
- Model expiry-managed stock as inventory batches, not just a single quantity number
- Add reorder signals from either `minimum_quantity` breaches or an explicit "last package taken" action

### Module 3: Projects / Baustellen

Scope:
- projects, sites, workshop jobs, scheduling, assignments, notes, media, and budget tracking
- time booking on-site and in workshop
- project feed as the source of truth for execution context
- customer intake and project creation flow

Key design ideas from the note:
- "Baustelle" and "Projekt" likely collapse into one project model with location-aware work modes
- Material usage should always be booked against a project for later billing and analytics
- The project feed should become the operational memory of the job

Recommended approach:
- Evolve current Baustelle model into a broader `Project` bounded context without forcing a separate external/internal distinction yet
- Keep one canonical project timeline for notes, documents, photos, voice notes, and derived AI summaries
- Require project linkage for material withdrawals, while allowing fast defaults from the active selected project
- Add budget, quote, and invoice readiness fields directly on the project aggregate

### Module 4: Fleet, Tools, and Maintenance

Scope:
- vehicles, mobile tools, fixed machines, reservations, usage windows, maintenance intervals, and location signals

Key design ideas from the note:
- Workers should know who has a tool, where it is, and for which Baustelle it is in use
- The process must reduce bureaucracy, not create more check-in/check-out work
- RFID and automatic detection are a preferred future optimization, not an MVP requirement

Recommended approach:
- Keep explicit reservations as the core source of truth
- Auto-fill reservation date range from the selected project where possible
- Add maintenance schedules as first-class records with due dates, recurrence rules, and visible reminders
- Treat RFID as an optional infrastructure adapter later; do not bake hardware assumptions into the core domain

### Module 5: Staff Planning and Capacity

Scope:
- employee calendar
- availability, holidays, assignments, and capacity estimation
- staffing view for project planning

Recommended approach:
- Build on the existing scheduling direction already established in fleet/project workflows
- Keep time booking, assignment, and availability separate but linkable
- Start with manager planning visibility before pursuing deeper optimization logic

### Module 6: Manufacturing and CAD/CNC Glue

Scope:
- DXF/BSolid file storage and status tracking
- part lists
- nesting preparation
- manufacturing context for workshop execution

Key design ideas from the note:
- The app should not try to replace specialist scanners or CAD/CAM software
- It should act as the glue between measurement, project context, manufacturing files, and workshop execution

Recommended approach:
- Start with file-centric workflows, metadata, and project linkage
- Allow upload/reference of DXF, point clouds, and manufacturing documents before attempting deep semantic editing
- Treat CAD/CNC automation as a separate bounded context so billing or project changes cannot destabilize it

### Module 7: Measurement (`Aufmass`)

Scope:
- digital room measurement
- hardware-assisted measurement import
- photodocumentation
- plausibility checks
- export to CAD/manufacturing flows

Recommended approach:
- Phase 1: app-native photo / LiDAR-assisted sketch capture and structured checklists
- Phase 2: Leica DISTO Bluetooth field injection for critical dimensions
- Phase 3: DXF import, point-cloud linking, and handoff to manufacturing context
- Add plausibility checks before attempting ambitious full AI automation

### Module 8: Billing and Finance

Scope:
- quote attachment
- budget tracking
- invoice creation
- PDF export
- e-invoicing readiness and DATEV integration

Recommended approach:
- Use project material usage and time booking as the canonical billing basis
- Keep this context event-driven from project completion and project cost updates
- Delay full accounting integration until project and inventory booking discipline is stable

### Module 9: AI and Automation

Scope:
- AI search
- voice capture and summarization
- voice-to-documentation
- AI-assisted measurement extraction
- contextual user feedback routing

Key design ideas from the note:
- AI should reduce friction at the point of work, not create extra admin effort
- Voice workflows should prefer efficient, local-first pipelines where possible

Recommended approach:
- Start with narrow, high-value AI tasks: semantic search, speech-to-note, issue capture, and suggestion routing
- Keep the AI layer as adapters on top of stable domain workflows
- Store structured outputs and provenance rather than relying on opaque LLM-only state

## Requirements Added From The Note

The detailed requirement backlog derived from the note is maintained in `.planning/REQUIREMENTS.md` under the `Product Backlog from 2026-05 Note` section.

## Immediate Planning Candidates

These are the strongest near-term candidates because they build directly on the shipped product instead of starting a brand new domain from zero.

1. Project feed completion and operationalization
2. Expiry-aware inventory and replenishment alerts
3. Maintenance intervals for machines and vehicles
4. Project-linked material/time analytics and dashboarding
5. Project budget and invoice preparation foundation
6. Multi-language support groundwork

## Explicitly Deferred or High-Risk Areas

- RFID hardware automation: valuable, but depends on physical rollout and hardware reliability
- Full CAD/CNC semantic automation: high leverage, but should begin with file orchestration rather than deep editor ambitions
- GPS employee tracking: requires careful GDPR review before product commitment
- Full voice agent stack: attractive, but should follow simpler speech-to-note and feedback capture wins
