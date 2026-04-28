# Phase 1: Auth & IAM Foundation - Research

**Gathered:** 2026-04-28
**Status:** Research Complete

---

## Executive Summary

Phase 1 establishes the authentication and identity management foundation for a multi-tenant SaaS application. The system uses Keycloak for authentication, Rust (Axum) for the backend API, PostgreSQL for data storage, and deploys to an existing Kubernetes cluster with ArgoCD GitOps.

---

## Standard Stack

| Layer | Technology | Version | Rationale |
|-------|------------|---------|-----------|
| Web Framework | Axum | 0.8.x | Ergonomic, modular, Tower ecosystem integration |
| Database | PostgreSQL | 16.x | Existing infrastructure, multi-tenant capable |
| Query Builder | SQLx | 0.8.x | Compile-time checked queries, async, type-safe |
| JWT Validation | jsonwebtoken | 0.12.x | RS256 support, JWKS integration |
| Serialization | serde + serde_json | 1.x | JSON handling |
| Async Runtime | tokio | 1.x | Industry standard |
| HTTP Client | reqwest | 0.12.x | For JWKS fetching from Keycloak |
| Frontend | Vite + React + TypeScript | 5.x / 18.x / 5.x | Fast builds, PWA support |
| Auth | Keycloak | Existing instance | Multi-tenant SSO ready |

---

## Architecture Patterns

### Modular Monolith with DDD Bounded Contexts

```
src/
├── main.rs                    # Entry point, router setup
├── config.rs                  # Configuration loading
├── lib.rs                     # Library root
│
├── common/                    # Shared types and utilities
│   ├── mod.rs
│   ├── types.rs               # TenantId, Money, etc.
│   ├── error.rs               # AppError, Result types
│   └── db.rs                  # Database pool setup
│
├── auth/                      # Authentication module
│   ├── mod.rs
│   ├── jwt.rs                 # JWT validation middleware
│   ├── jwks.rs                # JWKS client for Keycloak
│   ├── extractor.rs           # Axum extractors for user/tenant
│   └── middleware.rs          # Auth middleware
│
└── modules/
    └── iam/                   # Identity & Access Management
        ├── mod.rs
        ├── domain/
        │   ├── mod.rs
        │   ├── user.rs        # User aggregate
        │   ├── role.rs        # Role value object
        │   └── tenant.rs      # Tenant aggregate
        ├── application/
        │   ├── mod.rs
        │   ├── user_service.rs
        │   └── invite_service.rs
        ├── infrastructure/
        │   ├── mod.rs
        │   ├── user_repository.rs
        │   └── tenant_repository.rs
        └── api/
            ├── mod.rs
            └── routes.rs      # REST endpoints
```

### Multi-Tenant Data Model

Every table MUST include `tenant_id` for data isolation:

```sql
-- Core tenant table (managed by Keycloak, synced to app)
CREATE TABLE tenants (
    id UUID PRIMARY KEY,
    keycloak_realm VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users synced from Keycloak
CREATE TABLE users (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    keycloak_user_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'employee',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, keycloak_user_id)
);
CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);

-- Role constants: 'admin', 'employee'
```

---

## Keycloak Integration

### JWT Token Structure (from Keycloak)

```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "preferred_username": "username",
  "realm_access": {
    "roles": ["admin", "employee"]
  },
  "tenant_id": "tenant-uuid",
  "azp": "client-id",
  "exp": 1234567890,
  "iat": 1234567890
}
```

### JWKS Validation Flow

1. Fetch JWKS from Keycloak: `GET {KEYCLOAK_URL}/realms/{realm}/protocol/openid-connect/certs`
2. Cache JWKS with periodic refresh (every 1 hour)
3. Extract `kid` from JWT header
4. Find matching key in JWKS
5. Validate signature with RS256
6. Extract claims (sub, email, roles, tenant_id)

### Axum Middleware Pattern

```rust
// JWT validation middleware
async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let token = extract_bearer_token(&req)?;
    let claims = validate_jwt(&token, &state.jwks).await?;
    
    // Inject authenticated user into request extensions
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}

// Extractor for authenticated user
struct AuthenticatedUser {
    user_id: Uuid,
    tenant_id: Uuid,
    email: String,
    roles: Vec<String>,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(|c| Self::from(c))
            .ok_or(AuthError::NotAuthenticated)
    }
}
```

---

## Security Considerations

### Multi-Tenant Isolation

**CRITICAL:** Every database query MUST be scoped to `tenant_id`.

```rust
// WRONG - leaks data between tenants
let users = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(&pool)
    .await?;

// CORRECT - scoped to tenant
let users = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE tenant_id = $1",
    tenant_id
)
.fetch_all(&pool)
.await?;
```

### Tenant Context Pattern

```rust
// Middleware extracts tenant_id from JWT
// All services receive tenant context
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub roles: Vec<Role>,
}

// Repository methods enforce tenant isolation
impl UserRepository {
    pub async fn find_by_id(
        &self,
        id: Uuid,
        ctx: &TenantContext,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1 AND tenant_id = $2",
            id,
            ctx.tenant_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
```

