#!/bin/bash
# Setup test tenant and data for local development

set -e

DB_URL="postgres://schreinerei:bfGkOLzqH7klHp5ApkcTUUgDX1gTlDiG@localhost:5433/schreinerei"

TENANT_ID="${TENANT_ID:-a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d}"

echo "Setting up test data..."

# Create test tenant
psql "$DB_URL" <<EOF
-- Create test tenant if not exists
INSERT INTO tenants (id, keycloak_realm, name, slug)
VALUES (
    '$TENANT_ID',
    'schreinerei',
    'Test Schreinerei',
    'test-schreinerei'
) ON CONFLICT (id) DO NOTHING;

-- Verify tenant
SELECT * FROM tenants WHERE id = '$TENANT_ID';
EOF

echo ""
echo "✓ Test tenant created: $TENANT_ID"
echo ""
echo "Next steps:"
echo "1. In Keycloak, add attribute 'tenant_id' = '$TENANT_ID' to your test user"
echo "2. Run: ./scripts/test-api.sh"
