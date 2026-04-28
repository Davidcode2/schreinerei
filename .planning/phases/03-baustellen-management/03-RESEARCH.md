# Phase 3: Baustellen Management - Research

**Gathered:** 2026-04-28
**Status:** Ready for planning

## Domain Analysis

### Core Entities

| Entity | Description | Key Attributes |
|--------|-------------|----------------|
| **Site** | Baustelle (construction site) | location, customer, date_range, status, estimated_days |
| **SiteAssignment** | Mitarbeiter-Zuweisung | site_id, user_id, role |
| **TimeEntry** | Arbeitszeit-Buchung | site_id, user_id, hours, date, work_type |
| **Activity** | Foto/Notiz im Activity Feed | site_id, user_id, type, content, timestamp |

### Relationships

```
Tenant (1) ──── (N) Site
Site (1) ──── (N) SiteAssignment
Site (1) ──── (N) TimeEntry
Site (1) ──── (N) Activity
User (1) ──── (N) SiteAssignment
User (1) ──── (N) TimeEntry
User (1) ──── (N) Activity
```

### Status Flow for Site

```
Planned → Active → Completed → Archived
```

### Time Entry Types

| Type | Description |
|------|-------------|
| `site_work` | Arbeit auf Baustelle |
| `workshop` | Vorbereitung in Werkstatt |
| `cnc` | CNC-Arbeit |
| `delivery` | Materiallieferung |

## Technical Decisions

### Architecture Pattern

Follow established DDD pattern from IAM and Inventory modules:

```
src/modules/sites/
├── domain/
│   ├── site.rs          # Site aggregate
│   ├── time_entry.rs    # TimeEntry aggregate
│   ├── activity.rs      # Activity aggregate
│   └── events.rs        # Site-specific domain events
├── application/
│   └── site_service.rs  # Business logic and authorization
├── infrastructure/
│   └── site_repository.rs  # Database operations
└── api/
    └── routes.rs        # REST API endpoints
```

### Database Schema

Based on established patterns:

```sql
-- Sites table
CREATE TABLE sites (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(200) NOT NULL,
    customer_name VARCHAR(200) NOT NULL,
    location VARCHAR(255),
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'planned',
    start_date DATE,
    end_date DATE,
    estimated_days INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Site assignments (many-to-many site-user)
CREATE TABLE site_assignments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    site_id UUID NOT NULL REFERENCES sites(id),
    user_id UUID NOT NULL REFERENCES users(id),
    role VARCHAR(50) NOT NULL DEFAULT 'worker',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, site_id, user_id)
);

-- Time entries
CREATE TABLE time_entries (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    site_id UUID REFERENCES sites(id),  -- NULL for workshop work
    user_id UUID NOT NULL REFERENCES users(id),
    work_type VARCHAR(20) NOT NULL,
    hours DECIMAL(4,2) NOT NULL,
    work_date DATE NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Activity feed entries
CREATE TABLE site_activities (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    site_id UUID NOT NULL REFERENCES sites(id),
    user_id UUID NOT NULL REFERENCES users(id),
    activity_type VARCHAR(20) NOT NULL,  -- 'photo', 'note', 'status_change'
    content TEXT,
    photo_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### API Endpoints

| Method | Path | Description | Role |
|--------|------|-------------|------|
| GET | /api/v1/sites | List sites | Any |
| POST | /api/v1/sites | Create site | Admin |
| GET | /api/v1/sites/{id} | Get site details | Any |
| PATCH | /api/v1/sites/{id} | Update site | Admin |
| POST | /api/v1/sites/{id}/assign | Assign user to site | Admin |
| DELETE | /api/v1/sites/{id}/assign/{userId} | Remove assignment | Admin |
| GET | /api/v1/sites/{id}/time-entries | List time entries for site | Any |
| POST | /api/v1/time-entries | Create time entry | Any |
| GET | /api/v1/time-entries/my | Get my time entries | Any |
| POST | /api/v1/sites/{id}/activities | Add activity (photo/note) | Any |
| GET | /api/v1/sites/{id}/activities | Get activity feed | Any |
| GET | /api/v1/dashboard/sites | Dashboard - open sites | Any |

### Domain Events

Following the event pattern from Inventory:

- `SiteCreated` - New site created
- `SiteStatusChanged` - Site status updated
- `UserAssignedToSite` - User assigned
- `TimeEntryCreated` - Time booked
- `ActivityAdded` - Photo/note added

### Authorization Rules

| Operation | Admin | Employee |
|-----------|-------|----------|
| Create site | ✓ | ✗ |
| Update site | ✓ | ✗ |
| Assign users | ✓ | ✗ |
| List sites | ✓ | ✓ (own tenant) |
| Create time entry | ✓ | ✓ (self only) |
| View own time entries | ✓ | ✓ |
| Add activity | ✓ | ✓ |
| View dashboard | ✓ | ✓ |

## Dependencies

### From Phase 1 (Auth & IAM)
- TenantId from JWT context
- UserId and Role types
- AuthenticatedUser extractor
- AppState with database pool

### From Phase 2 (Inventory)
- Domain event infrastructure
- EventBus for publishing events
- Repository pattern with tenant isolation

## Common Pitfalls

1. **Tenant isolation**: Every query MUST include tenant_id filter
2. **Time entry validation**: Hours must be positive, date cannot be in future
3. **Site assignment**: User must exist in same tenant
4. **Status transitions**: Only valid state transitions allowed
5. **Activity feed ordering**: Order by created_at DESC for timeline

## Standard Stack

Following established patterns:
- Rust 2021 edition with Axum 0.8
- SQLx 0.8 with runtime queries
- Domain events via EventBus
- Multi-tenant isolation via TenantId

## Out of Scope

- Photo upload/storage (use URLs only for V1)
- Calendar integration
- Notification system for status changes
- Mobile-specific endpoints (PWA handles that)
