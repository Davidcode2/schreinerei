ALTER TABLE categories
    ADD COLUMN can_expire BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE materials
    ADD COLUMN legacy_quantity INTEGER NOT NULL DEFAULT 0;

UPDATE materials
SET legacy_quantity = quantity;

CREATE TABLE material_batches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE CASCADE,
    expires_on DATE NOT NULL,
    initial_quantity INTEGER NOT NULL,
    remaining_quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (initial_quantity > 0),
    CHECK (remaining_quantity >= 0),
    CHECK (remaining_quantity <= initial_quantity)
);

CREATE INDEX idx_material_batches_tenant ON material_batches(tenant_id);
CREATE INDEX idx_material_batches_material ON material_batches(material_id);
CREATE INDEX idx_material_batches_expires_on ON material_batches(expires_on);
