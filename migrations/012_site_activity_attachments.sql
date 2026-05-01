CREATE TABLE site_activity_attachments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    activity_id UUID NOT NULL REFERENCES site_activities(id) ON DELETE CASCADE,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    storage_key TEXT NOT NULL,
    thumbnail_key TEXT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    original_bytes BYTEA,
    thumbnail_bytes BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, activity_id)
);

CREATE INDEX idx_site_activity_attachments_tenant ON site_activity_attachments(tenant_id);
CREATE INDEX idx_site_activity_attachments_activity ON site_activity_attachments(activity_id);
CREATE INDEX idx_site_activity_attachments_site ON site_activity_attachments(site_id);
