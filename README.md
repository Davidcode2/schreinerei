# Schreinerei Backend

A multi-tenant construction management system built with Rust, Axum, and PostgreSQL.

## Tech Stack

- **Backend:** Rust with Axum web framework
- **Database:** PostgreSQL with SQLx
- **Auth:** Keycloak JWT validation
- **Architecture:** Modular Monolith with DDD Bounded Contexts

## Prerequisites

- Rust 1.70+ (edition 2021)
- PostgreSQL 14+
- Keycloak instance (for production auth)

## Local Development

### 1. Clone and Setup

```bash
git clone <repo-url>
cd schreinerei
cp .env.example .env
```

### 2. Configure Environment

Create `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:5432/schreinerei
KEYCLOAK_URL=https://auth.jakob-lingel.dev
KEYCLOAK_REALM=schreinerei
JWT_ISSUER=https://auth.jakob-lingel.dev/realms/schreinerei
APP_HOST=0.0.0.0
APP_PORT=3000
RUST_LOG=debug
```

### 3. Setup Database

```bash
# Create database
createdb schreinerei
```

Migrations run automatically on startup (only unapplied migrations).

Or run manually:
```bash
cargo sqlx migrate run
```

### 4. Run Server

```bash
cargo run
```

Server starts at `http://localhost:3000`

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

Integration tests require a running PostgreSQL database. They use `sqlx::test` which creates isolated test databases.

```bash
# Ensure DATABASE_URL is set
export DATABASE_URL=postgres://user:password@localhost:5432/schreinerei

# Run all tests including integration tests
cargo test --all

# Run specific integration test file
cargo test --test tenant_isolation_test
```

### Test Database Setup

The `sqlx::test` macro automatically creates a temporary database for each test. No manual setup required beyond having PostgreSQL running.

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root with AppState
├── config.rs            # Configuration loading
├── common/              # Shared types and utilities
│   ├── types.rs         # TenantId, UserId, Role
│   ├── error.rs         # Error types
│   └── db.rs            # Database utilities
├── auth/                # Authentication
│   ├── jwt.rs           # JWT validation
│   ├── jwks.rs          # JWKS client
│   ├── extractor.rs     # Axum extractors
│   └── middleware.rs    # Auth middleware
└── modules/             # Business modules (DDD)
    └── iam/             # Identity & Access Management
        ├── domain/      # Entities, value objects
        ├── application/ # Services, use cases
        ├── infrastructure/ # Repositories
        └── api/         # HTTP routes
```

## API Endpoints

### Health

- `GET /health` - Health check

### IAM (requires auth)

- `GET /api/v1/auth/me` - Get current user
- `PATCH /api/v1/users/me` - Update own profile
- `GET /api/v1/users` - List users in tenant
- `POST /api/v1/users/invite` - Invite user (admin only)
- `PATCH /api/v1/users/:id/role` - Update user role (admin only)

### Inventory (requires auth)

- `GET /api/v1/inventory/categories` - List categories
- `POST /api/v1/inventory/categories` - Create category
- `GET /api/v1/inventory/materials` - List materials
- `POST /api/v1/inventory/materials` - Create material
- `POST /api/v1/inventory/materials/:id/withdraw` - Withdraw stock
- `POST /api/v1/inventory/materials/:id/adjust` - Adjust stock
- `POST /api/v1/inventory/materials/:id/qr` - Generate QR code
- `GET /api/v1/inventory/low-stock` - List low stock items
- `GET /api/v1/inventory/orders` - List order requests
- `POST /api/v1/inventory/orders` - Create order request
- `POST /api/v1/inventory/orders/:id/approve` - Approve order

## API Testing

```bash
# Setup test tenant
./scripts/setup-test-data.sh

# Run API tests (requires Keycloak user with tenant_id attribute)
TEST_USER=your@email.com TEST_PASSWORD=yourpassword ./scripts/test-api.sh
```

See [docs/API-TESTING.md](docs/API-TESTING.md) for detailed setup instructions.

## Database Migrations

```bash
# Create new migration
cargo sqlx migrate add <migration_name>

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Check compilation
cargo check
```

## Selective Checks And Hooks

Use the selective runner to mirror CI without paying for unrelated checks:

```bash
# Fast staged checks, intended for pre-commit
./scripts/ci-selective.sh --staged --fast

# CI-equivalent checks for everything on your branch since it diverged from main
./scripts/ci-selective.sh --branch

# Force both backend and frontend checks
./scripts/ci-selective.sh --all
```

The runner uses these scopes:

- `frontend`: `frontend/**`, `dockerfile.frontend`, `nginx.conf`
- `backend`: `src/**`, `migrations/**`, `Cargo.toml`, `Cargo.lock`, `dockerfile.backend`
- `contract`: backend files that can change `frontend/src/types/generated.ts`
- `shared`: workflow files and `.dockerignore`, which fan out to both backend and frontend checks

That means frontend-only changes skip Rust checks locally and in GitHub Actions, while backend API or ts-rs changes still force the frontend build path.

When Docker-related files change, the full runner also validates the affected image build locally:

- `dockerfile.backend` -> backend image build
- `dockerfile.frontend` or `nginx.conf` -> frontend image build
- `.dockerignore` -> both image builds

To install the versioned hooks for this worktree:

```bash
./scripts/install-git-hooks.sh
```

Installed hooks:

- `pre-commit`: runs fast staged checks only
- `pre-push`: runs CI-equivalent checks for the current branch range
