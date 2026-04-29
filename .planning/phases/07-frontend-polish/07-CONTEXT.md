# Phase 7: Frontend Polish - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Connect non-functional UI buttons to working backend APIs, add dialogs for creating resources (materials, sites, vehicles, tools), implement user invitation, and improve QR scanner error handling with retry capability.

This phase does NOT add new features — it polishes existing UI to be fully functional.

</domain>

<decisions>
## Implementation Decisions

### Dialog UX Flow
- **D-01:** Dialog closes immediately on successful form submission
- **D-02:** Success toast appears bottom-right (standard Sonner position)
- **D-03:** Form resets to empty state when dialog reopens (fresh form each time)
- **D-04:** Dialogs follow existing `WithdrawDialog` pattern structure

### Form Validation
- **D-05:** Client-side validation matches backend validation rules
- **D-06:** Inline error messages below each field (not toast notifications for validation)
- **D-07:** Required fields marked with asterisk (*)
- **D-08:** Submit button disabled until form is valid

### QR Scanner Retry (ERR-01, ERR-02)
- **D-09:** Retry button appears when camera access denied
- **D-10:** Manual code entry text field as fallback option
- **D-11:** Friendly error message: "Kamera-Zugriff verweigert. Bitte Kamera-Berechtigung erteilen oder Code manuell eingeben."

### User Invitation (USER-01)
- **D-12:** Show Keycloak organization invite link for admins to copy and share
- **D-13:** No email sending from frontend — uses Keycloak's built-in invitation URL
- **D-14:** Display: "Organisation beitreten" section in UserManagementSection with copyable link

### User Management (USER-02)
- **D-15:** Replace mock users with real data from `/api/v1/iam/users` endpoint
- **D-16:** Show user name, email, role from API response

### the agent's Discretion
- Form field ordering and layout (follow existing patterns)
- Exact toast message wording (German, concise)
- Error boundary behavior for failed API calls

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Backend API Endpoints (existing)
- `src/modules/inventory/api/routes.rs` — `POST /api/v1/inventory/materials` for CreateMaterial
- `src/modules/sites/api/routes.rs` — `POST /api/v1/sites` for CreateSite
- `src/modules/fleet/api/routes.rs` — `POST /api/v1/fleet/vehicles`, `POST /api/v1/fleet/tools`
- `src/modules/iam/api/routes.rs` — `GET /api/v1/iam/users` for user list

### Frontend Patterns
- `frontend/src/components/ui/dialog.tsx` — shadcn/ui Dialog component
- `frontend/src/pages/inventory/WithdrawDialog.tsx` — Reference dialog pattern
- `frontend/src/components/qr/QrScanner.tsx` — Current QR scanner implementation

### Types
- `frontend/src/types/inventory.ts` — CreateMaterialRequest interface
- `frontend/src/types/sites.ts` — CreateSiteRequest interface
- `frontend/src/types/fleet.ts` — CreateVehicleRequest, CreateToolRequest interfaces

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `useCreateSite`, `useCreateVehicle`, `useCreateTool` hooks — already exist, just need UI
- `Dialog`, `DialogContent`, `DialogHeader`, `DialogFooter` — shadcn/ui components ready
- `apiClient` — authenticated HTTP client with error handling
- `toast()` from sonner — for success notifications

### Missing Pieces
- `useCreateMaterial` hook — needs to be added to `frontend/src/lib/api/hooks/useInventory.ts`
- User invitation link generation — needs Keycloak organization URL construction
- `GET /api/v1/iam/users` endpoint — verify exists or create

### Established Patterns
- Dialogs use `useState` for form state, mutation hooks for submission
- Forms use shadcn/ui `Input`, `Label`, `Button`, `Select` components
- Error handling via mutation `isError` and `error` properties
- Query invalidation on successful mutation

### Integration Points
- "Material hinzufügen" button in `InventoryListPage.tsx` → open AddMaterialDialog
- "Baustelle anlegen" button in `SitesListPage.tsx` → open AddSiteDialog
- "Fahrzeug hinzufügen" button in `VehiclesList.tsx` → open AddVehicleDialog
- "Werkzeug hinzufügen" button in `ToolsList.tsx` → open AddToolDialog
- "Einladen" button in `UserManagementSection.tsx` → show invite link or dialog
- Error state in `QrScanner.tsx` → show retry button and manual entry

</code_context>

<specifics>
## Specific Ideas

None — straightforward implementation following existing patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 07-Frontend-Polish*
*Context gathered: 2026-04-29*
