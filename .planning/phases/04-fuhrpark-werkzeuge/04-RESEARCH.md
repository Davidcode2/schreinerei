# Phase 4: Fuhrpark & Werkzeuge - Research

**Gathered:** 2026-04-28
**Status:** Ready for planning

## Domain Analysis

### Core Entities

| Entity | Description | Key Attributes |
|--------|-------------|----------------|
| **Vehicle** | Fahrzeug (Bulli, Transporter, etc.) | name, license_plate, type, status, location |
| **Tool** | Werkzeug/Gerät | name, category, status, qr_code, location |
| **Reservation** | Reservierung | resource_id, user_id, site_id, start_time, end_time, status |
| **ResourceStatus** | Status von Ressource | available, reserved, in_use, maintenance |

### Relationships

```
Tenant (1) ──── (N) Vehicle
Tenant (1) ──── (N) Tool
Tenant (1) ──── (N) Reservation
Vehicle (1) ──── (N) Reservation
Tool (1) ──── (N) Reservation
Site (1) ──── (N) Reservation (optional - can be without site)
User (1) ──── (N) Reservation
```

### Resource Types

| Type | Description |
|------|-------------|
| `vehicle` | Fahrzeug (Bulli, Transporter, PKW) |
| `tool` | Werkzeug/Gerät (Bohrmaschine, Kreissäge, etc.) |

### Reservation Status Flow

```
pending → confirmed → in_use → completed
                  ↘ cancelled
```

## Technical Decisions

### Architecture Pattern

Follow established DDD pattern from IAM, Inventory, and Sites modules:

```
src/modules/fleet/
├── domain/
│   ├── vehicle.rs       # Vehicle aggregate
│   ├── tool.rs          # Tool aggregate
│   ├── reservation.rs   # Reservation aggregate
│   └── events.rs        # Fleet-specific domain events
├── application/
│   └── fleet_service.rs # Business logic and authorization
├── infrastructure/
│   └── fleet_repository.rs  # Database operations
└── api/
    └── routes.rs        # REST API endpoints
```

### Database Schema

Based on established patterns:

```sql
-- Vehicles table
CREATE TABLE vehicles (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(200) NOT NULL,
    license_plate VARCHAR(20),
    vehicle_type VARCHAR(50) NOT NULL,  -- 'bulli', 'transporter', 'car'
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    location VARCHAR(255),
    qr_code VARCHAR(100) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

-- Tools table
CREATE TABLE tools (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(200) NOT NULL,
    category VARCHAR(100),
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    location VARCHAR(255),
    qr_code VARCHAR(100) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

-- Reservations table (unified for vehicles and tools)
CREATE TABLE reservations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    resource_type VARCHAR(20) NOT NULL,  -- 'vehicle' or 'tool'
    resource_id UUID NOT NULL,           -- references vehicle or tool
    user_id UUID NOT NULL REFERENCES users(id),
    site_id UUID REFERENCES sites(id),   -- optional link to site
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX idx_vehicles_tenant ON vehicles(tenant_id);
CREATE INDEX idx_vehicles_status ON vehicles(status);
CREATE INDEX idx_vehicles_qr_code ON vehicles(qr_code);
CREATE INDEX idx_tools_tenant ON tools(tenant_id);
CREATE INDEX idx_tools_status ON tools(status);
CREATE INDEX idx_tools_qr_code ON tools(qr_code);
CREATE INDEX idx_reservations_tenant ON reservations(tenant_id);
CREATE INDEX idx_reservations_resource ON reservations(resource_type, resource_id);
CREATE INDEX idx_reservations_user ON reservations(user_id);
CREATE INDEX idx_reservations_site ON reservations(site_id);
CREATE INDEX idx_reservations_time ON reservations(start_time, end_time);
```

### API Endpoints

