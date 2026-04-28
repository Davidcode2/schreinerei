-- Sites table (Baustellen)
CREATE TABLE sites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    customer_name VARCHAR(200) NOT NULL,
    location VARCHAR(255),
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'planned',
    start_date DATE,
    end_date DATE,
    estimated_days INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

-- Index for tenant-scoped queries
CREATE INDEX idx_sites_tenant ON sites(tenant_id);
CREATE INDEX idx_sites_status ON sites(status);

-- Update timestamp trigger for sites
CREATE TRIGGER sites_updated_at
    BEFORE UPDATE ON sites
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Site assignments (many-to-many site-user)
CREATE TABLE site_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'worker',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, site_id, user_id)
);

-- Indexes for site_assignments
CREATE INDEX idx_site_assignments_tenant ON site_assignments(tenant_id);
CREATE INDEX idx_site_assignments_site ON site_assignments(site_id);
CREATE INDEX idx_site_assignments_user ON site_assignments(user_id);

-- Time entries (Arbeitszeit-Buchungen)
CREATE TABLE time_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID REFERENCES sites(id) ON DELETE SET NULL,  -- NULL for workshop work
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    work_type VARCHAR(20) NOT NULL,
    hours DECIMAL(4,2) NOT NULL,
    work_date DATE NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for time_entries
CREATE INDEX idx_time_entries_tenant ON time_entries(tenant_id);
CREATE INDEX idx_time_entries_site ON time_entries(site_id);
CREATE INDEX idx_time_entries_user ON time_entries(user_id);
CREATE INDEX idx_time_entries_date ON time_entries(work_date DESC);

-- Site activities (Activity Feed - Fotos und Notizen)
CREATE TABLE site_activities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_type VARCHAR(20) NOT NULL,  -- 'photo', 'note', 'status_change'
    content TEXT,
    photo_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for site_activities
CREATE INDEX idx_site_activities_tenant ON site_activities(tenant_id);
CREATE INDEX idx_site_activities_site ON site_activities(site_id);
CREATE INDEX idx_site_activities_created ON site_activities(created_at DESC);
