ALTER TABLE reservations
    ADD COLUMN project_id UUID REFERENCES sites(id) ON DELETE SET NULL,
    ADD COLUMN purpose VARCHAR(160);

UPDATE reservations
SET project_id = site_id
WHERE project_id IS NULL
  AND site_id IS NOT NULL;

CREATE INDEX idx_reservations_project ON reservations(tenant_id, project_id);