### STRIDE Threat Analysis

| Threat ID | Category | Component | Disposition | Mitigation |
|-----------|----------|-----------|-------------|------------|
| T-01-01 | Spoofing | JWT validation | mitigate | Validate RS256 signature via JWKS, check `exp`, `iat` |
| T-01-02 | Tampering | API requests | mitigate | Require tenant_id in all queries, use parameterized queries |
| T-01-03 | Info Disclosure | Error messages | mitigate | Never expose internal details in API errors |
| T-01-04 | Elevation | Role checks | mitigate | Enforce role-based access at API layer |
| T-01-05 | Repudiation | Audit trail | accept | V1 does not require audit logging |

---

## Don't Hand-Roll

### Use Established Crates

- **jsonwebtoken** - JWT validation (don't implement crypto)
- **uuid** - UUID generation and parsing
- **chrono** - DateTime handling
- **thiserror** - Error types
- **tower-http** - CORS, request tracing
- **tracing** - Structured logging
- **config** / **dotenvy** - Configuration management

### Avoid Common Pitfalls

1. **Don't validate JWTs with HMAC (HS256)** - Use RS256 with Keycloak's public keys
2. **Don't trust `tenant_id` from request body** - Extract from JWT only
3. **Don't skip tenant scoping** - Every query, even "get by ID"
4. **Don't use `SELECT *`** - Explicit columns prevent accidental data leaks
5. **Don't store secrets in code** - Use External Secrets Operator

---

## API Endpoints (Phase 1)

### Public Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| GET | /api/v1/auth/me | Get current user profile |

### Protected Endpoints (Require Authentication)

| Method | Path | Description | Roles |
|--------|------|-------------|-------|
| GET | /api/v1/users | List users in tenant | admin |
| POST | /api/v1/users/invite | Invite new user | admin |
| PATCH | /api/v1/users/{id}/role | Update user role | admin |
| GET | /api/v1/users/{id} | Get user details | admin, self |
| PATCH | /api/v1/users/me | Update own profile | any |
| GET | /api/v1/tenant | Get tenant info | any |

---

## Kubernetes Deployment

### Existing Infrastructure

- **Cluster:** K3s (3 nodes, HA)
- **GitOps:** ArgoCD
- **Ingress:** nginx with cert-manager
- **Secrets:** External Secrets Operator → AWS Parameter Store

### Deployment Artifacts

```
schreinerei/
├── deployment.yaml           # App deployment
├── service.yaml              # ClusterIP service
├── ingress.yaml              # Ingress with TLS
├── configmap.yaml            # Non-secret config
├── external-secret.yaml      # Database URL, Keycloak config
└── namespace.yaml            # schreinerei namespace
```

### Environment Variables

| Variable | Source | Description |
|----------|--------|-------------|
| DATABASE_URL | ExternalSecret | PostgreSQL connection |
| KEYCLOAK_URL | ConfigMap | Keycloak base URL |
| KEYCLOAK_REALM | ConfigMap | Default realm |
| JWT_ISSUER | ConfigMap | `{KEYCLOAK_URL}/realms/{REALM}` |

---

## Common Pitfalls

### Multi-Tenant Gotchas

1. **Forgot tenant_id in WHERE clause** → Use SQLx compile-time checks
2. **Cross-tenant user enumeration** → Don't reveal if email exists in another tenant
3. **Admin invites to wrong tenant** → Validate tenant context before invite

### Keycloak Integration

1. **Stale JWKS cache** → Refresh every hour, handle key rotation
2. **Wrong issuer validation** → Use exact issuer from Keycloak
3. **Missing tenant_id in token** → Configure Keycloak mapper

### Rust Backend

1. **Blocking database calls** → Always use async SQLx methods
2. **Missing error handling** → Use `thiserror` for typed errors
3. **Large response bodies** → Paginate list endpoints

---

## Validation Architecture

### Test Categories

| Category | Tool | Scope |
|----------|------|-------|
| Unit Tests | cargo test | Domain logic, validators |
| Integration Tests | cargo test + test containers | API endpoints, database |
| E2E Tests | Playwright | Login flow, tenant isolation |

### Key Test Cases

1. **JWT Validation**
   - Valid token → 200 OK
   - Expired token → 401 Unauthorized
   - Invalid signature → 401 Unauthorized
   - Missing tenant_id → 401 Unauthorized

2. **Multi-Tenant Isolation**
   - User A cannot access User B's data (different tenant)
   - Admin in Tenant A cannot invite to Tenant B
   - All queries respect tenant_id boundary

3. **Role-Based Access**
   - Employee cannot access admin endpoints
   - Admin can manage users in their tenant only

---

## Out of Scope

- **Password reset flow** - Handled by Keycloak
- **Email verification** - Handled by Keycloak
- **MFA setup** - Handled by Keycloak
- **Session management** - Keycloak handles sessions
- **User registration** - Admin invite only in V1

---

## References

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [jsonwebtoken crate](https://docs.rs/jsonwebtoken)
- [Keycloak OIDC](https://www.keycloak.org/docs/latest/server_admin/#oidc)

---

*Research completed: 2026-04-28*
