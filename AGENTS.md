# GSD Workflow

This project follows the Get Shit Done methodology for structured software development.

## Current State

- **Phase:** 1 — Auth & IAM Foundation
- **Status:** Ready to Plan
- **Next:** `/gsd-plan-phase 1`

## Key Files

| File | Purpose |
|------|---------|
| `.planning/PROJECT.md` | Project context and vision |
| `.planning/REQUIREMENTS.md` | V1 requirements with traceability |
| `.planning/ROADMAP.md` | Phase structure and timeline |
| `.planning/STATE.md` | Current progress and next actions |
| `.planning/config.json` | Workflow preferences |

## Workflow Commands

- `/gsd-plan-phase N` — Create execution plan for phase N
- `/gsd-execute-phase N` — Execute phase N plans
- `/gsd-verify-phase N` — Verify phase N completion
- `/gsd-transition N` — Transition to phase N
- `/gsd-progress` — View current progress

## Coding Guidelines

### Clean Code
- Descriptive names
- Short functions (rarely more than 20 lines)
- Short classes (rarely more than 200 lines)
- SRP (Single Responsibility Principle)

### Rust Module Style (2018+ Edition)

Use the modern module file structure, NOT the deprecated `mod.rs` style:

```
src/
├── module_name.rs          # Module contents (not mod.rs)
└── module_name/
    └── submodule.rs        # Submodule contents
```

**Avoid:**
```
src/
└── module_name/
    └── mod.rs              # Deprecated style
```

In `src/lib.rs` or `src/main.rs`, declare modules with:
```rust
mod module_name;
```

The module file `src/module_name.rs` can declare submodules:
```rust
mod submodule;  // Looks for src/module_name/submodule.rs
```

### Tech Stack

**Backend:**
- Rust with SQLx
- REST API
- PostgreSQL
- Modular Monolith with DDD Bounded Contexts

**Frontend:**
- Vite + React
- PWA with Service Worker
- Offline-first with IndexedDB

**Type Generation:**
- ts-rs (Rust → TypeScript)

**Auth:**
- Keycloak (existing instance in cluster)

**Deployment:**
- Kubernetes (existing cluster)
- GitHub Actions CI/CD

### Multi-Tenancy

Every query MUST be scoped to TenantId. Use request context to extract TenantId from Keycloak JWT.

### Local Database Isolation

Local development must NOT use a shared PostgreSQL database.

- Create git worktrees in the repository root under `.worktrees/`.
- Every developer and every agent must use a dedicated local Postgres container and a dedicated database.
- Never point `DATABASE_URL` at a teammate's or another agent's database.
- Never run migrations against a shared local database instance.
- Shared infrastructure such as Keycloak may stay shared; the application database must be isolated per developer/agent.

Recommended naming convention:

- Container: `schreinerei-db-<owner>`
- Database: `schreinerei_<owner>`
- Port: a unique local port per owner, for example `5433`, `5434`, `5435`

Recommended local setup:

1. Start a dedicated Postgres container for your owner id.
2. Create an uncommitted local env file for that workspace or export shell variables for that session only.
3. Set `DATABASE_URL` to your dedicated database.
4. Copy baseline app data from the main local database so auth and manual testing work immediately.
5. Run migrations only against that dedicated database.

Concrete setup example:

1. Start a dedicated container:
```bash
docker run -d \
  --name "schreinerei-db-<owner>" \
  -e POSTGRES_USER="schreinerei_<owner>" \
  -e POSTGRES_PASSWORD="schreinerei_<owner>_pw" \
  -e POSTGRES_DB="schreinerei_<owner>" \
  -p <port>:5432 \
  postgres:16
```

2. Wait until Postgres is ready:
```bash
until docker exec "schreinerei-db-<owner>" pg_isready -U "schreinerei_<owner>" -d "schreinerei_<owner>"; do sleep 1; done
```

3. Create a workspace-local `.env` in this worktree:
```bash
DATABASE_URL=postgres://schreinerei_<owner>:schreinerei_<owner>_pw@localhost:<port>/schreinerei_<owner>
KEYCLOAK_URL=https://auth.jakob-lingel.dev
KEYCLOAK_REALM=schreinerei
JWT_ISSUER=https://auth.jakob-lingel.dev/realms/schreinerei
```

Use `https://auth.jakob-lingel.dev` for Keycloak in local development and new worktrees. Do not default frontend or backend auth config to `localhost` unless you are intentionally running a local Keycloak instance.

Create a separate workspace-local `frontend/.env` in each worktree as well. Set `VITE_API_URL` to the backend port used by that specific worktree, for example `http://localhost:3009`. Do not assume a shared default backend port across worktrees.

