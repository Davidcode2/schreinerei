# Frontend Testing Playbook

Fast, repeatable UI validation for the Schreinerei frontend. The code and migrations are the source of truth; the realistic dataset is designed to expose current UI states rather than preserve legacy table shapes.

## Quick Start

This repository requires an isolated PostgreSQL database per worktree. Never use another developer's database.

Current audit workspace:

| Service | Value |
|---|---|
| Worktree | `.worktrees/frontend-ui-audit` |
| Database container | `schreinerei-db-ui-audit` |
| Database | `schreinerei_ui_audit` |
| PostgreSQL port | `5447` |
| Backend | `http://localhost:3009` |
| Frontend | `http://localhost:5175` |
| Keycloak | `https://auth.jakob-lingel.dev` |

From the worktree root:

```bash
docker start schreinerei-db-ui-audit
set -a
source .env
set +a
sqlx migrate run
./scripts/setup-test-data.sh
cargo run --bin schreinerei
```

In another terminal:

```bash
cd frontend
npm ci
npm run dev
```

Health checks:

```bash
curl -fsS http://localhost:3009/health
curl -fsSI http://localhost:5175/login
```

The backend `.env` must point to the worktree-specific database. The frontend `.env` must set `VITE_API_URL=http://localhost:3009`.

## Authentication

Use `.agents/local/frontend-credentials.env`; it is ignored and must not be committed. The reusable browser flow is in `frontend/tests/helpers/auth.ts`.

Flow:

1. Open `/login`.
2. Click `Mit Keycloak anmelden`.
3. Fill the Keycloak username, then click `Sign In`.
4. Fill the password, then click `Sign In`.
5. Wait for `/auth/callback` to return to `/`.

The realistic seed maps its `Hans Saur` admin row to the stable local test account's Keycloak subject. Override it when using another account:

```bash
AUTH_KEYCLOAK_USER_ID=<jwt-sub> \
AUTH_EMAIL=<email> \
AUTH_NAME=<display-name> \
./scripts/setup-test-data.sh
```

## Reset And Capture

Reset all audit data without recreating the database:

```bash
set -a
source .env
set +a
./scripts/setup-test-data.sh
```

Capture all maintained desktop/mobile screenshots:

```bash
cd frontend
npx playwright test ui-audit.spec.ts --project=chromium
```

Focused captures:

```bash
npx playwright test ui-audit.spec.ts --grep "desktop pages"
npx playwright test ui-audit.spec.ts --grep "mobile navigation"
```

Screenshots are written to `.planning/frontend-ui-audit/screenshots/`. `frontend/playwright.config.ts` reuses a running Vite server, so the same command works with or without a prestarted frontend.

## Stable Seed Anchors

These IDs are intentionally stable for direct navigation and automation.

| State | Name | ID / route |
|---|---|---|
| Active project | Einbauschrank Dachgeschoss | `/sites/00000000-0000-0000-0000-000000000401` |
| Planned project | Empfang Praxis Dr. Seidel | `/sites/00000000-0000-0000-0000-000000000402` |
| Completed project | Garderoben Kindergarten St. Martin | `/sites/00000000-0000-0000-0000-000000000403` |
| Active project | Verkaufstheke Hofladen Braun | `/sites/00000000-0000-0000-0000-000000000404` |
| Archived project | Treppenaufgang Familie Riek | `/sites/00000000-0000-0000-0000-000000000405` |
| Low + expired material | PUR-Leim D4 500 g | `/inventory/00000000-0000-0000-0000-000000000311` |
| Available tool | Fein MultiMaster | `/tools/00000000-0000-0000-0000-000000000606` |
| In-use tool | Lamello Zeta P2 | `/tools/00000000-0000-0000-0000-000000000602` |
| Overdue maintenance | Mafell Erika 85 | `/tools/00000000-0000-0000-0000-000000000603` |
| Due maintenance | VW Crafter Montage 1 | `/fleet/00000000-0000-0000-0000-000000000501` |
| Current tool reservation | Lamello Zeta P2 | `00000000-0000-0000-0000-000000000701` |
| Pending tool reservation | Fein MultiMaster | `00000000-0000-0000-0000-000000000705` |

