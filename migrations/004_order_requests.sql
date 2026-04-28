CREATE TABLE order_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    requested_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    reason TEXT,
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    fulfilled_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_requests_tenant ON order_requests(tenant_id);
CREATE INDEX idx_order_requests_material ON order_requests(material_id);
CREATE INDEX idx_order_requests_status ON order_requests(tenant_id, status);

CREATE TRIGGER order_requests_updated_at
    BEFORE UPDATE ON order_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
