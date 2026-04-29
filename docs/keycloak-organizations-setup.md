# Keycloak Organizations Setup Guide

## Quick Start for Schreinerei Realm

**Prerequisite:** Organizations feature is already enabled in your Keycloak realm.

### Steps to Complete Migration

1. **Add Organization Scope to Client**
   - Go to **Clients** → `schreinerei-pwa` → **Client Scopes** tab
   - Click **Add client scope** → Select **organization** → Add

2. **Create Organizations for Existing Tenants**
   - Go to **Organizations** menu (left sidebar)
   - For each tenant in your database:
     - Click **Create organization**
     - Name: Tenant name from database
     - Alias: URL-friendly slug
     - Domains: Associated email domain (optional)
   - Note: Use the tenant UUID from database as reference

3. **Add Users as Organization Members**
   - Go to **Organizations** → Select organization → **Members** tab
   - Click **Add member** → **Add realm user**
   - Select users belonging to this tenant

4. **Verify Token Contains Organization Claim**
   - Log in and decode your JWT token
   - Check for `organization` claim with organization ID

5. **Run Database Migration**
   ```bash
   sqlx migrate run
   ```

---

## Overview

This guide explains how to migrate from **attribute-based tenancy** to **organization-based tenancy** in Keycloak for the Schreinerei SaaS application.

### Current State (Attribute-Based)

- `tenant_id` is stored as a custom user attribute in Keycloak
- JWT tokens contain a custom `tenant_id` claim via a user attribute mapper
- Tenant isolation is enforced at the application layer using `TenantId` from JWT

### Target State (Organization-Based)

- Each tenant is represented as a Keycloak **Organization**
- Users are members of organizations
- JWT tokens contain `organization` claim with organization membership
- Built-in Keycloak organization management (invitations, identity providers, domains)

---

## Prerequisites

### Keycloak Version

Organizations feature requires **Keycloak 26.x** or later. Verify your version:

```bash
curl -s http://localhost:8080/realms/master/.well-known/openid-configuration | jq '.issuer'
```

### Enable Organizations in Realm

1. Go to **Realm Settings** → **General**
2. Enable **Organizations** flag
3. Save

Or via Admin CLI:

```bash
kcadm.sh update realms/schreinerei -s organizationsEnabled=true
```

---

## Architecture Comparison

### Attribute-Based Tenancy (Current)

```
┌─────────────────────────────────────────────────────────────┐
│  Keycloak Realm: schreinerei                                │
│                                                             │
│  Users:                                                     │
│  ├── user1 (attributes: {tenant_id: "uuid-aaa"})            │
│  ├── user2 (attributes: {tenant_id: "uuid-aaa"})            │
│  └── user3 (attributes: {tenant_id: "uuid-bbb"})            │
│                                                             │
│  JWT Token:                                                 │
│  {                                                           │
│    "sub": "user-id",                                         │
│    "tenant_id": "uuid-aaa",  // custom claim                 │
│    "realm_access": { "roles": [...] }                        │
│  }                                                           │
└─────────────────────────────────────────────────────────────┘
```

### Organization-Based Tenancy (Target)

```
┌─────────────────────────────────────────────────────────────┐
│  Keycloak Realm: schreinerei                                │
│                                                             │
│  Organizations:                                             │
│  ├── Schreinerei Müller (id: uuid-aaa, domain: mueller.de)  │
│  │   └── Members: user1, user2                              │
│  └── Schreinerei Schmidt (id: uuid-bbb, domain: schmidt.de) │
│      └── Members: user3                                     │
│                                                             │
│  JWT Token:                                                 │
│  {                                                           │
│    "sub": "user-id",                                         │
│    "organization": {                                         │
│      "uuid-aaa": {}  // org ID as key                       │
│    },                                                        │
│    "realm_access": { "roles": [...] }                        │
│  }                                                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Migration Steps

### Step 1: Create Organizations for Existing Tenants

For each existing tenant in the database, create a corresponding Keycloak organization.

#### Via Admin Console

1. Navigate to **Organizations** in the left menu
2. Click **Create organization**
3. Fill in:
   - **Name**: Tenant name (e.g., "Schreinerei Müller")
   - **Alias**: URL-friendly identifier (e.g., "mueller")
   - **Domains**: Associated email domains (e.g., "mueller.de")
   - **Enabled**: Yes

#### Via REST API

```bash
# Get admin token
ADMIN_TOKEN=$(curl -s -X POST \
  "http://localhost:8080/realms/master/protocol/openid-connect/token" \
  -d "client_id=admin-cli" \
  -d "username=admin" \
  -d "password=admin" \
  -d "grant_type=password" | jq -r '.access_token')

