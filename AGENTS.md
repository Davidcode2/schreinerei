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

**Auth:**
- Keycloak (existing instance in cluster)

**Deployment:**
- Kubernetes (existing cluster)
- GitHub Actions CI/CD

### Multi-Tenancy

Every query MUST be scoped to TenantId. Use request context to extract TenantId from Keycloak JWT.

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

Format: `type(scope): description [REQ-ID]`

Examples:
- `feat(auth): add Keycloak JWT validation [AUTH-01]`
- `fix(inventory): correct stock deduction logic [INVT-03]`
- `docs: update README`