4. Copy baseline data from the main local database into the isolated database.

Assumptions:

- The shared/main local database is reachable from the parent repo's `../.env`.
- The isolated worktree database is configured by this worktree's `.env`.

Recommended copy flow:

```bash
set -a
source ../.env
SHARED_DATABASE_URL="$DATABASE_URL"
set +a

set -a
source ./.env
LOCAL_DATABASE_URL="$DATABASE_URL"
set +a

psql "$LOCAL_DATABASE_URL" -v ON_ERROR_STOP=1 -c "TRUNCATE TABLE user_preferences, time_entries, stock_entries, site_assignments, site_activity_attachments, site_activities, order_requests, materials, categories, reservations, tools, vehicles, sites, users, tenants, domain_events RESTART IDENTITY CASCADE;"

pg_dump "$SHARED_DATABASE_URL" --data-only --column-inserts --exclude-table=_sqlx_migrations \
  | rg -v '^SET transaction_timeout = ' \
  | psql "$LOCAL_DATABASE_URL" -v ON_ERROR_STOP=1
```

If the full data import hits schema drift on newer or older tables, import the auth/bootstrap data at minimum:

```bash
pg_dump "$SHARED_DATABASE_URL" --data-only --column-inserts --table=tenants --table=users --table=user_preferences \
  | rg -v '^SET transaction_timeout = ' \
  | psql "$LOCAL_DATABASE_URL" -v ON_ERROR_STOP=1
```

Minimum required data for auth:

- `tenants` must contain the `keycloak_organization_alias` used by the token.
- `users` and `user_preferences` should also be copied for realistic manual testing.

Verification checks:

```bash
set -a
source ./.env
set +a

psql "$DATABASE_URL" -c "select id, name, keycloak_organization_alias from tenants;"
cargo run
```

If you see `No tenant found for organization alias: ...`, the isolated database is missing tenant bootstrap data from the main database.

Agent rules:

- Before running `cargo run`, `cargo test`, or migrations, verify that `DATABASE_URL` points to an owner-specific database, not a shared one.
- If the current env points to a shared database, stop and create/use a dedicated database first.
- Do not change or repair another developer's or agent's database.

Follow-up implementation work to prefer:

- Add a bootstrap script such as `scripts/dev-db.sh` to create a per-owner container, database, and connection string.
- Prefer workspace-local env files over parent-directory env files so one workspace cannot silently reuse another workspace's database.
- Update README and env examples to document the per-owner database workflow.

### ts-rs Type Generation

Backend DTOs use `ts-rs` to auto-generate TypeScript types, preventing frontend-backend type drift.

**Adding ts-rs to a new DTO:**

1. Add derive macro to struct:
```rust
use ts_rs::TS;
use serde::Serialize;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct CreateMaterialRequest {
    pub name: String,
    pub quantity: i32,
    pub category_id: Uuid,
}
```

2. Run type generation:
```bash
cargo export-types
```

3. Types are generated to `frontend/src/types/generated.ts`

4. Import in frontend:
```typescript
import { CreateMaterialRequest } from './types/generated';
```

**Guidelines:**
- Add `#[ts(export)]` to all request/response DTOs
- Run `cargo export-types` after modifying DTOs
- CI should fail if generated types differ from committed

**Phase 15** will add ts-rs to all existing DTOs.

### Architecture

```
src/
├── common/           # Shared types (TenantId, Money, etc.)
├── auth/             # Keycloak integration & JWT verification
└── modules/
    ├── iam/          # Identity & Access Management
    ├── inventory/    # Material management
    ├── sites/        # Construction site management
    └── fleet/        # Vehicle & tool management
```

Each module has:
- `domain/` — Pure business logic
- `application/` — Use cases / services
- `infrastructure/` — Database, external APIs

## Commit Strategy

Commit frequently with descriptive messages. Reference requirements when applicable.

Before creating a commit, always run the necessary verification for the changed area and confirm it passes.

- Frontend changes: run at minimum the relevant `vitest` tests and `npx tsc --noEmit` or `npm run build` when appropriate.
- Backend changes: run `cargo fmt --check` (and `cargo fmt` if needed), the relevant `cargo test`, `cargo clippy`, and any required integration or migration checks.
- Full-stack or shared contract changes: run both backend and frontend checks, including ts-rs type generation when DTOs change.
- If a relevant check cannot be run, explicitly note that before committing and explain why.

Format: `type(scope): description [REQ-ID]`

Examples:
- `feat(auth): add Keycloak JWT validation [AUTH-01]`
- `fix(inventory): correct stock deduction logic [INVT-03]`
- `docs: update README`

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:ca08a54f -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

## Session Completion

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
<!-- END BEADS INTEGRATION -->
