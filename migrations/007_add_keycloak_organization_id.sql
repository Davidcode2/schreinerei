-- Add keycloak_organization_id column to tenants table
-- This column stores the Keycloak Organization UUID that corresponds to each tenant

ALTER TABLE tenants ADD COLUMN keycloak_organization_id UUID UNIQUE;

-- Create index for organization ID lookups
CREATE INDEX idx_tenants_keycloak_organization_id ON tenants(keycloak_organization_id);

-- Add comment documenting the column purpose
COMMENT ON COLUMN tenants.keycloak_organization_id IS 'Keycloak Organization UUID for organization-based tenancy. Populated after creating organizations in Keycloak.';
