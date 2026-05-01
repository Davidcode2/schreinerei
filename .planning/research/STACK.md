# Technology Stack

**Project:** Schreinerei SaaS — Inventory v1.9
**Researched:** 2026-05-01

## Recommended Stack

### Core Framework (UNCHANGED)

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust | stable | Backend language | Already in use, 10+ year maintainability |
| Axum | 0.8 | HTTP framework | Already in use, hexagonal routing |
| SQLx | 0.8 | Database client | Already in use, compile-time checked queries |
| PostgreSQL | 15+ | Database | Already in use, tenant-scoped queries |
| React | 18 | Frontend framework | Already in use |
| Vite | 6 | Build tool | Already in use |
| TypeScript | 5.x | Frontend types | Already in use |
| Tailwind CSS | 4 | Styling | Already in use |
| shadcn/ui | latest | Component library | Already in use |
| TanStack Query | latest | Data fetching | Already in use (React Query hooks) |
| ts-rs | latest | Type generation | Already in use (49 DTOs exported) |

### Database Changes (NEW in v1.9)

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| PostgreSQL ENUM | 15+ | `stock_entry_type` enum | Type-safe discrimination of history entries |
| Migration 014 | NEW | Add `entry_type` to `stock_entries` | Backfill existing rows with correct types |

### Supporting Libraries (UNCHANGED)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde | latest | JSON serialization | All DTOs and event payloads |
| serde_json | latest | Event payload construction | `json!()` macro for event payloads |
| chrono | latest | Timestamps | Already in use for all temporal data |
| uuid | latest | ID generation | Already in use for all entity IDs |
| qrcode | latest | QR code generation | Already in use, no changes needed |
| React Router | v6 | Frontend routing | Add `/settings/inventory` route |
| sonner | latest | Toast notifications | Already in use for action feedback |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| History type discrimination | PostgreSQL ENUM column | JSONB payload field | ENUM provides type safety at DB level, matches Unit/SiteStatus pattern |
| Category delete | Block with constraint check | Soft delete with "uncategorized" | Block is simpler, follows existing `delete_material` pattern |
| Stock-in implementation | New `StockIn` domain command | Reuse `AdjustStock` command | AdjustStock is admin-only + requires reason; stock-in is for all users with notes |
| Settings page routing | `/settings/inventory` route | Tab within SettingsPage | Separate page scales better, follows existing page pattern |
| History in frontend | New `MaterialHistoryFeed` component | Share `ActivityFeed` from sites | Data shapes differ fundamentally; sharing creates brittle adapter code |

## Installation

No new packages required. One database migration needed:

```bash
# Run migration
sqlx migrate run

# Regenerate TypeScript types after DTO changes
cargo test --features ts-rs/export
```

## Sources

- Codebase analysis: `Cargo.toml` — all dependencies verified
- Codebase analysis: `migrations/` — existing schema patterns
- Codebase analysis: `frontend/package.json` — all frontend dependencies verified