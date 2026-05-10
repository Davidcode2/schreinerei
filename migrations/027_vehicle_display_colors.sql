CREATE TABLE vehicle_display_colors (
    asset_id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    display_color VARCHAR(7) NOT NULL CHECK (display_color ~ '^#[0-9a-f]{6}$'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, display_color)
);

CREATE INDEX idx_vehicle_display_colors_tenant ON vehicle_display_colors(tenant_id);

CREATE TRIGGER vehicle_display_colors_updated_at
    BEFORE UPDATE ON vehicle_display_colors
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

WITH ordered_vehicles AS (
    SELECT
        id,
        tenant_id,
        ROW_NUMBER() OVER (PARTITION BY tenant_id ORDER BY md5(id::text))::bigint AS color_index
    FROM assets
    WHERE asset_kind = 'vehicle'
      AND deleted_at IS NULL
)
INSERT INTO vehicle_display_colors (asset_id, tenant_id, display_color)
SELECT
    id,
    tenant_id,
    '#' || lpad(to_hex(((color_index * 2654435761 + 104729) % 16777215)::bigint), 6, '0')
FROM ordered_vehicles
ON CONFLICT (asset_id) DO NOTHING;
