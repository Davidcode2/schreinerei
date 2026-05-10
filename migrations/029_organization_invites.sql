CREATE TABLE organization_invites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    token VARCHAR(255) NOT NULL UNIQUE,
    keycloak_invitation_id VARCHAR(255),
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_organization_invites_active_email
    ON organization_invites (tenant_id, lower(email))
    WHERE status = 'pending';

CREATE INDEX idx_organization_invites_tenant ON organization_invites(tenant_id);
CREATE INDEX idx_organization_invites_token ON organization_invites(token);
CREATE INDEX idx_organization_invites_status ON organization_invites(status);

CREATE TRIGGER organization_invites_updated_at
    BEFORE UPDATE ON organization_invites
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
