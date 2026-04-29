# Keycloak Setup Guide

This guide explains how to configure Keycloak for the Schreinerei SaaS application using organization-based multi-tenancy.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  Keycloak Realm: schreinerei                                │
│                                                             │
│  Client: schreinerei-pwa (Public)                           │
│  └── Used by: Frontend PWA                                  │
│  └── Flow: OAuth2 Authorization Code + PKCE                 │
│                                                             │
│  Organizations:                                              │
│  ├── Schreinerei Müller (id: uuid-aaa)                      │
│  │   └── Members: user1, user2                              │
│  └── Schreinerei Schmidt (id: uuid-bbb)                     │
│      └── Members: user3                                     │
│                                                             │
│  JWT Token:                                                  │
│  {                                                           │
│    "sub": "user-id",                                         │
│    "organization": { "uuid-aaa": {} },                       │
│    "realm_access": { "roles": ["admin"] }                    │
│  }                                                           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  Backend (Rust)                                              │
│  └── Validates JWTs via JWKS (no client needed)              │
│  └── Extracts tenant_id from organization claim              │
└─────────────────────────────────────────────────────────────┘
```

**Note:** The backend does NOT use a Keycloak client. It validates JWTs using the public JWKS endpoint.

## Prerequisites

- Keycloak 26.x or later (for Organizations feature)
- Admin access to Keycloak
- `schreinerei` realm created

## Quick Start

### 1. Enable Organizations in Realm

1. Go to **Realm Settings** → **General**
2. Enable **Organizations** flag
3. Save

### 2. Create the PWA Client

1. Go to **Clients** → **Create client**

| Field | Value |
|-------|-------|
| Client type | OpenID Connect |
| Client ID | `schreinerei-pwa` |
| Client authentication | **Off** (public client) |
| Authorization | Off |
| Authentication flow | Standard flow |

| Field | Value |
|-------|-------|
| Root URL | `http://localhost:5173` (dev) |
| Valid redirect URIs | `http://localhost:5173/auth/callback`, `http://localhost:5173/*` |
| Valid post logout redirect URIs | `/*` |
| Web origins | `+` |

### 3. Add Required Scopes

Go to **Clients** → `schreinerei-pwa` → **Client Scopes** → **Add client scope**:

- `openid` (built-in)
- `profile` (built-in)
- `email` (built-in)
- `organization` (built-in)

### 4. Add Realm Role Mapper

1. Go to **Clients** → `schreinerei-pwa` → **Mappers**
2. **Add mapper**:

| Field | Value |
|-------|-------|
| Name | `realm-roles` |
| Mapper type | User Realm Role |
| Multivalued | On |
| Token claim name | `realm_access.roles` |

### 5. Create Realm Roles

Go to **Realm Roles** → **Create role**:

- `admin` — Full access (create/update/delete)
- `mitarbeiter` — Employee access (read/withdraw/book time)

### 6. Create Organizations

1. Go to **Organizations** → **Create organization**
2. For each tenant:
   - **Name**: Tenant name (e.g., "Schreinerei Müller")
   - **Alias**: URL-friendly slug (e.g., "mueller")
   - **Domains**: Email domain (optional)

### 7. Add Users to Organizations

1. Go to **Organizations** → [select org] → **Members**
2. **Add member** → **Add realm user**
3. Select users belonging to this organization

### 8. Run Database Migration

```bash
sqlx migrate run
```

## Testing

### Verify Token Contains Organization Claim

```bash
# Get token
TOKEN=$(curl -s -X POST \
  "https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token" \
  -d "client_id=schreinerei-pwa" \
  -d "username=test@example.com" \
  -d "password=your-password" \
  -d "grant_type=password" \
  -d "scope=openid profile email organization" | jq -r '.access_token')

# Decode and check organization claim
echo $TOKEN | cut -d'.' -f2 | base64 -d 2>/dev/null | jq '.organization'
```

Expected output:
```json
{
  "cec54914-b702-4c7b-9431-b407817d059a": {}
}
```

## Configuration Files

### Backend (.env)

```env
DATABASE_URL=postgresql://user:password@localhost:5432/schreinerei
KEYCLOAK_URL=https://auth.jakob-lingel.dev
KEYCLOAK_REALM=schreinerei
JWT_ISSUER=https://auth.jakob-lingel.dev/realms/schreinerei
HOST=0.0.0.0
PORT=3000
```

### Frontend (.env)

```env
VITE_KEYCLOAK_URL=https://auth.jakob-lingel.dev
VITE_KEYCLOAK_REALM=schreinerei
VITE_KEYCLOAK_CLIENT_ID=schreinerei-pwa
VITE_API_URL=http://localhost:3000
```

## Troubleshooting

### Organization claim not in token

1. Verify user is organization member
2. Check `organization` scope is added to client
3. Ensure Organizations enabled in realm

### Invalid redirect URI

- Match redirect URI exactly (no trailing slashes)
- Check protocol (http vs https)

### CORS errors

- Add frontend URL to Web Origins in client config

### Missing roles in token

- Check realm role mapper is configured
- Verify user has role assigned

## API Reference

### Organization Management

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/admin/realms/{realm}/orgs` | GET | List organizations |
| `/admin/realms/{realm}/orgs` | POST | Create organization |
| `/admin/realms/{realm}/orgs/{orgId}` | GET/PUT/DELETE | CRUD organization |
| `/admin/realms/{realm}/orgs/{orgId}/members` | GET/POST | List/add members |
| `/admin/realms/{realm}/orgs/{orgId}/members/invite` | POST | Invite member |

### Example: Create Organization via API

```bash
ADMIN_TOKEN=$(curl -s -X POST \
  "https://auth.jakob-lingel.dev/realms/master/protocol/openid-connect/token" \
  -d "client_id=admin-cli" \
  -d "username=admin" \
  -d "password=admin-password" \
  -d "grant_type=password" | jq -r '.access_token')

curl -X POST \
  "https://auth.jakob-lingel.dev/admin/realms/schreinerei/orgs" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Schreinerei Müller",
    "alias": "mueller",
    "enabled": true
  }'
```

## Security Notes

1. **Never use client secrets in frontend** — PKCE provides security for public clients
2. **Use HTTPS in production** — OAuth2 requires secure connections
3. **Backend validates JWTs** — Uses JWKS, no client credentials needed
4. **Short token lifespans** — Configure 5-15 minute access tokens
5. **Refresh tokens** — Enable for better UX; automatically rotated

## References

- [Keycloak Documentation](https://www.keycloak.org/documentation)
- [Keycloak Organizations](https://www.keycloak.org/docs/latest/server_admin/index.html#organizations)
- [OAuth2 PKCE RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636)

---

*Last updated: 2026-04-29*