# Create organization
curl -X POST \
  "http://localhost:8080/admin/realms/schreinerei/orgs" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Schreinerei Müller",
    "alias": "mueller",
    "enabled": true,
    "domains": [
      {"name": "mueller.de", "verified": false}
    ]
  }'

# Response includes the organization ID
# {"id": "cec54914-b702-4c7b-9431-b407817d059a", ...}
```

#### Via Terraform

```hcl
resource "keycloak_organization" "tenants" {
  for_each = {
    mueller = {
      name    = "Schreinerei Müller"
      domains = ["mueller.de"]
    }
    schmidt = {
      name    = "Schreinerei Schmidt"
      domains = ["schmidt.de"]
    }
  }

  realm   = "schreinerei"
  name    = each.value.name
  alias   = each.key
  enabled = true

  domain {
    name = each.value.domains[0]
  }
}
```

### Step 2: Migrate Users to Organizations

Add existing users as members of their corresponding organizations.

#### Via Admin Console

1. Go to **Organizations** → Select organization
2. Click **Members** tab
3. Click **Add member** → **Add realm user**
4. Select users to add

#### Via REST API

```bash
# Get organization ID
ORG_ID=$(curl -s \
  "http://localhost:8080/admin/realms/schreinerei/orgs?search=mueller" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[0].id')

# Add user to organization
curl -X POST \
  "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/members" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "userId": "user-uuid-here"
  }'
```

### Step 3: Map Organization Claim to Token

Configure the client to include the `organization` scope.

1. Go to **Clients** → `schreinerei-pwa` → **Client Scopes**
2. Add **organization** scope (built-in)
3. The `organization` claim will now be included in tokens

When a user is a member of an organization, the token will contain:

```json
{
  "organization": {
    "org-uuid-here": {}
  }
}
```

### Step 4: Update Application Code

#### Update JWT Claims Structure

**Before (attribute-based):**

```rust
// src/auth/jwt.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub tenant_id: String,  // ← Custom attribute claim
    pub realm_access: RealmAccess,
}
```

**After (organization-based):**

```rust
// src/auth/jwt.rs
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub organization: HashMap<String, serde_json::Value>,  // ← Organization claim
    pub realm_access: RealmAccess,
}

impl Claims {
    /// Get the first organization ID (single-org users)
    pub fn organization_id(&self) -> Option<String> {
        self.organization.keys().next().cloned()
    }
}
```

#### Update AuthenticatedUser Extractor

```rust
// src/auth/extractor.rs
impl AuthenticatedUser {
    pub fn from_claims(claims: &crate::auth::jwt::Claims) -> Result<Self, AppError> {
        let user_id = Uuid::parse_str(&claims.sub)
            .map(UserId)
            .map_err(|e| AppError::Auth(format!("Invalid user ID: {}", e)))?;

        // Get tenant_id from organization claim
        let org_id = claims.organization_id()
            .ok_or_else(|| AppError::Auth("No organization membership".to_string()))?;
        
        let tenant_id = Uuid::parse_str(&org_id)
            .map(TenantId)
            .map_err(|e| AppError::Auth(format!("Invalid org ID: {}", e)))?;

        // ... rest of implementation
    }
}
```

### Step 5: Remove Attribute-Based Configuration

After migration is complete:

1. Remove the `tenant_id` user attribute mapper:
   - Go to **Clients** → `schreinerei-pwa` → **Mappers**
   - Delete the `tenant_id` mapper

2. Remove `tenant_id` attributes from users (optional cleanup):
```bash
# Remove tenant_id attribute from all users
kcadm.sh get users -r schreinerei | jq -r '.[].id' | while read USER_ID; do
  kcadm.sh update users/$USER_ID -r schreinerei -s 'attributes=null'
