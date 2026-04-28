-- Vehicles table (Fahrzeuge)
CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    license_plate VARCHAR(20),
    vehicle_type VARCHAR(50) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    location VARCHAR(255),
    qr_code VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name),
    UNIQUE(tenant_id, qr_code) WHERE qr_code IS NOT NULL
);

-- Indexes for vehicles
CREATE INDEX idx_vehicles_tenant ON vehicles(tenant_id);
CREATE INDEX idx_vehicles_status ON vehicles(status);
CREATE INDEX idx_vehicles_qr_code ON vehicles(qr_code);

-- Update timestamp trigger for vehicles
CREATE TRIGGER vehicles_updated_at
    BEFORE UPDATE ON vehicles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Tools table (Werkzeuge)
CREATE TABLE tools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    category VARCHAR(100),
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    location VARCHAR(255),
    qr_code VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name),
    UNIQUE(tenant_id, qr_code) WHERE qr_code IS NOT NULL
);

-- Indexes for tools
CREATE INDEX idx_tools_tenant ON tools(tenant_id);
CREATE INDEX idx_tools_status ON tools(status);
CREATE INDEX idx_tools_qr_code ON tools(qr_code);

-- Update timestamp trigger for tools
CREATE TRIGGER tools_updated_at
    BEFORE UPDATE ON tools
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Reservations table (Reservierungen)
CREATE TABLE reservations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type VARCHAR(20) NOT NULL,
    resource_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    site_id UUID REFERENCES sites(id) ON DELETE SET NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for reservations
CREATE INDEX idx_reservations_tenant ON reservations(tenant_id);
CREATE INDEX idx_reservations_resource ON reservations(resource_type, resource_id);
CREATE INDEX idx_reservations_user ON reservations(user_id);
CREATE INDEX idx_reservations_site ON reservations(site_id);
CREATE INDEX idx_reservations_time ON reservations(start_time, end_time);

-- Update timestamp trigger for reservations
CREATE TRIGGER reservations_updated_at
    BEFORE UPDATE ON reservations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
