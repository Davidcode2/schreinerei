CREATE TABLE site_appointments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    appointment_kind TEXT NOT NULL CHECK (appointment_kind IN ('customer_appointment', 'worker_deployment', 'milestone')),
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    notes TEXT,
    assigned_user_ids UUID[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (ends_at > starts_at)
);

CREATE INDEX idx_site_appointments_tenant_site ON site_appointments(tenant_id, site_id);
CREATE INDEX idx_site_appointments_range ON site_appointments(tenant_id, starts_at, ends_at);

CREATE TRIGGER site_appointments_updated_at
    BEFORE UPDATE ON site_appointments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
