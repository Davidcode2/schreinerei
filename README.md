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
KEYCLOAK_URL=https://your-keycloak.example.com
KEYCLOAK_REALM=your-realm
APP_HOST=0.0.0.0
APP_PORT=3000
RUST_LOG=debug
```

### 3. Setup Database

```bash
# Create database
createdb schreinerei

# Run migrations
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

### Users (requires auth)

- `GET /api/v1/users` - List users in tenant
- `POST /api/v1/users` - Invite user (admin only)
- `PATCH /api/v1/users/:id/role` - Update user role (admin only)

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
