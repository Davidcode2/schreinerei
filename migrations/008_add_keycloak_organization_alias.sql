-- Add keycloak_organization_alias column to tenants table
-- Keycloak returns organization aliases (not UUIDs) in the JWT claim
-- Example claim: {"organization": ["schreinerei_saur_affalterwang"]}

ALTER TABLE tenants ADD COLUMN keycloak_organization_alias VARCHAR(255) UNIQUE;

-- Create index for organization alias lookups
CREATE INDEX idx_tenants_keycloak_organization_alias ON tenants(keycloak_organization_alias);

-- Add comment documenting the column purpose
COMMENT ON COLUMN tenants.keycloak_organization_alias IS 'Keycloak Organization alias for organization-based tenancy. Used to match JWT organization claim to tenant.';
