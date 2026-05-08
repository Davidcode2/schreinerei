ALTER TABLE order_requests
    ADD COLUMN request_kind VARCHAR(30) NOT NULL DEFAULT 'manual';

CREATE INDEX idx_order_requests_kind_status
    ON order_requests(tenant_id, material_id, request_kind, status);