| Method | Path | Description | Role |
|--------|------|-------------|------|
| GET | /api/v1/fleet/vehicles | List vehicles | Any |
| POST | /api/v1/fleet/vehicles | Create vehicle | Admin |
| GET | /api/v1/fleet/vehicles/{id} | Get vehicle details | Any |
| PATCH | /api/v1/fleet/vehicles/{id} | Update vehicle | Admin |
| DELETE | /api/v1/fleet/vehicles/{id} | Delete vehicle | Admin |
| GET | /api/v1/fleet/tools | List tools | Any |
| POST | /api/v1/fleet/tools | Create tool | Admin |
| GET | /api/v1/fleet/tools/{id} | Get tool details | Any |
| PATCH | /api/v1/fleet/tools/{id} | Update tool | Admin |
| DELETE | /api/v1/fleet/tools/{id} | Delete tool | Admin |
| POST | /api/v1/fleet/reservations | Create reservation | Any |
| GET | /api/v1/fleet/reservations | List reservations | Any |
| GET | /api/v1/fleet/reservations/{id} | Get reservation | Any |
| PATCH | /api/v1/fleet/reservations/{id} | Update reservation | Owner/Admin |
| DELETE | /api/v1/fleet/reservations/{id} | Cancel reservation | Owner/Admin |
| GET | /api/v1/fleet/calendar | Calendar view | Any |
| GET | /api/v1/fleet/availability | Check availability | Any |
| GET | /api/v1/fleet/qr/{code} | QR code lookup | Any |

### Domain Events

Following the event pattern from Inventory and Sites:

- `VehicleCreated` - New vehicle created
- `ToolCreated` - New tool created
- `ReservationCreated` - New reservation made
- `ReservationUpdated` - Reservation changed
- `ReservationCancelled` - Reservation cancelled
- `ResourceStatusChanged` - Vehicle/tool status changed

### Authorization Rules

| Operation | Admin | Employee |
|-----------|-------|----------|
| Create vehicle/tool | ✓ | ✗ |
| Update vehicle/tool | ✓ | ✗ |
| Delete vehicle/tool | ✓ | ✗ |
| List vehicles/tools | ✓ | ✓ (own tenant) |
| Create reservation | ✓ | ✓ |
| Update own reservation | ✓ | ✓ |
| Cancel own reservation | ✓ | ✓ |
| View calendar | ✓ | ✓ |
| QR code lookup | ✓ | ✓ |

### QR Code Pattern

Follow established pattern from Phase 2 (Inventory):
- QR code format: `{tenant_prefix}:{resource_type}:{id}`
- Example: `SCH123:vehicle:abc123`
- Lookup is tenant-scoped

### Calendar View

Return reservations grouped by resource and date:
```json
{
  "resources": [
    {
      "id": "...",
      "name": "Bulli 1",
      "type": "vehicle",
      "reservations": [
        {
          "id": "...",
          "start_time": "2026-04-28T08:00:00Z",
          "end_time": "2026-04-28T17:00:00Z",
          "user_name": "Max Mustermann",
          "site_name": "Baustelle Berlin"
        }
      ]
    }
  ]
}
```

## Dependencies

### From Phase 1 (Auth & IAM)
- TenantId from JWT context
- UserId and Role types
- AuthenticatedUser extractor
- AppState with database pool

### From Phase 2 (Inventory)
- Domain event infrastructure
- EventBus for publishing events
- QR code generation pattern (for reference)
- Repository pattern with tenant isolation

### From Phase 3 (Sites)
- SiteId type for linking reservations to sites
- Site entity for site lookup

## Common Pitfalls

1. **Tenant isolation**: Every query MUST include tenant_id filter
2. **Reservation overlap**: Must check for conflicting reservations before creating
3. **Time validation**: end_time must be after start_time
4. **Resource availability**: Must check resource status before confirming reservation
5. **QR code uniqueness**: QR codes must be unique across tenant

## Standard Stack

Following established patterns:
- Rust 2021 edition with Axum 0.8
- SQLx 0.8 with runtime queries
- Domain events via EventBus
- Multi-tenant isolation via TenantId

## Out of Scope

- GPS tracking of vehicles
- Maintenance scheduling (V2 feature - EXT-01, EXT-02)
- Fuel consumption tracking
- Driver license verification
- Photo upload for vehicles/tools (use URLs only for V1)
