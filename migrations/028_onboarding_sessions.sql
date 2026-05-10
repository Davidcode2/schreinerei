-- Public self-service onboarding state.
-- These records are intentionally not tenant-scoped until a tenant exists.
CREATE TABLE onboarding_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_name VARCHAR(255) NOT NULL,
    organization_slug VARCHAR(100) NOT NULL UNIQUE,
    admin_email VARCHAR(255) NOT NULL,
    admin_name VARCHAR(255),
    selected_plan VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending_payment',
    payment_provider VARCHAR(50),
    payment_id VARCHAR(255),
    checkout_url TEXT,
    tenant_id UUID REFERENCES tenants(id) ON DELETE SET NULL,
    keycloak_organization_id VARCHAR(255),
    keycloak_organization_alias VARCHAR(255),
    error_code VARCHAR(100),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    UNIQUE(payment_provider, payment_id)
);

CREATE INDEX idx_onboarding_sessions_payment
    ON onboarding_sessions(payment_provider, payment_id)
    WHERE payment_provider IS NOT NULL AND payment_id IS NOT NULL;

CREATE INDEX idx_onboarding_sessions_status ON onboarding_sessions(status);

CREATE TRIGGER onboarding_sessions_updated_at
    BEFORE UPDATE ON onboarding_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TABLE payment_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    provider VARCHAR(50) NOT NULL,
    provider_event_id VARCHAR(255) NOT NULL,
    payment_id VARCHAR(255) NOT NULL,
    raw_payload JSONB NOT NULL,
    payment_status VARCHAR(50),
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_event_id)
);

CREATE INDEX idx_payment_events_payment ON payment_events(provider, payment_id);
