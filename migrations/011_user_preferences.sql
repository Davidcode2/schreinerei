-- User preferences table for storing user-specific settings
CREATE TABLE user_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    preferences JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, user_id)
);

-- Index for user-scoped queries
CREATE INDEX idx_user_preferences_tenant ON user_preferences(tenant_id);
CREATE INDEX idx_user_preferences_user ON user_preferences(user_id);

-- Update timestamp trigger
CREATE TRIGGER user_preferences_updated_at
    BEFORE UPDATE ON user_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Add site_id to stock_entries for linking deductions to Baustellen
ALTER TABLE stock_entries 
    ADD COLUMN site_id UUID REFERENCES sites(id) ON DELETE SET NULL;

-- Index for site-scoped stock queries
CREATE INDEX idx_stock_entries_site ON stock_entries(site_id);
