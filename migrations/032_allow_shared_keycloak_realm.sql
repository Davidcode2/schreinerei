-- Keycloak Organizations run many tenants inside one realm.
-- The original schema assumed one realm per tenant, so onboarding multiple
-- organizations into the same realm must not violate a tenant-level unique key.
ALTER TABLE tenants DROP CONSTRAINT IF EXISTS tenants_keycloak_realm_key;

CREATE INDEX IF NOT EXISTS idx_tenants_keycloak_realm ON tenants(keycloak_realm);
