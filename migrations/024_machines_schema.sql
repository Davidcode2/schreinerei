-- Shared asset identity for vehicles, tools, and workshop machines.
CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    asset_kind VARCHAR(20) NOT NULL CHECK (asset_kind IN ('vehicle', 'tool', 'machine')),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    location VARCHAR(255),
    qr_code VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    UNIQUE(tenant_id, asset_kind, name)
);

CREATE UNIQUE INDEX idx_assets_qr_code_unique
    ON assets(tenant_id, qr_code)
    WHERE qr_code IS NOT NULL;

CREATE INDEX idx_assets_tenant ON assets(tenant_id);
CREATE INDEX idx_assets_kind ON assets(asset_kind);
CREATE INDEX idx_assets_status ON assets(status);
CREATE INDEX idx_assets_not_deleted ON assets(tenant_id, asset_kind) WHERE deleted_at IS NULL;

CREATE TRIGGER assets_updated_at
    BEFORE UPDATE ON assets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Backfill existing vehicle/tool rows as core assets. The old tables remain
-- for migration compatibility; new application code uses assets + detail rows.
INSERT INTO assets (
    id,
    tenant_id,
    asset_kind,
    name,
    description,
    status,
    location,
    qr_code,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    id,
    tenant_id,
    'vehicle',
    name,
    description,
    status,
    location,
    qr_code,
    created_at,
    updated_at,
    deleted_at
FROM vehicles
ON CONFLICT (id) DO NOTHING;

INSERT INTO assets (
    id,
    tenant_id,
    asset_kind,
    name,
    description,
    status,
    location,
    qr_code,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    id,
    tenant_id,
    'tool',
    name,
    description,
    status,
    location,
    qr_code,
    created_at,
    updated_at,
    deleted_at
FROM tools
ON CONFLICT (id) DO NOTHING;

CREATE TABLE vehicle_details (
    asset_id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    license_plate VARCHAR(20),
    vehicle_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vehicle_details_tenant ON vehicle_details(tenant_id);

CREATE TRIGGER vehicle_details_updated_at
    BEFORE UPDATE ON vehicle_details
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

INSERT INTO vehicle_details (
    asset_id,
    tenant_id,
    license_plate,
    vehicle_type,
    created_at,
    updated_at
)
SELECT id, tenant_id, license_plate, vehicle_type, created_at, updated_at
FROM vehicles
ON CONFLICT (asset_id) DO NOTHING;

CREATE TABLE tool_details (
    asset_id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    category VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tool_details_tenant ON tool_details(tenant_id);

CREATE TRIGGER tool_details_updated_at
    BEFORE UPDATE ON tool_details
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

INSERT INTO tool_details (
    asset_id,
    tenant_id,
    category,
    created_at,
    updated_at
)
SELECT id, tenant_id, category, created_at, updated_at
FROM tools
ON CONFLICT (asset_id) DO NOTHING;

CREATE TABLE machine_details (
    asset_id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    machine_type VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_machine_details_tenant ON machine_details(tenant_id);

CREATE TRIGGER machine_details_updated_at
    BEFORE UPDATE ON machine_details
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

ALTER TABLE reservations
    ADD COLUMN asset_id UUID;

UPDATE reservations
SET asset_id = resource_id
WHERE asset_id IS NULL;

ALTER TABLE reservations
    ALTER COLUMN asset_id SET NOT NULL;

ALTER TABLE reservations
    ADD CONSTRAINT reservations_asset_id_fkey
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE RESTRICT;

CREATE INDEX idx_reservations_asset ON reservations(tenant_id, asset_id);