The seed also includes current-week customer, deployment, and milestone appointments; all reservation lifecycle statuses; vehicle display colors; current reservation project context; expiry batches; billing defaults; priced and unpriced material; maintenance states; and project timeline entries.

## Page Checklist

| Page | Fastest path | Key checks |
|---|---|---|
| Login | `/login` | Keycloak CTA, signup link |
| Signup | `/signup` | Normal form; separately test `?invite=<token>` |
| Onboarding result | `/onboarding/complete?session=<id>` | Missing, pending, completed, failed states |
| Dashboard | `/` | Four stats, expiry warnings, status filters, active-project action |
| Inventory | `/inventory` | Categories, search, expiry warnings, settings, cards, delete |
| Material detail | Stable route above | Low stock, expired/expiring batches, history, stock actions |
| Projects | `/sites` | Filters, search, active project, add/delete |
| Project overview | Stable route above | Timeline, material history, team, appointments, status |
| Project details | Append `/details` | Billing metadata and invoice creation |
| Project times | Append `/time` | Time list and editable own entries |
| Historical report | `/sites/history` | Customer/type/date/employee/hour filters |
| Fleet | `/fleet` | Calendar, colored resources, maintenance, reservation actions |
| Vehicle detail | Stable route above | Edit, reserve, reservation history, maintenance |
| Tools | `/tools` | Calendar, pending/current reservations, list states |
| Tool detail | Stable route above | Edit, reserve, maintenance states |
| QR scanner | `/scan` | Camera denied, retry, manual entry, known/unknown result |
| Settings | `/settings` | Profile, billing, users, invite, logout |
| Inventory settings | `/settings/inventory` | Create/edit/delete categories and conflict state |
| 404 | Any unknown protected route | Return-to-dashboard action |

Desktop navigation is always visible. On mobile, click the button named `Menü öffnen` to open the navigation sheet.

## Overlay Checklist

| Overlay | Fastest trigger | Important variants |
|---|---|---|
| Add material | `/inventory` -> `Material hinzufügen` | Step 1/2; expiry category; nested `+ Neu` category |
| Edit material | Material detail -> pencil | Stock correction and billing defaults |
| Stock in | Material detail -> `Einlagern` | Expiry, batch and goods-receipt fields |
| Withdraw | Material detail -> `Material entnehmen` | Normal, disposal, last package, preselected project |
| Delete material | Inventory card -> trash | Generic destructive confirmation |
| Add/edit category | Inventory settings | Expiry toggle and category delete conflict |
| Add project | `/sites` -> `Projekt anlegen` | External/internal and hourly/fixed billing |
| Project planning | Project -> `Planen` | Bottom sheet, all project/billing fields |
| Project status | Project -> status badge | planned -> active -> completed -> archived |
| Time entry | Project -> `Zeit buchen` | Create/edit, work types, delete confirmation |
| Timeline composer | Project -> `Eintrag` | Notes, file picker, camera, multiple files |
| Media viewer | Click a timeline attachment | Image/PDF, copy, download, failed preview |
| Appointment | Project -> `Termin` or calendar slot | Create/edit, type, team, delete |
| Invoice | Project details -> `Rechnung erstellen` | Hourly/fixed, priced/unpriced materials |
| Add/edit vehicle | Fleet add button or vehicle pencil | Two steps, status and display color |
| Add/edit tool | Tools add button or tool pencil | Two steps and status |
| Create reservation | Resource card/detail -> `Reservieren` | Availability conflict and optional project |
| Calendar confirmation | Click two empty calendar cells | Project, purpose, optional custom times |
| Edit reservation | Click occupied calendar entry | Details, transitions, dates, delete |
| Maintenance | Resource detail -> `Planen` | Schedule, due date, resolve action |
| Invite user | Settings -> `Einladen` | Form and generated-invite state |
| Pending actions | Sidebar/header pending badge | Requires queued offline mutation |
| Mobile navigation | Mobile hamburger | Main routes, active project, profile/logout |
| PWA install | Browser `beforeinstallprompt` | Install and dismiss |
| QR result | Successful scan/manual code | Material, vehicle/tool, unknown |

