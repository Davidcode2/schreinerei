CREATE TABLE maintenance_schedules (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    task_description TEXT NOT NULL,
    interval_days INTEGER NOT NULL CHECK (interval_days > 0),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_maintenance_schedules_tenant ON maintenance_schedules(tenant_id);
CREATE INDEX idx_maintenance_schedules_asset ON maintenance_schedules(tenant_id, asset_id);
CREATE INDEX idx_maintenance_schedules_active ON maintenance_schedules(tenant_id, asset_id) WHERE is_active = true;

CREATE TRIGGER maintenance_schedules_updated_at
    BEFORE UPDATE ON maintenance_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TABLE maintenance_due (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    schedule_id UUID NOT NULL REFERENCES maintenance_schedules(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    due_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'resolved')),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (
        (status = 'open' AND resolved_at IS NULL)
        OR (status = 'resolved' AND resolved_at IS NOT NULL)
    )
);

CREATE INDEX idx_maintenance_due_tenant ON maintenance_due(tenant_id);
CREATE INDEX idx_maintenance_due_asset ON maintenance_due(tenant_id, asset_id);
CREATE INDEX idx_maintenance_due_open ON maintenance_due(tenant_id, due_date) WHERE status = 'open';
CREATE UNIQUE INDEX idx_maintenance_due_schedule_open
    ON maintenance_due(tenant_id, schedule_id)
    WHERE status = 'open';

CREATE TRIGGER maintenance_due_updated_at
    BEFORE UPDATE ON maintenance_due
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
