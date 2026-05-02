#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ -z "${DATABASE_URL:-}" ] && [ -f "$REPO_ROOT/.env" ]; then
  DATABASE_URL="$(grep '^DATABASE_URL=' "$REPO_ROOT/.env" | cut -d= -f2-)"
  export DATABASE_URL
fi

: "${DATABASE_URL:?DATABASE_URL must be set or present in .env}"

TENANT_ID="${TENANT_ID:-a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d}"
ORG_ALIAS="${ORG_ALIAS:-schreinerei_saur_affalterwang}"

echo "Loading realistic test data into $DATABASE_URL"
echo "Tenant: $TENANT_ID"
echo "Organization alias: $ORG_ALIAS"
echo "Existing data for this tenant will be replaced."

psql "$DATABASE_URL" \
  -v ON_ERROR_STOP=1 \
  -v tenant_id="$TENANT_ID" \
  -v org_alias="$ORG_ALIAS" \
  -f "$SCRIPT_DIR/realistic-test-data.sql"

echo
echo "Realistic test data loaded successfully."
echo "Use the Keycloak organization alias '$ORG_ALIAS' for tenant resolution."
