ALTER TABLE sites
ADD COLUMN project_type VARCHAR(32) NOT NULL DEFAULT 'external_site';

UPDATE sites
SET project_type = 'external_site'
WHERE project_type IS NULL OR project_type = '';

CREATE INDEX idx_sites_project_type ON sites(project_type);
