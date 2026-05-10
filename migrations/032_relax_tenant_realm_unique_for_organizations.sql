ALTER TABLE tenants DROP CONSTRAINT IF EXISTS tenants_keycloak_realm_key;

COMMENT ON COLUMN tenants.keycloak_realm IS
    'Shared Keycloak realm for organization-based tenancy. Tenants are distinguished by keycloak_organization_alias.';
