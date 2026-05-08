CREATE TABLE goods_receipts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    received_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    supplier_name TEXT,
    receipt_reference TEXT,
    receipt_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE goods_receipt_lines (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    receipt_id UUID NOT NULL REFERENCES goods_receipts(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL,
    batch_code TEXT,
    expires_on DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (quantity > 0)
);

CREATE INDEX idx_goods_receipts_tenant ON goods_receipts(tenant_id, created_at DESC);
CREATE INDEX idx_goods_receipts_reference ON goods_receipts(tenant_id, receipt_reference);
CREATE INDEX idx_goods_receipt_lines_receipt ON goods_receipt_lines(receipt_id);
CREATE INDEX idx_goods_receipt_lines_material ON goods_receipt_lines(tenant_id, material_id);

CREATE TRIGGER goods_receipts_updated_at
    BEFORE UPDATE ON goods_receipts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
