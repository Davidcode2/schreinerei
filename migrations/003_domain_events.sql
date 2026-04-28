-- Domain events table for event sourcing and inter-module communication
CREATE TABLE domain_events (
    id UUID PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id VARCHAR(36) NOT NULL,
    payload JSONB NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for querying events by tenant and type
CREATE INDEX idx_domain_events_tenant_type ON domain_events(tenant_id, event_type);
CREATE INDEX idx_domain_events_aggregate ON domain_events(aggregate_type, aggregate_id);
CREATE INDEX idx_domain_events_occurred ON domain_events(occurred_at DESC);

-- Index for event replay scenarios
CREATE INDEX idx_domain_events_created ON domain_events(created_at DESC);
