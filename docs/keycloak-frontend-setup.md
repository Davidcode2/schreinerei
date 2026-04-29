# Keycloak Setup for Frontend (PWA)

This guide explains how to configure Keycloak for the Schreinerei PWA frontend using OAuth2 PKCE flow.

## Prerequisites

- Keycloak instance running (e.g., `https://auth.jakob-lingel.dev`)
- Admin access to Keycloak
- Existing `schreinerei` realm

## Overview

The frontend uses **OAuth2 Authorization Code + PKCE** flow, which requires a **public client** (no client secret). This is different from the backend client which uses a confidential client.

| Client | Type | Used By | Flow |
|--------|------|---------|------|
| `schreinerei-app` | Confidential | Backend (service-to-service) | Client credentials |
| `schreinerei-pwa` | Public | Frontend (PWA) | Authorization Code + PKCE |

## Step 1: Create the Client

1. Navigate to **Keycloak Admin Console**
2. Select the `schreinerei` realm
3. Go to **Clients** → **Create client**

### General Settings

| Field | Value |
|-------|-------|
| Client type | OpenID Connect |
| Client ID | `schreinerei-pwa` |

### Capability Config

| Field | Value |
|-------|-------|
| Client authentication | **Off** (public client) |
| Authorization | Off |
| Authentication flow | Standard flow (checked) |
| Direct access grants | Off |

### Login Settings

| Field | Value |
|-------|-------|
| Root URL | `http://localhost:5173` (dev) or your production URL |
| Home URL | `/` |
| Valid redirect URIs | See below |
| Valid post logout redirect URIs | `/*` |
| Web origins | `+` (allows all from root URL) |

#### Redirect URIs (Development)

```
http://localhost:5173/auth/callback
http://localhost:5173/*
```

#### Redirect URIs (Production)

Add your production domain:

```
https://your-app-domain.com/auth/callback
https://your-app-domain.com/*
```

## Step 2: Configure Client Scopes

The frontend needs access to user claims including `tenant_id`.

### Option A: Use Existing Scope

If you already have a `tenant_id` mapper for the backend client:

1. Go to **Clients** → `schreinerei-pwa` → **Client scopes**
2. Add the scope that includes `tenant_id` mapper

### Option B: Create New Mapper

1. Go to **Clients** → `schreinerei-pwa` → **Mappers** (or **Client scopes** → **Dedicated scopes**)
2. Click **Add mapper**

| Field | Value |
|-------|-------|
| Name | `tenant-id` |
| Mapper type | User Attribute |
| User Attribute | `tenant_id` |
| Token Claim name | `tenant_id` |
| Claim JSON type | String |
| Add to ID token | On |
| Add to access token | On |
| Add to userinfo | On |

## Step 3: Configure Roles

The frontend reads roles from the JWT to determine user permissions.

### Realm Roles

Ensure these roles exist in the realm:

- `admin` — Full access (create/update/delete)
- `mitarbeiter` — Employee access (read/withdraw/book time)

### Role Mapping in Token

1. Go to **Clients** → `schreinerei-pwa` → **Mappers**
2. Add mapper for roles:

| Field | Value |
|-------|-------|
| Name | `realm-roles` |
| Mapper type | User Realm Role |
| Multivalued | On |
| Token claim name | `realm_access.roles` |
| Claim JSON type | String |

## Step 4: Create a Test User

1. Go to **Users** → **Add user**

| Field | Value |
|-------|-------|
| Username | `test-admin` |
| Email | `admin@test.com` |
| Email verified | On |
| Enabled | On |

2. Go to **Credentials** tab → Set password

3. Go to **Attributes** tab:

| Key | Value |
|-----|-------|
| `tenant_id` | (your test tenant UUID) |

4. Go to **Role mappings** tab:
   - Assign `admin` role

## Step 5: Verify Configuration

### Test the Token Endpoint

```bash
# Get a token manually (for testing)
curl -X POST "https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=schreinerei-pwa" \
  -d "username=test-admin" \
  -d "password=your-password"
```

### Decode the Token

Use [jwt.io](https://jwt.io) to decode the access token. Verify it contains:

```json
{
  "sub": "user-uuid",
  "email": "admin@test.com",
  "preferred_username": "test-admin",
  "tenant_id": "your-tenant-uuid",
  "realm_access": {
    "roles": ["admin", "offline_access", "uma_authorization"]
  }
}
```

## Frontend Configuration

The frontend reads configuration from environment variables:

```env
# frontend/.env
VITE_KEYCLOAK_URL=https://auth.jakob-lingel.dev
VITE_KEYCLOAK_REALM=schreinerei
VITE_KEYCLOAK_CLIENT_ID=schreinerei-pwa
VITE_API_URL=http://localhost:3000
```

## Troubleshooting

### "Invalid redirect URI"

- Ensure the redirect URI exactly matches what's configured in Keycloak
- Check for trailing slashes
- Verify protocol (http vs https)

### "Invalid client"

- Verify client ID matches exactly: `schreinerei-pwa`
- Ensure client authentication is **Off** (public client)

### Missing tenant_id in token

- Check the mapper is configured correctly
- Verify user has the `tenant_id` attribute set
- Ensure mapper is added to access token

### CORS errors

- Add frontend URL to Web Origins in client config
- Use `+` to allow all origins from root URL

### PKCE validation failed

- Ensure code_verifier is properly generated (43-128 characters)
- Verify code_challenge_method is `S256`
- Check that verifier is sent in token exchange

## Security Notes

1. **Never use client secrets in frontend code** — PKCE provides security for public clients
2. **Use HTTPS in production** — OAuth2 requires secure connections
3. **Validate tokens on backend** — The backend validates JWT signatures using JWKS
4. **Short token lifespans** — Configure reasonable access token lifespan (e.g., 5-15 minutes)
5. **Refresh tokens** — Enable refresh tokens for better UX; they're rotated automatically

## Related Files

- `frontend/src/lib/auth/keycloak.ts` — OAuth2 PKCE implementation
- `frontend/src/lib/auth/pkce.ts` — PKCE helper functions
- `src/auth/jwks.rs` — Backend JWKS validation
- `src/auth/middleware.rs` — Backend JWT middleware

## References

- [Keycloak Documentation](https://www.keycloak.org/documentation)
- [OAuth2 PKCE RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636)
- [OpenID Connect Specification](https://openid.net/connect/)