## Priority: Tool Reservation Editing

This is the fastest reliable path to the dialog highlighted for special review:

1. Reset the realistic seed.
2. Open `/tools` in the current week.
3. Click the occupied `Fein MultiMaster` cell showing `Lukas Eisele` and `Anfrage`.
4. Wait for `Reservierung bearbeiten`; a brief `Reservierung laden` dialog is valid.
5. Inspect status details, project, current holder, dates, purpose, note, transitions, delete and footer behavior.

Stable Playwright locator:

```ts
const entry = page.locator('[role="button"]').filter({
  hasText: /Lukas Eisele/,
}).first();
await entry.click();
await expect(page.getByRole('dialog', {
  name: 'Reservierung bearbeiten',
})).toBeVisible();
```

Calendar range creation:

```ts
const emptySlots = page.locator('button[data-selection-state="idle"]');
await emptySlots.nth(0).click();
await emptySlots.nth(1).click();
await expect(page.getByRole('dialog')).toBeVisible();
```

Both cells must belong to the same resource. Clicking another resource restarts selection. Clicking the same cell twice creates a one-day range.

## State Coverage Gaps

Some states require temporary data or browser controls and are intentionally not persisted in the baseline:

| State | Fast setup |
|---|---|
| Reservation conflict | Open an available resource's create dialog and overlap a seeded pending/confirmed reservation |
| Category delete conflict | Delete a category used by a seeded material |
| Offline queue/popover | Set browser offline, perform a supported mutation, return online |
| PWA install | Dispatch/use a real `beforeinstallprompt` event in a supported browser |
| QR camera denied | Deny camera permission before opening `/scan` |
| Valid QR result | Manually enter `MAT-CHEMIE-011`, `FLT-TL-606`, or `FLT-VEH-501` |
| Invoice history/PDF | Create an invoice through the UI; generated PDF artifacts are not faked by SQL |
| Attachment viewer | Upload a real image/PDF through the timeline composer |
| Invitation states | Create an invite through Settings, then use its token on `/signup` |
| Onboarding states | Use a real onboarding session ID; payment history is not reset by the demo seed |

## Screenshot Catalog

The numbered files in `.planning/frontend-ui-audit/screenshots/` are grouped as follows:

| Range | Coverage |
|---|---|
| `01`-`07` | Dashboard and inventory pages/dialogs |
| `08`-`17` | Projects, planning, appointments, invoice and time |
| `18`-`24` | Fleet/tools calendars, priority reservation dialog, maintenance |
| `25`-`30` | Settings, categories, history and 404 |
| `31`-`34` | Mobile dashboard, navigation, tools and priority dialog |

## Existing Automated Coverage

Use focused suites before running the entire E2E collection:

```bash
cd frontend
npx vitest run src/pages/fleet/ReservationDialog.test.tsx
npx vitest run src/pages/fleet/CalendarView.test.tsx
npx playwright test reservation-status.spec.ts
npx playwright test calendar-click-create.spec.ts
npx playwright test edit-operations.spec.ts
```

Several older E2E assertions use obsolete labels such as `Aktive Baustellen` and `Niedrige Bestände`. Treat current source labels (`Aktive Projekte`, `Materialwarnungen`) as authoritative and update stale tests rather than changing current UI copy to satisfy them.

## Unreachable Legacy Components

Do not spend browser time on these unrouted files:

- `frontend/src/pages/FleetPage.tsx`
- `frontend/src/pages/InventoryPage.tsx`
- `frontend/src/pages/SitesPage.tsx`
- `frontend/src/pages/SettingsPage.tsx`
- `frontend/src/pages/fleet/ReservationsList.tsx`

The route map in `frontend/src/App.tsx` is authoritative.