done
```

---

## Organization Management

### Creating New Organizations (Self-Service)

For public website with self-service registration:

1. User registers on landing page
2. Backend creates Keycloak organization via Admin API
3. User is added as org admin
4. User can invite team members

```rust
// Example: Create organization on signup
async fn create_organization(
    keycloak_admin: &KeycloakAdmin,
    name: &str,
    admin_user_id: &str,
) -> Result<String, AppError> {
    let org = OrganizationRepresentation {
        name: Some(name.to_string()),
        alias: Some(name.to_lowercase().replace(" ", "-")),
        enabled: Some(true),
        ..Default::default()
    };
    
    let org_id = keycloak_admin
        .create_organization("schreinerei", org)
        .await?;
    
    // Add creating user as org admin
    keycloak_admin
        .add_organization_member("schreinerei", &org_id, admin_user_id)
        .await?;
    
    Ok(org_id)
}
```

### Inviting Members

#### Via Admin Console

1. Go to **Organizations** → Select organization
2. Click **Members** → **Add member** → **Invite member**
3. Enter email address
4. User receives invitation email with registration link

#### Via REST API

```bash
# Send invitation
curl -X POST \
  "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/members/invite" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@example.com",
    "firstName": "Max",
    "lastName": "Mustermann"
  }'
```

### Managing Invitations

```bash
# List pending invitations
curl -s "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/invitations?status=pending" \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Resend invitation
curl -X POST \
  "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/invitations/$INVITATION_ID/resend" \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Delete/cancel invitation
curl -X DELETE \
  "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/invitations/$INVITATION_ID" \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

---

## Identity Provider Integration

Organizations can have their own identity providers for SSO.

### Linking an Identity Provider to Organization

```bash
# Create OIDC identity provider for organization
curl -X POST \
  "http://localhost:8080/admin/realms/schreinerei/identity-provider/instances" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "alias": "mueller-idp",
    "providerId": "oidc",
    "enabled": true,
    "config": {
      "clientId": "schreinerei-app",
      "clientSecret": "secret",
      "authorizationUrl": "https://idp.mueller.de/auth",
      "tokenUrl": "https://idp.mueller.de/token"
    },
    "organizationId": "'$ORG_ID'",
    "orgDomain": "mueller.de",
    "orgRedirectModeEmailMatches": true
  }'
```

### Automatic Redirect by Email Domain

When `orgRedirectModeEmailMatches` is enabled:
1. User enters email with `@mueller.de` domain
2. Keycloak automatically redirects to the organization's identity provider
3. After authentication, user is automatically added to the organization

---

## Token Claims Reference

### Organization Claim Format

```json
{
  "organization": {
    "cec54914-b702-4c7b-9431-b407817d059a": {}
  }
}
```

For multi-org users:

```json
{
  "organization": {
    "org-uuid-1": {},
    "org-uuid-2": {}
  }
}
```

### Requesting Organization Scope

In the frontend, request the `organization` scope:

```typescript
// OAuth2 authorization URL
const authUrl = new URL(`${keycloakUrl}/protocol/openid-connect/auth`);
authUrl.searchParams.set('client_id', 'schreinerei-pwa');
authUrl.searchParams.set('redirect_uri', redirectUri);
authUrl.searchParams.set('response_type', 'code');
authUrl.searchParams.set('scope', 'openid profile email organization'); // ← Include organization
```

---

## Database Migration

### Update Tenant Table

