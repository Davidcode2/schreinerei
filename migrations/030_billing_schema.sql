CREATE TABLE invoice_number_sequences (
    tenant_id UUID PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    prefix TEXT NOT NULL DEFAULT 'RE',
    next_number BIGINT NOT NULL DEFAULT 1 CHECK (next_number > 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE sites
    ADD CONSTRAINT sites_tenant_id_id_unique UNIQUE (tenant_id, id);

CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID NOT NULL,
    invoice_number BIGINT NOT NULL CHECK (invoice_number > 0),
    invoice_number_display TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'generated', 'void')),
    sender_name TEXT,
    sender_address TEXT,
    issued_at TIMESTAMPTZ,
    due_on DATE,
    voided_at TIMESTAMPTZ,
    pdf_storage_path TEXT,
    pdf_sha256_hash TEXT,
    pdf_content_type TEXT,
    pdf_size_bytes BIGINT CHECK (pdf_size_bytes IS NULL OR pdf_size_bytes > 0),
    pdf_created_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT invoices_site_tenant_fk
        FOREIGN KEY (tenant_id, site_id)
        REFERENCES sites(tenant_id, id)
        ON DELETE CASCADE,
    CONSTRAINT invoices_pdf_metadata_complete_check CHECK (
        (
            pdf_storage_path IS NULL
            AND pdf_sha256_hash IS NULL
            AND pdf_content_type IS NULL
            AND pdf_size_bytes IS NULL
            AND pdf_created_at IS NULL
        )
        OR (
            pdf_storage_path IS NOT NULL
            AND pdf_sha256_hash IS NOT NULL
            AND pdf_content_type IS NOT NULL
            AND pdf_size_bytes IS NOT NULL
            AND pdf_created_at IS NOT NULL
        )
    ),
    UNIQUE (tenant_id, invoice_number),
    UNIQUE (tenant_id, invoice_number_display)
);

CREATE INDEX idx_invoices_tenant ON invoices(tenant_id);
CREATE INDEX idx_invoices_site ON invoices(tenant_id, site_id);
CREATE INDEX idx_invoices_status ON invoices(tenant_id, status);

CREATE TRIGGER invoices_updated_at
    BEFORE UPDATE ON invoices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
