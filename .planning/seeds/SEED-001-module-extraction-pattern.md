# SEED-001: Module Extraction Pattern

## WHY

The current IAM module was implemented with mod.rs files (deprecated Rust 2015 style) instead of the modern module_name.rs pattern (Rust 2018+). This seed captures the architectural pattern for future module implementations to ensure consistency.

## Surface When

- Starting implementation of new modules (Inventory, Sites, Fleet)
- Code review finds mod.rs usage
- Developer asks "How should I structure a new module?"

## Details

### Modern Rust Module Structure

**DO:**
```
src/
├── module_name.rs          # Module contents (not mod.rs)
└── module_name/
    └── submodule.rs        # Submodule contents
```

**DON'T:**
```
src/
└── module_name/
    └── mod.rs              # Deprecated style
```

### Hexagonal Layer Structure

Each module has three layers:

```
modules/inventory/
├── domain/           # Pure business logic, no dependencies
├── application/      # Use cases, port traits
└── infrastructure/   # Database adapters, external APIs
```

### Inter-Module Communication

Use Domain Events, not direct calls:

```rust
// In inventory/application
pub struct StockDepletedEvent {
    pub material_id: MaterialId,
    pub tenant_id: TenantId,
}

// In sites/application
impl EventHandler<StockDepletedEvent> for NotificationService {
    async fn handle(&self, event: StockDepletedEvent) {
        // Notify admin
    }
}
```

### Ports as Traits

```rust
// domain/port.rs
pub trait MaterialRepository: Send + Sync {
    async fn find_by_id(&self, id: MaterialId) -> Result<Option<Material>>;
    async fn save(&self, material: &Material) -> Result<()>;
}

// infrastructure/postgres.rs
pub struct PostgresMaterialRepository { /* ... */ }
impl MaterialRepository for PostgresMaterialRepository { /* ... */ }
```

## Breadcrumbs

- Updated AGENTS.md with module style guide
- PROJECT.md contains full architecture documentation
- REQUIREMENTS.md has ARCH-01 through ARCH-06 requirements
- Phase 1 commit `d577d23` migrated to modern style

## Related Seeds

- SEED-002: CAD/CNC Integration (DXF, Bsolid)
- SEED-003: RFID Hardware Integration

---
*Planted: 2026-04-28*