```sql
-- Add keycloak_organization_id column
ALTER TABLE tenants ADD COLUMN keycloak_organization_id UUID UNIQUE;

-- Populate from existing data (run after creating organizations in Keycloak)
-- You'll need to map the Keycloak org IDs to your tenant records

-- Optional: Remove keycloak_realm column (no longer needed)
-- ALTER TABLE tenants DROP COLUMN keycloak_realm;
```

---

## API Endpoints

### Organization Management API

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/admin/realms/{realm}/orgs` | GET | List organizations |
| `/admin/realms/{realm}/orgs` | POST | Create organization |
| `/admin/realms/{realm}/orgs/{orgId}` | GET | Get organization |
| `/admin/realms/{realm}/orgs/{orgId}` | PUT | Update organization |
| `/admin/realms/{realm}/orgs/{orgId}` | DELETE | Delete organization |
| `/admin/realms/{realm}/orgs/{orgId}/members` | GET | List members |
| `/admin/realms/{realm}/orgs/{orgId}/members` | POST | Add member |
| `/admin/realms/{realm}/orgs/{orgId}/members/{userId}` | DELETE | Remove member |
| `/admin/realms/{realm}/orgs/{orgId}/members/invite` | POST | Invite member |
| `/admin/realms/{realm}/orgs/{orgId}/invitations` | GET | List invitations |
| `/admin/realms/{realm}/orgs/{orgId}/invitations/{id}/resend` | POST | Resend invitation |
| `/admin/realms/{realm}/orgs/{orgId}/invitations/{id}` | DELETE | Delete invitation |

---

## Frontend Changes

### Update Auth Token Handling

```typescript
// Parse organization from token
interface TokenClaims {
  sub: string;
  email: string;
  organization: Record<string, unknown>;
  realm_access: { roles: string[] };
}

function getOrganizationId(claims: TokenClaims): string | null {
  const orgIds = Object.keys(claims.organization);
  return orgIds.length > 0 ? orgIds[0] : null;
}
```

### Organization Selection (Multi-Org Users)

If users can belong to multiple organizations:

```typescript
// Store selected organization in localStorage
const selectedOrgId = localStorage.getItem('selectedOrganization') || getOrganizationId(claims);

// Include in API requests
const apiClient = axios.create({
  headers: {
    'X-Organization-Id': selectedOrgId
  }
});
```

---

## Testing

### Verify Organization Setup

```bash
# List all organizations
curl -s "http://localhost:8080/admin/realms/schreinerei/orgs" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq

# Check organization members
curl -s "http://localhost:8080/admin/realms/schreinerei/orgs/$ORG_ID/members" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq

# Get user's token and verify organization claim
USER_TOKEN=$(curl -s -X POST \
  "http://localhost:8080/realms/schreinerei/protocol/openid-connect/token" \
  -d "client_id=schreinerei-pwa" \
  -d "username=test@example.com" \
  -d "password=password" \
  -d "grant_type=password" \
  -d "scope=openid profile email organization" | jq -r '.access_token')

# Decode and inspect
echo $USER_TOKEN | cut -d'.' -f2 | base64 -d | jq '.organization'
```

---

## Troubleshooting

### Organization Claim Not in Token

1. Verify user is a member of an organization
2. Check that `organization` scope is requested
3. Ensure organizations are enabled in realm settings

### Users Not Auto-Added to Organization

1. Check identity provider configuration
2. Verify `orgDomain` matches user's email domain
3. Ensure `orgRedirectModeEmailMatches` is set

### Invitation Email Not Sent

1. Check email configuration in realm settings
2. Verify SMTP settings are correct
3. Check server logs for email errors

---

## References

- [Keycloak Organizations Documentation](https://www.keycloak.org/docs/latest/server_admin/index.html#organizations)
- [Keycloak 26 Release Notes](https://www.keycloak.org/docs/latest/release_notes/index.html#keycloak-26-0-0)
- [Keycloak Admin REST API](https://www.keycloak.org/docs-api/latest/rest-api/index.html)

---

*Last updated: 2026-04-29*
