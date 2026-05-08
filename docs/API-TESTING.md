# API Testing Guide

## Prerequisites

1. PostgreSQL running (via Docker)
2. Keycloak realm configured with custom `tenant_id` claim
3. Test user created

## Keycloak Configuration

### Step 1: Add Custom `tenant_id` Claim to Tokens

1. Go to **Realm Settings** → **Client Scopes**
2. Create new client scope: `tenant`
3. Add mapper:
   - **Name**: `tenant_id`
   - **Mapper Type**: `User Attribute`
   - **User Attribute**: `tenant_id`
   - **Token Claim Name**: `tenant_id`
   - **Claim JSON Type**: `String`
   - **Add to ID token**: ON
   - **Add to access token**: ON
4. Go to **Clients** → `schreinerei_pwa` → **Client Scopes**
5. Add `tenant` scope to **Default Client Scopes**

### Step 2: Create Test User with Tenant

1. Go to **Users** → **Add User**
   - Email: `test@schreinerei.test`
   - Email verified: ON
   - Enabled: ON
2. Go to **Attributes** tab
   - Add: `tenant_id` = `<TENANT_UUID>` (use UUID from database)
3. Go to **Credentials** tab
   - Set password: `test123`
   - Temporary: OFF
4. Go to **Role Mappings** tab
   - Assign `admin` role (create realm role first if needed)

## Database Setup

Run this to create a test tenant:

```sql
-- Connect to database
psql postgres://schreinerei:bfGkOLzqH7klHp5ApkcTUUgDX1gTlDiG@localhost:5433/schreinerei

-- Create test tenant
INSERT INTO tenants (id, keycloak_realm, name, slug)
VALUES (
    'a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d',
    'schreinerei',
    'Test Schreinerei',
    'test-schreinerei'
);

-- Note: The user will be auto-created on first login via get_or_create_from_auth
```

## Running Tests

### 1. Start the server

```bash
cargo run
```

### 2. Run test script

```bash
# Set your test user credentials
export TEST_USER=test@schreinerei.test
export TEST_PASSWORD=test123

# Run tests
./scripts/test-api.sh
```

### 3. Manual curl testing

```bash
# Get token
TOKEN=$(curl -s -X POST \
  "https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=schreinerei_pwa" \
  -d "client_secret=TG776i8yxgKVuFPakoOpQZDWMHRycYkG" \
  -d "username=test@schreinerei.test" \
  -d "password=test123" | jq -r '.access_token')

# Test health endpoint
curl http://localhost:3000/health

# Test authenticated endpoint
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/v1/auth/me | jq
```

## Available Endpoints

### IAM (requires auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/auth/me` | Get current user |
| PATCH | `/api/v1/users/me` | Update own profile |
| GET | `/api/v1/users` | List users (admin) |
| POST | `/api/v1/users/invite` | Invite user (admin) |
| PATCH | `/api/v1/users/{id}/role` | Update user role (admin) |

### Inventory (requires auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/inventory/categories` | List categories |
| POST | `/api/v1/inventory/categories` | Create category |
| GET | `/api/v1/inventory/categories/{id}` | Get category |
| GET | `/api/v1/inventory/materials` | List materials |
| POST | `/api/v1/inventory/materials` | Create material |
| GET | `/api/v1/inventory/materials/{id}` | Get material |
| POST | `/api/v1/inventory/materials/{id}/withdraw` | Withdraw stock |
| POST | `/api/v1/inventory/materials/{id}/adjust` | Adjust stock |
| POST | `/api/v1/inventory/materials/{id}/qr` | Generate QR code |
| GET | `/api/v1/inventory/materials/{id}/qr/svg` | Get QR SVG |
| GET | `/api/v1/inventory/low-stock` | List low stock items |
| GET | `/api/v1/inventory/qr/{code}` | Get material by QR |
| GET | `/api/v1/inventory/orders` | List order requests |
| POST | `/api/v1/inventory/orders` | Create order request |
| POST | `/api/v1/inventory/orders/{id}/approve` | Approve order |
| POST | `/api/v1/inventory/orders/{id}/fulfill` | Fulfill order |
